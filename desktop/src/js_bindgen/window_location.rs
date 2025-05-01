use wasm_bindgen::prelude::*;

// create rust functions from the javascript functions
#[wasm_bindgen(module = "/src/js_bindgen/window_location.js")]
extern "C" {
    pub fn get_window_location() -> String;
}
