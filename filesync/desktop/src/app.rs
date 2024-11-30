use leptos::*;
use leptos_router::{Route, Router, Routes};
use wasm_bindgen::prelude::*;
// use leptos_router::Paren
use crate::layout::desktop_layout::DesktopLayout;
use crate::views::home::HomeView;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn DesktopApplication() -> impl IntoView {
    view! {
        <DesktopLayout>
            <Router>
                <Routes>
                    <Route path="/" view=HomeView />
                    // <Route path="/users" view=Users />
                    // <Route path="/users/:id" view=UserProfile />
                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> } />
                </Routes>
            </Router>
        </DesktopLayout>
    }
}
