use rand::{Rng, seq::IndexedRandom};

use crate::game::Observation;

#[derive(Clone, Debug)]
pub struct Experience {
    pub state: Observation,
    pub action: usize,
    pub reward: f32,
    pub next_state: Observation,
    pub is_done: bool,
}

pub struct ReplayBuffer {
    capacity: usize,
    buffer: Vec<Experience>,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: vec![],
        }
    }
    pub fn push(&mut self, experience: Experience) {
        self.buffer.push(experience);
    }

    pub fn sample(&self, batch_size: usize) -> Vec<Experience> {
        let mut rng = rand::rng();
        self.buffer
            .choose_multiple(&mut rng, batch_size)
            .cloned()
            .collect()
    }
}
