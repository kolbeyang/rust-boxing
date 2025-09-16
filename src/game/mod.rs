use std::f32::consts::PI;

use parry2d::{math::Vector, na::Rotation2};

pub mod control;
pub use control::*;
pub mod utils;
pub use utils::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum FistState {
    Resting,
    Extending { target: Vector<f32>, speed: f32 },
    Retracting { speed: f32 },
}

impl FistState {
    pub fn to_int(&self) -> usize {
        match self {
            FistState::Resting => 0,
            FistState::Extending { .. } => 0,
            FistState::Retracting { .. } => 0,
        }
    }
}

#[derive(Clone)]
pub struct Fist {
    pub state: FistState,
    pub position: Vector<f32>,
}

impl Fist {
    fn new(start_pos: Vector<f32>) -> Self {
        Self {
            state: FistState::Resting,
            position: start_pos,
        }
    }

    pub fn retract(&mut self) {
        self.state = FistState::Retracting {
            speed: Player::PUNCH_SPEED,
        };
        // Move to avoid double contact
        let target = self.position;
        let direction = target - self.position;
        let delta = direction * Player::PUNCH_SPEED;
        self.position += delta;
    }
}

pub struct Player {
    pub position: Vector<f32>,
    pub rotation: f32, // Radians
    pub velocity: Vector<f32>,
    pub fists: [Fist; 2],
    pub health: f32,
}

impl Player {
    pub const STARTING_HEALTH: f32 = 10.0;
    pub const RADIUS: f32 = 32.0;
    pub const ACCELERATION: f32 = 2.0;
    pub const DECCELERATION: f32 = 0.75;
    pub const FORWARD_DRIFT_SPEED: f32 = 0.75;
    pub const KNOCKBACK_ACCELERATION: f32 = 5.0;

    pub const FIST_RADIUS: f32 = 16.0;
    pub const FIST_DISTANCE: f32 = 52.0;
    pub const FIST_OFFSET_ANGLE: f32 = PI * 0.35; // Radians
    pub const REACH: f32 = 100.0;
    pub const PUNCH_SPEED: f32 = 12.0;

    pub const ZERO_ANGLE: Vector<f32> = Vector::new(0.0, -1.0); // Up is 0

    pub fn new(start_pos: Vector<f32>, rotation: f32) -> Self {
        let mut player = Self {
            position: start_pos,
            rotation,
            velocity: Vector::new(0.0, 0.0),
            health: Player::STARTING_HEALTH,
            fists: [
                Fist::new(Vector::new(0.0, 0.0)),
                Fist::new(Vector::new(0.0, 0.0)),
            ],
        };

        for i in 0..=1 {
            let resting_pos = player.get_fist_resting_pos(i);
            player.fists[i].position = resting_pos;
        }

        player
    }

    pub fn get_fist_resting_pos(&self, fist_index: usize) -> Vector<f32> {
        let fist_offset_angle = match fist_index {
            0 => -Player::FIST_OFFSET_ANGLE, // left
            1 => Player::FIST_OFFSET_ANGLE,  // right
            _ => 0.0,
        };
        let angle = self.rotation + fist_offset_angle;
        let rotation = Rotation2::new(angle);

        self.position + rotation * Player::ZERO_ANGLE * Player::FIST_DISTANCE
    }

    pub fn handle_move(&mut self, controls: Control) {
        let Control { move_x, move_y, .. } = controls;

        let delta_x = move_x.to_num() as f32;
        let delta_y = move_y.to_num() as f32;
        // Flip y so that negative y means "backward" relative to facing direction
        let delta = Vector::new(delta_x, -delta_y);
        let _ = delta.normalize();

        let rotation = Rotation2::new(self.rotation);
        let rotated_delta = rotation * delta;

        let mut new_velocity = self.velocity + Player::ACCELERATION * rotated_delta;
        new_velocity *= Player::DECCELERATION;
        self.velocity = new_velocity;

        self.position += self.velocity;

        let fists_resting_pos = [self.get_fist_resting_pos(0), self.get_fist_resting_pos(1)];

        // Handle Fist Resting States
        for (i, fist) in self.fists.iter_mut().enumerate() {
            if fist.state == FistState::Resting {
                fist.position = fists_resting_pos[i];
            }
        }
    }

    pub fn get_hit(&mut self) {
        let delta = Vector::new(0.0, 1.0); // backward
        let rotation = Rotation2::new(self.rotation);
        let rotated_delta = rotation * delta;

        let new_velocity = self.velocity + Player::KNOCKBACK_ACCELERATION * rotated_delta;
        self.velocity = new_velocity;
        self.position += self.velocity;
        self.health -= 1.0;
    }

    pub fn handle_move_fists(&mut self) {
        let fists_resting_pos = [self.get_fist_resting_pos(0), self.get_fist_resting_pos(1)];

        for (i, fist) in self.fists.iter_mut().enumerate() {
            match fist.state {
                FistState::Resting => {
                    fist.position = fists_resting_pos[i];
                }
                FistState::Extending { target, speed } => {
                    let direction = (target - fist.position).normalize();
                    let delta = direction * speed;
                    fist.position += delta;

                    if (fist.position - self.position).magnitude() > Player::REACH {
                        println!("Fist {i} retracting");
                        fist.state = FistState::Retracting {
                            speed: Player::PUNCH_SPEED,
                        };
                    }
                }
                FistState::Retracting { speed } => {
                    let direction = (fists_resting_pos[i] - fist.position).normalize();
                    let delta = direction * speed;
                    fist.position += delta;

                    if (fist.position - fists_resting_pos[i]).magnitude() < Player::PUNCH_SPEED {
                        println!("Fist {i} ended punch");
                        fist.state = FistState::Resting;
                    }
                }
            }
        }
    }

    pub fn rotate_and_face(&mut self, position: Vector<f32>) {
        let direction_vector = position - self.position;
        self.rotation = direction_vector.y.atan2(direction_vector.x) + PI / 2.0;
    }

    pub fn move_forwand(&mut self, distance: f32) {
        let up_vector = Vector::new(0.0, -1.0);
        let rotation = Rotation2::new(self.rotation);
        let delta = rotation * (up_vector * distance);
        self.position += delta;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Observation {
    pub health: f32,
    pub op_health: f32,

    // These are the only two in world coordinates
    pub position: [f32; 2],
    pub rotation: f32,

    pub velocity: [f32; 2],
    pub left_fist_position: [f32; 2],
    pub right_fist_position: [f32; 2],
    pub left_fist_state: usize,  // 0 resting, 1 extending, 2 retracting
    pub right_fist_state: usize, // 0 resting, 1 extending, 2 retracting

    pub op_position: [f32; 2],
    pub op_velocity: [f32; 2],
    pub op_left_fist_position: [f32; 2],
    pub op_right_fist_position: [f32; 2],
    pub op_left_fist_state: usize, // 0 resting, 1 extending, 2 retracting
    pub op_right_fist_state: usize, // 0 resting, 1 extending, 2 retracting
}

impl Observation {
    const MAX_VELOCITY: f32 = 20.0;
    const MAX_LOCAL_DISTANCE: f32 = 300.0;

    pub fn normalize(&self) -> Vec<f32> {
        vec![
            // Health values (0-1 range)
            self.health / Player::STARTING_HEALTH,
            self.op_health / Player::STARTING_HEALTH,
            // World coordinates (0-1 range)
            self.position[0] / GameState::RING_SIZE.x,
            self.position[1] / GameState::RING_SIZE.y,
            // Rotation (0-1 range, could also use -1 to 1)
            (self.rotation + PI) / (2.0 * PI),
            // Player velocity in local coordinates (-1 to 1 range)
            (self.velocity[0] / Self::MAX_VELOCITY).clamp(-1.0, 1.0),
            (self.velocity[1] / Self::MAX_VELOCITY).clamp(-1.0, 1.0),
            // Player fist positions in local coordinates (-1 to 1 range)
            (self.left_fist_position[0] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            (self.left_fist_position[1] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            (self.right_fist_position[0] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            (self.right_fist_position[1] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            // Player fist states (0, 0.5, 1.0)
            self.left_fist_state as f32 / 2.0,
            self.right_fist_state as f32 / 2.0,
            // Opponent position in local coordinates (-1 to 1 range)
            (self.op_position[0] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            (self.op_position[1] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            // Opponent velocity in local coordinates (-1 to 1 range)
            (self.op_velocity[0] / Self::MAX_VELOCITY).clamp(-1.0, 1.0),
            (self.op_velocity[1] / Self::MAX_VELOCITY).clamp(-1.0, 1.0),
            // Opponent fist positions in local coordinates (-1 to 1 range)
            (self.op_left_fist_position[0] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            (self.op_left_fist_position[1] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            (self.op_right_fist_position[0] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            (self.op_right_fist_position[1] / Self::MAX_LOCAL_DISTANCE).clamp(-1.0, 1.0),
            // Opponent fist states (0, 0.5, 1.0)
            self.op_left_fist_state as f32 / 2.0,
            self.op_right_fist_state as f32 / 2.0,
        ]
    }
}

pub struct StepResult {
    pub observations: [Observation; 2],
    pub rewards: [f32; 2],
    pub is_done: bool,
}

pub struct GameState {
    pub players: [Player; 2],
}

impl GameState {
    pub const RING_SIZE: Vector<f32> = Vector::new(400.0, 400.0);
    pub const MIN_PLAYER_DISTANCE: f32 = 120.0;

    pub fn new() -> Self {
        let player_0 = Player::new(Vector::new(200.0, 100.0), PI); // On top, facing down
        let player_1 = Player::new(Vector::new(200.0, 300.0), 0.0); // On bottom, facing up

        Self {
            players: [player_0, player_1],
        }
    }

    pub fn step(&mut self, controls: [Control; 2]) -> StepResult {
        let initial_health: Vec<f32> = self.players.iter().map(|p| p.health).collect();

        for (i, player) in self.players.iter_mut().enumerate() {
            player.handle_move(controls[i])
        }
        // Check punch contact
        let players_pos: Vec<Vector<f32>> = self.players.iter().map(|p| p.position).collect();

        for player in self.players.iter_mut() {
            player.handle_move_fists();
        }

        // Fist / Player contact
        let mut is_players_hit = [false, false];
        for (i, player) in self.players.iter_mut().enumerate() {
            let other_player_pos = players_pos[1 - i];
            for fist in player.fists.iter_mut() {
                let distance = get_contact_distance(
                    fist.position,
                    Player::FIST_RADIUS,
                    other_player_pos,
                    Player::RADIUS,
                );

                if let Some(d) = distance {
                    println!("Player Hit! {d}");
                    // The other player is hit
                    is_players_hit[1 - i] = true;
                    fist.retract();
                }
            }
        }

        // Handle knockback
        for (i, is_player_hit) in is_players_hit.iter().enumerate() {
            if *is_player_hit {
                self.players[i].get_hit()
            }
        }

        // Fist / Fist contact
        let player_0_fists_pos = self.players[0].fists.clone().map(|f| f.position);
        let player_1_fists_pos = self.players[1].fists.clone().map(|f| f.position);

        for (player_0_i, player_0_fist_pos) in player_0_fists_pos.iter().enumerate() {
            for (player_1_i, player_1_fist_pos) in player_1_fists_pos.iter().enumerate() {
                let distance = get_contact_distance(
                    *player_0_fist_pos,
                    Player::FIST_RADIUS,
                    *player_1_fist_pos,
                    Player::FIST_RADIUS,
                );

                if let Some(d) = distance {
                    println!("Contact between P0 fist {player_0_i} and P1 fist {player_1_i}");
                    if let FistState::Extending { .. } = self.players[0].fists[player_0_i].state {
                        self.players[0].fists[player_0_i].retract();
                    }
                    if let FistState::Extending { .. } = self.players[1].fists[player_1_i].state {
                        self.players[1].fists[player_1_i].retract();
                    }
                }
            }
        }

        // Initiate punches
        for (i, player) in self.players.iter_mut().enumerate() {
            let Control {
                left_punch,
                right_punch,
                ..
            } = controls[i];
            let other_player_pos = players_pos[1 - i];
            let is_fists_punching = [left_punch, right_punch];
            for (fist_i, fist) in player.fists.iter_mut().enumerate() {
                let is_fist_punching = is_fists_punching[fist_i];
                if fist.state == FistState::Resting && is_fist_punching {
                    println!("Initiating punch on player {i} and fist {fist_i}");
                    fist.state = FistState::Extending {
                        target: other_player_pos,
                        speed: Player::PUNCH_SPEED,
                    }
                }
            }
        }

        // Players should face each other
        for (i, player) in self.players.iter_mut().enumerate() {
            let other_player_pos = players_pos[1 - i];
            player.rotate_and_face(other_player_pos);
        }

        // Players should drift toward each other
        let player_distance = (players_pos[1] - players_pos[0]).magnitude();
        let gap_left = player_distance - GameState::MIN_PLAYER_DISTANCE;
        let delta =
            (gap_left / 2.0).clamp(-Player::FORWARD_DRIFT_SPEED, Player::FORWARD_DRIFT_SPEED);
        for player in self.players.iter_mut() {
            player.move_forwand(delta);
        }

        // Check wall boundaries
        for player in self.players.iter_mut() {
            let [x, y] = player.position.into();
            let is_valid_left = 0.0 <= x - Player::RADIUS;
            if !is_valid_left {
                player.position.x = Player::RADIUS;
            }
            let is_valid_top = 0.0 <= y - Player::RADIUS;
            if !is_valid_top {
                player.position.y = Player::RADIUS;
            }
            let is_valid_right = x + Player::RADIUS <= GameState::RING_SIZE.x;
            if !is_valid_right {
                player.position.x = GameState::RING_SIZE.x - Player::RADIUS;
            }
            let is_valid_bottom = y + Player::RADIUS <= GameState::RING_SIZE.y;
            if !is_valid_bottom {
                player.position.y = GameState::RING_SIZE.y - Player::RADIUS;
            }
        }

        let player_0_observation = self.get_observation(0);
        let player_1_observation = self.get_observation(1);

        let mut player_0_reward = 0.0;
        let mut player_1_reward = 0.0;

        let player_0_health_delta = self.players[0].health - initial_health[0];
        let player_1_health_delta = self.players[1].health - initial_health[1];

        player_0_reward += player_0_health_delta - player_1_health_delta;
        player_1_reward += player_1_health_delta - player_0_health_delta;

        let is_done = self.players[0].health <= 0.0 || self.players[1].health <= 0.0;

        StepResult {
            observations: [player_0_observation, player_1_observation],
            rewards: [player_0_reward, player_1_reward],
            is_done,
        }
    }

    pub fn get_observation(&self, player_i: usize) -> Observation {
        let player = &self.players[player_i];
        let opponent = &self.players[1 - player_i];

        // Helper function to transform world position to player's local coordinate frame
        let world_to_local = |world_pos: Vector<f32>| -> [f32; 2] {
            // Translate to player's origin
            let relative_pos = world_pos - player.position;
            // Rotate by negative player rotation to get local coordinates
            let rotation = Rotation2::new(-player.rotation);
            let local_pos = rotation * relative_pos;
            // Flip Y so positive Y is "forward" from player's perspective
            [local_pos.x, -local_pos.y]
        };

        // Helper function to transform world velocity to player's local coordinate frame
        let world_velocity_to_local = |world_vel: Vector<f32>| -> [f32; 2] {
            // Only rotate velocity (no translation needed)
            let rotation = Rotation2::new(-player.rotation);
            let local_vel = rotation * world_vel;
            // Flip Y so positive Y is "forward" from player's perspective
            [local_vel.x, -local_vel.y]
        };

        Observation {
            health: player.health,
            op_health: opponent.health,

            // World coordinates (as requested)
            position: [player.position.x, player.position.y],
            rotation: player.rotation,

            // Local coordinates
            velocity: world_velocity_to_local(player.velocity),
            left_fist_position: world_to_local(player.fists[0].position),
            right_fist_position: world_to_local(player.fists[1].position),
            left_fist_state: player.fists[0].state.to_int(),
            right_fist_state: player.fists[1].state.to_int(),

            // Opponent in local coordinates
            op_position: world_to_local(opponent.position),
            op_velocity: world_velocity_to_local(opponent.velocity),
            op_left_fist_position: world_to_local(opponent.fists[0].position),
            op_right_fist_position: world_to_local(opponent.fists[1].position),
            op_left_fist_state: opponent.fists[0].state.to_int(),
            op_right_fist_state: opponent.fists[1].state.to_int(),
        }
    }
}
