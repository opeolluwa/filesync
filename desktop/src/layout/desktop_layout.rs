use crate::components::side_navigation::{SideNavigation, SideNavigationRoute};
use filesync_icons::{info_icon::InformationIconOutline, settings_icon::SettingsIconOutline};
use leptos::prelude::ElementChild;
use leptos::{
    children::Children,
    prelude::{ClassAttribute, StyleAttribute},
    view,
};

#[leptos::component]
pub fn DesktopLayout(children: Children) -> impl leptos::IntoView {
    let children = children();
    let info_icon = InformationIconOutline();
    let settings_icon = SettingsIconOutline();

    view! {
        <div
            class="grid grid-cols-12 mb-0 pb-0 w-[100vw] gap-x-2"
            style="height:100vh; overflow-y: hidden; margin-bottom:0 dark:bg-gray-900"
        >
            <nav
                class="col-span-1  dark:bg-gray-900/50 px-[2px] fixed  text-gray-600 pt-4 hidden"
                style="height: 100%; overflowY: hidden; position: relative"
            >
                <SideNavigation />
                <div class="divider"></div>
                <div class="absolute bottom-3 left-0 right-0 w-full">
                    <SideNavigationRoute label="settings" href="/settings" icon=settings_icon />
                    <SideNavigationRoute label="about" href="/about" icon=info_icon />
                </div>
            </nav>
            <main class="col-span-12 py-4 px-4 overflow-y-scroll">{children}</main>
        </div>
    }
}
