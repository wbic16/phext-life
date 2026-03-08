use wasm_bindgen::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// Instruction opcodes (Brainfuck-like)
const LT: u8 = b'<';
const GT: u8 = b'>';
const LB: u8 = b'{';
const RB: u8 = b'}';
const MINUS: u8 = b'-';
const PLUS: u8 = b'+';
const DOT: u8 = b'.';
const COMMA: u8 = b',';
const LBRACK: u8 = b'[';
const RBRACK: u8 = b']';
const NORMALIZED: u8 = 255;

const OPCODES: [u8; 10] = [LT, GT, LB, RB, MINUS, PLUS, DOT, COMMA, LBRACK, RBRACK];

fn is_opcode(val: u8) -> bool {
    OPCODES.contains(&val)
}

fn normalize(val: u8) -> u8 {
    if is_opcode(val) { val } else { NORMALIZED }
}

/// Execute Brainfuck-like program on tape
fn run_tape(tape: &mut [u8], max_iterations: usize) {
    let tape_size = tape.len();
    let mut pc: usize = 0;
    let mut head0: usize = 0;
    let mut head1: usize = 0;

    for _ in 0..max_iterations {
        if pc >= tape_size {
            break;
        }

        let opcode = tape[pc];

        match opcode {
            LT => head0 = (head0 + tape_size - 1) % tape_size,
            GT => head0 = (head0 + 1) % tape_size,
            LB => head1 = (head1 + tape_size - 1) % tape_size,
            RB => head1 = (head1 + 1) % tape_size,
            MINUS => tape[head0] = tape[head0].wrapping_sub(1),
            PLUS => tape[head0] = tape[head0].wrapping_add(1),
            DOT => tape[head1] = tape[head0],
            COMMA => tape[head0] = tape[head1],
            LBRACK if tape[head0] == 0 => {
                if let Some(new_pc) = seek_match(tape, pc, 1, LBRACK, RBRACK) {
                    pc = new_pc;
                } else {
                    break;
                }
            }
            RBRACK if tape[head0] != 0 => {
                if let Some(new_pc) = seek_match(tape, pc, -1, RBRACK, LBRACK) {
                    pc = new_pc;
                } else {
                    break;
                }
            }
            _ => {}
        }

        pc += 1;
    }
}

fn seek_match(tape: &[u8], start_pc: usize, step: isize, open_tok: u8, close_tok: u8) -> Option<usize> {
    let mut depth = 1;
    let mut pc = start_pc as isize + step;
    
    while pc >= 0 && (pc as usize) < tape.len() && depth > 0 {
        let opcode = tape[pc as usize];
        if opcode == open_tok {
            depth += 1;
        } else if opcode == close_tok {
            depth -= 1;
        }
        pc += step;
    }
    
    if depth == 0 {
        Some((pc - step) as usize)
    } else {
        None
    }
}

#[wasm_bindgen]
pub struct PhextLife {
    grid_size: usize,
    num_programs: usize,
    tape_size: usize,
    programs: Vec<u8>,
    neighbors: Vec<Vec<usize>>,
    rng: SmallRng,
    epoch: u32,
}

#[wasm_bindgen]
impl PhextLife {
    #[wasm_bindgen(constructor)]
    pub fn new(grid_size: usize, tape_size: usize, seed: u64) -> Self {
        let num_programs = grid_size.pow(3);
        let mut rng = SmallRng::seed_from_u64(seed);
        
        // Initialize random programs
        let mut programs = vec![0u8; num_programs * tape_size];
        for byte in programs.iter_mut() {
            *byte = if rng.gen::<f32>() < 0.5 {
                OPCODES[rng.gen_range(0..OPCODES.len())]
            } else {
                rng.gen::<u8>()
            };
        }

        // Build neighbor table
        let neighbors = Self::build_neighbors(grid_size);

        PhextLife {
            grid_size,
            num_programs,
            tape_size,
            programs,
            neighbors,
            rng,
            epoch: 0,
        }
    }

    fn build_neighbors(grid_size: usize) -> Vec<Vec<usize>> {
        let num_programs = grid_size.pow(3);
        let mut neighbors = vec![Vec::new(); num_programs];

        for idx in 0..num_programs {
            let coord = Self::index_to_coord(idx, grid_size);
            
            // ±1 in each of 3 dimensions
            for dim in 0..3 {
                // -1 neighbor
                if coord[dim] > 1 {
                    let mut n_coord = coord;
                    n_coord[dim] -= 1;
                    neighbors[idx].push(Self::coord_to_index(&n_coord, grid_size));
                }
                
                // +1 neighbor
                if coord[dim] < grid_size {
                    let mut n_coord = coord;
                    n_coord[dim] += 1;
                    neighbors[idx].push(Self::coord_to_index(&n_coord, grid_size));
                }
            }
        }

        neighbors
    }

    fn coord_to_index(coord: &[usize; 3], grid_size: usize) -> usize {
        let [l, s, c] = *coord;
        ((l - 1) * grid_size * grid_size) + ((s - 1) * grid_size) + (c - 1)
    }

    fn index_to_coord(idx: usize, grid_size: usize) -> [usize; 3] {
        let c = (idx % grid_size) + 1;
        let idx = idx / grid_size;
        let s = (idx % grid_size) + 1;
        let l = idx / grid_size + 1;
        [l, s, c]
    }

    pub fn step(&mut self) {
        // Select pairs randomly
        let mut taken = vec![false; self.num_programs];
        let mut pairs = Vec::new();

        let order: Vec<usize> = (0..self.num_programs).collect();
        
        for &prog_idx in &order {
            if taken[prog_idx] {
                continue;
            }

            // Pick random neighbor
            if self.neighbors[prog_idx].is_empty() {
                continue;
            }
            
            let neighbor_idx = self.neighbors[prog_idx][self.rng.gen_range(0..self.neighbors[prog_idx].len())];
            
            if taken[neighbor_idx] {
                continue;
            }

            taken[prog_idx] = true;
            taken[neighbor_idx] = true;
            pairs.push((prog_idx, neighbor_idx));
        }

        // Execute pairs
        for (idx_a, idx_b) in pairs {
            let mut concat_tape = vec![0u8; self.tape_size * 2];
            
            // Concatenate tapes
            let start_a = idx_a * self.tape_size;
            let start_b = idx_b * self.tape_size;
            concat_tape[..self.tape_size].copy_from_slice(&self.programs[start_a..start_a + self.tape_size]);
            concat_tape[self.tape_size..].copy_from_slice(&self.programs[start_b..start_b + self.tape_size]);

            // Execute
            run_tape(&mut concat_tape, 8192);

            // Split and write back
            self.programs[start_a..start_a + self.tape_size].copy_from_slice(&concat_tape[..self.tape_size]);
            self.programs[start_b..start_b + self.tape_size].copy_from_slice(&concat_tape[self.tape_size..]);
        }

        // Apply mutations (0.1% per byte)
        for byte in self.programs.iter_mut() {
            if self.rng.gen::<f32>() < 0.001 {
                *byte = OPCODES[self.rng.gen_range(0..OPCODES.len())];
            }
        }

        self.epoch += 1;
    }

    pub fn get_epoch(&self) -> u32 {
        self.epoch
    }

    pub fn get_program_data(&self) -> Vec<u8> {
        self.programs.clone()
    }

    pub fn get_visualization_data(&self) -> Vec<u8> {
        // Return RGBA image data for canvas
        let mut rgba = Vec::with_capacity(self.programs.len() * 4);
        
        for &byte in &self.programs {
            let normalized = normalize(byte);
            let (r, g, b) = match normalized {
                LT => (255, 0, 0),
                GT => (0, 255, 0),
                LB => (0, 0, 255),
                RB => (255, 255, 0),
                MINUS => (255, 0, 255),
                PLUS => (0, 255, 255),
                DOT => (255, 128, 0),
                COMMA => (128, 0, 255),
                LBRACK => (255, 255, 255),
                RBRACK => (128, 128, 128),
                _ => (0, 0, 0),
            };
            rgba.push(r);
            rgba.push(g);
            rgba.push(b);
            rgba.push(255); // Alpha
        }

        rgba
    }
}
