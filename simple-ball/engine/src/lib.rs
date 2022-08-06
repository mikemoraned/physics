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
use web_sys::Performance;

#[wasm_bindgen]
pub struct Simulation {
    screen: Screen,
    arena: Arena,
    performance: Performance
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(num_balls: u8, terrain: &Terrain, screen: &Screen) -> Simulation {
        let arena = Arena::new(50.0, num_balls, terrain);
        console_log!("Creating Simulation, with num_balls {:?}, using screen {:?}, terrain of {}x{}, and arena {:?}", 
            num_balls, screen, terrain.width, terrain.height, arena.dimension);
        let performance = Self::get_performance();
        Simulation { screen: screen.clone(), arena, performance }
    }

    fn get_performance() -> Performance {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        performance
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

    pub fn update(&mut self, elapsed_since_last_update: u32) {
        let max_milliseconds = 1000 / 60;
        self.arena.physics.step(elapsed_since_last_update, &self.performance, max_milliseconds);
    }   
}
