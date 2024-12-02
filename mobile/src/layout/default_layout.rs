use leptos::{view, Children};
#[leptos::component]

pub fn DefaultLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! { <div class="px-2 py-16 my-10 min-h-screen w-full">{children}</div> }
}
