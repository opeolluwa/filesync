use crate::platform::Platform;
use desktop_ui::desktop_application::DesktopApplication;
use leptos::*;
use mobile_ui::mobile_application::MobileApplication;
use std::str::FromStr;
// use os_info; 
#[component]
pub fn App() -> impl IntoView {
    //TODO: remove static binding
    let device_platform = Platform::from_str("android").unwrap_or_default();

    match device_platform {
        Platform::Android | Platform::Ios => {
            view! {
                <MobileApplication/>
            }
        }
        Platform::Linux | Platform::Mac | Platform::Windows => {
            view! {<DesktopApplication/>}
        }
    }
}
