//! 9D Universe containing programs at phext coordinates

use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use crate::coordinate::Coordinate;
use crate::program::{Program, StepResult};

/// The 9D universe
pub struct Universe {
    pub programs: HashMap<Coordinate, Program>,
    pub extents: [u8; 9],
    pub tick: u64,
    pub total_replications: u64,
    pub total_extinctions: u64,
}

impl Universe {
    /// Create universe with given extents, randomly populated
    pub fn new(extents: [u8; 9], fill_ratio: f64) -> Self {
        let mut programs = HashMap::new();
        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        // Calculate total coordinates
        let total: usize = extents.iter().map(|&e| e as usize).product();
        let fill_count = (total as f64 * fill_ratio) as usize;

        // Randomly place programs
        for _ in 0..fill_count {
            let coord = Coordinate::random(&extents);
            if !programs.contains_key(&coord) {
                programs.insert(coord, Program::random());
            }
        }

        Self {
            programs,
            extents,
            tick: 0,
            total_replications: 0,
            total_extinctions: 0,
        }
    }

    /// Create small test universe
    pub fn small() -> Self {
        Self::new([3, 3, 3, 3, 3, 3, 3, 3, 3], 0.1)
    }

    /// Create medium universe
    pub fn medium() -> Self {
        Self::new([5, 5, 5, 3, 3, 3, 2, 2, 2], 0.05)
    }

    /// Total possible coordinates
    pub fn total_coordinates(&self) -> usize {
        self.extents.iter().map(|&e| e as usize).product()
    }

    /// Current population
    pub fn population(&self) -> usize {
        self.programs.len()
    }

    /// Run one tick of the simulation
    pub fn tick(&mut self) {
        self.tick += 1;

        // Collect coordinates to process (avoid borrow issues)
        let coords: Vec<Coordinate> = self.programs.keys().copied().collect();
        
        // Process each program
        let mut actions: Vec<Action> = Vec::new();

        for coord in coords {
            if let Some(program) = self.programs.get_mut(&coord) {
                program.age += 1;

                // Run program for some steps
                for _ in 0..64 {
                    let result = program.step();
                    match result {
                        StepResult::Continue => {}
                        StepResult::Halted => break,
                        StepResult::CopySelf { dim, direction } => {
                            actions.push(Action::Copy {
                                from: coord,
                                dim: dim as usize,
                                direction,
                            });
                        }
                        StepResult::WriteNeighbor { dim, direction, value } => {
                            actions.push(Action::Write {
                                from: coord,
                                dim: dim as usize,
                                direction,
                                value,
                            });
                        }
                        StepResult::ReadNeighbor { dim, direction } => {
                            // Defer read action
                            actions.push(Action::Read {
                                target: coord,
                                dim: dim as usize,
                                direction,
                            });
                        }
                    }
                }

                // Reset program for next tick
                if let Some(p) = self.programs.get_mut(&coord) {
                    p.reset();
                }
            }
        }

        // Execute actions
        for action in actions {
            match action {
                Action::Copy { from, dim, direction } => {
                    if let Some(target_coord) = from.neighbor(dim, direction, &self.extents) {
                        if let Some(source) = self.programs.get(&from) {
                            let child = source.replicate();
                            self.programs.insert(target_coord, child);
                            self.total_replications += 1;
                        }
                    }
                }
                Action::Write { from, dim, direction, value } => {
                    if let Some(target_coord) = from.neighbor(dim, direction, &self.extents) {
                        if let Some(target) = self.programs.get_mut(&target_coord) {
                            target.instructions[0] = value;
                        }
                    }
                }
                Action::Read { target, dim, direction } => {
                    if let Some(neighbor_coord) = target.neighbor(dim, direction, &self.extents) {
                        let value = self.programs.get(&neighbor_coord)
                            .map(|n| n.data[0])
                            .unwrap_or(0);
                        if let Some(p) = self.programs.get_mut(&target) {
                            p.data[p.dp] = value;
                        }
                    }
                }
            }
        }
    }

    /// Get statistics
    pub fn stats(&self) -> UniverseStats {
        let mut total_age = 0u64;
        let mut max_generation = 0u64;
        let mut replicator_count = 0usize;

        for program in self.programs.values() {
            total_age += program.age;
            max_generation = max_generation.max(program.generation);
            if program.replicator_score() > 0 {
                replicator_count += 1;
            }
        }

        UniverseStats {
            tick: self.tick,
            population: self.population(),
            total_coordinates: self.total_coordinates(),
            avg_age: if self.population() > 0 {
                total_age as f64 / self.population() as f64
            } else {
                0.0
            },
            max_generation,
            replicator_count,
            total_replications: self.total_replications,
        }
    }

    /// Get dimensional occupancy (how many coordinates occupied per dimension value)
    pub fn dimensional_density(&self, dim: usize) -> Vec<usize> {
        let mut density = vec![0usize; self.extents[dim] as usize];
        for coord in self.programs.keys() {
            density[coord.dims[dim] as usize - 1] += 1;
        }
        density
    }
}

/// Deferred action
enum Action {
    Copy { from: Coordinate, dim: usize, direction: i8 },
    Write { from: Coordinate, dim: usize, direction: i8, value: u8 },
    Read { target: Coordinate, dim: usize, direction: i8 },
}

/// Universe statistics
#[derive(Debug, Clone)]
pub struct UniverseStats {
    pub tick: u64,
    pub population: usize,
    pub total_coordinates: usize,
    pub avg_age: f64,
    pub max_generation: u64,
    pub replicator_count: usize,
    pub total_replications: u64,
}

impl std::fmt::Display for UniverseStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Tick: {} | Pop: {}/{} ({:.1}%) | Avg Age: {:.1} | Gen: {} | Replicators: {} | Total Copies: {}",
            self.tick,
            self.population,
            self.total_coordinates,
            100.0 * self.population as f64 / self.total_coordinates as f64,
            self.avg_age,
            self.max_generation,
            self.replicator_count,
            self.total_replications,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_universe() {
        let u = Universe::small();
        assert!(u.population() > 0);
        assert_eq!(u.total_coordinates(), 3usize.pow(9)); // 19683
    }

    #[test]
    fn test_tick() {
        let mut u = Universe::small();
        let initial_pop = u.population();
        u.tick();
        assert_eq!(u.tick, 1);
        // Population may have changed due to replication
        println!("Pop: {} -> {}", initial_pop, u.population());
    }
}
