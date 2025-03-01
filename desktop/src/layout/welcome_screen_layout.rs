use js_bindgen::navigate::change_location_to;
use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};
use leptos::{prelude::Children, view};

use filesync_icons::chevron::ChevronLeftIcon;
use shared::r#enum::TransferAction;
use thaw::Text;

#[leptos::component]
pub fn WelcomeScreenLayout(children: Children, action: TransferAction) -> impl leptos::IntoView {
    let children = children();
    let go_back_icon = ChevronLeftIcon();
    let transfer_action = action.to_string();
    view! {
        <header
            class="text-gray-500 inline-flex items-center p-2 rounded"
            on:click=move |_| change_location_to("/")
        >

            {go_back_icon}
            <Text class="font-medium leading-2 text-xl text-gray-700 dark:text-gray-500 capitalize pl-1">
                {transfer_action}
            </Text>
        </header>
        <main class="py-4 px-4 overflow-y-scroll">

            {children}
        </main>
    }
}
