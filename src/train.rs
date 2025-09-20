use burn::{
    nn::loss::{MseLoss, Reduction},
    optim::{AdamConfig, GradientsParams, Optimizer},
    prelude::*,
    tensor::{backend::AutodiffBackend, cast::ToElement},
};
use rand::Rng;

use crate::{
    game::{Control, GameState, Observation, StepResult},
    model::{DQN, DQNConfig},
    replay_buffer::{BatchTensors, Experience, ReplayBuffer},
};

static EPS_START: f32 = 1.0;
static EPS_MIN: f32 = 0.05;
static EPS_DECAY: f32 = 0.995;

pub fn get_epsilon(steps_done: usize) -> f32 {
    EPS_MIN.max(EPS_START * EPS_DECAY.powi(steps_done as i32))
}

pub fn select_action<B: Backend>(
    observation: Observation,
    model: &DQN<B>,
    epsilon: f32,
    n_actions: usize,
    device: &B::Device,
) -> usize {
    let mut rng = rand::rng();
    let random: f32 = rng.random();
    if random < epsilon {
        return rng.random_range(0..n_actions);
    } else {
        let observation = observation.normalize();
        let obs_tensor = Tensor::<B, 1, Float>::from_floats(observation, device).unsqueeze_dim(0);
        let all_qvalues: Tensor<B, 1, Float> = model.forward(obs_tensor).squeeze(0);
        all_qvalues.clone().argmax(0).into_scalar().to_usize()
    }
}

#[derive(Config)]
pub struct TrainingConfig {
    pub optimizer: AdamConfig,
    pub gamma: f32,
    pub batch_size: usize,
    pub learning_rate: f64,
    pub num_episodes: usize,
    pub max_iters: usize,
}

pub fn train_step<B: AutodiffBackend>(
    policy_net: DQN<B>,
    target_net: &DQN<B>,
    buffer: &mut ReplayBuffer,
    device: &B::Device,
    config: &TrainingConfig,
) -> DQN<B> {
    let mut policy_net = policy_net;

    let BatchTensors {
        states,
        actions,
        rewards,
        next_states,
        is_dones,
    } = buffer.sample_batch_tensors::<B>(config.batch_size, device);

    let mut optimizer = config.optimizer.init();

    let q_values = policy_net
        .forward(states)
        .gather(1, actions.unsqueeze_dim(1));
    let next_q_values: Tensor<B, 1, Float> = target_net.forward(next_states).max_dim(1).squeeze(1);
    let future_q: Tensor<B, 1, Float> = (1.0 - is_dones.float()) * config.gamma;
    let future_q: Tensor<B, 1, Float> = next_q_values.mul(future_q);
    let expected_q: Tensor<B, 2, Float> = (rewards + future_q).unsqueeze_dim(1);

    assert_eq!(
        q_values.shape().dims,
        expected_q.shape().dims,
        "q_values and expected_q values differ in shape {:?} != {:?}",
        q_values.shape().dims,
        expected_q.shape().dims,
    );

    let loss = MseLoss::new().forward(q_values, expected_q, Reduction::Mean);
    let grads = loss.backward();
    let grads = GradientsParams::from_grads(grads, &policy_net);

    policy_net = optimizer.step(config.learning_rate, policy_net, grads);
    policy_net
}

static NUM_ACTIONS: usize = 24; // TODO: this probably shouldn't be written right here
static TRAIN_START: usize = 200;
static TARGET_UPDATE: usize = 250;
static INPUT_SIZE: usize = 23;
static OUTPUT_SIZE: usize = 24;
static MEMORY_SIZE: usize = 100_000;

pub fn train<B: AutodiffBackend>(device: &B::Device, config: TrainingConfig) -> (DQN<B>, DQN<B>) {
    let mut env = GameState::new();

    let mut policy_net0: DQN<B> = DQNConfig::new(INPUT_SIZE, OUTPUT_SIZE).init(device);
    let mut policy_net1: DQN<B> = DQNConfig::new(INPUT_SIZE, OUTPUT_SIZE).init(device);

    let mut target_net0 = DQNConfig::new(INPUT_SIZE, OUTPUT_SIZE).init(device);
    let mut replay_buffer0 = ReplayBuffer::new(MEMORY_SIZE);
    let mut steps_done0 = 0;
    let mut all_rewards0: Vec<f32> = vec![];

    let mut target_net1 = DQNConfig::new(INPUT_SIZE, OUTPUT_SIZE).init(device);
    let mut replay_buffer1 = ReplayBuffer::new(MEMORY_SIZE);
    let mut steps_done1 = 0;
    let mut all_rewards1: Vec<f32> = vec![];

    let mut best_avg0 = -1000;
    let mut best_avg1 = -1000;

    let mut iters = 0;

    for episode in 0..config.num_episodes {
        println!("Beginning episode {episode}");
        env = GameState::new();

        let mut p0_obs = env.get_observation(0);
        let mut p1_obs = env.get_observation(1);

        let mut total_reward0 = 0.0;
        let mut total_reward1 = 0.0;

        let mut is_episode_done = false;

        while !is_episode_done {
            let epsilon0 = get_epsilon(steps_done0);
            let epsilon1 = get_epsilon(steps_done1);

            let action0 = select_action(p0_obs, &policy_net0, epsilon0, NUM_ACTIONS, device);
            let action1 = select_action(p1_obs, &policy_net1, epsilon1, NUM_ACTIONS, device);

            let StepResult {
                observations,
                rewards,
                is_done,
            } = env.step([Control::from_int(action0), Control::from_int(action1)]);

            let p0_obs_next = observations[0];
            let p1_obs_next = observations[1];

            replay_buffer0.push(Experience {
                state: p0_obs,
                action: action0,
                reward: rewards[0],
                next_state: p0_obs_next,
                is_done,
            });

            replay_buffer1.push(Experience {
                state: p1_obs,
                action: action1,
                reward: rewards[1],
                next_state: p1_obs_next,
                is_done,
            });

            total_reward0 += rewards[0];
            total_reward1 += rewards[1];

            p0_obs = p0_obs_next;
            p1_obs = p1_obs_next;

            steps_done0 += 1;
            steps_done1 += 1;

            if steps_done0 > TRAIN_START {
                policy_net0 = train_step(
                    policy_net0,
                    &target_net0,
                    &mut replay_buffer0,
                    device,
                    &config,
                );
            }
            if steps_done1 > TRAIN_START {
                policy_net1 = train_step(
                    policy_net1,
                    &target_net1,
                    &mut replay_buffer1,
                    device,
                    &config,
                );
            }

            if steps_done0 % TARGET_UPDATE == 0 {
                target_net0 = policy_net0.clone();
            }
            if steps_done0 % TARGET_UPDATE == 0 {
                target_net1 = policy_net1.clone();
            }

            is_episode_done = is_done;

            iters += 1;
            if iters % 100 == 0 {
                println!(
                    "   Running iter {iters } Reward 0:{} Reward 1:{} num_punches: {:?} num_landed_punches: {:?}",
                    total_reward0, total_reward1, env.num_punches, env.num_landed_punches
                )
            }

            if iters > config.max_iters {
                break;
            }
        }

        all_rewards0.push(total_reward0);
        all_rewards1.push(total_reward1);

        // TODO: avg0 mean of the last 20 in all_rewards0;
        // TODO: avg1 mean of the last 20 in all_rewards1;

        println!("Finishing episode {episode}");
    }

    (policy_net0, policy_net1)
}
