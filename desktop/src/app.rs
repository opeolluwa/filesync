use leptos::*;
use leptos_router::{Route, Router, Routes};
use wasm_bindgen::prelude::*;
// use leptos_router::Paren
use crate::layout::desktop_layout::DesktopLayout;
use crate::views::about::AboutUI;
use crate::views::history::HistoryUI;
use crate::views::settings::SettingsUi;
use crate::views::share::ShareUI;
use crate::views::transfer::TransferUI;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub async fn invoke_without_args(cmd: &str) -> JsValue;

}

#[component]
pub fn DesktopApplication() -> impl IntoView {
    view! {
        <DesktopLayout>
            <Router>
                <Routes>
                    <Route path="/" view=TransferUI />
                    <Route path="/about" view=AboutUI />
                    <Route path="/settings" view=SettingsUi />
                    <Route path="/share" view=ShareUI />
                    <Route path="/about" view=HistoryUI />

                </Routes>
            </Router>
        </DesktopLayout>
    }
}
