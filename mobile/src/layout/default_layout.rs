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
        <div class="relative h-screen overflow-y-hidden w-full">
            <header class="flex fixed top-4 left-0 mb-5 right-0 w-full px-4 justify-between items-center text-gray-600">
                <button>{MenuIcon}</button>
                <button>{ScanQrIcon}</button>
            </header>
            <main
                class="px-4 my-6 py-12 overflow-y-scroll my-4"
                style="height:calc(100vh-10rem); overflow-y:scroll"
            >
                {children}
            </main>
            <nav class="absolute pt-4 mb-0 fixed bottom-0 bg-app/90 rounded-t-lg  shadow-gray-400 w-full left-0 right-0 py-2  text-white shadow-xl py-3 ">
                <BottomNavigation />
            </nav>
        </div>
    }
}
