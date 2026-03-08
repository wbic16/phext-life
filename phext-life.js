// Phext Life - Artificial Life in 11D Space (Browser Edition)
// Port of phext-life-11d.py to JavaScript

// Constants
const ZERO = 0;
const ONE = 1;
const LT = '<'.charCodeAt(0);
const GT = '>'.charCodeAt(0);
const LB = '{'.charCodeAt(0);
const RB = '}'.charCodeAt(0);
const MINUS = '-'.charCodeAt(0);
const PLUS = '+'.charCodeAt(0);
const DOT = '.'.charCodeAt(0);
const COMMA = ','.charCodeAt(0);
const LBRACK = '['.charCodeAt(0);
const RBRACK = ']'.charCodeAt(0);

const OPCODE_TOKENS = [LT, GT, LB, RB, MINUS, PLUS, DOT, COMMA, LBRACK, RBRACK];
const NORMALIZED_VALUE = 255;

// Color palette for visualization
const COLOR_LUT = new Array(256).fill(null).map(() => [0, 0, 0]);
COLOR_LUT[LT] = [255, 0, 0];        // Red
COLOR_LUT[GT] = [0, 255, 0];        // Green
COLOR_LUT[LB] = [0, 0, 255];        // Blue
COLOR_LUT[RB] = [255, 255, 0];      // Yellow
COLOR_LUT[MINUS] = [255, 0, 255];   // Magenta
COLOR_LUT[PLUS] = [0, 255, 255];    // Cyan
COLOR_LUT[DOT] = [255, 128, 0];     // Orange
COLOR_LUT[COMMA] = [128, 0, 255];   // Purple
COLOR_LUT[LBRACK] = [255, 255, 255];  // White
COLOR_LUT[RBRACK] = [128, 128, 128];  // Gray
COLOR_LUT[NORMALIZED_VALUE] = [0, 0, 0];  // Black

// Configuration
const GRID_SIZE = 9;  // 9×9×9 cube
const NUM_PROGRAMS = GRID_SIZE ** 3;  // 729
const TAPE_SIZE = 64;
const MAX_ITERATIONS = 8192;  // 2^13
const MUTATION_RATE = 0.001;
const PROG_DISPLAY_SIZE = 8;  // 8×8 pixels per program
const PIXEL_SCALE = 10;  // Scale factor for display

// Global state
let programs = new Array(NUM_PROGRAMS);
let neighbors = [];
let neighborCounts = new Int32Array(NUM_PROGRAMS);
let epoch = 0;
let running = false;
let speedMultiplier = 1.0;
let animationId = null;

// Canvas
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d', { alpha: false });

// Initialize
function init() {
  buildNeighbors();
  resetSimulation();
  render();
}

// Build neighbor table for 9×9×9 cube
function buildNeighbors() {
  neighbors = new Array(NUM_PROGRAMS).fill(null).map(() => []);
  
  for (let idx = 0; idx < NUM_PROGRAMS; idx++) {
    const coord = indexToCoord(idx);
    const nbrs = [];
    
    // ±1 in each of 3 dimensions (L, S, C)
    for (let dim = 0; dim < 3; dim++) {
      // -1 neighbor
      if (coord[dim] > 1) {
        const nCoord = [...coord];
        nCoord[dim]--;
        nbrs.push(coordToIndex(nCoord));
      }
      
      // +1 neighbor
      if (coord[dim] < GRID_SIZE) {
        const nCoord = [...coord];
        nCoord[dim]++;
        nbrs.push(coordToIndex(nCoord));
      }
    }
    
    neighbors[idx] = nbrs;
    neighborCounts[idx] = nbrs.length;
  }
}

// Convert coordinate to index
function coordToIndex(coord) {
  const [l, s, c] = coord;
  return ((l-1) * GRID_SIZE * GRID_SIZE) + ((s-1) * GRID_SIZE) + (c-1);
}

// Convert index to coordinate
function indexToCoord(idx) {
  const c = (idx % GRID_SIZE) + 1;
  idx = Math.floor(idx / GRID_SIZE);
  const s = (idx % GRID_SIZE) + 1;
  idx = Math.floor(idx / GRID_SIZE);
  const l = idx + 1;
  return [l, s, c];
}

// Normalize instruction value
function normalize(val) {
  return OPCODE_TOKENS.includes(val) ? val : NORMALIZED_VALUE;
}

// Run program tape
function runTape(tape) {
  const tapeSize = tape.length;
  let pc = 0, head0 = 0, head1 = 0;
  
  function seekMatch(startPc, step, openTok, closeTok) {
    let depth = 1;
    let p = startPc + step;
    while (p >= 0 && p < tapeSize && depth > 0) {
      const opcode = tape[p];
      if (opcode === openTok) depth++;
      else if (opcode === closeTok) depth--;
      p += step;
    }
    return depth === 0 ? p - step : -1;
  }
  
  for (let iter = 0; iter < MAX_ITERATIONS; iter++) {
    if (pc < 0 || pc >= tapeSize) break;
    
    const opcode = tape[pc];
    
    if (opcode === LT || opcode === GT) {
      head0 = (head0 + (opcode === GT ? 1 : -1) + tapeSize) % tapeSize;
    } else if (opcode === LB || opcode === RB) {
      head1 = (head1 + (opcode === RB ? 1 : -1) + tapeSize) % tapeSize;
    } else if (opcode === MINUS || opcode === PLUS) {
      tape[head0] = (tape[head0] + (opcode === PLUS ? 1 : -1) + 256) % 256;
    } else if (opcode === DOT) {
      tape[head1] = tape[head0];
    } else if (opcode === COMMA) {
      tape[head0] = tape[head1];
    } else if (opcode === LBRACK && tape[head0] === ZERO) {
      pc = seekMatch(pc, 1, LBRACK, RBRACK);
      if (pc < 0) break;
    } else if (opcode === RBRACK && tape[head0] !== ZERO) {
      pc = seekMatch(pc, -1, RBRACK, LBRACK);
      if (pc < 0) break;
    }
    
    pc++;
  }
  
  return tape;
}

// Run one epoch
function runEpoch() {
  // Propose random neighbors
  const proposals = new Int32Array(NUM_PROGRAMS);
  for (let i = 0; i < NUM_PROGRAMS; i++) {
    const count = neighborCounts[i];
    if (count > 0) {
      const nbrIdx = Math.floor(Math.random() * count);
      proposals[i] = neighbors[i][nbrIdx];
    } else {
      proposals[i] = -1;
    }
  }
  
  // Select pairs
  const order = Array.from({ length: NUM_PROGRAMS }, (_, i) => i);
  for (let i = order.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [order[i], order[j]] = [order[j], order[i]];
  }
  
  const taken = new Uint8Array(NUM_PROGRAMS);
  const pairs = [];
  
  for (const p of order) {
    const n = proposals[p];
    if (n < 0 || taken[p] || taken[n]) continue;
    taken[p] = 1;
    taken[n] = 1;
    pairs.push([p, n]);
  }
  
  // Execute pairs
  for (const [idxA, idxB] of pairs) {
    const tape = new Uint8Array(TAPE_SIZE * 2);
    tape.set(programs[idxA], 0);
    tape.set(programs[idxB], TAPE_SIZE);
    
    runTape(tape);
    
    programs[idxA] = tape.slice(0, TAPE_SIZE);
    programs[idxB] = tape.slice(TAPE_SIZE);
  }
  
  // Background mutation
  if (MUTATION_RATE > 0) {
    for (let i = 0; i < NUM_PROGRAMS; i++) {
      for (let j = 0; j < TAPE_SIZE; j++) {
        if (Math.random() < MUTATION_RATE) {
          programs[i][j] = Math.floor(Math.random() * 256);
        }
      }
    }
  }
  
  epoch++;
}

// Render visualization
function render() {
  const imageData = ctx.createImageData(canvas.width, canvas.height);
  const data = imageData.data;
  
  let progIdx = 0;
  for (let l = 1; l <= GRID_SIZE; l++) {
    for (let s = 1; s <= GRID_SIZE; s++) {
      for (let c = 1; c <= GRID_SIZE; c++) {
        // Map to grid position (3×3 super-grid)
        const superCol = Math.floor((c - 1) / 3);
        const superRow = Math.floor((c - 1) % 3);
        const blockCol = (s - 1) % 3;
        const blockRow = (l - 1) % 3;
        
        const xOffset = (superCol * 3 + blockCol) * GRID_SIZE * PIXEL_SCALE;
        const yOffset = (superRow * 3 + blockRow) * GRID_SIZE * PIXEL_SCALE;
        
        // Draw program as 8×8 colored pixels (scaled)
        const prog = programs[progIdx];
        for (let i = 0; i < PROG_DISPLAY_SIZE; i++) {
          for (let j = 0; j < PROG_DISPLAY_SIZE; j++) {
            const instrIdx = i * PROG_DISPLAY_SIZE + j;
            if (instrIdx < TAPE_SIZE) {
              const instr = normalize(prog[instrIdx]);
              const color = COLOR_LUT[instr];
              
              // Draw scaled pixel
              for (let dy = 0; dy < PIXEL_SCALE; dy++) {
                for (let dx = 0; dx < PIXEL_SCALE; dx++) {
                  const px = xOffset + j * PIXEL_SCALE + dx;
                  const py = yOffset + i * PIXEL_SCALE + dy;
                  const idx = (py * canvas.width + px) * 4;
                  
                  data[idx] = color[0];
                  data[idx + 1] = color[1];
                  data[idx + 2] = color[2];
                  data[idx + 3] = 255;
                }
              }
            }
          }
        }
        
        progIdx++;
      }
    }
  }
  
  ctx.putImageData(imageData, 0, 0);
  
  // Update stats
  document.getElementById('epoch').textContent = epoch;
  document.getElementById('speed').textContent = speedMultiplier.toFixed(1) + 'x';
}

// Reset simulation
function resetSimulation() {
  programs = new Array(NUM_PROGRAMS);
  for (let i = 0; i < NUM_PROGRAMS; i++) {
    programs[i] = new Uint8Array(TAPE_SIZE);
    for (let j = 0; j < TAPE_SIZE; j++) {
      programs[i][j] = Math.floor(Math.random() * 256);
    }
  }
  epoch = 0;
  render();
}

// Start simulation
function startSimulation() {
  if (running) return;
  running = true;
  document.getElementById('startBtn').disabled = true;
  document.getElementById('pauseBtn').disabled = false;
  
  function loop() {
    if (!running) return;
    
    const steps = Math.max(1, Math.floor(speedMultiplier));
    for (let i = 0; i < steps; i++) {
      runEpoch();
    }
    render();
    
    const delay = Math.max(16, 100 / speedMultiplier);
    animationId = setTimeout(loop, delay);
  }
  
  loop();
}

// Pause simulation
function pauseSimulation() {
  running = false;
  if (animationId) {
    clearTimeout(animationId);
    animationId = null;
  }
  document.getElementById('startBtn').disabled = false;
  document.getElementById('pauseBtn').disabled = true;
}

// Step one epoch
function stepSimulation() {
  if (running) pauseSimulation();
  runEpoch();
  render();
}

// Change speed
function changeSpeed(multiplier) {
  speedMultiplier = Math.max(0.1, Math.min(10, speedMultiplier * multiplier));
  document.getElementById('speed').textContent = speedMultiplier.toFixed(1) + 'x';
}

// Initialize on load
window.addEventListener('load', init);

// Prevent zoom on double-tap (mobile)
let lastTouchEnd = 0;
document.addEventListener('touchend', (event) => {
  const now = Date.now();
  if (now - lastTouchEnd <= 300) {
    event.preventDefault();
  }
  lastTouchEnd = now;
}, false);
