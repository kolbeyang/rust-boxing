use boxing::train::{TrainingConfig, train};
use burn::{
    backend::{Autodiff, Wgpu},
    optim::AdamConfig,
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
        learning_rate: 0.0001,
        num_episodes: 10,
    };

    let device = Default::default();

    train::<MyAutodiffBackend>(&device, config);

    println!("Total time: {:?}", start.elapsed());
}
