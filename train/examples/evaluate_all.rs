use ::rand::rng;
use ::rand::Rng;
use core::model::{DQN, DQNConfig};
use core::{Control, GameState};
use std::path::PathBuf;
use train::train::select_action;

use burn::record::Recorder;
use burn::{
    backend::Wgpu,
    prelude::*,
    record::{FullPrecisionSettings, NamedMpkFileRecorder},
};
use std::fs;

type MyBackend = Wgpu<f32, i32>;

struct EvaluationResult {
    model1_name: String,
    model2_name: String,
    winner: String,
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

    let winner = if player0_health > player1_health {
        player0_name.to_string()
    } else if player1_health > player0_health {
        player1_name.to_string()
    } else {
        "Tie".to_string()
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
        winner,
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

    println!("Found {} model files:", model_files.len());
    for (_, name) in &model_files {
        println!("  - {name}");
    }
    println!();

    // Load all models
    let mut loaded_models = Vec::new();
    for (path, name) in &model_files {
        match load_model(path, &device) {
            Ok(model) => {
                loaded_models.push((model, name.clone()));
                println!("Successfully loaded: {name}");
            }
            Err(e) => {
                println!("Failed to load {name}: {e}");
                continue;
            }
        }
    }

    if loaded_models.len() < 2 {
        println!(
            "Need at least 2 models to evaluate. Found: {}",
            loaded_models.len()
        );
        return Ok(());
    }

    println!("\nStarting evaluations...\n");

    // Evaluate all pairs (avoiding duplicate comparisons)
    let mut results = Vec::new();
    for i in 0..loaded_models.len() {
        for j in (i + 1)..loaded_models.len() {
            let (model1, name1) = &loaded_models[i];
            let (model2, name2) = &loaded_models[j];

            println!("Evaluating {name1} vs {name2}...");

            let result = evaluate_models(model1, model2, name1, name2, &device);
            results.push(result);
        }
    }

    // Print results
    println!("\n=== EVALUATION RESULTS ===");
    for result in &results {
        println!(
            "{} vs {}: {} wins (health: {:.1} vs {:.1})",
            result.model1_name,
            result.model2_name,
            result.winner,
            result.model1_health,
            result.model2_health
        );
    }

    // Print summary statistics
    println!("\n=== SUMMARY ===");
    let mut win_counts = std::collections::HashMap::new();
    let mut loss_counts = std::collections::HashMap::new();

    // Initialize win and loss counts
    for (_, name) in &loaded_models {
        win_counts.insert(name.clone(), 0);
        loss_counts.insert(name.clone(), 0);
    }

    // Count wins and losses
    for result in &results {
        if result.winner != "Tie" {
            *win_counts.get_mut(&result.winner).unwrap() += 1;

            // The loser is the other model in the match
            let loser = if result.winner == result.model1_name {
                &result.model2_name
            } else {
                &result.model1_name
            };
            *loss_counts.get_mut(loser).unwrap() += 1;
        }
    }

    // Sort by wins (descending)
    let mut sorted_results: Vec<_> = win_counts.iter().collect();
    sorted_results.sort_by(|a, b| b.1.cmp(a.1));

    for (model_name, wins) in sorted_results {
        let losses = loss_counts.get(model_name).unwrap();
        println!("{model_name}: {wins} wins, {losses} losses");
    }

    Ok(())
}
