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

// use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};
// use leptos::view;

// #[leptos::component]
// pub fn MenuIcon() -> impl leptos::IntoView {
//     view! {
//         <svg
//             xmlns="http://www.w3.org/2000/svg"
//             class="size-6"
//             viewBox="0 0 24 24"
//             fill="currentColor"
//         >
//             <path d="M15 3H21V8H19V5H15V3ZM9 3V5H5V8H3V3H9ZM15 21V19H19V16H21V21H15ZM9 21H3V16H5V19H9V21ZM3 11H21V13H3V11Z"></path>
//         </svg>
//     }
// }
