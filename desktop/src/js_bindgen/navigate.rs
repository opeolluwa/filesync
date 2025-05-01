use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_bindgen/navigate.js")]
extern "C" {
    pub fn change_location_to(location: &str);
}
