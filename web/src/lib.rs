use core::{Control, Fist, FistState, GameState, OUTPUT_SIZE, Player, model::DQN, select_action};

extern crate web_sys;

use parry2d::math::Vector;
use rand::rngs::ThreadRng;
use wasm_bindgen::prelude::*;

use crate::state::{FIGHTERS, MyBackend, MyDevice, build_and_load_model};

mod state;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Game {
    game_state: GameState,
    model0: DQN<MyBackend>,
    model1: DQN<MyBackend>,
    rng: ThreadRng,
    #[wasm_bindgen(skip)]
    device: MyDevice,
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    fn from_rust(vector: Vector<f32>) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum FistStateWeb {
    Resting,
    Extending,
    Retracting,
}

impl FistStateWeb {
    fn from_rust(fist_state: &FistState) -> Self {
        match fist_state {
            FistState::Resting => FistStateWeb::Resting,
            FistState::Extending { .. } => FistStateWeb::Extending,
            FistState::Retracting { .. } => FistStateWeb::Retracting,
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
struct FistWeb {
    pub position: Point,
    pub state: FistStateWeb,
}

impl FistWeb {
    fn from_rust(fist: &Fist) -> Self {
        Self {
            position: Point::from_rust(fist.position),
            state: FistStateWeb::from_rust(&fist.state),
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
struct PlayerWeb {
    pub position: Point,
    pub rotation: f32,
    pub velocity: Point,
    pub health: f32,
    pub energy: f32,
    pub fist_0: FistWeb,
    pub fist_1: FistWeb,
    pub last_control: Control,
}

impl PlayerWeb {
    fn from_rust(player: &Player) -> Self {
        Self {
            position: Point::from_rust(player.position),
            velocity: Point::from_rust(player.velocity),
            health: player.health,
            energy: player.energy,
            rotation: player.rotation,
            fist_0: FistWeb::from_rust(&player.fists[0]),
            fist_1: FistWeb::from_rust(&player.fists[1]),
            last_control: Control::from_int(0),
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
struct GameStateWeb {
    pub player_0: PlayerWeb,
    pub player_1: PlayerWeb,
    pub is_done: bool,
}

impl GameStateWeb {
    fn from_rust(game_state: &GameState) -> Self {
        let is_done = game_state.players[0].health <= 0.0 || game_state.players[1].health <= 0.0;
        Self {
            player_0: PlayerWeb::from_rust(&game_state.players[0]),
            player_1: PlayerWeb::from_rust(&game_state.players[1]),
            is_done,
        }
    }
}

#[wasm_bindgen]
impl Game {
    pub async fn new(player0_number: usize, player1_number: usize) -> Self {
        let fighter0 = FIGHTERS
            .iter()
            .find(|f| f.number == player0_number)
            .unwrap_or(&FIGHTERS[0]);
        let fighter1 = FIGHTERS
            .iter()
            .find(|f| f.number == player1_number)
            .unwrap_or(&FIGHTERS[0]);

        let device: MyDevice = MyDevice::default();
        Self {
            game_state: GameState::new(),
            model0: build_and_load_model(fighter0.model_bytes, &device).await,
            model1: build_and_load_model(fighter1.model_bytes, &device).await,
            rng: rand::rng(),
            device,
        }
    }

    pub fn step(&mut self) -> GameStateWeb {
        let obs0 = self.game_state.get_observation(0);
        let obs1 = self.game_state.get_observation(1);

        let control0_int = select_action(
            obs0,
            &self.model0,
            0.01,
            OUTPUT_SIZE,
            &mut self.rng,
            &self.device,
        );
        let control0 = Control::from_int(control0_int);
        let control1_int = select_action(
            obs1,
            &self.model1,
            0.005,
            OUTPUT_SIZE,
            &mut self.rng,
            &self.device,
        );
        let control1 = Control::from_int(control1_int);
        self.game_state.step([control0, control1]);
        let mut output = GameStateWeb::from_rust(&self.game_state);
        output.player_0.last_control = control0;
        output.player_1.last_control = control1;
        output
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}
