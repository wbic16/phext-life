//! Phext Life: 9D Artificial Life Simulation
//!
//! Self-replicating programs in 11-dimensional phext space
//! (9 navigation dimensions + 2D text = 11D)

mod coordinate;
mod program;
mod universe;

use universe::Universe;
use std::time::{Duration, Instant};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║         PHEXT LIFE v0.1.0              ║");
    println!("║   9D Artificial Life Simulation        ║");
    println!("║   Self-replicating programs in         ║");
    println!("║   11-dimensional phext space           ║");
    println!("╚════════════════════════════════════════╝");
    println!();

    // Parse args
    let args: Vec<String> = std::env::args().collect();
    let seed: u64 = args.get(1)
        .and_then(|s| s.strip_prefix("--seed="))
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());

    let max_ticks: u64 = args.get(2)
        .and_then(|s| s.strip_prefix("--ticks="))
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);

    println!("Seed: {}", seed);
    println!("Max ticks: {}", max_ticks);
    println!();

    // Seed RNG
    use rand::SeedableRng;
    let _rng = rand::rngs::StdRng::seed_from_u64(seed);

    // Create universe
    // Small: 3^9 = 19,683 coordinates
    // Medium: 5^3 * 3^3 * 2^3 = 27,000 coordinates
    println!("Creating universe...");
    let mut universe = Universe::medium();
    println!("Universe: {} total coordinates", universe.total_coordinates());
    println!("Initial population: {}", universe.population());
    println!();

    // Run simulation
    let start = Instant::now();
    let mut last_print = Instant::now();

    for tick in 0..max_ticks {
        universe.tick();

        // Print stats every second or every 100 ticks
        if last_print.elapsed() > Duration::from_secs(1) || tick % 100 == 0 {
            let stats = universe.stats();
            println!("{}", stats);
            
            // Print dimensional density for first 3 dims
            if tick % 500 == 0 {
                for dim in 0..3 {
                    let density = universe.dimensional_density(dim);
                    let sparkline = density_sparkline(&density);
                    println!("  Dim {}: {}", dim, sparkline);
                }
            }
            
            last_print = Instant::now();
        }

        // Check for extinction or total domination
        if universe.population() == 0 {
            println!("\n💀 EXTINCTION at tick {}", tick);
            break;
        }
        if universe.population() >= universe.total_coordinates() {
            println!("\n🌟 TOTAL DOMINATION at tick {}", tick);
            break;
        }
    }

    let elapsed = start.elapsed();
    println!();
    println!("════════════════════════════════════════");
    println!("Simulation complete!");
    println!("Duration: {:.2}s", elapsed.as_secs_f64());
    println!("Ticks per second: {:.1}", universe.tick as f64 / elapsed.as_secs_f64());
    println!();
    let final_stats = universe.stats();
    println!("Final: {}", final_stats);
}

/// Create sparkline from density values
fn density_sparkline(values: &[usize]) -> String {
    if values.is_empty() {
        return String::new();
    }
    
    let max = *values.iter().max().unwrap_or(&1) as f64;
    let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    
    values.iter()
        .map(|&v| {
            let normalized = if max > 0.0 { v as f64 / max } else { 0.0 };
            let idx = (normalized * 7.0).round() as usize;
            blocks[idx.min(7)]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline() {
        let values = vec![0, 2, 4, 6, 8, 6, 4, 2, 0];
        let spark = density_sparkline(&values);
        assert_eq!(spark.chars().count(), 9);
    }
}
