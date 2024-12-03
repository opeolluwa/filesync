use crate::platform::Platform;
use desktop_ui::desktop_application::DesktopApplication;
use leptos::prelude::{signal, Get, RwSignal, Set};
use leptos::{component, control_flow::Show, view, IntoView};
use mobile_ui::mobile_application::MobileApplication;
use std::str::FromStr;
use tauri_wasm_bindgen::plugins::os::get_device_operating_system;
use thaw::{ConfigProvider, Theme};

#[component]
pub fn App() -> impl IntoView {
    let (device_operating_system, set_device_operating_system) = signal(String::new());
    let operating_system = get_device_operating_system().as_string();
    set_device_operating_system.set(operating_system.unwrap());

    let device_platform = Platform::from_str(&device_operating_system.get()).unwrap_or_default();

    let theme = RwSignal::new(Theme::light());
    view! {
        <Show
            when=move || {
                device_platform == Platform::Android || device_platform == Platform::Ios
            }
            fallback=move || {
                view! {
                    <ConfigProvider theme>
                        <DesktopApplication />
                    </ConfigProvider>
                }
            }
        >
            <ConfigProvider theme>
                <MobileApplication />
            </ConfigProvider>
        </Show>
    }
}
