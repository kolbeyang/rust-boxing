#![recursion_limit = "256"] // or higher like "1024"

use std::path::PathBuf;

use boxing::{
    model::{DQN, DQNConfig},
    train::{TrainingConfig, train},
};
use burn::{
    backend::{Autodiff, Wgpu},
    module::Module,
    optim::AdamConfig,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};

fn main() {
    // TODO: remove
    let start = std::time::Instant::now();

    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let config = TrainingConfig {
        optimizer: AdamConfig::new(),
        gamma: 0.99,
        batch_size: 64,
        learning_rate: 0.001,
        num_episodes: 10,
        max_iters: 10_000,
    };

    let device = Default::default();

    let (dqn0, dqn1) = train::<MyAutodiffBackend>(&device, config);

    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    dqn0.save_file(PathBuf::from("./dqn0"), &recorder)
        .expect("Should save");
    dqn1.save_file(PathBuf::from("./dqn1"), &recorder)
        .expect("Should save");

    println!("Total time: {:?}", start.elapsed());
}
