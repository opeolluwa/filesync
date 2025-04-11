use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "os"], js_name = platform)]
    pub fn get_device_operating_system() -> JsValue;

}
