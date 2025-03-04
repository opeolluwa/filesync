use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "api"], js_name= getVersion)]
    pub async fn get_current_app_version() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "api"], js_name = getName)]
    pub async fn get_app_name(cmd: &str) -> JsValue;
}
