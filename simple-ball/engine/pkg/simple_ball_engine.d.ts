/* tslint:disable */
/* eslint-disable */
/**
*/
export class Ball {
  free(): void;
/**
* @param {number} x
* @param {number} y
*/
  constructor(x: number, y: number);
/**
*/
  readonly x: number;
/**
*/
  readonly y: number;
}
/**
*/
export class RapierState {
  free(): void;
/**
* @param {number} x
* @param {number} z
*/
  set_ball_force(x: number, z: number): void;
}
/**
*/
export class Simulation {
  free(): void;
/**
* @param {number} num_balls
* @param {View} view
*/
  constructor(num_balls: number, view: View);
/**
* @param {number} x
* @param {number} y
*/
  set_force(x: number, y: number): void;
/**
* @param {Function} iter_fn
*/
  iter_ball_positions(iter_fn: Function): void;
/**
* @param {number} elapsed_since_last_update
*/
  update(elapsed_since_last_update: number): void;
}
/**
*/
export class Terrain {
  free(): void;
/**
* @param {Uint8Array} data
* @returns {Terrain}
*/
  static from_png_terrain_image(data: Uint8Array): Terrain;
/**
* @returns {Uint8Array}
*/
  as_grayscale_height_image(): Uint8Array;
}
/**
*/
export class View {
  free(): void;
/**
* @param {number} side_length
*/
  constructor(side_length: number);
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_terrain_free: (a: number) => void;
  readonly terrain_from_png_terrain_image: (a: number, b: number) => number;
  readonly terrain_as_grayscale_height_image: (a: number, b: number) => void;
  readonly __wbg_rapierstate_free: (a: number) => void;
  readonly rapierstate_set_ball_force: (a: number, b: number, c: number) => void;
  readonly __wbg_simulation_free: (a: number) => void;
  readonly __wbg_view_free: (a: number) => void;
  readonly view_new: (a: number) => number;
  readonly __wbg_ball_free: (a: number) => void;
  readonly ball_new: (a: number, b: number) => number;
  readonly ball_x: (a: number) => number;
  readonly ball_y: (a: number) => number;
  readonly simulation_new: (a: number, b: number) => number;
  readonly simulation_set_force: (a: number, b: number, c: number) => void;
  readonly simulation_iter_ball_positions: (a: number, b: number) => void;
  readonly simulation_update: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
