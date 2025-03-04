
use filesync_icons::platform::{AndroidLogo, LinuxLogo, MacOsLogo, WindowsPlatformLogo};
use leptos::prelude::{ClassAttribute, ElementChild, };
use leptos::view;

#[leptos::component]
pub fn SelectPlatformScreen() -> impl leptos::IntoView {
    let platform_logo_class_rules = "dark:bg-gray-700 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200  p-4 rounded-xl shadow hover:shadow-none cursor-pointer dark:hover:bg-gray-700/50";

    let android = AndroidLogo();
    let macOs = MacOsLogo();
    let windows = WindowsPlatformLogo();
    let linux = LinuxLogo();

    view! {
        <div class="text-center flex flex-col align-center justify-center items-center h-[90%]">
            <div>
                <h1 class="font-medium leading-2 text-2xl text-gray-700 dark:text-gray-400 ">
                    "Select platform of the other device"
                </h1>

                <div class="flex justify-center items-center gap-x-5 mt-8  ">
                    <button class=platform_logo_class_rules>{android}</button>
                    <button class=platform_logo_class_rules>{macOs}</button>
                    <button class=platform_logo_class_rules>{windows}</button>
                    <button class=platform_logo_class_rules>{linux}</button>
                </div>
            </div>
        </div>
    }
}
