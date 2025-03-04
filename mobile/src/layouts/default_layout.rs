use leptos::prelude::{ClassAttribute, ElementChild, StyleAttribute};
use leptos::{children::Children, view};
use filesync_icons::scan_qr_icon::ScanQrIcon;


#[leptos::component]

pub fn DefaultLayout(children: Children) -> impl leptos::IntoView {
    let children = children();
    let ScanQrIcon = ScanQrIcon();

    view! {
        <div class="relative h-screen overflow-y-scroll w-full">
            <header class="flex fixed bg-white top-0 py-3 text-gray-500/90 left-0 mb-5 right-0 w-full border-b px-4 justify-between items-center text-gray-600 hidden">
                <button>FileSync</button>
                <button>{ScanQrIcon}</button>
            </header>
            <main class="px-4" style="height:calc(100vh-10rem); overflow-y:scroll">
                {children}
            </main>
        </div>
    }
}
