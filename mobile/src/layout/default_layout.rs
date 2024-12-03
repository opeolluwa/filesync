use crate::mobile_application_components::bottom_navigation::BottomNavigation;
use filesync_icons::menu_icon::MenuIcon;
use filesync_icons::scan_qr_icon::ScanQrIcon;
use leptos::prelude::{ClassAttribute, ElementChild, StyleAttribute};
use leptos::{children::Children, view};

#[leptos::component]

pub fn DefaultLayout(children: Children) -> impl leptos::IntoView {
    let children = children();
    let MenuIcon = MenuIcon();
    let ScanQrIcon = ScanQrIcon();

    view! {
        <div class="relative h-screen overflow-y-scroll w-full">
            <header class="flex fixed bg-white top-0 py-4 text-gray-500/90 left-0 mb-5 right-0 w-full border-b px-4 justify-between items-center text-gray-600">
                <button>{MenuIcon}</button>
                <button>{ScanQrIcon}</button>
            </header>
            <main
                class="px-4 my-6 py-12 overflow-y-scroll my-4"
                style="height:calc(100vh-10rem); overflow-y:scroll"
            >
                {children}
            </main>
            <nav class="fixed pt-4 mb-0 fixed bottom-0 z-50 bg-white border-t text-gray-500/90 shadow-gray-400 w-full left-0 right-0 py-3 ">
                <BottomNavigation />
            </nav>
        </div>
    }
}
