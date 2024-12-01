use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn MobileApplication() -> impl IntoView {
    view! { <button>mobile applicatsdvl;wkg;ion</button> }
}
