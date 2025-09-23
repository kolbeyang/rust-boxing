#![recursion_limit = "256"]

use std::path::PathBuf;

pub mod replay_buffer;
pub mod train;
use crate::train::train;

use burn::{
    backend::{Autodiff, Wgpu},
    module::Module,
    optim::AdamConfig,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};

use crate::train::TrainingConfig;

fn main() {
    // TODO: remove
    let start = std::time::Instant::now();

    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let config = TrainingConfig {
        optimizer: AdamConfig::new(),
        gamma: 0.999,
        batch_size: 128,
        learning_rate: 0.0001,
        num_episodes: 20_000,
        max_iters: 20_000,
        epsilon_decay: 0.0001,
        iters_per_training_step: 4,
        seed: 45,
    };

    let device = Default::default();

    let (dqn0, dqn1) = train::<MyAutodiffBackend>(&device, config);

    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    dqn0.save_file(PathBuf::from("./assets/models/dqn0"), &recorder)
        .expect("Should save");
    dqn1.save_file(PathBuf::from("./assets/models/dqn1"), &recorder)
        .expect("Should save");

    println!("Total time: {:?}", start.elapsed());
}
