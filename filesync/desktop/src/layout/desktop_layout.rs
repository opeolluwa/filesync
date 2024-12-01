use crate::components::side_navigation::SideNavigation;
use leptos::{view, Children, SignalGet, SignalSet};
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

#[leptos::component]
pub fn DesktopLayout(children: Children) -> impl leptos::IntoView {
    let children = children();
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();
    let color_theme = mode.get();
    let toggle_theme = move || {
        if color_theme == ColorMode::Dark {
            set_mode.set(ColorMode::Light);
        }
        if color_theme == ColorMode::Light {
            set_mode.set(ColorMode::Dark);
        }
    };
    view! {
        <div
            class="grid grid-cols-12 mb-0 pb-0 w-[100vw] gap-x-2"
            style="height:100vh; overflow-y: hidden; margin-bottom:0 dark:bg-gray-900"
        >
            <nav
                class="col-span-1 bg-[rgba(249,250,254,255)] dark:bg-gray-900/50 px-[2px] fixed  text-gray-600 pt-4"
                style="height: 100%; overflowY: hidden; position: relative"
            >
                <SideNavigation />
                <button
                    class="text-gray-500 flex flex-col justify-center items-center  p-3  rounded-lg absolute bottom-3 left-0 right-0 w-full"
                    on:click=move |_| toggle_theme()
                >
                    <i class="ri-sun-line ri-lg" style="box-sizing:border-box"></i>
                    <span class="sr-only">theme</span>
                </button>
            </nav>
            <main class="col-span-11 py-4 px-4 overflow-y-scroll">{children}</main>
        </div>
    }
}
