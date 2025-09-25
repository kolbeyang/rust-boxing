#![recursion_limit = "256"]

use core::model::{DQN, DQNConfig};
use std::path::PathBuf;

pub mod replay_buffer;
pub mod train;
use crate::train::{train, train_against};

use burn::{
    backend::{Autodiff, Wgpu},
    module::Module,
    optim::AdamConfig,
    prelude::Backend,
    record::{FullPrecisionSettings, NamedMpkFileRecorder, Recorder},
    tensor::Device,
};

use crate::train::TrainingConfig;

fn train_2(model_name0: &str, model_name1: &str, config: &TrainingConfig) {
    let start = std::time::Instant::now();

    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = Default::default();

    let (dqn0, dqn1) = train::<MyAutodiffBackend>(&device, config);

    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    dqn0.save_file(
        PathBuf::from(format!("./assets/models/{model_name0}")),
        &recorder,
    )
    .expect("Should save");
    dqn1.save_file(
        PathBuf::from(format!("./assets/models/{}", model_name1)),
        &recorder,
    )
    .expect("Should save");

    // Save training configs
    let config_json = serde_json::to_string_pretty(&config).expect("Should serialize config");
    std::fs::write(
        PathBuf::from(format!("./assets/training_configs/{}.json", model_name0)),
        &config_json,
    )
    .expect("Should save config");
    std::fs::write(
        PathBuf::from(format!("./assets/training_configs/{}.json", model_name1)),
        &config_json,
    )
    .expect("Should save config");

    println!("Total time: {:?}", start.elapsed());
}

//fn main() {
//    let mut config = TrainingConfig {
//        optimizer: AdamConfig::new(),
//        gamma: 0.999,
//        batch_size: 128,
//        learning_rate: 0.0005,
//        num_episodes: 20000,
//        max_iters: 20000,
//        epsilon_decay: 0.0007,
//        seed: 456,
//        iters_per_training_step: 8,
//    };
//
//    let starting_index: usize = 64;
//
//    train_2(
//        &format!("dqn{:03}", starting_index),
//        &format!("dqn{:03}", starting_index + 1),
//        &config,
//    );
//    config.seed += 1;
//    train_2(
//        &format!("dqn{:03}", starting_index + 2),
//        &format!("dqn{:03}", starting_index + 3),
//        &config,
//    );
//    config.seed += 1;
//    train_2(
//        &format!("dqn{:03}", starting_index + 4),
//        &format!("dqn{:03}", starting_index + 5),
//        &config,
//    );
//    config.seed += 1;
//    train_2(
//        &format!("dqn{:03}", starting_index + 6),
//        &format!("dqn{:03}", starting_index + 7),
//        &config,
//    );
//}

fn load_model<B: Backend>(
    model_path: &PathBuf,
    device: &B::Device,
) -> Result<DQN<B>, Box<dyn std::error::Error>> {
    let record =
        NamedMpkFileRecorder::<FullPrecisionSettings>::new().load(model_path.clone(), device)?;
    let model: DQN<B> = DQNConfig::new(23, 24).init(device).load_record(record);
    Ok(model)
}

fn main() {
    let mut config = TrainingConfig {
        optimizer: AdamConfig::new(),
        gamma: 0.999,
        batch_size: 128,
        learning_rate: 0.0002,
        num_episodes: 20000,
        max_iters: 20000,
        epsilon_decay: 0.0007,
        epsilon_start: 0.05,
        seed: 456,
        iters_per_training_step: 8,
    };

    type MyBackend = Wgpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device: Device<MyAutodiffBackend> = Default::default();

    let student_index = 64;
    let student_net: DQN<MyAutodiffBackend> = load_model(
        &PathBuf::from(format!("./assets/models/dqn{student_index:03}.mpk")),
        &device,
    )
    .expect("Should load student element");

    let mut teacher_nets = vec![];
    let teacher_net_indices = vec![11, 55, 70, 5];
    for i in teacher_net_indices {
        let teacher_net = load_model(
            &PathBuf::from(format!("./assets/models/dqn{i:03}.mpk")),
            &device,
        )
        .expect("Should load teacher model");
        teacher_nets.push(teacher_net);
    }

    let trained_model = train_against(student_net, teacher_nets, &device, &config);

    let new_model_index = 72;
    let recorder = NamedMpkFileRecorder::<FullPrecisionSettings>::new();
    trained_model
        .save_file(
            PathBuf::from(format!("./assets/models/dqn{new_model_index:03}.mpk")),
            &recorder,
        )
        .expect("Should save");
}
