use crate::platform::Platform;
use desktop_ui::desktop_application::DesktopApplication;
use leptos::control_flow::Show;
use leptos::*;
use mobile_ui::mobile_application::MobileApplication;
use prelude::{signal, Get, Set};
use std::str::FromStr;
use tauri_wasm_bindgen::plugins::os::get_device_operating_system;

#[component]
pub fn App() -> impl IntoView {
    let (device_operating_system, set_device_operating_system) = signal(String::new());
    let operating_system = get_device_operating_system().as_string();
    set_device_operating_system.set(operating_system.unwrap());

    let device_platform = Platform::from_str(&device_operating_system.get()).unwrap_or_default();

    // let App = match device_platform {
    //     Platform::Android | Platform::Ios =>  MobileApplication(),
    //     Platform::Linux | Platform::Mac | Platform::Windows => DesktopApplication(),
    // };

    view! {<MobileApplication/>}
    // view! { App }
}
