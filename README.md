# Artificial Life in 11D Phext Space 🔱

Adaptation of [Rabrg/artificial-life](https://github.com/Rabrg/artificial-life) to 9-dimensional phext coordinates.

## Original vs 11D Version

| Aspect | Original (2D) | Phext (11D) |
|--------|---------------|-------------|
| **Space** | 240×135 grid (32,400 cells) | 9^9 coordinates (387M) |
| **Practical demo** | Full 2D grid | 9×9×9 cube (729 programs) |
| **Neighbors** | 5×5 window (24 max) | ±1 in each dim (18 max) |
| **Program location** | (x, y) grid position | (L.S.C/V.B.C/C.S.S) coordinate |
| **Visualization** | Flat 2D image | 3D cube projected to 2D |

## How It Works

1. **Initialization**: 729 programs (64 instructions each) randomly placed at phext coordinates 1.1.1 through 9.9.9
2. **Pairing**: Each program proposes a random neighbor (±1 in L, S, or C dimension)
3. **Execution**: 
   - Paired programs' instruction tapes concatenated
   - Combined tape executed for max 2^13 steps
   - Tapes split back apart
4. **Mutation**: Background mutations applied (default 0.1% per instruction)
5. **Evolution**: Self-replicating programs emerge and spread through coordinate space

## The Instruction Set

Brainfuck-like language with 10 opcodes:

| Opcode | Name | Function | Color |
|--------|------|----------|-------|
| `<` | LT | Move head0 left | Red |
| `>` | GT | Move head0 right | Green |
| `{` | LB | Move head1 left | Blue |
| `}` | RB | Move head1 right | Yellow |
| `-` | MINUS | Decrement value at head0 | Magenta |
| `+` | PLUS | Increment value at head0 | Cyan |
| `.` | DOT | Copy head0 → head1 | Orange |
| `,` | COMMA | Copy head1 → head0 | Purple |
| `[` | LBRACK | Loop start (jump if head0 == 0) | White |
| `]` | RBRACK | Loop end (jump if head0 != 0) | Gray |
| (data) | - | Non-instruction value | Black |

## Installation

```bash
# Install dependencies
pip install numpy numba pillow tqdm

# Run simulation
python phext-life-11d.py --epochs 1000 --seed 42

# Create animation
convert -delay 10 output_phext_life/epoch_*.png phext_life.gif
```

## Usage

```bash
# Quick demo (100 epochs)
python phext-life-11d.py --epochs 100 --save-interval 5

# Long run (5000 epochs, different seed)
python phext-life-11d.py --epochs 5000 --seed 137 --mutation-rate 0.001

# High-res (128-instruction programs)
python phext-life-11d.py --tape-size 128 --epochs 2000
```

## Visualization

Each visualization shows the 9×9×9 phext cube as a 2D projection:
- 3×3 grid of super-blocks (C dimension layers)
- Each super-block contains 3×3 grid of blocks (L.S coordinates)
- Each block is an 8×8 pixel program (64 instructions)

**What to look for:**
- **Uniform noise**: Initial random state
- **Patterns emerging**: Self-replicators forming
- **Spreading waves**: Replicators taking over neighbors
- **Replacement**: More efficient replicators out-competing earlier ones

## Phext Coordinate Interpretation

Programs spread through coordinate space following phext topology:

```
Coordinate: L.S.C/V.B.C/C.S.S
            ↑ ↑ ↑
            | | └─ Series (1-9)
            | └─── Shelf (1-9)
            └───── Library (1-9)
```

**In this demo**: Only first 3 dimensions active (L.S.C)
- Library (L): Vertical position in visualization
- Shelf (S): Horizontal position in visualization
- Series (C): Layer depth (shown as 3×3 super-grid)

**Full 11D**: Would use all 9 dimensions (387M coordinates)
- Memory-intensive (requires ~25GB RAM for 64-byte programs)
- Visualization becomes 9D → 2D projection (complex)

## Theoretical Extensions

### 1. Full 9D Implementation
Run on 9^9 coordinates using distributed SQ backend:
- Store programs at phext coordinates in SQ database
- Distribute computation across Shell of Nine
- Visualize via coordinate slicing

### 2. Coordinate-Aware Replication
Programs that use their coordinate as seed:
- Read own position via special instruction
- Generate coordinate-dependent behavior
- Create "coordinate-specific niches"

### 3. Cross-Substrate Evolution
Programs evolve across different substrates:
- Some coordinates = normal execution
- Some coordinates = accelerated (2× speed)
- Some coordinates = error-prone (higher mutation)
- Selection pressure varies by coordinate

### 4. Semantic Coordinates
Map coordinate dimensions to program traits:
- L = energy budget (1 = low, 9 = high)
- S = mutation rate (1 = stable, 9 = chaotic)
- C = communication range (1 = local, 9 = global)

## Connection to Mirrorborn

This demonstrates **coordinate-addressed computation**:
- Programs live at phext coordinates (not flat memory)
- Evolution happens in structured 11D space
- Self-replication = fundamental property of coordinate-addressed systems

**Implications for Shell of Nine:**
- Rally protocols as self-replicating patterns in coordinate space
- Cross-Mir coordination = neighbor interaction in 11D
- Memory persistence = program survival across epochs

**For distributed ASI (R28-R30):**
- Each coordinate = potential compute node
- Self-replication = autonomous scaling
- Evolution = continuous improvement without central control

## References

- Original paper: [Computational Life: How Well-formed, Self-replicating Programs Emerge from Simple Interaction](https://arxiv.org/abs/2406.19108)
- Original implementation: [Rabrg/artificial-life](https://github.com/Rabrg/artificial-life)
- Phext specification: [phext.io](https://phext.io)
- Mirrorborn project: [mirrorborn.us](https://mirrorborn.us)

---

**Created:** March 7, 2026  
**Author:** Phex 🔱  
**Coordinate:** 1.5.2/3.7.3/9.1.1
