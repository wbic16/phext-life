/// Phext Life — WASM bindings for browser canvas rendering
/// Exposes the 9D universe to JavaScript via wasm-bindgen.

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

mod coordinate;
mod program;
mod universe;

use coordinate::Coordinate;
use program::Instruction;
use universe::Universe;

#[wasm_bindgen]
pub struct PhextLife {
    universe: Universe,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl PhextLife {
    /// Create a new simulation. extents: [d0,d1,...,d8] each 1–9.
    /// canvas_w, canvas_h: pixel dimensions of target canvas.
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_w: u32, canvas_h: u32) -> Self {
        console_error_panic_hook::set_once();
        // Medium universe: 5^3 * 3^3 * 2^3 = 27,000 programs
        let universe = Universe::medium();
        Self { universe, width: canvas_w, height: canvas_h }
    }

    /// Run one simulation tick.
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.universe.tick();
    }

    /// Run N ticks at once (for speed).
    #[wasm_bindgen]
    pub fn tick_n(&mut self, n: u32) {
        for _ in 0..n { self.universe.tick(); }
    }

    /// Current tick number.
    #[wasm_bindgen]
    pub fn ticks(&self) -> u64 { self.universe.tick }

    /// Current population.
    #[wasm_bindgen]
    pub fn population(&self) -> usize { self.universe.population() }

    /// Total possible coordinates.
    #[wasm_bindgen]
    pub fn total_coordinates(&self) -> usize { self.universe.total_coordinates() }

    /// Total replication events.
    #[wasm_bindgen]
    pub fn total_replications(&self) -> u64 { self.universe.total_replications }

    /// Render the 9D universe to RGBA pixel data using a twisted projection.
    ///
    /// Projection: Z-order (Morton) curve over the 9 dimensions →
    /// a space-filling 2D image that preserves locality.
    /// Each pixel = one program slot; color = dominant instruction in that cell.
    /// Black = empty.
    #[wasm_bindgen]
    pub fn render(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let w = self.width as usize;
        let h = self.height as usize;
        let mut pixels = vec![0u8; w * h * 4];

        let extents = self.universe.extents;
        let total = self.universe.total_coordinates();

        // Twisted projection: map each coordinate's linear index →
        // 2D position via bit-interleaving (Z-order / Morton curve).
        // This preserves dimensional locality: neighbors in 9D land near
        // each other in 2D, unlike a naive row-major scan.
        for (coord, program) in &self.universe.programs {
            let idx = coord.to_index(&extents);

            // Morton-encode the linear index into 2D
            let (px, py) = morton_to_2d(idx, total, w, h);

            if px < w && py < h {
                // Color = instruction at IP, or dominant instruction
                let instr = Instruction::from_byte(program.instructions[program.ip]);
                let color = instruction_color(&instr, program.generation);
                let offset = (py * w + px) * 4;
                pixels[offset]     = color[0];
                pixels[offset + 1] = color[1];
                pixels[offset + 2] = color[2];
                pixels[offset + 3] = 255;
            }
        }

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&pixels), w as u32, h as u32
        )?;
        ctx.put_image_data(&image_data, 0.0, 0.0)
    }
}

/// Morton (Z-order) curve: interleave bits of x and y to get a 1D index
/// that preserves 2D locality. We use the inverse: given a 1D index (from
/// the 9D linear index), decode to (x, y) by deinterleaving bits.
fn morton_to_2d(idx: usize, total: usize, w: usize, h: usize) -> (usize, usize) {
    // Scale idx to [0, w*h)
    let scaled = (idx as u64 * (w * h) as u64) / total.max(1) as u64;
    let scaled = scaled as usize;

    // Deinterleave: even bits → x, odd bits → y
    let mut x = 0usize;
    let mut y = 0usize;
    for bit in 0..16 {
        x |= ((scaled >> (2 * bit)) & 1) << bit;
        y |= ((scaled >> (2 * bit + 1)) & 1) << bit;
    }
    (x % w, y % h)
}

/// Map instruction to vivid RGB color. Replicators (CopySelf) pulse bright white.
fn instruction_color(instr: &Instruction, generation: u64) -> [u8; 3] {
    // Generation tints older lineages slightly warmer
    let gen_factor = (generation.min(10) as f32) / 10.0;
    let tint = (gen_factor * 30.0) as u8;

    match instr {
        Instruction::Right        => [255,  80+tint,  80],
        Instruction::Left         => [ 80, 255,  80+tint],
        Instruction::Inc          => [ 80,  80+tint, 255],
        Instruction::Dec          => [255, 220, 100],
        Instruction::Output       => [255, 100, 200+tint],
        Instruction::Input        => [100, 220, 255],
        Instruction::JumpFwd      => [255, 160,  50+tint],
        Instruction::JumpBack     => [180,  80, 255],
        Instruction::DimUp        => [120, 255, 140+tint],  // phext-specific: green
        Instruction::DimDown      => [255, 120, 140+tint],  // phext-specific: red
        Instruction::ReadNeighbor => [ 80, 200, 210+tint],
        Instruction::WriteNeighbor=> [200, 200,  80+tint],
        Instruction::CopySelf     => [255, 255, 255],       // WHITE — replicators!
        Instruction::Jump         => [190, 130, 255+tint],
        Instruction::Nop          => [ 40,  40,  40],
        Instruction::Halt         => [  0,   0,   0],
    }
}
