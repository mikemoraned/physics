extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use rapier3d::prelude::*;

mod log;
mod terrain;
mod screen;
mod dimension;
mod arena;

use log::*;
use terrain::*;
use screen::*;
use dimension::*;
use arena::*;

#[wasm_bindgen]
pub struct Simulation {
    screen: Screen,
    arena: Arena
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(num_balls: u8, terrain: &Terrain, screen: &Screen) -> Simulation {
        console_error_panic_hook::set_once();

        let arena = Arena::new(50.0, num_balls, terrain);
        console_log!("Creating Simulation, with num_balls {:?}, using screen {:?}, terrain of {}x{}, and arena {:?}", 
            num_balls, screen, terrain.width, terrain.height, arena.dimension);
        Simulation { screen: screen.clone(), arena }
    }

    pub fn set_force(&mut self, x: f32, y: f32) { 
        self.arena.physics.set_ball_force(x, y);
    }

    pub fn iter_ball_positions(&self, iter_fn: &js_sys::Function) {
        let arena_ball_radius = self.arena.physics.ball_radius();
        let p 
            = map_arena_to_screen(&self.screen.dimension, &self.arena.dimension, vector![arena_ball_radius, arena_ball_radius, arena_ball_radius]);
        let ball_radius = p.x;
        let ball_arena_translations = self.arena.physics.ball_translations();
        for i in 0 .. ball_arena_translations.len() {
            let ball_arena_translation = ball_arena_translations[i];
            let ball_position 
                = map_arena_to_screen(&self.screen.dimension, &self.arena.dimension, ball_arena_translation.clone());
            let this = JsValue::null();
            let _ = iter_fn.call3(&this, 
                &JsValue::from(ball_position.x), 
                &JsValue::from(ball_position.y), 
                &JsValue::from(ball_radius));
        }
    }

    pub fn update(&mut self, _elapsed_since_last_update: u32) {
        self.arena.physics.step();
    }   
}
