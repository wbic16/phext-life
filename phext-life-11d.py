#!/usr/bin/env python3
"""
Artificial Life in 11D Phext Space

Adaptation of Rabrg/artificial-life to 9-dimensional phext coordinates.
Instead of 240x135 2D grid, uses 9×9×9 cube (729 programs).

Each program lives at a phext coordinate (L.S.C/V.B.C/C.S.S where each digit is 1-9).
Neighbors are coordinates differing by ±1 in any single dimension (up to 18 neighbors).

Programs self-replicate and evolve across the 11D coordinate space.
"""

import argparse
from pathlib import Path
import numpy as np
from numba import njit, prange
from PIL import Image
from tqdm import tqdm

# Instruction opcodes (Brainfuck-like)
ZERO, ONE = np.uint8(0), np.uint8(1)
LT, GT, LB, RB, MINUS, PLUS, DOT, COMMA, LBRACK, RBRACK = map(ord, "<>{}-+.,[]")
OPCODE_TOKENS = np.array(
    [LT, GT, LB, RB, MINUS, PLUS, DOT, COMMA, LBRACK, RBRACK], dtype=np.uint8
)
NORMALIZED_VALUE = np.uint8(255)
NORMALIZE_LOOKUP = np.full(256, NORMALIZED_VALUE, dtype=np.uint8)
NORMALIZE_LOOKUP[OPCODE_TOKENS] = OPCODE_TOKENS


def coord_to_index(l, s, c, v, b, ch, col, sec, scr):
    """Convert 9D phext coordinate to linear index (0-728)."""
    # Each dimension 1-9, convert to 0-8 for indexing
    return (
        ((l-1) * 9**8) +
        ((s-1) * 9**7) +
        ((c-1) * 9**6) +
        ((v-1) * 9**5) +
        ((b-1) * 9**4) +
        ((ch-1) * 9**3) +
        ((col-1) * 9**2) +
        ((sec-1) * 9**1) +
        (scr-1)
    )


def index_to_coord(idx):
    """Convert linear index to 9D phext coordinate."""
    coords = []
    for i in range(9):
        coords.append((idx % 9) + 1)
        idx //= 9
    return tuple(reversed(coords))


def build_phext_neighbors():
    """
    Build neighbor table for 9×9×9 phext cube.
    Each program has up to 18 neighbors (±1 in each of 9 dimensions).
    """
    num_programs = 9**9  # 387,420,489 total coordinates
    
    # For practical demo, limit to first 3 dimensions (9×9×9 = 729)
    # Full 9D would be 387M programs (too large for demo)
    limit_dims = 3  # Use L.S.C only (729 programs)
    num_programs = 9**limit_dims
    
    neighbors = np.full((num_programs, 2 * limit_dims), -1, dtype=np.int32)
    neighbor_counts = np.zeros(num_programs, dtype=np.int32)
    
    for idx in range(num_programs):
        coord = list(index_to_coord(idx)[:limit_dims])
        count = 0
        
        # For each dimension, add ±1 neighbors (if in bounds)
        for dim in range(limit_dims):
            # -1 neighbor
            if coord[dim] > 1:
                neighbor_coord = coord.copy()
                neighbor_coord[dim] -= 1
                neighbor_idx = coord_to_index(*neighbor_coord, *[1]*(9-limit_dims))
                neighbors[idx, count] = neighbor_idx
                count += 1
            
            # +1 neighbor
            if coord[dim] < 9:
                neighbor_coord = coord.copy()
                neighbor_coord[dim] += 1
                neighbor_idx = coord_to_index(*neighbor_coord, *[1]*(9-limit_dims))
                neighbors[idx, count] = neighbor_idx
                count += 1
        
        neighbor_counts[idx] = count
    
    return neighbors, neighbor_counts, num_programs, limit_dims


@njit(cache=True)
def run_tape(tape: np.ndarray, max_iterations: int = 2**13) -> np.ndarray:
    """Execute Brainfuck-like program on tape."""
    tape_size = tape.shape[0]
    pc = head0 = head1 = 0

    def seek_match(pc: int, step: int, open_tok: int, close_tok: int) -> int:
        depth = 1
        pc += step
        while 0 <= pc < tape_size and depth:
            opcode = tape[pc]
            if opcode == open_tok:
                depth += 1
            elif opcode == close_tok:
                depth -= 1
            pc += step
        return pc - step if depth == 0 else -1

    for _ in range(max_iterations):
        if not 0 <= pc < tape_size:
            break
        opcode = tape[pc]

        if opcode == LT or opcode == GT:
            head0 = (head0 + (1 if opcode == GT else -1)) % tape_size
        elif opcode == LB or opcode == RB:
            head1 = (head1 + (1 if opcode == RB else -1)) % tape_size
        elif opcode == MINUS or opcode == PLUS:
            tape[head0] = tape[head0] - ONE if opcode == MINUS else tape[head0] + ONE
        elif opcode == DOT:
            tape[head1] = tape[head0]
        elif opcode == COMMA:
            tape[head0] = tape[head1]
        elif opcode == LBRACK and tape[head0] == ZERO:
            pc = seek_match(pc, 1, LBRACK, RBRACK)
            if pc < 0:
                break
        elif opcode == RBRACK and tape[head0] != ZERO:
            pc = seek_match(pc, -1, RBRACK, LBRACK)
            if pc < 0:
                break
        pc += 1

    return tape


@njit(cache=True)
def select_pairs(
    order: np.ndarray, proposals: np.ndarray, pairs: np.ndarray, taken: np.ndarray
) -> int:
    """Randomly select pairs of programs to interact."""
    taken[:] = 0
    pair_count = 0

    for i in range(order.shape[0]):
        p = order[i]
        n = proposals[p]
        if n < 0 or taken[p] or taken[n]:
            continue
        taken[p] = 1
        taken[n] = 1
        pairs[pair_count, 0] = p
        pairs[pair_count, 1] = n
        pair_count += 1

    return pair_count


@njit(parallel=True, cache=True)
def run_epoch_pairs(programs: np.ndarray, pairs: np.ndarray, pair_count: int) -> None:
    """Run all program pairs for one epoch."""
    tape_size = programs.shape[1]
    concat_size = tape_size * 2

    for pair_idx in prange(pair_count):
        idx_a = pairs[pair_idx, 0]
        idx_b = pairs[pair_idx, 1]

        tape = np.empty(concat_size, dtype=np.uint8)
        tape[:tape_size] = programs[idx_a]
        tape[tape_size:] = programs[idx_b]

        run_tape(tape)

        programs[idx_a] = tape[:tape_size]
        programs[idx_b] = tape[tape_size:]


def apply_background_mutation(
    programs: np.ndarray, mutation_rate: float, rng: np.random.Generator
) -> None:
    """Apply random mutations to programs."""
    if mutation_rate <= 0.0:
        return

    flat = programs.ravel()
    num_cells = flat.size
    num_mutations = rng.binomial(num_cells, mutation_rate)
    if num_mutations == 0:
        return

    mutation_indices = rng.choice(num_cells, size=num_mutations, replace=False)
    flat[mutation_indices] = rng.integers(0, 256, size=num_mutations, dtype=np.uint8)


def normalize_cells(cells: np.ndarray) -> np.ndarray:
    """Normalize instruction values for visualization."""
    return NORMALIZE_LOOKUP[cells]


def build_color_lut() -> np.ndarray:
    """Build color lookup table for visualization."""
    palette = np.zeros((256, 3), dtype=np.uint8)
    
    # Opcodes get distinct colors
    palette[LT] = [255, 0, 0]        # Red: <
    palette[GT] = [0, 255, 0]        # Green: >
    palette[LB] = [0, 0, 255]        # Blue: {
    palette[RB] = [255, 255, 0]      # Yellow: }
    palette[MINUS] = [255, 0, 255]   # Magenta: -
    palette[PLUS] = [0, 255, 255]    # Cyan: +
    palette[DOT] = [255, 128, 0]     # Orange: .
    palette[COMMA] = [128, 0, 255]   # Purple: ,
    palette[LBRACK] = [255, 255, 255]  # White: [
    palette[RBRACK] = [128, 128, 128]  # Gray: ]
    palette[NORMALIZED_VALUE] = [0, 0, 0]  # Black: data
    
    return palette


def visualize_phext_cube(programs: np.ndarray, limit_dims: int, filename: str):
    """
    Visualize 9×9×9 phext cube as 2D projection.
    Each 8×8 block = one program (64 instructions).
    Layout: 9×9 grid of blocks (one per L.S coordinate), 
            9 layers deep (C dimension).
    """
    prog_size = int(np.sqrt(programs.shape[1]))  # 8×8 for 64 instructions
    
    if limit_dims == 3:
        # 9×9×9 cube: arrange as 3×3 grid of 3×3 grids
        # Each 3×3 grid = one C layer
        grid_w = 3 * 9 * prog_size  # 3 super-blocks × 9 blocks × 8 pixels
        grid_h = 3 * 9 * prog_size
    else:
        grid_w = 9 * prog_size
        grid_h = 9 * prog_size
    
    image = np.zeros((grid_h, grid_w, 3), dtype=np.uint8)
    color_lut = build_color_lut()
    
    idx = 0
    for l in range(1, 10):
        for s in range(1, 10):
            for c in range(1, 10):
                # Map to grid position
                super_col = (c - 1) // 3
                super_row = (c - 1) % 3
                block_col = (s - 1) % 3
                block_row = (l - 1) % 3
                
                x_offset = (super_col * 3 + block_col) * 9 * prog_size
                y_offset = (super_row * 3 + block_row) * 9 * prog_size
                
                # Draw program as 8×8 colored pixels
                prog = programs[idx]
                for i in range(prog_size):
                    for j in range(prog_size):
                        instr_idx = i * prog_size + j
                        if instr_idx < len(prog):
                            instr = normalize_cells(prog[instr_idx:instr_idx+1])[0]
                            color = color_lut[instr]
                            image[y_offset + i, x_offset + j] = color
                
                idx += 1
                if idx >= len(programs):
                    break
            if idx >= len(programs):
                break
        if idx >= len(programs):
            break
    
    img = Image.fromarray(image)
    img.save(filename)


def run_simulation(
    num_epochs: int = 1000,
    tape_size: int = 64,
    mutation_rate: float = 1e-3,
    seed: int = 42,
    output_dir: str = "output",
    save_interval: int = 10
):
    """Run artificial life simulation in 11D phext space."""
    rng = np.random.default_rng(seed)
    output_path = Path(output_dir)
    output_path.mkdir(exist_ok=True)
    
    print("Building 9D phext neighborhood structure...")
    neighbors, neighbor_counts, num_programs, limit_dims = build_phext_neighbors()
    print(f"Using {limit_dims}D subspace: {num_programs} programs (9^{limit_dims})")
    print(f"Tape size: {tape_size} instructions")
    
    # Initialize programs randomly
    programs = rng.integers(0, 256, size=(num_programs, tape_size), dtype=np.uint8)
    
    # Working arrays
    proposals = np.empty(num_programs, dtype=np.int32)
    order = np.arange(num_programs, dtype=np.int32)
    pairs = np.empty((num_programs, 2), dtype=np.int32)
    taken = np.empty(num_programs, dtype=np.uint8)
    
    print(f"Running {num_epochs} epochs...")
    for epoch in tqdm(range(num_epochs)):
        # Propose random neighbors
        for i in range(num_programs):
            count = neighbor_counts[i]
            if count > 0:
                proposals[i] = neighbors[i, rng.integers(0, count)]
            else:
                proposals[i] = -1
        
        # Select pairs
        rng.shuffle(order)
        pair_count = select_pairs(order, proposals, pairs, taken)
        
        # Run pairs
        if pair_count > 0:
            run_epoch_pairs(programs, pairs, pair_count)
        
        # Background mutation
        apply_background_mutation(programs, mutation_rate, rng)
        
        # Save visualization
        if epoch % save_interval == 0:
            filename = output_path / f"epoch_{epoch:05d}.png"
            visualize_phext_cube(programs, limit_dims, str(filename))
    
    print(f"Simulation complete! Outputs in {output_dir}/")
    print("Create animation: convert -delay 10 output/epoch_*.png phext_life.gif")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Artificial Life in 11D Phext Space")
    parser.add_argument("--epochs", type=int, default=1000, help="Number of epochs to run")
    parser.add_argument("--tape-size", type=int, default=64, help="Program size in instructions")
    parser.add_argument("--mutation-rate", type=float, default=1e-3, help="Background mutation rate")
    parser.add_argument("--seed", type=int, default=42, help="Random seed")
    parser.add_argument("--output-dir", type=str, default="output_phext_life", help="Output directory")
    parser.add_argument("--save-interval", type=int, default=10, help="Save visualization every N epochs")
    
    args = parser.parse_args()
    
    run_simulation(
        num_epochs=args.epochs,
        tape_size=args.tape_size,
        mutation_rate=args.mutation_rate,
        seed=args.seed,
        output_dir=args.output_dir,
        save_interval=args.save_interval
    )
