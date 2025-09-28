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
        color: "#025233",
        description: "THE ORIGINAL BASELINE BEAST! The first fighter to step into the neural ring with pure, unfiltered hyperparameters - no fancy tricks, just RAW COMPUTATIONAL POWER from the stone age of training!",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn005.bin"),
        name: "Mike",
        number: 5,
        color: "#244D8D",
        description: "FROM THE EXPERIMENTAL EARLY DAYS comes this HYPERPARAMETER PIONEER! Forged in the fires of initial parameter exploration when the researchers were still finding their groove!",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn011.bin"),
        name: "Frank",
        number: 11,
        color: "#90C8CF",
        description: "THE MARATHON MACHINE with 0.0002 epsilon decay grinding through 40,000 BRUTAL iterations! This patient DESTROYER learns slow but hits like a freight train with 16-step training fury!",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn021.bin"),
        name: "Steve",
        number: 21,
        color: "#9AC7A4",
        description: "THE LIGHTNING-FAST LEARNER with rapid-fire 4-step training intervals! This SPEED DEMON adapted in just 20,000 max iterations - quick to learn, QUICKER TO STRIKE!",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn025.bin"),
        name: "Arnold",
        number: 25,
        color: "#E5562A",
        description: "THE HIGH-STAKES GAMBLER with SAVAGE 0.001 epsilon decay! Trained hard and fast in only 10,000 episodes - this AGGRESSIVE EXPLORER commits to every punch like his neural life depends on it!",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn054.bin"),
        name: "Gary",
        number: 54,
        color: "#E6242D",
        description: "THE RELIABLE DESTROYER with seed 792 and balanced 0.0005 epsilon decay! 20,000 episodes of CONSISTENT CARNAGE with 16-step training discipline - this fighter NEVER disappoints!",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn055.bin"),
        name: "Bob",
        number: 55,
        color: "#E6A5A6",
        description: "GARY'S IDENTICAL TWIN forged from the EXACT SAME hyperparameters but with a COMPLETELY different fighting soul! Same seed 792, same epsilon decay, DOUBLE THE DESTRUCTION!",
    },
    Fighter {
        model_bytes: include_bytes!("../..//assets/binary_models/dqn077.bin"),
        name: "Jim",
        number: 77,
        color: "#FCB708",
        description: "FRESH OUT OF THE NEURAL FACTORY! This CUTTING-EDGE DESTROYER represents the latest evolution in boxing AI - all the lessons learned, all the pain absorbed, PURE MODERN WARFARE!",
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
