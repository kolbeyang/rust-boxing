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

pub static FIGHTERS: [Fighter; 8] = [
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn000.bin"),
        name: "Dave",
        number: 0,
        color: "#0A99FF",
        description: "the original, the legend, the ZERO-HOUR destroyer with baseline hyperparameters that started it all, pure vanilla neural networks throwing haymakers since day one",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn005.bin"),
        name: "Mike",
        number: 5,
        color: "#19B265",
        description: "cutting through the training loop with a razor-sharp learning rate of 0.0005, striking fast with batch normalization and leaving opponents stunned",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn011.bin"),
        name: "Frank",
        number: 11,
        color: "#37CAC3",
        description: "with a WILD linear epsilon decay from 1.0 to 0.01, featuring 8 attention heads of pure computational fury, the model that's all action and no overfitting",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn021.bin"),
        name: "Steve",
        number: 21,
        color: "#FD2B2B",
        description: "the SHADOW WARRIOR trained in darkness with double DQN architecture, featuring target network updates every 1000 steps of relentless precision striking",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn025.bin"),
        name: "Arnold",
        number: 25,
        color: "#4867FE",
        description: "weighing in at 175 million parameters of concentrated neural fire, with a SAVAGE Adam optimizer and dropout rate of 0.2, the model that sees every punch coming",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn054.bin"),
        name: "Gary",
        number: 54,
        color: "#63C942",
        description: "the COLOSSAL CRUSHER with prioritized experience replay and dueling network architecture, this behemoth learns from every mistake and hits back twice as hard",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn055.bin"),
        name: "Bob",
        number: 55,
        color: "#AC50E4",
        description: "rising from the ashes of failed training runs with rainbow DQN enhancements, distributional value learning, and the FIRE of a thousand gradient updates",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn077.bin"),
        name: "Jim",
        number: 77,
        color: "#F58300",
        description: "the ULTIMATE EVOLUTION with multi-step learning, noisy networks, and categorical DQN distribution, the apex predator that devoured the leaderboard",
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
