use wasm_bindgen::prelude::*;
use crate::dimension::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Screen {
    pub dimension: Dimension
}

#[wasm_bindgen]
impl Screen {
    #[wasm_bindgen(constructor)]
    pub fn new(side_length: f32) -> Screen {
        Screen { 
            dimension: Dimension { side_length } 
        }
    }
}