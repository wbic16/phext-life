//! Phext Life: 9D Artificial Life Simulation
//!
//! WASM-compatible library for browser visualization

pub mod coordinate;
pub mod program;
pub mod universe;

use wasm_bindgen::prelude::*;
use universe::Universe;

/// WASM-exposed universe wrapper
#[wasm_bindgen]
pub struct PhextLife {
    universe: Universe,
}

#[wasm_bindgen]
impl PhextLife {
    /// Create new simulation with given size
    #[wasm_bindgen(constructor)]
    pub fn new(size: u8, fill_ratio: f64) -> PhextLife {
        // size^9 coordinates, but capped for performance
        let capped_size = size.min(4); // Max 4^9 = 262,144 coords
        PhextLife {
            universe: Universe::new([capped_size; 9], fill_ratio),
        }
    }

    /// Create small test simulation (3^9 = 19,683 coordinates)
    pub fn small() -> PhextLife {
        PhextLife {
            universe: Universe::small(),
        }
    }

    /// Create medium simulation
    pub fn medium() -> PhextLife {
        PhextLife {
            universe: Universe::medium(),
        }
    }

    /// Run one tick
    pub fn tick(&mut self) {
        self.universe.tick();
    }

    /// Run multiple ticks
    pub fn run(&mut self, ticks: u32) {
        for _ in 0..ticks {
            self.universe.tick();
        }
    }

    /// Get current tick number
    pub fn current_tick(&self) -> u64 {
        self.universe.tick
    }

    /// Get population count
    pub fn population(&self) -> usize {
        self.universe.population()
    }

    /// Get total coordinates
    pub fn total_coordinates(&self) -> usize {
        self.universe.total_coordinates()
    }

    /// Get stats as JSON string
    pub fn stats_json(&self) -> String {
        let stats = self.universe.stats();
        format!(
            r#"{{"tick":{},"population":{},"total":{},"avgAge":{:.1},"maxGen":{},"replicators":{},"replications":{}}}"#,
            stats.tick,
            stats.population,
            stats.total_coordinates,
            stats.avg_age,
            stats.max_generation,
            stats.replicator_count,
            stats.total_replications,
        )
    }

    /// Get dimensional density for given dimension (0-8)
    pub fn density(&self, dim: usize) -> Vec<usize> {
        if dim < 9 {
            self.universe.dimensional_density(dim)
        } else {
            vec![]
        }
    }

    /// Get 2D projection of occupied coordinates
    /// Projects 9D to 2D by summing dimension pairs
    /// Returns flat array of (x, y, intensity) triples
    pub fn projection_2d(&self, width: u32, height: u32) -> Vec<u8> {
        let mut grid = vec![0u8; (width * height) as usize];
        
        for coord in self.universe.programs.keys() {
            // Project 9D to 2D:
            // x = (d0 + d1 + d2) mapped to width
            // y = (d3 + d4 + d5) mapped to height
            // intensity from (d6 + d7 + d8)
            
            let x_sum: u32 = coord.dims[0..3].iter().map(|&d| d as u32).sum();
            let y_sum: u32 = coord.dims[3..6].iter().map(|&d| d as u32).sum();
            let z_sum: u32 = coord.dims[6..9].iter().map(|&d| d as u32).sum();
            
            // Map to grid
            let max_sum = self.universe.extents[0] as u32 * 3;
            let x = (x_sum * width / (max_sum + 1)).min(width - 1);
            let y = (y_sum * height / (max_sum + 1)).min(height - 1);
            
            let idx = (y * width + x) as usize;
            let intensity = ((z_sum * 255 / (max_sum + 1)) as u8).max(50);
            grid[idx] = grid[idx].saturating_add(intensity);
        }
        
        grid
    }

    /// Get RGB image data for canvas (RGBA format)
    pub fn render_rgba(&self, width: u32, height: u32) -> Vec<u8> {
        let projection = self.projection_2d(width, height);
        let mut rgba = Vec::with_capacity((width * height * 4) as usize);
        
        for &intensity in &projection {
            if intensity > 0 {
                // Purple-ish color for occupied cells
                let r = (intensity as u16 * 157 / 255) as u8;
                let g = (intensity as u16 * 78 / 255) as u8;
                let b = (intensity as u16 * 221 / 255) as u8;
                rgba.extend_from_slice(&[r, g, b, 255]);
            } else {
                // Dark background
                rgba.extend_from_slice(&[10, 10, 15, 255]);
            }
        }
        
        rgba
    }
}

/// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
