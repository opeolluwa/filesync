use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js_bindgen/is_active_route.js")]
extern "C" {
    pub fn is_active_route(route: &str) -> bool;
}
