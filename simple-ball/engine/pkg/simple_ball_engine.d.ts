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
* @param {Ball} ball
* @param {View} view
*/
  constructor(ball: Ball, view: View);
/**
* @param {number} x
* @param {number} y
*/
  set_force(x: number, y: number): void;
/**
* @param {number} elapsed_since_last_update
*/
  update(elapsed_since_last_update: number): void;
/**
*/
  readonly ball: Ball;
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
  readonly simulation_ball: (a: number) => number;
  readonly simulation_update: (a: number, b: number) => void;
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
