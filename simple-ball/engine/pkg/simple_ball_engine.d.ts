/* tslint:disable */
/* eslint-disable */
/**
*/
export class Engine {
  free(): void;
/**
*/
  constructor();
/**
* @param {number} elapsed_since_last_update
* @param {number} x
* @param {number} y
* @param {Function} update_fn
*/
  update(elapsed_since_last_update: number, x: number, y: number, update_fn: Function): void;
}
/**
*/
export class RapierState {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_engine_free: (a: number) => void;
  readonly __wbg_rapierstate_free: (a: number) => void;
  readonly engine_new: () => number;
  readonly engine_update: (a: number, b: number, c: number, d: number, e: number) => void;
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
