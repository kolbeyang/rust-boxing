use core::{OBSERVATION_LENGTH, Observation};

use burn::{
    prelude::Backend,
    tensor::{Bool, Float, Int, Shape, Tensor, TensorData},
};
use rand::{rngs::ThreadRng, seq::IndexedRandom};

#[derive(Clone, Debug)]
pub struct Experience {
    pub state: Observation,
    pub action: usize,
    pub reward: f32,
    pub next_state: Observation,
    pub is_done: bool,
}

pub struct BatchTensors<B: Backend> {
    pub states: Tensor<B, 2, Float>,
    pub actions: Tensor<B, 1, Int>,
    pub rewards: Tensor<B, 1, Float>,
    pub next_states: Tensor<B, 2, Float>,
    pub is_dones: Tensor<B, 1, Bool>,
}

pub struct ReplayBuffer {
    capacity: usize,
    position: usize,
    size: usize,
    buffer: Vec<Experience>,
    rng: ThreadRng,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: Vec::with_capacity(capacity),
            position: 0,
            size: 0,
            rng: rand::rng(),
        }
    }
    pub fn push(&mut self, experience: Experience) {
        if self.size < self.capacity {
            self.buffer.push(experience);
            self.size += 1;
        } else {
            self.buffer[self.position] = experience;
        }

        self.position = (self.position + 1) % self.capacity;
    }

    pub fn sample_batch_tensors<B: Backend>(
        &mut self,
        batch_size: usize,
        device: &B::Device,
    ) -> BatchTensors<B> {
        let mut states: Vec<f32> = Vec::with_capacity(batch_size * OBSERVATION_LENGTH); // Flattened
        let mut actions: Vec<i32> = Vec::with_capacity(batch_size);
        let mut rewards: Vec<f32> = Vec::with_capacity(batch_size);
        let mut next_states: Vec<f32> = Vec::with_capacity(batch_size * OBSERVATION_LENGTH); // Flattened
        let mut is_dones: Vec<bool> = Vec::with_capacity(batch_size);

        for experience in self.buffer.choose_multiple(&mut self.rng, batch_size) {
            states.extend_from_slice(&experience.state.normalize());
            actions.push(experience.action as i32);
            rewards.push(experience.reward);
            next_states.extend_from_slice(&experience.next_state.normalize());
            is_dones.push(experience.is_done);
        }

        let state_tensor_data =
            TensorData::new(states, Shape::new([batch_size, OBSERVATION_LENGTH]));
        let action_tensor_data = TensorData::new(actions, Shape::new([batch_size]));
        let reward_tensor_data = TensorData::new(rewards, Shape::new([batch_size]));
        let next_state_tensor_data =
            TensorData::new(next_states, Shape::new([batch_size, OBSERVATION_LENGTH]));
        let is_dones_tensor_data = TensorData::new(is_dones, Shape::new([batch_size]));

        BatchTensors {
            states: Tensor::from_floats(state_tensor_data, device),
            actions: Tensor::from_ints(action_tensor_data, device),
            rewards: Tensor::from_floats(reward_tensor_data, device),
            next_states: Tensor::from_floats(next_state_tensor_data, device),
            is_dones: Tensor::from_bool(is_dones_tensor_data, device),
        }
    }
}
