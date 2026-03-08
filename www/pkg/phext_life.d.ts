/* tslint:disable */
/* eslint-disable */

/**
 * WASM-exposed universe wrapper
 */
export class PhextLife {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Get current tick number
     */
    current_tick(): bigint;
    /**
     * Get dimensional density for given dimension (0-8)
     */
    density(dim: number): Uint32Array;
    /**
     * Create medium simulation
     */
    static medium(): PhextLife;
    /**
     * Create new simulation with given size
     */
    constructor(size: number, fill_ratio: number);
    /**
     * Get population count
     */
    population(): number;
    /**
     * Get 2D projection of occupied coordinates
     * Projects 9D to 2D by summing dimension pairs
     * Returns flat array of (x, y, intensity) triples
     */
    projection_2d(width: number, height: number): Uint8Array;
    /**
     * Get RGB image data for canvas (RGBA format)
     */
    render_rgba(width: number, height: number): Uint8Array;
    /**
     * Run multiple ticks
     */
    run(ticks: number): void;
    /**
     * Create small test simulation (3^9 = 19,683 coordinates)
     */
    static small(): PhextLife;
    /**
     * Get stats as JSON string
     */
    stats_json(): string;
    /**
     * Run one tick
     */
    tick(): void;
    /**
     * Get total coordinates
     */
    total_coordinates(): number;
}

/**
 * Initialize panic hook for better error messages
 */
export function init(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_phextlife_free: (a: number, b: number) => void;
    readonly init: () => void;
    readonly phextlife_current_tick: (a: number) => bigint;
    readonly phextlife_density: (a: number, b: number) => [number, number];
    readonly phextlife_medium: () => number;
    readonly phextlife_new: (a: number, b: number) => number;
    readonly phextlife_population: (a: number) => number;
    readonly phextlife_projection_2d: (a: number, b: number, c: number) => [number, number];
    readonly phextlife_render_rgba: (a: number, b: number, c: number) => [number, number];
    readonly phextlife_run: (a: number, b: number) => void;
    readonly phextlife_small: () => number;
    readonly phextlife_stats_json: (a: number) => [number, number];
    readonly phextlife_tick: (a: number) => void;
    readonly phextlife_total_coordinates: (a: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
