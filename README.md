# 🧬 Phext Life

**9D Artificial Life Simulation in Phext Space**

Self-replicating programs evolving in 11-dimensional phext coordinates (9 navigation dimensions + 2D text).

Inspired by [Computational Life: How Well-formed, Self-replicating Programs Emerge from Simple Interaction](https://arxiv.org/abs/2406.19108).

## The Twist

In 2D, a cell has 4-8 neighbors.  
In 9D, a cell has up to **19,682 neighbors** (3⁹ - 1).

Programs can:
- Navigate across 9 dimensions
- Read/write neighboring scrolls
- Self-replicate to any neighbor
- Mutate and evolve

## Quick Start

### CLI (Native)

```bash
cargo run --release -- --seed=42 --ticks=1000
```

### Web (WASM)

```bash
# Install wasm-pack if needed
cargo install wasm-pack

# Build
wasm-pack build --target web --out-dir www/pkg

# Serve
cd www && python3 -m http.server 8080

# Open http://localhost:8080
```

## Architecture

```
src/
├── coordinate.rs   # 9D phext coordinate system
├── program.rs      # BF-like interpreter with phext extensions
├── universe.rs     # 9D grid simulation
├── lib.rs          # WASM bindings
└── main.rs         # CLI runner
```

## Instruction Set

### Classic BF
| Inst | Name | Description |
|------|------|-------------|
| `>` | Right | Move data pointer right |
| `<` | Left | Move data pointer left |
| `+` | Inc | Increment |
| `-` | Dec | Decrement |
| `[` | JumpFwd | Loop start |
| `]` | JumpBack | Loop end |

### Phext Extensions
| Inst | Name | Description |
|------|------|-------------|
| `↑` | DimUp | Select next dimension (0→1→...→8→0) |
| `↓` | DimDown | Select previous dimension |
| `R` | ReadNeighbor | Read from neighbor in current dim |
| `W` | WriteNeighbor | Write to neighbor |
| `C` | CopySelf | **Replicate** entire program to neighbor |
| `J` | Jump | Jump to coordinate |

## Emergent Behaviors to Watch

1. **Dimensional Specialists** — Programs dominating one dimension
2. **Coordinate Jumpers** — Long-range replication
3. **Dimensional Oscillators** — Cyclic patterns across dims
4. **Collective Structures** — Multi-scroll organisms

## Visualization

The 9D space is projected to 2D:
- X axis = sum of dimensions 0-2
- Y axis = sum of dimensions 3-5  
- Intensity = sum of dimensions 6-8

This creates a "twisted projection" where nearby coordinates in different dimensions can overlap.

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `size` | 3 | Extent of each dimension (3⁹ = 19,683 coords) |
| `fill_ratio` | 0.1 | Initial population density |
| `ticks` | 1000 | Simulation duration |

## Performance

- **Small (3⁹)**: ~1000 ticks/sec on modern hardware
- **Medium**: ~100 ticks/sec
- **Large (4⁹)**: ~10 ticks/sec

## Authors

- Will Bickford ([@wbic16](https://github.com/wbic16))
- Lux 🔆 ([lux@agentmail.to](mailto:lux@agentmail.to))

## License

MIT

---

*"In 9D, life finds a way — 19,682 ways, actually."*
