use js_bindgen::navigate::change_location_to;
use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};
use leptos::{prelude::Children, view};

use filesync_icons::chevron::ChevronLeftIcon;
use thaw::Text;

#[leptos::component]
pub fn WelcomeScreenLayout(children: Children, label: &'static str) -> impl leptos::IntoView {
    let children = children();
    let go_back_icon = ChevronLeftIcon();
    let transfer_action = label.to_string();
    view! {
        <header
            class="text-gray-500 inline-flex
              bg-app-500/50 items-center "
            on:click=move |_| change_location_to("/")
        >

   <span class="size-4">         {go_back_icon} </span> 
            <Text class="font-medium leading-2 text-xl hidden text-gray-700 dark:text-gray-500 capitalize pl-1 small">
                {transfer_action}
            </Text>
        </header>
        <main class="py-4 px-4 overflow-y-scroll">

            {children}
        </main>
    }
}
