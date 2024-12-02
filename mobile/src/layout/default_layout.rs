use crate::mobile_application_components::bottom_navigation::BottomNavigation;
use filesync_icons::menu_icon::MenuIcon;
use filesync_icons::scan_qr_icon::ScanQrIcon;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{children::Children, view};

#[leptos::component]

pub fn DefaultLayout(children: Children) -> impl leptos::IntoView {
    let children = children();
    let MenuIcon = MenuIcon();
    let ScanQrIcon = ScanQrIcon();

    view! {
        <div class="relative h-screen overflow-y-hidden">
            <header class="flex fixed top-4 left-0 mb-5 right-0 w-full px-4 justify-between items-center text-gray-600">
                <button>{MenuIcon}</button>
                <button>{ScanQrIcon}</button>
            </header>
            <main class="px-4 py-6 my-6 min-h-screen w-full my-4">{children}</main>
            <nav class="absolute pt-4 mb-0 pb-0 fixed bottom-3 w-full left-0 right-0 py-2 rounded-t-lg bg-app-50/20">
                <BottomNavigation />
            </nav>
        </div>
    }
}
