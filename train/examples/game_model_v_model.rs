use ::rand::rng;
use core::model::{DQN, DQNConfig};
use core::{Control, GameState, Player};
use std::path::PathBuf;
use train::train::select_action;

use burn::record::Recorder;
use burn::{
    backend::Wgpu,
    prelude::*,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};
use macroquad::prelude::*;

type MyBackend = Wgpu<f32, i32>;

fn window_conf() -> Conf {
    Conf {
        window_title: "Boxing Game - AI vs AI".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Initialize device
    let device = Default::default();

    // Load model 0
    let record0 = NamedMpkFileRecorder::<FullPrecisionSettings>::new()
        .load(PathBuf::from("./assets/models/dqn11.mpk"), &device)
        .expect("Should be able to load model 0 weights");
    let model0: DQN<MyBackend> = DQNConfig::new(23, 24).init(&device).load_record(record0);

    // Load model 1
    let record1 = NamedMpkFileRecorder::<FullPrecisionSettings>::new()
        .load(PathBuf::from("./assets/models/dqn54.mpk"), &device)
        .expect("Should be able to load model 0 weights");
    let model1: DQN<MyBackend> = DQNConfig::new(23, 24).init(&device).load_record(record1);

    let mut game_state = GameState::new();
    let target_fps = 24.0;
    let frame_time = 1.0 / target_fps;

    let mut rng = rng();

    // Epsilon for action selection (0 for greedy play)
    let epsilon = 0.005;
    let n_actions = 24;

    loop {
        let frame_start = get_time();

        // Get observations for both players
        let obs0 = game_state.get_observation(0);
        let obs1 = game_state.get_observation(1);

        // Select actions using the models
        let action0 = select_action(obs0, &model0, epsilon, n_actions, &mut rng, &device);
        let action1 = select_action(obs1, &model1, epsilon, n_actions, &mut rng, &device);

        //println!("Action {} {}", action0, action1);

        // Convert actions to controls
        let controls = [
            Control::from_int(action0), // Player 0
            Control::from_int(action1), // Player 1
        ];

        // Update game state
        game_state.step(controls);

        // Render
        clear_background(DARKGRAY);
        draw_game(&game_state);

        // Check for manual controls to pause/unpause or exit
        if is_key_pressed(KeyCode::Space) {
            // Pause until space is pressed again
            while !is_key_pressed(KeyCode::Space) {
                clear_background(DARKGRAY);
                draw_game(&game_state);
                draw_text(
                    "PAUSED - Press SPACE to continue",
                    screen_width() / 2.0 - 150.0,
                    screen_height() / 2.0,
                    30.0,
                    WHITE,
                );
                next_frame().await;
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Wait for the remaining frame time
        let elapsed = get_time() - frame_start;
        let sleep_time = frame_time - elapsed;
        if sleep_time > 0.0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
        }

        next_frame().await;
    }
}

fn draw_game(game_state: &GameState) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Calculate scaling to fit the ring nicely on screen
    let ring_size = GameState::RING_SIZE;
    let scale = f32::min(
        (screen_width - 100.0) / ring_size.x,
        (screen_height - 100.0) / ring_size.y,
    );

    let ring_screen_width = ring_size.x * scale;
    let ring_screen_height = ring_size.y * scale;
    let ring_x = (screen_width - ring_screen_width) / 2.0;
    let ring_y = (screen_height - ring_screen_height) / 2.0;

    // Draw ring boundary
    draw_rectangle_lines(
        ring_x,
        ring_y,
        ring_screen_width,
        ring_screen_height,
        3.0,
        WHITE,
    );

    // Draw players
    let player_colors = [RED, BLUE];

    for (i, player) in game_state.players.iter().enumerate() {
        let pos = player.position;

        // Convert game coordinates to screen coordinates
        let screen_x = ring_x + pos.x * scale;
        let screen_y = ring_y + pos.y * scale;
        let radius = Player::RADIUS * scale;

        // Draw player body
        draw_circle(screen_x, screen_y, radius, player_colors[i]);

        // Draw direction indicator (small line showing facing direction)
        let rotation = player.rotation;
        let dir_length = radius;
        let end_x = screen_x + rotation.sin() * dir_length;
        let end_y = screen_y + -rotation.cos() * dir_length;
        draw_line(screen_x, screen_y, end_x, end_y, 3.0, WHITE);

        // Draw fists at their current positions
        for (fist_idx, fist) in player.fists.iter().enumerate() {
            let fist_screen_x = ring_x + fist.position.x * scale;
            let fist_screen_y = ring_y + fist.position.y * scale;
            let fist_radius = Player::FIST_RADIUS * scale;

            // Slightly different colors for left/right fists
            let fist_color = if fist_idx == 0 { YELLOW } else { ORANGE };
            draw_circle(fist_screen_x, fist_screen_y, fist_radius, fist_color);
        }

        // Draw health bar
        let health_bar_width = 200.0;
        let health_bar_height = 20.0;
        let health_x = if i == 0 {
            20.0
        } else {
            screen_width - health_bar_width - 20.0
        };
        let health_y = 20.0;

        // Background
        draw_rectangle(
            health_x,
            health_y,
            health_bar_width,
            health_bar_height,
            DARKGRAY,
        );

        // Health
        let health_ratio = player.health / Player::STARTING_HEALTH;
        let health_color = if health_ratio > 0.6 {
            GREEN
        } else if health_ratio > 0.3 {
            YELLOW
        } else {
            RED
        };
        draw_rectangle(
            health_x,
            health_y,
            health_bar_width * health_ratio,
            health_bar_height,
            health_color,
        );

        // Border
        draw_rectangle_lines(
            health_x,
            health_y,
            health_bar_width,
            health_bar_height,
            2.0,
            WHITE,
        );

        // Draw energy bar (right under health bar)
        let energy_y = health_y + health_bar_height + 5.0; // 5 pixels gap

        // Background
        draw_rectangle(
            health_x,
            energy_y,
            health_bar_width,
            health_bar_height,
            DARKGRAY,
        );

        // Energy
        let energy_ratio = player.energy / Player::MAX_ENERGY;
        let energy_color = if energy_ratio > 0.6 {
            SKYBLUE
        } else if energy_ratio > 0.3 {
            GOLD
        } else {
            ORANGE
        };
        draw_rectangle(
            health_x,
            energy_y,
            health_bar_width * energy_ratio,
            health_bar_height,
            energy_color,
        );

        // Border
        draw_rectangle_lines(
            health_x,
            energy_y,
            health_bar_width,
            health_bar_height,
            2.0,
            WHITE,
        );

        // Player label - changed to show AI models
        let label = format!("AI Model {}", i);
        draw_text(&label, health_x, health_y - 5.0, 20.0, WHITE);
    }

    // Draw controls help - updated for AI vs AI
    draw_text(
        "AI Model 0 (Red) vs AI Model 1 (Blue)",
        20.0,
        screen_height - 60.0,
        20.0,
        WHITE,
    );
    draw_text(
        "Press SPACE to pause/unpause, ESC to quit",
        20.0,
        screen_height - 30.0,
        20.0,
        WHITE,
    );
}
