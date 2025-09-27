use core::{
    OBSERVATION_LENGTH, OUTPUT_SIZE,
    model::{DQN, DQNConfig},
};

use burn::{
    module::Module,
    record::{BinBytesRecorder, FullPrecisionSettings, Recorder},
};
use wasm_bindgen::prelude::wasm_bindgen;

pub type MyBackend = burn::backend::ndarray::NdArray<f32>;
pub type MyDevice = burn::backend::ndarray::NdArrayDevice;

pub struct Fighter {
    pub model_bytes: &'static [u8],
    pub name: &'static str,
    pub number: usize, // Unique
    pub description: &'static str,
    pub color: &'static str,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct FighterWeb {
    name: String,
    pub number: usize,
    description: String,
    color: String,
}

#[wasm_bindgen]
impl FighterWeb {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.description.to_string()
    }
    #[wasm_bindgen(getter)]
    pub fn color(&self) -> String {
        self.color.to_string()
    }
}

impl From<&Fighter> for FighterWeb {
    fn from(fighter: &Fighter) -> Self {
        Self {
            name: fighter.name.to_string(),
            number: fighter.number,
            description: fighter.description.to_string(),
            color: fighter.color.to_string(),
        }
    }
}

pub static FIGHTERS: [Fighter; 2] = [
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn025.bin"),
        name: "Arnold",
        number: 25,
        color: "#FF3131",
        description: "weighing in at 175 million parameters of concentrated neural fire, with a SAVAGE Adam optimizer and dropout rate of 0.2, the model that sees every punch coming",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn011.bin"),
        name: "Frank",
        number: 11,
        color: "#3154FF",
        description: "with a WILD linear epsilon decay from 1.0 to 0.01, featuring 8 attention heads of pure computational fury, the model that's all action and no overfitting",
    },
];

pub async fn build_and_load_model(bytes: &'static [u8], device: &MyDevice) -> DQN<MyBackend> {
    let model: DQN<MyBackend> = DQNConfig::new(OBSERVATION_LENGTH, OUTPUT_SIZE).init(device);
    let record = BinBytesRecorder::<FullPrecisionSettings, &'static [u8]>::default()
        .load(bytes, device)
        .expect("Failed to decode model");

    model.load_record(record)
}

#[wasm_bindgen]
pub fn get_fighters() -> Vec<FighterWeb> {
    FIGHTERS.iter().map(|f| f.into()).collect()
}
