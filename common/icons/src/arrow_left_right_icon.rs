use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn ArrowLeftRightIconSolid() -> impl leptos::IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            class=format!("size-6")
        >
            <path d="M16 16V12L21 17L16 22V18H4V16H16ZM8 2V5.999L20 6V8H8V12L3 7L8 2Z"></path>
        </svg>
    }
}
