use core::{
    Control, GameState, OBSERVATION_LENGTH, Observation, StepResult,
    model::{DQN, DQNConfig},
};

use burn::{
    nn::loss::{MseLoss, Reduction},
    optim::{AdamConfig, GradientsParams, Optimizer},
    prelude::*,
    tensor::{backend::AutodiffBackend, cast::ToElement},
};
use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::replay_buffer::{BatchTensors, Experience, ReplayBuffer};

static EPS_MIN: f32 = 0.005;

pub fn get_epsilon(steps_done: usize, epsilon_start: f32, decay: f32) -> f32 {
    //EPS_MIN.max(EPS_START * EPS_DECAY.powi(steps_done as i32))
    EPS_MIN + (epsilon_start - EPS_MIN) * (-decay * steps_done as f32).exp()
}

pub fn select_action<B: Backend, R: Rng>(
    observation: Observation,
    model: &DQN<B>,
    epsilon: f32,
    n_actions: usize,
    rng: &mut R,
    device: &B::Device,
) -> usize {
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

#[derive(serde::Serialize)]
pub struct TrainingConfig {
    #[serde(skip)]
    pub optimizer: AdamConfig,
    pub gamma: f32,
    pub batch_size: usize,
    pub learning_rate: f64,
    pub num_episodes: usize,
    pub max_iters: usize,
    pub epsilon_decay: f32,
    pub epsilon_start: f32,
    pub seed: u64,
    pub iters_per_training_step: usize,
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
static OUTPUT_SIZE: usize = 24;
static MEMORY_SIZE: usize = 100_000;

pub fn train<B: AutodiffBackend>(device: &B::Device, config: &TrainingConfig) -> (DQN<B>, DQN<B>) {
    let mut env = GameState::new();
    let mut rng = StdRng::seed_from_u64(config.seed);

    let mut policy_net0: DQN<B> = DQNConfig::new(OBSERVATION_LENGTH, OUTPUT_SIZE).init(device);
    let mut policy_net1: DQN<B> = DQNConfig::new(OBSERVATION_LENGTH, OUTPUT_SIZE).init(device);

    let mut target_net0 = DQNConfig::new(OBSERVATION_LENGTH, OUTPUT_SIZE).init(device);
    let mut replay_buffer0 = ReplayBuffer::new(MEMORY_SIZE);
    let mut steps_done0 = 0;

    let mut target_net1 = DQNConfig::new(OBSERVATION_LENGTH, OUTPUT_SIZE).init(device);
    let mut replay_buffer1 = ReplayBuffer::new(MEMORY_SIZE);
    let mut steps_done1 = 0;

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
            let epsilon = get_epsilon(steps_done0, config.epsilon_start, config.epsilon_decay);

            let action0 =
                select_action(p0_obs, &policy_net0, epsilon, NUM_ACTIONS, &mut rng, device);
            let action1 =
                select_action(p1_obs, &policy_net1, epsilon, NUM_ACTIONS, &mut rng, device);

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

            if steps_done0 > TRAIN_START && steps_done0 % config.iters_per_training_step == 0 {
                policy_net0 = train_step(
                    policy_net0,
                    &target_net0,
                    &mut replay_buffer0,
                    device,
                    &config,
                );
            }
            if steps_done1 > TRAIN_START && steps_done1 % config.iters_per_training_step == 0 {
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
            if steps_done1 % TARGET_UPDATE == 0 {
                target_net1 = policy_net1.clone();
            }

            is_episode_done = is_done;
            if is_episode_done {
                println!(
                    "->> Episode finished with final health 0:{} 1:{}",
                    env.players[0].health, env.players[1].health,
                );
            }

            iters += 1;
            if iters % 1000 == 0 {
                println!(
                    "   Running iter {iters } Reward 0:{:.2} Reward 1:{:.2} num_punches: {:?} num_landed_punches: {:?} epsilon {}",
                    total_reward0, total_reward1, env.num_punches, env.num_landed_punches, epsilon
                )
            }

            if iters > config.max_iters {
                break;
            }
        }

        println!("Finishing episode {episode}");

        if iters > config.max_iters {
            break;
        }
    }

    (policy_net0, policy_net1)
}

pub fn train_against<B: AutodiffBackend>(
    student_net: DQN<B>,
    teacher_nets: Vec<DQN<B>>,
    device: &B::Device,
    config: &TrainingConfig,
) -> DQN<B> {
    let mut env = GameState::new();
    let mut rng = StdRng::seed_from_u64(config.seed);

    let mut student_net = student_net.clone();

    let mut target_net = DQNConfig::new(OBSERVATION_LENGTH, OUTPUT_SIZE).init(device);
    let mut replay_buffer = ReplayBuffer::new(MEMORY_SIZE);
    let mut steps_done = 0;

    let mut iters = 0;

    for episode in 0..config.num_episodes {
        let teacher_index = rng.random_range(0..teacher_nets.len());
        let teacher_net = &teacher_nets[teacher_index];
        println!("Beginning episode {episode} against teacher index {teacher_index}");
        env = GameState::new();

        let mut p0_obs = env.get_observation(0);
        let mut p1_obs = env.get_observation(1);

        let mut total_reward0 = 0.0;
        let mut total_reward1 = 0.0;

        let mut is_episode_done = false;

        while !is_episode_done {
            let epsilon = get_epsilon(steps_done, config.epsilon_start, config.epsilon_decay);

            let action0 =
                select_action(p0_obs, &student_net, epsilon, NUM_ACTIONS, &mut rng, device);
            let action1 = select_action(p1_obs, &teacher_net, 0.05, NUM_ACTIONS, &mut rng, device);

            let StepResult {
                observations,
                rewards,
                is_done,
            } = env.step([Control::from_int(action0), Control::from_int(action1)]);

            let p0_obs_next = observations[0];
            let p1_obs_next = observations[1];

            replay_buffer.push(Experience {
                state: p0_obs,
                action: action0,
                reward: rewards[0],
                next_state: p0_obs_next,
                is_done,
            });

            // Push student and teacher experiences to the student
            replay_buffer.push(Experience {
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

            steps_done += 1;

            if steps_done > TRAIN_START && steps_done % config.iters_per_training_step == 0 {
                student_net = train_step(
                    student_net,
                    &target_net,
                    &mut replay_buffer,
                    device,
                    &config,
                );
            }

            if steps_done % TARGET_UPDATE == 0 {
                target_net = student_net.clone();
            }

            is_episode_done = is_done;
            if is_episode_done {
                println!(
                    "->> Episode finished with final health 0:{} 1:{}",
                    env.players[0].health, env.players[1].health,
                );
            }

            iters += 1;
            if iters % 1000 == 0 {
                println!(
                    "   Running iter {iters } Reward 0:{:.2} Reward 1:{:.2} num_punches: {:?} num_landed_punches: {:?} epsilon {}",
                    total_reward0, total_reward1, env.num_punches, env.num_landed_punches, epsilon
                )
            }

            if iters > config.max_iters {
                break;
            }
        }

        println!("Finishing episode {episode}");

        if iters > config.max_iters {
            break;
        }
    }

    student_net
}
