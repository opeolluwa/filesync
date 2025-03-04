use js_bindgen::navigate::change_location_to;
use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};
use leptos::{prelude::Children, view};

use filesync_icons::chevron::ChevronLeftIcon;

#[leptos::component]
pub fn WelcomeScreenLayout(children: Children, label: &'static str) -> impl leptos::IntoView {
    let children = children();
    let go_back_icon = ChevronLeftIcon();
    let _transfer_action = label.to_string();
    view! {
        <header
            class="text-gray-500 items-center items-center justify-center p-2"
            on:click=move |_| change_location_to("/")
        >

            {go_back_icon}

        </header>
        <main class="py-4 px-4 overflow-y-scroll">

            {children}
        </main>
    }
}
