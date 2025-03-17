use rand::SeedableRng;
use rand::rngs::StdRng;
use std::fs::File;
use std::io::Write;
use crate::organism_manager::OrganismManager;
use crate::utils::*;

// Define hyperparameter search space
const PRODUCER_RATES: [f32; 3] = [0.05, 0.07, 0.1];
const MUTATION_RATES: [f32; 3] = [0.4, 0.6, 0.8];
const FOOD_BENEFITS: [f32; 3] = [0.15, 0.18, 0.25];
const LIFETIME_MULTIPLIERS: [i32; 3] = [28, 32, 40];
const REPRODUCTION_MULTIPLIERS: [f32; 3] = [1.5, 1.85, 2.2];

struct SimResult {
    producer_rate: f32,
    mutation_rate: f32,
    food_benefit: f32,
    lifetime_multiplier: i32,
    reproduction_multiplier: f32,
    avg_population: f32,
    extinction_rate: f32,
    growth_stability: f32,
}

fn run_simulation(seed: u64, producer_rate: f32, mutation_rate: f32, food_benefit: f32, 
                  lifetime_multiplier: i32, reproduction_multiplier: f32) -> SimResult {
    
    println!("Running simulation with seed: {}, producer_rate: {}, mutation_rate: {}, food_benefit: {}, lifetime_multiplier: {}, reproduction_multiplier: {}",
             seed, producer_rate, mutation_rate, food_benefit, lifetime_multiplier, reproduction_multiplier);

    let mut rng = StdRng::seed_from_u64(seed);
    let mut organism_manager = OrganismManager::new();
    organism_manager.init();

    let mut population_history = Vec::new();
    let mut extinction_count = 0;
    let mut last_population = 10;

    for generation in 0..500 {  // Run for 500 generations
        organism_manager.update();
        let pop = organism_manager.organisms.len() as f32;
        population_history.push(pop);

        if pop == 0.0 {
            extinction_count += 1;
            organism_manager.init(); // Restart if extinction occurs
            println!("Extinction occurred at generation {}. Restarting...", generation);
        }
        
        last_population = pop as usize;
    }

    // Compute metrics
    let avg_population = population_history.iter().sum::<f32>() / population_history.len() as f32;
    let extinction_rate = extinction_count as f32 / 500.0; // % of frames where pop hit zero
    let growth_stability = population_history.windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .sum::<f32>() / population_history.len() as f32; // Smoother = better

    println!("Simulation results - avg_population: {}, extinction_rate: {}, growth_stability: {}",
             avg_population, extinction_rate, growth_stability);

    SimResult {
        producer_rate,
        mutation_rate,
        food_benefit,
        lifetime_multiplier,
        reproduction_multiplier,
        avg_population,
        extinction_rate,
        growth_stability,
    }
}

pub fn main() {
    let mut results = Vec::new();
    
    for &producer_rate in &PRODUCER_RATES {
        for &mutation_rate in &MUTATION_RATES {
            for &food_benefit in &FOOD_BENEFITS {
                for &lifetime_multiplier in &LIFETIME_MULTIPLIERS {
                    for &reproduction_multiplier in &REPRODUCTION_MULTIPLIERS {
                        let result = run_simulation(
                            42, // Fixed seed for consistency
                            producer_rate, mutation_rate, food_benefit, 
                            lifetime_multiplier, reproduction_multiplier
                        );
                        results.push(result);
                    }
                }
            }
        }
    }

    // Sort by best balance: High avg_population, Low extinction rate, Smooth growth
    results.sort_by(|a, b| 
        ((b.avg_population * 10.0 - b.extinction_rate * 20.0 - b.growth_stability * 5.0) 
        .partial_cmp(&(a.avg_population * 10.0 - a.extinction_rate * 20.0 - a.growth_stability * 5.0)))
        .unwrap()
    );

    let best_result = &results[0];

    println!("Best result - producer_rate: {}, mutation_rate: {}, food_benefit: {}, lifetime_multiplier: {}, reproduction_multiplier: {}, avg_population: {}, extinction_rate: {}, growth_stability: {}",
             best_result.producer_rate, best_result.mutation_rate, best_result.food_benefit, best_result.lifetime_multiplier, best_result.reproduction_multiplier, best_result.avg_population, best_result.extinction_rate, best_result.growth_stability);

    // Save best hyperparameters to a file
    let mut file = File::create("best_hyperparams.txt").expect("Failed to create file");
    writeln!(file, "Best Hyperparameters:").unwrap();
    writeln!(file, "Producer Rate: {}", best_result.producer_rate).unwrap();
    writeln!(file, "Mutation Rate: {}", best_result.mutation_rate).unwrap();
    writeln!(file, "Food Benefit: {}", best_result.food_benefit).unwrap();
    writeln!(file, "Lifetime Multiplier: {}", best_result.lifetime_multiplier).unwrap();
    writeln!(file, "Reproduction Multiplier: {}", best_result.reproduction_multiplier).unwrap();
}
