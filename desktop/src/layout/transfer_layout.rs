use js_bindgen::navigate::change_location_to;
use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};
use leptos::{prelude::Children, view};

use filesync_icons::chevron::ChevronLeftIcon;
use js_bindgen::go_to_prev_location::go_to_prev_location;

#[leptos::component]
pub fn TransferLayout(children: Children) -> impl leptos::IntoView {
    let children = children();
    let go_back_icon = ChevronLeftIcon();

    view! {
        <header
            class="text-gray-500 flex flex-col justify-center items-center dark:hover:bg-gray-700/40 hover:bg-app-50 hover:text-app p-3 w-full rounded-lg"
            on:click=move |_| change_location_to("/")
        >
            view!
            {go_back_icon}
        </header>
        <main class="py-4 px-4 overflow-y-scroll h-screen">

            {children}
        </main>
    }
}
