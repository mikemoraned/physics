/* tslint:disable */
/* eslint-disable */
/**
*/
export class RapierState {
  free(): void;
}
/**
*/
export class Simulation {
  free(): void;
/**
* @param {number} ball_x
* @param {number} ball_y
* @param {number} ball_radius
*/
  constructor(ball_x: number, ball_y: number, ball_radius: number);
/**
* @param {number} elapsed_since_last_update
* @param {Function} update_fn
*/
  update(elapsed_since_last_update: number, update_fn: Function): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_simulation_free: (a: number) => void;
  readonly __wbg_rapierstate_free: (a: number) => void;
  readonly simulation_new: (a: number, b: number, c: number) => number;
  readonly simulation_update: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
