/* tslint:disable */
/* eslint-disable */

export class PhextLife {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new simulation. extents: [d0,d1,...,d8] each 1–9.
     * canvas_w, canvas_h: pixel dimensions of target canvas.
     */
    constructor(canvas_w: number, canvas_h: number);
    /**
     * Current population.
     */
    population(): number;
    /**
     * Render the 9D universe to RGBA pixel data using a twisted projection.
     *
     * Projection: Z-order (Morton) curve over the 9 dimensions →
     * a space-filling 2D image that preserves locality.
     * Each pixel = one program slot; color = dominant instruction in that cell.
     * Black = empty.
     */
    render(ctx: CanvasRenderingContext2D): void;
    /**
     * Run one simulation tick.
     */
    tick(): void;
    /**
     * Run N ticks at once (for speed).
     */
    tick_n(n: number): void;
    /**
     * Current tick number.
     */
    ticks(): bigint;
    /**
     * Total possible coordinates.
     */
    total_coordinates(): number;
    /**
     * Total replication events.
     */
    total_replications(): bigint;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_phextlife_free: (a: number, b: number) => void;
    readonly phextlife_new: (a: number, b: number) => number;
    readonly phextlife_population: (a: number) => number;
    readonly phextlife_render: (a: number, b: any) => [number, number];
    readonly phextlife_tick: (a: number) => void;
    readonly phextlife_tick_n: (a: number, b: number) => void;
    readonly phextlife_ticks: (a: number) => bigint;
    readonly phextlife_total_coordinates: (a: number) => number;
    readonly phextlife_total_replications: (a: number) => bigint;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
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
