use wasm_bindgen::prelude::*;
use js_sys::Math::random;
use std::f64::consts::PI;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Engine {
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine {
        console_log!("Creating Engine");
        Engine {  }
    }

    pub fn update(&self, elapsed_since_last_update: u32, x: u32, y: u32, update_fn: &js_sys::Function) {        
        let speed = 0.3f64; // pixels per millisecond
        let distance = speed * (elapsed_since_last_update as f64);
        console_log!("e: {}, speed: {}, distance: {}", elapsed_since_last_update, speed, distance);
        let angle = 2.0 * PI * random();
        let x_change = (angle.cos() * distance) as i32;
        let y_change = (angle.sin() * distance) as i32;

        console_log!("e: {}, angle: {}, x_change: {}, y_change: {}", 
            elapsed_since_last_update,
            angle,
            x_change,
            y_change);

        let new_x = (x as i32 + x_change) as u32;
        let new_y = (y as i32 + y_change) as u32;
        console_log!("x: {} + {} = {}", x, x_change, new_x);
        console_log!("y: {} + {} = {}", y, y_change, new_y);

        let this = JsValue::null();
        let _ = update_fn.call2(&this, 
            &JsValue::from(new_x), 
            &JsValue::from(new_y));
    }   
}