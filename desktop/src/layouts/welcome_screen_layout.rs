use js_bindgen::navigate::change_location_to;
use leptos::prelude::CustomAttribute;
use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};
use leptos::{prelude::Children, view};

#[leptos::component]
pub fn WelcomeScreenLayout(
    children: Children,
    #[prop(optional)] label: &'static str,
    #[prop(optional)] show_label: bool,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView {
    let children = children();

    let _transfer_action = label.to_string();
    let _show_label = show_label;

    view! {
        <header
            class="text-gray-500 items-center items-center justify-center p-2"
            on:click=move |_| change_location_to("/")
        >

            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="size-6 cursor-pointer hover:color-app-500"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18"
                />
            </svg>

        </header>
        <main class=format!("py-4 px-4 overflow-y-scroll, {}", class)>

            {children}
        </main>
    }
}
