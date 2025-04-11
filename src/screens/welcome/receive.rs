use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;

use crate::layouts::welcome_screen_layout::WelcomeScreenLayout;

#[leptos::component]
pub fn ReceiveScreen() -> impl leptos::IntoView {
    view! {
        <WelcomeScreenLayout label="Receive">
            <div class="text-center flex flex-col align-center justify-center items-center h-[500px]">
                <p>heheh</p>
                "hh"
            </div>

        </WelcomeScreenLayout>
    }
}
