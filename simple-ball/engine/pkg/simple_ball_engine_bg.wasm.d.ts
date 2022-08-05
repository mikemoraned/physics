/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_screen_free(a: number): void;
export function __wbg_get_screen_dimension(a: number): number;
export function __wbg_set_screen_dimension(a: number, b: number): void;
export function screen_new(a: number): number;
export function __wbg_terrain_free(a: number): void;
export function __wbg_get_terrain_width(a: number): number;
export function __wbg_set_terrain_width(a: number, b: number): void;
export function __wbg_get_terrain_height(a: number): number;
export function __wbg_set_terrain_height(a: number, b: number): void;
export function terrain_from_png_terrain_image(a: number, b: number): number;
export function terrain_as_grayscale_height_image(a: number, b: number): void;
export function __wbg_simulation_free(a: number): void;
export function simulation_new(a: number, b: number, c: number): number;
export function simulation_set_force(a: number, b: number, c: number): void;
export function simulation_iter_ball_positions(a: number, b: number): void;
export function simulation_update(a: number, b: number): void;
export function __wbg_dimension_free(a: number): void;
export function __wbg_get_dimension_side_length(a: number): number;
export function __wbg_set_dimension_side_length(a: number, b: number): void;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
export function __wbindgen_exn_store(a: number): void;
