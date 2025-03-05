use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn MenuIcon() -> impl leptos::IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            class="size-6"
            viewBox="0 0 24 24"
            fill="currentColor"
        >
            <path d="M18 18V20H6V18H18ZM21 11V13H3V11H21ZM18 4V6H6V4H18Z"></path>
        </svg>
    }
}
