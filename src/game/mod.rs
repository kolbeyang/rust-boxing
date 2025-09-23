use std::{cmp::Ordering, f32::consts::PI};

use parry2d::{math::Vector, na::Rotation2};

pub mod control;
pub use control::*;
pub mod utils;
pub use utils::*;

pub const OBSERVATION_LENGTH: usize = 25;

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
            FistState::Extending { .. } => 1,
            FistState::Retracting { .. } => 2,
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
            speed: Player::PUNCH_RETRACT_SPEED,
        };
        // Move to avoid double contact
        let target = self.position;
        let direction = target - self.position;
        let delta = direction * Player::MAX_PUNCH_SPEED;
        self.position += delta;
    }
}

pub struct Player {
    pub position: Vector<f32>,
    pub rotation: f32, // Radians
    pub velocity: Vector<f32>,
    pub fists: [Fist; 2],
    pub health: f32,
    pub energy: f32,
}

impl Player {
    pub const STARTING_HEALTH: f32 = 5.0;
    pub const MAX_ENERGY: f32 = 10.0;
    pub const STARTING_ENERGY: f32 = 7.0;
    pub const ENERGY_REGEN: f32 = 0.5 * 1.0 / 24.0;
    pub const RADIUS: f32 = 26.0;
    pub const ACCELERATION: f32 = 3.0;
    pub const DECCELERATION: f32 = 0.80;
    pub const ATTRACTION_FACTOR: f32 = 2.0;
    pub const REPULSION_FACTOR: f32 = 8.0;
    pub const KNOCKBACK_ACCELERATION: f32 = 8.0;

    pub const FIST_RADIUS: f32 = 15.0;
    pub const FIST_DISTANCE: f32 = 56.0;
    pub const MIN_FIST_OFFSET_ANGLE: f32 = PI * 0.25; // Radians
    pub const MAX_FIST_OFFSET_ANGLE: f32 = PI * 0.30; // Radians
    pub const MIN_REACH: f32 = 100.0;
    pub const MAX_REACH: f32 = 140.0;
    pub const MIN_PUNCH_SPEED: f32 = 8.0;
    pub const MAX_PUNCH_SPEED: f32 = 14.0;
    pub const PUNCH_RETRACT_SPEED: f32 = 14.0;

    pub const ZERO_ANGLE: Vector<f32> = Vector::new(0.0, -1.0); // Up is 0

    pub fn new(start_pos: Vector<f32>, rotation: f32) -> Self {
        let mut player = Self {
            position: start_pos,
            rotation,
            velocity: Vector::new(0.0, 0.0),
            health: Player::STARTING_HEALTH,
            energy: Player::STARTING_ENERGY,
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
            0 => -self.get_fist_offset_angle(), // left
            1 => self.get_fist_offset_angle(),  // right
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
        self.rotation += move_x.to_num() as f32 * 0.03;

        if self.rotation > PI {
            self.rotation -= 2.0 * PI;
        } else if self.rotation < -PI {
            self.rotation += 2.0 * PI;
        }

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

    pub fn get_fist_offset_angle(&self) -> f32 {
        let percentage = 1.0 - self.energy / Player::MAX_ENERGY;
        (Player::MAX_FIST_OFFSET_ANGLE - Player::MIN_FIST_OFFSET_ANGLE) * percentage
            + Player::MIN_FIST_OFFSET_ANGLE
    }

    pub fn get_punch_speed(&self) -> f32 {
        let percentage = self.energy / Player::MAX_PUNCH_SPEED;
        (Player::MAX_PUNCH_SPEED - Player::MIN_PUNCH_SPEED) * percentage + Player::MIN_PUNCH_SPEED
    }

    pub fn get_reach(&self) -> f32 {
        let percentage = self.energy / Player::MAX_REACH;
        (Player::MAX_REACH - Player::MIN_REACH) * percentage + Player::MIN_REACH
    }

    // NOTE: match factor is how much to turn in that direction
    pub fn rotate_and_face(&mut self, position: Vector<f32>, match_factor: f32) {
        let direction_vector = position - self.position;
        let new_rotation = direction_vector.y.atan2(direction_vector.x) + PI / 2.0;
        let mut delta = new_rotation - self.rotation;
        if delta > PI {
            delta -= 2.0 * PI;
        } else if delta < -PI {
            delta += 2.0 * PI;
        }
        self.rotation += delta * match_factor;
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

    pub energy: f32,
    pub op_energy: f32,

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

    pub fn normalize(&self) -> [f32; OBSERVATION_LENGTH] {
        [
            // Health values (0-1 range)
            self.health / Player::STARTING_HEALTH,
            self.op_health / Player::STARTING_HEALTH,
            // Energy values (0-1 range)
            self.energy / Player::MAX_ENERGY,
            self.op_energy / Player::MAX_ENERGY,
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

    // TODO: remove
    pub num_punches: [usize; 2],
    pub num_landed_punches: [usize; 2],
}

impl GameState {
    pub const RING_SIZE: Vector<f32> = Vector::new(400.0, 400.0);
    pub const MIN_PLAYER_DISTANCE: f32 = 88.0;

    pub fn new() -> Self {
        let player_0 = Player::new(Vector::new(200.0, 100.0), PI); // On top, facing down
        let player_1 = Player::new(Vector::new(200.0, 300.0), 0.0); // On bottom, facing up

        Self {
            players: [player_0, player_1],
            num_punches: [0, 0],
            num_landed_punches: [0, 0],
        }
    }

    pub fn step(&mut self, controls: [Control; 2]) -> StepResult {
        let mut rewards = [0.0, 0.0];

        for (i, player) in self.players.iter_mut().enumerate() {
            if controls[i].move_y == MoveY::Back {
                rewards[i] -= 0.1;
            }
            player.handle_move(controls[i])
        }
        // Check punch contact
        let players_pos = [self.players[0].position, self.players[1].position];

        for (i, player) in self.players.iter_mut().enumerate() {
            let op_position = players_pos[1 - i];
            let fists_resting_pos = [
                player.get_fist_resting_pos(0),
                player.get_fist_resting_pos(1),
            ];
            let reach = player.get_reach();

            for (i, fist) in player.fists.iter_mut().enumerate() {
                match fist.state {
                    FistState::Resting => {
                        fist.position = fists_resting_pos[i];
                    }
                    FistState::Extending { target, speed } => {
                        let direction = (target - fist.position).normalize();
                        let delta = direction * speed;
                        fist.position += delta;

                        if (fist.position - player.position).magnitude() > reach {
                            // Reward near misses
                            let distance_from_op = (op_position - fist.position).magnitude()
                                - Player::RADIUS
                                - Player::FIST_RADIUS;

                            let distance_percentage =
                                (distance_from_op / Player::MAX_REACH).clamp(0.0, 1.0);

                            // Reward anything under a 10 percent distance_percentage
                            let mut reward = 8.0 * (1.0 - distance_percentage).powi(4);
                            // Reward high energy states
                            let energy_percentage = player.energy / Player::MAX_ENERGY;
                            reward *= (energy_percentage * 0.8) + 0.2;

                            rewards[i] += reward;

                            //println!(
                            //    "Fist {i} retracting because of maximum reach distance from op {distance_from_op} distance_ratio {distance_percentage} energy_percentage {energy_percentage} reward {reward} "
                            //);

                            fist.retract();
                        }
                    }
                    FistState::Retracting { speed } => {
                        let direction = (fists_resting_pos[i] - fist.position).normalize();
                        let delta = direction * speed;
                        fist.position += delta;

                        if (fist.position - fists_resting_pos[i]).magnitude()
                            < Player::MAX_PUNCH_SPEED
                        {
                            //println!("Fist {i} ended punch");
                            fist.state = FistState::Resting;
                        }
                    }
                }
            }
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

                if distance.is_some() {
                    //println!("Player Hit! {d} fist retracting");
                    // The other player is hit
                    is_players_hit[1 - i] = true;
                    rewards[i] += 10.0;

                    self.num_landed_punches[i] += 1;
                    fist.retract();
                }
            }
        }

        // Handle knockback
        for (i, is_player_hit) in is_players_hit.iter().enumerate() {
            if *is_player_hit {
                self.players[i].get_hit();
                rewards[i] -= 6.0;
            }
        }

        // Fist / Fist contact
        let player_0_fists_pos = [
            self.players[0].fists[0].position,
            self.players[0].fists[1].position,
        ];
        let player_1_fists_pos = [
            self.players[1].fists[0].position,
            self.players[1].fists[1].position,
        ];

        for (player_0_i, player_0_fist_pos) in player_0_fists_pos.iter().enumerate() {
            for (player_1_i, player_1_fist_pos) in player_1_fists_pos.iter().enumerate() {
                let distance = get_contact_distance(
                    *player_0_fist_pos,
                    Player::FIST_RADIUS,
                    *player_1_fist_pos,
                    Player::FIST_RADIUS,
                );

                if distance.is_some() {
                    //println!("Contact between P0 fist {player_0_i} and P1 fist {player_1_i}");
                    if let FistState::Extending { .. } = self.players[0].fists[player_0_i].state {
                        //println!("Punch from player 0 hit fists, retracting");
                        self.players[0].fists[player_0_i].retract();
                        //rewards[0] += 0.2;
                        //rewards[1] += 0.1;
                    }
                    if let FistState::Extending { .. } = self.players[1].fists[player_1_i].state {
                        //println!("Punch from player 1 hit fists, retracting");
                        self.players[1].fists[player_1_i].retract();
                        //rewards[1] += 0.2;
                        //rewards[0] += 0.1;
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
            let is_fists_start_punching = [left_punch, right_punch];
            let fists_state = [player.fists[0].state, player.fists[1].state];
            let punch_speed = player.get_punch_speed();
            for (fist_i, fist) in player.fists.iter_mut().enumerate() {
                let is_fist_start_punching = is_fists_start_punching[fist_i];
                let is_other_fist_start_punching = is_fists_start_punching[1 - fist_i];
                let is_other_fist_punching = fists_state[1 - fist_i] != FistState::Resting;
                if fist.state == FistState::Resting
                    && is_fist_start_punching
                    && !is_other_fist_start_punching
                    && !is_other_fist_punching
                    && player.energy > 1.0
                {
                    //println!("Initiating punch on player {i} and fist {fist_i}");
                    self.num_punches[i] += 1;
                    player.energy -= 1.0;
                    //rewards[i] += 0.1;
                    fist.state = FistState::Extending {
                        target: other_player_pos,
                        speed: punch_speed,
                    }
                } else if is_fist_start_punching {
                    // NOTE: punch when not allowed
                    //println!("Illegal punch input");
                    //rewards[i] -= 0.1;
                }
            }
        }

        // Players should face each other
        for (i, player) in self.players.iter_mut().enumerate() {
            let other_player_pos = players_pos[1 - i];
            player.rotate_and_face(other_player_pos, 0.08);
        }

        // Players should drift toward the center
        let center: Vector<f32> = GameState::RING_SIZE / 2.0;
        let dead_zone: f32 = 0.3; // proportion of the ring where center drift doesn't apply

        for player in self.players.iter_mut() {
            let difference = center - player.position;
            let direction = difference.normalize();
            let magnitude = (difference.magnitude() / GameState::RING_SIZE.x - dead_zone).max(0.0);

            player.position += direction * magnitude * 100.0;
        }

        // Players should drift toward each other
        let player_distance = (players_pos[1] - players_pos[0]).magnitude() - Player::RADIUS * 2.0;

        let delta = match player_distance < GameState::MIN_PLAYER_DISTANCE {
            true => {
                (player_distance / GameState::MIN_PLAYER_DISTANCE).log2() * Player::REPULSION_FACTOR
            }
            false => {
                (player_distance / GameState::MIN_PLAYER_DISTANCE).log2()
                    * Player::ATTRACTION_FACTOR
            }
        };

        let players_pos = [self.players[0].position, self.players[1].position];
        for (i, player) in self.players.iter_mut().enumerate() {
            let other_player_pos = players_pos[1 - i];
            let delta_vector = (other_player_pos - player.position).normalize() * delta;

            player.position += delta_vector;
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

        // Increment energy
        self.players[0].energy =
            (self.players[0].energy + Player::ENERGY_REGEN).clamp(0.0, Player::MAX_ENERGY);
        self.players[1].energy =
            (self.players[1].energy + Player::ENERGY_REGEN).clamp(0.0, Player::MAX_ENERGY);

        // Reward higher energy states
        for (i, player) in self.players.iter().enumerate() {
            let energy_percentage = player.energy / Player::MAX_ENERGY;
            rewards[i] += energy_percentage * 0.0001;
        }

        let player_0_observation = self.get_observation(0);
        let player_1_observation = self.get_observation(1);

        let is_done = self.players[0].health <= 0.0 || self.players[1].health <= 0.0;

        let winner = match self.players[0].health.partial_cmp(&self.players[1].health) {
            Some(Ordering::Less) => Some(1),
            Some(Ordering::Equal) => None,
            Some(Ordering::Greater) => Some(0),
            None => None,
        };

        if is_done && let Some(win_i) = winner {
            // NOTE: winner reward
            rewards[win_i] += 50.0;
        }

        StepResult {
            observations: [player_0_observation, player_1_observation],
            rewards,
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

            energy: player.energy,
            op_energy: opponent.energy,

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
