use core::model::{DQN, DQNConfig};
use std::path::PathBuf;

use burn::{
    backend::Wgpu,
    module::Module,
    record::{BinFileRecorder, FullPrecisionSettings, NamedMpkFileRecorder, Recorder},
    tensor::Device,
};

type MyBackend = Wgpu<f32, i32>;

fn main() {
    let device: Device<MyBackend> = Default::default();
    let models_to_save = vec![77, 5, 21, 55, 54, 0];

    for model_num in models_to_save {
        let source_file_name = format!("./assets/models/dqn{:03}.mpk", model_num);

        let record = NamedMpkFileRecorder::<FullPrecisionSettings>::new()
            .load(PathBuf::from(source_file_name), &device)
            .expect("Should be able to load model 0 weights");
        let model: DQN<MyBackend> = DQNConfig::new(23, 24).init(&device).load_record(record);

        let dest_file_name = format!("./assets/binary_models/dqn{:03}.bin", model_num);

        let bin_record = BinFileRecorder::<FullPrecisionSettings>::new();
        model.save_file(dest_file_name, &bin_record);
    }
}
