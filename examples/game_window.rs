use boxing::game::{Control, GameState, MoveX, MoveY, Player};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Boxing Game".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::new();
    let target_fps = 24.0;
    let frame_time = 1.0 / target_fps;

    loop {
        let frame_start = get_time();

        // Handle input for both players
        let controls = [
            get_player_controls(0), // Player 1
            get_player_controls(1), // Player 2
        ];

        // Update game state
        game_state.step(controls);

        // Render
        clear_background(DARKGRAY);
        draw_game(&game_state);

        // Wait for the remaining frame time
        let elapsed = get_time() - frame_start;
        let sleep_time = frame_time - elapsed;
        if sleep_time > 0.0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
        }

        next_frame().await;
    }
}

fn get_player_controls(player_id: usize) -> Control {
    match player_id {
        0 => {
            // Player 1 controls: WASD + QE for punching
            let move_x = if is_key_down(KeyCode::A) {
                MoveX::Left
            } else if is_key_down(KeyCode::D) {
                MoveX::Right
            } else {
                MoveX::None
            };

            let move_y = if is_key_down(KeyCode::S) {
                MoveY::Back
            } else {
                MoveY::None
            };

            Control {
                move_x,
                move_y,
                left_punch: is_key_down(KeyCode::Q),
                right_punch: is_key_down(KeyCode::E),
            }
        }
        1 => {
            // Player 2 controls: Arrow keys + NM for punching
            let move_x = if is_key_down(KeyCode::Left) {
                MoveX::Left
            } else if is_key_down(KeyCode::Right) {
                MoveX::Right
            } else {
                MoveX::None
            };

            let move_y = if is_key_down(KeyCode::Down) {
                MoveY::Back
            } else {
                MoveY::None
            };

            Control {
                move_x,
                move_y,
                left_punch: is_key_down(KeyCode::N),
                right_punch: is_key_down(KeyCode::M),
            }
        }
        _ => Control {
            move_x: MoveX::None,
            move_y: MoveY::None,
            left_punch: false,
            right_punch: false,
        },
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

        // Player label
        let label = format!("Player {}", i + 1);
        draw_text(&label, health_x, health_y - 5.0, 20.0, WHITE);
    }

    // Draw controls help
    draw_text(
        "Player 1: WASD + Q/E to punch",
        20.0,
        screen_height - 60.0,
        20.0,
        WHITE,
    );
    draw_text(
        "Player 2: Arrows + N/M to punch",
        20.0,
        screen_height - 30.0,
        20.0,
        WHITE,
    );
}
