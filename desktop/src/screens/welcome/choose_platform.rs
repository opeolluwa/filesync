use crate::layouts::welcome_screen_layout::WelcomeScreenLayout;
use filesync_icons::platform::{AndroidLogo, LinuxLogo, MacOsLogo, WindowsPlatformLogo};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::{signal, ClassAttribute, ElementChild, OnAttribute, RwSignal, Set};
use leptos::task::spawn_local;
use leptos::view;
use leptos_qr::QrCode;
use tauri_bindgen::wifi_bindgen::WifiCredentials;
use tauri_sys::core::invoke;
use tauri_wasm_bindgen::api::invoke::invoke_tauri_command_without_args;
use thaw::{Dialog, DialogBody, DialogContent, DialogSurface, DialogTitle};

#[leptos::component]
pub fn SelectPlatformScreen() -> impl leptos::IntoView {
    let platform_logo_class_rules = "dark:bg-gray-700 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200  p-4 rounded-xl shadow hover:shadow-none cursor-pointer dark:hover:bg-gray-700/50";

    let android = AndroidLogo();
    let macOs = MacOsLogo();
    let windows = WindowsPlatformLogo();
    let linux = LinuxLogo();

    let (wifi_creds, set_wifi_creds) = signal::<WifiCredentials>(WifiCredentials::default());
    let open_android_qr_modal = RwSignal::new(false);

    // let creds = wifi_creds.get();
    let manage_android_click_event = move || {
        open_android_qr_modal.set(true);

        spawn_local(async move {
            let wifi_creds =
                invoke::<WifiCredentials>("generate_android_wifi_credentials", &()).await;
            set_wifi_creds.set(wifi_creds);
        });
    };
    view! {
        Ç
        <WelcomeScreenLayout class="h-[85%]  flex flex-col align-center justify-center">

            <div class="text-center flex flex-col align-center justify-center items-center ">
                <div>
                    <h1 class="font-medium leading-2 text-2xl text-gray-700 dark:text-gray-400">
                        "Select platform of the other device"
                    </h1>

                    <div class="flex justify-center items-center gap-x-5 mt-8  ">
                        <button
                            class=platform_logo_class_rules
                            on:click=move |_| manage_android_click_event()
                        >
                            {android}
                        </button>
                        <button class=platform_logo_class_rules>{macOs}</button>
                        <button class=platform_logo_class_rules>{windows}</button>
                        <button class=platform_logo_class_rules>{linux}</button>
                    </div>
                </div>
            </div>

            <Dialog open=open_android_qr_modal>
                <DialogSurface class="dark:bg-gray-400 dark:text-gray-500">
                    <DialogBody class="">
                        <DialogTitle class="dark:text-gray-500">
                            "Scan QR code to connect Android device"
                        </DialogTitle>
                        <DialogContent class="flex mt-6 flex-col items-center justify-center">
                            <div class="w-[175px] h-[175px]">
                                <QrCode
                                    data="wifi_creds"
                                    ecl=leptos_qr::ECL::Q
                                    shape=leptos_qr::Shape::Square
                                    fg_color="#111111"
                                    bg_color="transparent"
                                />
                            </div>
                        </DialogContent>

                    </DialogBody>
                </DialogSurface>
            </Dialog>
        </WelcomeScreenLayout>
    }
}
