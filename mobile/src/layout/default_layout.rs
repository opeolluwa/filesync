use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{children::Children, view};

#[leptos::component]

pub fn DefaultLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! { <div class="px-2 py-16 my-10 min-h-screen w-full">{children}</div> }
}
