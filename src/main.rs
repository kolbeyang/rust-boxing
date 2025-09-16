use boxing::game::{Control, GameState, StepResult};
use burn::{
    backend::{Autodiff, Wgpu},
    nn::{
        Linear, LinearConfig, Relu,
        loss::{MseLoss, Reduction},
    },
    prelude::*,
    tensor::{Distribution, backend::AutodiffBackend},
    train::{RegressionOutput, TrainOutput, TrainStep},
};

mod game;

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    l0: Linear<B>,
    l1: Linear<B>,
    l2: Linear<B>,
    out: Linear<B>,
    activation: Relu,
}

impl<B: Backend> Model<B> {
    // Input: [batch_size, length of observation]
    // Output: [batch_size, output controls]
    pub fn forward(&self, observation: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.l0.forward(observation);
        let x = self.l1.forward(x);
        let x = self.l2.forward(x);
        let x = self.out.forward(x);
        self.activation.forward(x)
    }

    pub fn forward_regression(
        &self,
        observations: Tensor<B, 2>,
        actions_taken: Tensor<B, 1, Int>, // indices of the actions taken
        actions_taken_qs: Tensor<B, 1>,   // target Q value for actions taken
    ) -> RegressionOutput<B> {
        let batch_size = observations.shape().dims[0];

        let predicted_qs = self.forward(observations);
        let actions_taken_predicted_qs =
            predicted_qs.gather(1, actions_taken.reshape([batch_size, 1]));

        let actions_taken_qs_2d = actions_taken_qs.reshape([batch_size, 1]);

        let loss = MseLoss::new().forward(
            actions_taken_predicted_qs.clone(),
            actions_taken_qs_2d.clone(),
            Reduction::Mean,
        );

        RegressionOutput::new(loss, actions_taken_predicted_qs, actions_taken_qs_2d)
    }
}

#[derive(Clone, Debug)]
pub struct Experience<B: Backend> {
    pub state: Tensor<B, 1>,
    pub action: usize, // Action taken
    pub reward: f32,
    pub next_state: Tensor<B, 1>,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct Batch<B: Backend> {
    pub observations: Tensor<B, 2>,
    pub actions_taken: Tensor<B, 1, Int>,
    pub actions_taken_qs: Tensor<B, 1>,
}

impl<B: AutodiffBackend> TrainStep<Batch<B>, RegressionOutput<B>> for Model<B> {
    fn step(&self, batch: Batch<B>) -> TrainOutput<RegressionOutput<B>> {
        let item = self.forward_regression(
            batch.observations,
            batch.actions_taken,
            batch.actions_taken_qs,
        );

        TrainOutput::new(self, item.loss.backward(), item)
    }
}

#[derive(Config, Debug)]
pub struct ModelConfig {
    num_input: usize,
    num_output: usize,
}

impl ModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Model<B> {
        Model {
            l0: LinearConfig::new(self.num_input, 256).init(device),
            l1: LinearConfig::new(256, 256).init(device),
            l2: LinearConfig::new(256, 128).init(device),
            out: LinearConfig::new(128, self.num_output).init(device),
            activation: Relu::new(),
        }
    }
}

fn select_epsilon_greedy<B: Backend>(
    q_values: Tensor<B, 2>,
    epsilon: f32,
    device: &B::Device,
) -> Tensor<B, 1, Int> {
    // NOTE: number of players in this case
    let num_values = q_values.dims()[0];
    let random_indices = Tensor::<B, 1, Int>::random(
        [num_values],
        Distribution::Uniform(0.0, num_values as f64),
        &device,
    );
    let dice_rolls =
        Tensor::<B, 1, Float>::random([num_values], Distribution::Uniform(0.0, 1.0), &device);
    let max_indices = q_values.argmax(0).squeeze(1);
    let mask = dice_rolls.lower_elem(epsilon);
    max_indices.mask_where(mask, random_indices)
}

fn run_game<B: Backend>(device: &B::Device) -> Vec<Experience<B>> {
    let mut experiences: Vec<Experience<B>> = vec![];

    let mut game = GameState::new();
    let mut is_done = false;

    let mut player_0_observation: Tensor<B, 1> =
        Tensor::from_floats(game.get_observation(0).normalize().as_slice(), device);
    let mut player_1_observation: Tensor<B, 1> =
        Tensor::from_floats(game.get_observation(1).normalize().as_slice(), device);

    while !is_done {
        let qs: Tensor<B, 2> = Tensor::stack(
            vec![player_0_observation.clone(), player_1_observation.clone()],
            0,
        );

        let controls_vec = select_epsilon_greedy(qs, 0.01, device)
            .into_data()
            .into_vec::<i64>()
            .expect("Unable to convert actions into vec");

        let controls: [Control; 2] = controls_vec
            .iter()
            .map(|&x| Control::from_int(x as usize))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let StepResult {
            observations,
            rewards,
            is_done: next_is_done,
        } = game.step(controls);

        let player_0_next_observation =
            Tensor::from_floats(observations[0].normalize().as_slice(), device);
        let player_1_next_observation =
            Tensor::from_floats(observations[1].normalize().as_slice(), device);

        let player_0_experience = Experience {
            state: player_0_observation,
            action: controls_vec[0] as usize,
            reward: rewards[0],
            done: next_is_done,
            next_state: player_0_next_observation.clone(),
        };
        let player_1_experience = Experience {
            state: player_1_observation,
            action: controls_vec[1] as usize,
            reward: rewards[1],
            done: next_is_done,
            next_state: player_1_next_observation.clone(),
        };

        experiences.push(player_0_experience);
        experiences.push(player_1_experience);

        player_0_observation = player_0_next_observation;
        player_1_observation = player_1_next_observation;

        is_done = next_is_done
    }

    experiences
}

fn main() {
    type MyBackend = Wgpu<f32, i32>;

    let device = burn::backend::wgpu::WgpuDevice::default();

    let experiences = run_game::<MyBackend>(&device);
}
