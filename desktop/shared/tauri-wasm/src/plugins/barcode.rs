use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "barcodeScanner"], js_name = scan)]
    pub fn scan_barcode() -> JsValue;
}
