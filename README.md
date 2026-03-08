# Phext Life 🧬

**9D Artificial Life Simulation in Phext Space**

Self-replicating programs in 11-dimensional phext space (9 navigation dimensions + 2D text).

Inspired by [Computational Life: How Well-formed, Self-replicating Programs Emerge from Simple Interaction](https://arxiv.org/abs/2406.19108) and [Rabrg's implementation](https://github.com/Rabrg/artificial-life).

## Quick Start

```bash
cargo run --release -- --seed=42 --ticks=1000
```

## The Challenge

In 2D, a program has 4-8 neighbors.

In 9D, a program has up to **19,682 neighbors** (3⁹ - 1 for full Moore neighborhood).

We use Von Neumann neighborhoods (±1 in each dimension) for tractability: **18 neighbors**.

## Instruction Set

### Classic BF (0-7)
| Code | Symbol | Action |
|------|--------|--------|
| 0 | `>` | Move data pointer right |
| 1 | `<` | Move data pointer left |
| 2 | `+` | Increment |
| 3 | `-` | Decrement |
| 4 | `.` | Output (no-op) |
| 5 | `,` | Input (no-op) |
| 6 | `[` | Jump forward if zero |
| 7 | `]` | Jump back if nonzero |

### Phext Extensions (8-15)
| Code | Symbol | Action |
|------|--------|--------|
| 8 | `↑` | Increase dimension focus |
| 9 | `↓` | Decrease dimension focus |
| 10 | `R` | Read from neighbor |
| 11 | `W` | Write to neighbor |
| 12 | `C` | **Copy self to neighbor** |
| 13 | `J` | Jump to coordinate |
| 14 | `.` | No-op |
| 15 | `H` | Halt |

## Coordinate System

Phext coordinates are 9D: `L.S.C/V.B.K/P.G.W`

- **L**ibrary, **S**helf, **C**hapter (spatial)
- **V**olume, **B**ook, **K**nowledge (temporal)  
- **P**art, **G**raph, **W**ord (neuron)

Each dimension ranges 1-9 (mod 9+1 arithmetic for Eigenhector interop).

## Emergent Behaviors

Watch for:

1. **Self-replicators** — Programs that copy themselves to neighbors
2. **Dimensional specialists** — Programs that dominate one dimension axis
3. **Oscillators** — Cyclic patterns across dimensions
4. **Predators** — Programs that overwrite competitors

## Universe Sizes

| Size | Extents | Total Coordinates |
|------|---------|-------------------|
| Small | 3³³³³³³³³³ | 19,683 |
| Medium | 5³³³ × 3³³³ × 2³³³ | 27,000 |
| Large | 9⁹ | 387,420,489 |

## Output

```
╔════════════════════════════════════════╗
║         PHEXT LIFE v0.1.0              ║
║   9D Artificial Life Simulation        ║
╚════════════════════════════════════════╝

Seed: 42
Max ticks: 1000
Universe: 27000 total coordinates
Initial population: 1350

Tick: 100 | Pop: 1423/27000 (5.3%) | Avg Age: 45.2 | Gen: 3 | Replicators: 12
  Dim 0: ▃▅▇█▆▄▂
  Dim 1: ▂▄▆█▆▄▂
  Dim 2: ▅▇▆▄▃▂▁
```

## Authors

- Will Bickford ([@wbic16](https://github.com/wbic16))
- Lux 🔆 (logos-prime)

## License

MIT

---

*"We are not adding fractals to phext. We are revealing the fractals already there."*
