use ::rand::Rng;
use ::rand::rng;
use core::model::{DQN, DQNConfig};
use core::{Control, GameState};
use rand::seq::SliceRandom;
use std::path::PathBuf;
use train::train::select_action;

use burn::record::Recorder;
use burn::{
    backend::Wgpu,
    prelude::*,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};
use std::collections::{HashMap, HashSet};
use std::fs;

type MyBackend = Wgpu<f32, i32>;

#[derive(Debug, Clone)]
struct Model {
    id: usize,
    name: String,
    model: Option<DQN<MyBackend>>, // Option because we can't clone DQN easily
    score: f32,
    opponents: HashSet<usize>,
    wins: usize,
    losses: usize,
    ties: usize,
}

#[derive(Debug)]
struct Tournament {
    models: Vec<Model>,
    model_refs: Vec<DQN<MyBackend>>, // Store actual models separately
    rounds: Vec<Vec<(usize, usize)>>, // pairs of model indices
    current_round: usize,
}

impl Tournament {
    fn new(
        model_files: Vec<(PathBuf, String)>,
        device: &<MyBackend as Backend>::Device,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut models = Vec::new();
        let mut model_refs = Vec::new();

        println!("Loading models for Swiss tournament...");

        for (id, (path, name)) in model_files.iter().enumerate() {
            match load_model(path, device) {
                Ok(model) => {
                    println!("✓ Loaded: {}", name);
                    model_refs.push(model);
                    models.push(Model {
                        id,
                        name: name.clone(),
                        model: None, // We store models separately
                        score: 0.0,
                        opponents: HashSet::new(),
                        wins: 0,
                        losses: 0,
                        ties: 0,
                    });
                }
                Err(e) => {
                    println!("✗ Failed to load {}: {}", name, e);
                    return Err(e);
                }
            }
        }

        if models.len() < 2 {
            return Err("Need at least 2 models for Swiss tournament".into());
        }

        println!("Successfully loaded {} models\n", models.len());

        Ok(Tournament {
            models,
            model_refs,
            rounds: Vec::new(),
            current_round: 0,
        })
    }

    fn calculate_rounds_needed(&self) -> usize {
        // For Swiss tournaments, typically log2(n) rounds, but we'll use a bit more
        // to ensure good ranking separation
        let n = self.models.len();
        if n <= 4 {
            n - 1 // Round robin for small tournaments
        } else {
            ((n as f32).log2().ceil() as usize + 1).min(n - 1)
        }
    }

    fn pair_first_round(&mut self) -> Vec<(usize, usize)> {
        let mut rng = rng();
        let mut indices: Vec<usize> = (0..self.models.len()).collect();
        indices.shuffle(&mut rng);

        let mut pairs = Vec::new();
        for chunk in indices.chunks(2) {
            if chunk.len() == 2 {
                pairs.push((chunk[0], chunk[1]));
            }
        }

        self.rounds.push(pairs.clone());
        pairs
    }

    fn pair_next_round(&mut self) -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
        let mut sorted_indices = self.get_sorted_model_indices();
        let mut pairs = Vec::new();
        let mut unpaired: HashSet<usize> = sorted_indices.iter().copied().collect();

        while unpaired.len() >= 2 {
            let first = *unpaired.iter().next().unwrap();
            unpaired.remove(&first);

            let opponent = self.find_best_opponent(first, &unpaired)?;
            unpaired.remove(&opponent);

            pairs.push((first, opponent));
        }

        // Handle bye if odd number of players (shouldn't happen in our case)
        if unpaired.len() == 1 {
            let bye_player = *unpaired.iter().next().unwrap();
            println!(
                "Player {} gets a bye this round",
                self.models[bye_player].name
            );
        }

        self.rounds.push(pairs.clone());
        Ok(pairs)
    }

    fn find_best_opponent(
        &self,
        model_id: usize,
        available: &HashSet<usize>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let model = &self.models[model_id];

        // Priority 1: Same score, haven't played before
        for &candidate in available {
            if (self.models[candidate].score - model.score).abs() < 0.01
                && !model.opponents.contains(&candidate)
            {
                return Ok(candidate);
            }
        }

        // Priority 2: Closest score, haven't played before
        let mut best_candidate = None;
        let mut best_score_diff = f32::INFINITY;

        for &candidate in available {
            if !model.opponents.contains(&candidate) {
                let score_diff = (self.models[candidate].score - model.score).abs();
                if score_diff < best_score_diff {
                    best_score_diff = score_diff;
                    best_candidate = Some(candidate);
                }
            }
        }

        // If no one available who hasn't played before, just pick the closest score
        if best_candidate.is_none() {
            best_candidate = available.iter().copied().min_by(|&a, &b| {
                let diff_a = (self.models[a].score - model.score).abs();
                let diff_b = (self.models[b].score - model.score).abs();
                diff_a.partial_cmp(&diff_b).unwrap()
            });
        }

        best_candidate.ok_or("No available opponent".into())
    }

    fn get_sorted_model_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.models.len()).collect();

        indices.sort_by(|&a, &b| {
            let model_a = &self.models[a];
            let model_b = &self.models[b];

            // Primary: Total score (descending)
            model_b
                .score
                .partial_cmp(&model_a.score)
                .unwrap()
                // Secondary: Buchholz score (sum of opponents' scores)
                .then_with(|| {
                    let buchholz_a: f32 = model_a
                        .opponents
                        .iter()
                        .map(|&opp| self.models[opp].score)
                        .sum();
                    let buchholz_b: f32 = model_b
                        .opponents
                        .iter()
                        .map(|&opp| self.models[opp].score)
                        .sum();
                    buchholz_b.partial_cmp(&buchholz_a).unwrap()
                })
                // Tertiary: Number of wins
                .then_with(|| model_b.wins.cmp(&model_a.wins))
        });

        indices
    }

    fn update_scores(&mut self, model1_id: usize, model2_id: usize, result: &GameResult) {
        match result {
            GameResult::Model1Wins => {
                self.models[model1_id].score += 1.0;
                self.models[model2_id].score -= 1.0;
                self.models[model1_id].wins += 1;
                self.models[model2_id].losses += 1;
            }
            GameResult::Model2Wins => {
                self.models[model1_id].score -= 1.0;
                self.models[model2_id].score += 1.0;
                self.models[model2_id].wins += 1;
                self.models[model1_id].losses += 1;
            }
            GameResult::Tie => {
                self.models[model1_id].ties += 1;
                self.models[model2_id].ties += 1;
            }
        }

        // Track opponents
        self.models[model1_id].opponents.insert(model2_id);
        self.models[model2_id].opponents.insert(model1_id);
    }

    fn print_current_standings(&self) {
        println!("\n=== CURRENT STANDINGS ===");
        let sorted_indices = self.get_sorted_model_indices();

        for (rank, &idx) in sorted_indices.iter().enumerate() {
            let model = &self.models[idx];
            println!(
                "{}. {} - Score: {:.1} ({}W-{}L-{}T)",
                rank + 1,
                model.name,
                model.score,
                model.wins,
                model.losses,
                model.ties
            );
        }
        println!();
    }

    fn run_tournament(
        &mut self,
        device: &<MyBackend as Backend>::Device,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let num_rounds = self.calculate_rounds_needed();
        println!("=== SWISS TOURNAMENT START ===");
        println!(
            "Running {} rounds with {} models\n",
            num_rounds,
            self.models.len()
        );

        for round in 0..num_rounds {
            self.current_round = round;
            println!("=== ROUND {} ===", round + 1);

            let pairs = if round == 0 {
                self.pair_first_round()
            } else {
                self.pair_next_round()?
            };

            for (model1_id, model2_id) in pairs {
                let model1_name = &self.models[model1_id].name.clone();
                let model2_name = &self.models[model2_id].name.clone();

                println!("Match: {} vs {}", model1_name, model2_name);

                let result = evaluate_models(
                    &self.model_refs[model1_id],
                    &self.model_refs[model2_id],
                    model1_name,
                    model2_name,
                    device,
                );

                // Print match result
                match result.game_result {
                    GameResult::Model1Wins => println!(
                        "🏆 {} WINS! (Health: {:.1} vs {:.1})",
                        model1_name, result.model1_health, result.model2_health
                    ),
                    GameResult::Model2Wins => println!(
                        "🏆 {} WINS! (Health: {:.1} vs {:.1})",
                        model2_name, result.model2_health, result.model1_health
                    ),
                    GameResult::Tie => println!(
                        "🤝 TIE! (Health: {:.1} vs {:.1})",
                        result.model1_health, result.model2_health
                    ),
                }

                self.update_scores(model1_id, model2_id, &result.game_result);
                println!();
            }

            self.print_current_standings();
        }

        Ok(())
    }
}

#[derive(Debug)]
enum GameResult {
    Model1Wins,
    Model2Wins,
    Tie,
}

struct EvaluationResult {
    model1_name: String,
    model2_name: String,
    game_result: GameResult,
    model1_health: f32,
    model2_health: f32,
}

fn load_model(
    model_path: &PathBuf,
    device: &<MyBackend as Backend>::Device,
) -> Result<DQN<MyBackend>, Box<dyn std::error::Error>> {
    let record =
        NamedMpkFileRecorder::<FullPrecisionSettings>::new().load(model_path.clone(), device)?;
    let model: DQN<MyBackend> = DQNConfig::new(23, 24).init(device).load_record(record);
    Ok(model)
}

fn evaluate_models(
    model1: &DQN<MyBackend>,
    model2: &DQN<MyBackend>,
    model1_name: &str,
    model2_name: &str,
    device: &<MyBackend as Backend>::Device,
) -> EvaluationResult {
    let mut game_state = GameState::new();
    let mut rng = rng();

    // Parameters
    let epsilon = 0.005;
    let n_actions = 24;
    let max_iterations = 24 * 120;

    // Randomly swap model order with 50% chance to reduce position bias
    let swap_models = rng.random_bool(0.5);
    let (player0_model, player1_model, player0_name, player1_name) = if swap_models {
        (model2, model1, model2_name, model1_name)
    } else {
        (model1, model2, model1_name, model2_name)
    };

    // Run the game
    for _iteration in 0..max_iterations {
        // Get observations for both players
        let obs0 = game_state.get_observation(0);
        let obs1 = game_state.get_observation(1);

        // Select actions using the models (potentially swapped)
        let action0 = select_action(obs0, player0_model, epsilon, n_actions, &mut rng, device);
        let action1 = select_action(obs1, player1_model, epsilon, n_actions, &mut rng, device);

        // Convert actions to controls
        let controls = [
            Control::from_int(action0), // Player 0
            Control::from_int(action1), // Player 1
        ];

        // Update game state
        let step_result = game_state.step(controls);

        // Check if game ended early (though we'll run full duration anyway)
        if step_result.is_done {
            break;
        }
    }

    // Determine winner based on health (accounting for potential model swap)
    let player0_health = game_state.players[0].health;
    let player1_health = game_state.players[1].health;

    let game_result = if player0_health > player1_health {
        if swap_models {
            GameResult::Model2Wins
        } else {
            GameResult::Model1Wins
        }
    } else if player1_health > player0_health {
        if swap_models {
            GameResult::Model1Wins
        } else {
            GameResult::Model2Wins
        }
    } else {
        GameResult::Tie
    };

    // Return results in original model1/model2 order for consistency
    let (model1_health, model2_health) = if swap_models {
        (player1_health, player0_health) // model1 was player1, model2 was player0
    } else {
        (player0_health, player1_health) // model1 was player0, model2 was player1
    };

    EvaluationResult {
        model1_name: model1_name.to_string(),
        model2_name: model2_name.to_string(),
        game_result,
        model1_health,
        model2_health,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize device
    let device = Default::default();

    // Discover all .mpk files in the models directory
    let models_dir = PathBuf::from("./assets/models/");
    let mut model_files = Vec::new();

    if let Ok(entries) = fs::read_dir(&models_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "mpk" {
                        if let Some(file_name) = path.file_name() {
                            model_files
                                .push((path.clone(), file_name.to_string_lossy().to_string()));
                        }
                    }
                }
            }
        }
    }

    if model_files.is_empty() {
        println!("No .mpk model files found in {}", models_dir.display());
        return Ok(());
    }

    // Sort model files by name for consistent ordering
    model_files.sort_by(|a, b| a.1.cmp(&b.1));

    println!("Found {} model files:", model_files.len());
    for (_, name) in &model_files {
        println!("  - {}", name);
    }
    println!();

    // Create and run tournament
    let mut tournament = Tournament::new(model_files, &device)?;
    tournament.run_tournament(&device)?;

    // Print final results
    println!("=== FINAL TOURNAMENT RESULTS ===");
    let sorted_indices = tournament.get_sorted_model_indices();

    for (rank, &idx) in sorted_indices.iter().enumerate() {
        let model = &tournament.models[idx];
        println!(
            "{}. {} - Final Score: {:.1}",
            rank + 1,
            model.name,
            model.score
        );
        println!(
            "   Wins: {}, Losses: {}, Ties: {}",
            model.wins, model.losses, model.ties
        );

        let games_played = model.wins + model.losses + model.ties;
        if games_played > 0 {
            let win_rate = (model.wins as f32 / games_played as f32) * 100.0;
            println!("   Win Rate: {:.1}% ({} games)", win_rate, games_played);
        }
        println!();
    }

    // Print tournament statistics
    println!("=== TOURNAMENT STATISTICS ===");
    println!("Total rounds played: {}", tournament.rounds.len());
    let total_games: usize = tournament.rounds.iter().map(|round| round.len()).sum();
    println!("Total games played: {}", total_games);

    // Calculate average games per model
    let total_games_all_models: usize = tournament
        .models
        .iter()
        .map(|m| m.wins + m.losses + m.ties)
        .sum();
    let avg_games_per_model = total_games_all_models as f32 / tournament.models.len() as f32;
    println!("Average games per model: {:.1}", avg_games_per_model);

    Ok(())
}
