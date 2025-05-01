use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_bindgen/go_to_prev_location.js")]
extern "C" {
    pub fn prev_location();
}
