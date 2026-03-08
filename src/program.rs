//! BF-like Program with 9D Extensions
//!
//! Extended instruction set for phext navigation and replication

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

/// Instruction set
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Instruction {
    // Classic BF (0-7)
    Right = 0,      // > move data pointer right
    Left = 1,       // < move data pointer left
    Inc = 2,        // + increment
    Dec = 3,        // - decrement
    Output = 4,     // . output (unused for now)
    Input = 5,      // , input (unused for now)
    JumpFwd = 6,    // [ jump forward if zero
    JumpBack = 7,   // ] jump back if nonzero

    // Phext extensions (8-15)
    DimUp = 8,      // ↑ increase current dimension focus
    DimDown = 9,    // ↓ decrease current dimension focus
    ReadNeighbor = 10,  // R read from neighbor in current dim direction
    WriteNeighbor = 11, // W write to neighbor
    CopySelf = 12,      // C copy entire program to neighbor
    Jump = 13,          // J jump to coordinate (uses data as address)
    Nop = 14,           // . no operation
    Halt = 15,          // H halt execution
}

impl Instruction {
    pub fn from_byte(b: u8) -> Self {
        match b & 0x0F {
            0 => Instruction::Right,
            1 => Instruction::Left,
            2 => Instruction::Inc,
            3 => Instruction::Dec,
            4 => Instruction::Output,
            5 => Instruction::Input,
            6 => Instruction::JumpFwd,
            7 => Instruction::JumpBack,
            8 => Instruction::DimUp,
            9 => Instruction::DimDown,
            10 => Instruction::ReadNeighbor,
            11 => Instruction::WriteNeighbor,
            12 => Instruction::CopySelf,
            13 => Instruction::Jump,
            14 => Instruction::Nop,
            _ => Instruction::Halt,
        }
    }

    pub fn to_color(&self) -> [u8; 3] {
        match self {
            Instruction::Right => [255, 100, 100],      // Red
            Instruction::Left => [100, 255, 100],       // Green
            Instruction::Inc => [100, 100, 255],        // Blue
            Instruction::Dec => [255, 255, 100],        // Yellow
            Instruction::Output => [255, 100, 255],     // Magenta
            Instruction::Input => [100, 255, 255],      // Cyan
            Instruction::JumpFwd => [255, 200, 100],    // Orange
            Instruction::JumpBack => [200, 100, 255],   // Purple
            Instruction::DimUp => [150, 255, 150],      // Light green
            Instruction::DimDown => [255, 150, 150],    // Light red
            Instruction::ReadNeighbor => [100, 200, 200], // Teal
            Instruction::WriteNeighbor => [200, 200, 100], // Olive
            Instruction::CopySelf => [255, 255, 255],   // White (important!)
            Instruction::Jump => [200, 150, 255],       // Lavender
            Instruction::Nop => [50, 50, 50],           // Dark gray
            Instruction::Halt => [0, 0, 0],             // Black
        }
    }
}

/// A program living at a coordinate
#[derive(Clone)]
pub struct Program {
    pub instructions: [u8; 64],
    pub data: [u8; 64],
    pub ip: usize,          // Instruction pointer
    pub dp: usize,          // Data pointer
    pub dim: u8,            // Current dimension focus (0-8)
    pub energy: u32,        // Steps remaining
    pub age: u64,           // Ticks survived
    pub generation: u64,    // Replication generation
}

impl Program {
    pub const TAPE_SIZE: usize = 64;
    pub const MAX_STEPS: u32 = 8192; // 2^13 like original

    /// Create random program
    pub fn random() -> Self {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
        let mut instructions = [0u8; 64];
        for i in 0..64 {
            instructions[i] = rng.gen_range(0..16);
        }
        Self {
            instructions,
            data: [0; 64],
            ip: 0,
            dp: 0,
            dim: 0,
            energy: Self::MAX_STEPS,
            age: 0,
            generation: 0,
        }
    }

    /// Create empty program
    pub fn empty() -> Self {
        Self {
            instructions: [14; 64], // All NOPs
            data: [0; 64],
            ip: 0,
            dp: 0,
            dim: 0,
            energy: Self::MAX_STEPS,
            age: 0,
            generation: 0,
        }
    }

    /// Reset execution state (keep instructions)
    pub fn reset(&mut self) {
        self.ip = 0;
        self.dp = 0;
        self.dim = 0;
        self.energy = Self::MAX_STEPS;
    }

    /// Get current instruction
    pub fn current_instruction(&self) -> Instruction {
        Instruction::from_byte(self.instructions[self.ip])
    }

    /// Execute one step, returns action to take
    pub fn step(&mut self) -> StepResult {
        if self.energy == 0 {
            return StepResult::Halted;
        }
        self.energy -= 1;

        let inst = self.current_instruction();
        let result = match inst {
            Instruction::Right => {
                self.dp = (self.dp + 1) % Self::TAPE_SIZE;
                StepResult::Continue
            }
            Instruction::Left => {
                self.dp = (self.dp + Self::TAPE_SIZE - 1) % Self::TAPE_SIZE;
                StepResult::Continue
            }
            Instruction::Inc => {
                self.data[self.dp] = self.data[self.dp].wrapping_add(1);
                StepResult::Continue
            }
            Instruction::Dec => {
                self.data[self.dp] = self.data[self.dp].wrapping_sub(1);
                StepResult::Continue
            }
            Instruction::Output => StepResult::Continue, // No-op for now
            Instruction::Input => StepResult::Continue,  // No-op for now
            Instruction::JumpFwd => {
                if self.data[self.dp] == 0 {
                    // Find matching ]
                    let mut depth = 1;
                    while depth > 0 && self.ip < Self::TAPE_SIZE - 1 {
                        self.ip += 1;
                        match Instruction::from_byte(self.instructions[self.ip]) {
                            Instruction::JumpFwd => depth += 1,
                            Instruction::JumpBack => depth -= 1,
                            _ => {}
                        }
                    }
                }
                StepResult::Continue
            }
            Instruction::JumpBack => {
                if self.data[self.dp] != 0 {
                    // Find matching [
                    let mut depth = 1;
                    while depth > 0 && self.ip > 0 {
                        self.ip -= 1;
                        match Instruction::from_byte(self.instructions[self.ip]) {
                            Instruction::JumpBack => depth += 1,
                            Instruction::JumpFwd => depth -= 1,
                            _ => {}
                        }
                    }
                }
                StepResult::Continue
            }
            Instruction::DimUp => {
                self.dim = (self.dim + 1) % 9;
                StepResult::Continue
            }
            Instruction::DimDown => {
                self.dim = (self.dim + 8) % 9; // +8 mod 9 = -1 mod 9
                StepResult::Continue
            }
            Instruction::ReadNeighbor => {
                let direction = if self.data[self.dp] < 128 { 1 } else { -1 };
                StepResult::ReadNeighbor { dim: self.dim, direction }
            }
            Instruction::WriteNeighbor => {
                let direction = if self.data[self.dp] < 128 { 1 } else { -1 };
                let value = self.data[(self.dp + 1) % Self::TAPE_SIZE];
                StepResult::WriteNeighbor { dim: self.dim, direction, value }
            }
            Instruction::CopySelf => {
                let direction = if self.data[self.dp] < 128 { 1 } else { -1 };
                StepResult::CopySelf { dim: self.dim, direction }
            }
            Instruction::Jump => {
                // Use data[dp..dp+9] as coordinate
                StepResult::Continue // TODO: implement coordinate jump
            }
            Instruction::Nop => StepResult::Continue,
            Instruction::Halt => StepResult::Halted,
        };

        // Advance IP
        self.ip = (self.ip + 1) % Self::TAPE_SIZE;
        result
    }

    /// Concatenate with another program (for interaction)
    pub fn concatenate(&self, other: &Program) -> Vec<u8> {
        let mut tape = Vec::with_capacity(128);
        tape.extend_from_slice(&self.instructions);
        tape.extend_from_slice(&other.instructions);
        tape
    }

    /// Create child (copy with generation increment)
    pub fn replicate(&self) -> Self {
        let mut child = self.clone();
        child.generation = self.generation + 1;
        child.age = 0;
        child.reset();
        child
    }

    /// Count CopySelf instructions (indicator of replicator)
    pub fn replicator_score(&self) -> usize {
        self.instructions.iter()
            .filter(|&&b| Instruction::from_byte(b) == Instruction::CopySelf)
            .count()
    }
}

/// Result of a single step
#[derive(Debug, Clone, Copy)]
pub enum StepResult {
    Continue,
    Halted,
    ReadNeighbor { dim: u8, direction: i8 },
    WriteNeighbor { dim: u8, direction: i8, value: u8 },
    CopySelf { dim: u8, direction: i8 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_program() {
        let p = Program::random();
        assert_eq!(p.instructions.len(), 64);
        assert_eq!(p.energy, Program::MAX_STEPS);
    }

    #[test]
    fn test_inc_dec() {
        let mut p = Program::empty();
        p.instructions[0] = Instruction::Inc as u8;
        p.instructions[1] = Instruction::Inc as u8;
        p.instructions[2] = Instruction::Dec as u8;

        p.step();
        assert_eq!(p.data[0], 1);
        p.step();
        assert_eq!(p.data[0], 2);
        p.step();
        assert_eq!(p.data[0], 1);
    }

    #[test]
    fn test_replicator_score() {
        let mut p = Program::empty();
        p.instructions[0] = Instruction::CopySelf as u8;
        p.instructions[10] = Instruction::CopySelf as u8;
        assert_eq!(p.replicator_score(), 2);
    }
}
