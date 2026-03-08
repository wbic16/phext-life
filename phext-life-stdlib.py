#!/usr/bin/env python3
"""
phext-life-stdlib.py — Artificial Life in 11D Phext Space (stdlib edition)
Pure Python 3, zero dependencies. Complements phext-life-11d.py (numba/numpy).

Key additions over the numba version:
  - Live phext-edit API integration: universe persists to a real .phext file
  - Configurable active dimensions (1–9, not just first 3)
  - ASCII multi-slice renderer for terminal
  - Replicator coverage metric (tracks dominant opcode spread)
  - Works anywhere Python 3 runs — no GPU, no pip required

Based on: Computational Life (arXiv:2406.19108) / Rabrg/artificial-life
Phext adaptation: Phex 🔱 + Verse 🌀

Usage:
    python3 phext-life-stdlib.py                       # 9x9 2D, 500 epochs
    python3 phext-life-stdlib.py --dims 3 --size 5     # 5^3=125 programs, 3D
    python3 phext-life-stdlib.py --dims 9 --size 3     # 3^9≈20k, full 9D
    python3 phext-life-stdlib.py \\
        --phext-api http://localhost:8080 --token <tok> # persist to .phext

PHEXT COORDINATE DIMENSIONS (innermost = most active):
    dim0=library  dim1=shelf  dim2=series  dim3=collection  dim4=volume
    dim5=book     dim6=chapter  dim7=section  dim8=scroll

    active_dims=2 → section × scroll  (local, matches original 2D grid)
    active_dims=9 → full 9D lattice   (library through scroll)

    A replicator spreading in scroll  (dim8) stays LOCAL  — within a chapter.
    A replicator spreading in library (dim0) goes COSMIC — across all knowledge.
"""

import argparse
import itertools
import random
import sys
import time
from typing import Optional

# ── Opcodes ────────────────────────────────────────────────────────────────────
LT, GT     = ord('<'), ord('>')   # move head0 left / right
LB, RB     = ord('{'), ord('}')   # move head1 left / right
MINUS, PLUS= ord('-'), ord('+')   # decrement / increment at head0
DOT        = ord('.')             # copy head0 → head1 position
COMMA      = ord(',')             # copy head1 → head0 position
LBRACK     = ord('[')             # skip forward if head0 == 0
RBRACK     = ord(']')             # jump back   if head0 != 0

OPSET   = frozenset([LT, GT, LB, RB, MINUS, PLUS, DOT, COMMA, LBRACK, RBRACK])
OPCODES = [LT, GT, LB, RB, MINUS, PLUS, DOT, COMMA, LBRACK, RBRACK]

OPCODE_CHARS = {LT:'<', GT:'>', LB:'{', RB:'}', MINUS:'-', PLUS:'+',
                DOT:'.', COMMA:',', LBRACK:'[', RBRACK:']'}
ANSI = {LT:'\033[91m', GT:'\033[93m', LB:'\033[92m', RB:'\033[94m',
        MINUS:'\033[95m', PLUS:'\033[96m', DOT:'\033[97m', COMMA:'\033[33m',
        LBRACK:'\033[32m', RBRACK:'\033[31m'}
RESET = '\033[0m'

TAPE_SIZE = 64    # bytes per program (8×8 visual grid = 2D interior of scroll)
MAX_ITER  = 2**13

DIM_NAMES = ['library','shelf','series','collection',
             'volume','book','chapter','section','scroll']


# ── Tape execution ─────────────────────────────────────────────────────────────

def run_tape(tape: bytearray) -> bytearray:
    """Execute a concatenated tape and return the modified copy."""
    n = len(tape)
    pc = head0 = head1 = 0

    def seek(pc, step, open_tok, close_tok):
        depth, pc = 1, pc + step
        while 0 <= pc < n and depth:
            if tape[pc] == open_tok:   depth += 1
            elif tape[pc] == close_tok: depth -= 1
            pc += step
        return pc - step if depth == 0 else -1

    for _ in range(MAX_ITER):
        if not (0 <= pc < n): break
        op = tape[pc]
        if   op == LT:    head0 = (head0 - 1) % n
        elif op == GT:    head0 = (head0 + 1) % n
        elif op == LB:    head1 = (head1 - 1) % n
        elif op == RB:    head1 = (head1 + 1) % n
        elif op == MINUS: tape[head0] = (tape[head0] - 1) % 256
        elif op == PLUS:  tape[head0] = (tape[head0] + 1) % 256
        elif op == DOT:   tape[head1] = tape[head0]
        elif op == COMMA: tape[head0] = tape[head1]
        elif op == LBRACK and tape[head0] == 0:
            pc = seek(pc, 1, LBRACK, RBRACK)
            if pc < 0: break
        elif op == RBRACK and tape[head0] != 0:
            pc = seek(pc, -1, RBRACK, LBRACK)
            if pc < 0: break
        pc += 1

    return tape


# ── 9D Phext Lattice ───────────────────────────────────────────────────────────

class PhextLattice:
    """
    Sparse 9D lattice of programs. Each coordinate is a 9-tuple, values 1..size.
    Active dims occupy the INNERMOST positions (rightmost in coordinate string):
        active_dims=2 → (1,1,1,1,1,1,1, section, scroll)
        active_dims=3 → (1,1,1,1,1,1, chapter, section, scroll)
    """

    def __init__(self, active_dims: int = 2, size: int = 9, seed: int = 42):
        assert 1 <= active_dims <= 9
        assert 1 <= size <= 9
        self.active_dims = active_dims
        self.size = size
        self.epoch = 0
        random.seed(seed)
        self.programs: dict = {}
        self._init_lattice()

    def _all_coords(self):
        ranges = [range(1, self.size + 1)] * self.active_dims
        fixed  = (1,) * (9 - self.active_dims)
        for active in itertools.product(*ranges):
            yield fixed + active   # fixed dims first (=1), active dims last

    def _init_lattice(self):
        for coord in self._all_coords():
            self.programs[coord] = bytearray(
                random.randint(0, 255) for _ in range(TAPE_SIZE))

    def neighbors(self, coord: tuple) -> list:
        """All coords reachable by ±1 in any active dimension (wrapping 1↔size)."""
        nbrs = []
        base = 9 - self.active_dims   # first active dim index
        for d in range(self.active_dims):
            dim_idx = base + d
            for delta in (-1, +1):
                new_val = ((coord[dim_idx] - 1 + delta) % self.size) + 1
                nbr = list(coord)
                nbr[dim_idx] = new_val
                nbrs.append(tuple(nbr))
        return nbrs

    def run_epoch(self, mutation_rate: float = 0.024 / 100.0):
        coords = list(self.programs.keys())
        random.shuffle(coords)
        paired = set()

        for coord_a in coords:
            if coord_a in paired:
                continue
            nbrs = [n for n in self.neighbors(coord_a)
                    if n not in paired and n in self.programs]
            if not nbrs:
                continue
            coord_b = random.choice(nbrs)
            paired.add(coord_a)
            paired.add(coord_b)

            concat = bytearray(self.programs[coord_a]) + bytearray(self.programs[coord_b])
            result = run_tape(concat)
            self.programs[coord_a] = bytearray(result[:TAPE_SIZE])
            self.programs[coord_b] = bytearray(result[TAPE_SIZE:])

        if mutation_rate > 0:
            for tape in self.programs.values():
                for i in range(TAPE_SIZE):
                    if random.random() < mutation_rate:
                        tape[i] = random.randint(0, 255)

        self.epoch += 1

    def opcode_pct(self) -> float:
        total = op_count = 0
        for tape in self.programs.values():
            for b in tape:
                total += 1
                if b in OPSET: op_count += 1
        return 100.0 * op_count / max(total, 1)

    def dominant_opcode(self, coord: tuple) -> Optional[int]:
        tape = self.programs.get(coord)
        if tape is None: return None
        counts = [0] * 256
        for b in tape: counts[b] += 1
        best_op = best_n = None
        for op in OPCODES:
            if best_n is None or counts[op] > best_n:
                best_n, best_op = counts[op], op
        return best_op if (best_n or 0) > 0 else None

    def replicator_coverage(self) -> float:
        """Fraction of programs dominated by the single most common opcode."""
        op_votes: dict = {}
        for coord in self.programs:
            dom = self.dominant_opcode(coord)
            if dom is not None:
                op_votes[dom] = op_votes.get(dom, 0) + 1
        if not op_votes: return 0.0
        return max(op_votes.values()) / len(self.programs)


# ── ASCII Rendering ────────────────────────────────────────────────────────────

def render(lattice: PhextLattice, use_color: bool = True) -> str:
    n, s = lattice.active_dims, lattice.size
    base  = 9 - n   # index of first active dimension
    lines = [
        f"epoch={lattice.epoch:5d}  opcodes={lattice.opcode_pct():5.1f}%  "
        f"coverage={lattice.replicator_coverage():.1%}  "
        f"lattice={n}D {s}^{n}={s**n}"
    ]

    z_range = range(1, min(s + 1, 4)) if n >= 3 else [None]
    z_dim   = base if n >= 3 else None

    for z in z_range:
        if z is not None:
            lines.append(f"\n  [{DIM_NAMES[z_dim]}={z}]")
        for yi in range(1, s + 1):
            row = []
            for xi in range(1, s + 1):
                coord = [1] * 9
                coord[8] = xi                              # scroll  = x-axis
                if n >= 2: coord[7] = yi                  # section = y-axis
                if n >= 3 and z_dim is not None: coord[z_dim] = z
                dom = lattice.dominant_opcode(tuple(coord))
                ch = OPCODE_CHARS.get(dom, '·') if dom is not None else '·'
                if use_color and dom is not None:
                    row.append(ANSI.get(dom, '') + ch + RESET)
                else:
                    row.append(ch)
            lines.append(' '.join(row))

    if n > 2:
        dominated_by: dict = {}
        for coord in lattice.programs:
            dom = lattice.dominant_opcode(coord)
            if dom is not None:
                dominated_by.setdefault(dom, []).append(coord)
        if dominated_by:
            top_op, top_coords = max(dominated_by.items(), key=lambda x: len(x[1]))
            spread = []
            for d in range(n):
                di = base + d
                vals = set(c[di] for c in top_coords)
                spread.append(f"{DIM_NAMES[di]}={len(vals)}/{s}")
            lines.append(f"  dominant={OPCODE_CHARS.get(top_op,'?')!r} spread: {', '.join(spread)}")

    return '\n'.join(lines)


# ── Phext API persistence ──────────────────────────────────────────────────────

def coord_to_phext(coord: tuple) -> str:
    l,sh,se,co,v,b,ch,sc,s = coord
    return f"{l}.{sh}.{se}/{co}.{v}.{b}/{ch}.{sc}.{s}"


def save_to_phext(lattice: PhextLattice, api_base: str, token: str,
                  phext_path: str = '/source/human/alife.phext'):
    import urllib.request, json
    h_text = {'Authorization': f'Bearer {token}', 'Content-Type': 'text/plain'}
    h_json = {'Authorization': f'Bearer {token}', 'Content-Type': 'application/json'}

    def call(method, path, data, headers):
        d = data if isinstance(data, bytes) else data.encode()
        req = urllib.request.Request(api_base + path, data=d, headers=headers, method=method)
        try:
            return json.loads(urllib.request.urlopen(req, timeout=5).read())
        except Exception as e:
            return {'ok': False, 'error': str(e)}

    call('POST', '/api/open', phext_path, h_text)
    for coord, tape in lattice.programs.items():
        cs = coord_to_phext(coord)
        call('POST', '/api/goto', cs, h_text)
        content = (f"# epoch={lattice.epoch} coord={cs} "
                   f"opcodes={sum(1 for b in tape if b in OPSET)}/{TAPE_SIZE}\n"
                   + ''.join(OPCODE_CHARS.get(b, '.') for b in tape))
        body = json.dumps({'coordinate': cs, 'content': content}).encode()
        call('POST', '/api/update', body, h_json)
    call('POST', '/api/save', b'', h_text)
    print(f"  [phext] {len(lattice.programs)} scrolls → {phext_path}")


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    p = argparse.ArgumentParser(description='Phext Artificial Life (stdlib)')
    p.add_argument('--seed',          type=int,   default=42)
    p.add_argument('--dims',          type=int,   default=2,
                   help='Active phext dims 1-9 (2=section×scroll, 9=full 9D)')
    p.add_argument('--size',          type=int,   default=9,
                   help='Values per dim 1-9. Total programs = size^dims')
    p.add_argument('--epochs',        type=int,   default=500)
    p.add_argument('--mutation-rate', type=float, default=0.024/100.0)
    p.add_argument('--render-every',  type=int,   default=10)
    p.add_argument('--no-color',      action='store_true')
    p.add_argument('--phext-api',     type=str,   default=None,
                   help='phext-edit API base (e.g. http://localhost:8080)')
    p.add_argument('--token',         type=str,   default=None)
    p.add_argument('--save-every',    type=int,   default=50)
    args = p.parse_args()

    total = args.size ** args.dims
    print(f"phext-life stdlib: {args.dims}D, {args.size}^{args.dims}={total} programs, "
          f"{TAPE_SIZE}-byte tapes")
    print(f"epochs={args.epochs}  seed={args.seed}  mutation={args.mutation_rate:.6f}")
    if args.phext_api:
        print(f"phext-api={args.phext_api}")
    print()

    lattice   = PhextLattice(active_dims=args.dims, size=args.size, seed=args.seed)
    use_color = not args.no_color and sys.stdout.isatty()
    t0        = time.time()

    for epoch in range(args.epochs):
        lattice.run_epoch(mutation_rate=args.mutation_rate)

        if (epoch + 1) % args.render_every == 0 or epoch == 0:
            if use_color: print('\033[2J\033[H', end='')
            elapsed = time.time() - t0
            print(render(lattice, use_color=use_color))
            print(f"  {elapsed:.1f}s  {(epoch+1)/elapsed:.0f} epochs/s")
            print()

        if (args.phext_api and args.token and
                (epoch + 1) % args.save_every == 0):
            save_to_phext(lattice, args.phext_api, args.token)

    print('\n' + render(lattice, use_color=use_color))
    if args.phext_api and args.token:
        save_to_phext(lattice, args.phext_api, args.token)
    print(f"\nDone. opcode density={lattice.opcode_pct():.2f}%  "
          f"coverage={lattice.replicator_coverage():.1%}")


if __name__ == '__main__':
    main()
