use leptos::prelude::ElementChild;
use leptos::{
    children::Children,
    prelude::{ClassAttribute, StyleAttribute},
    view,
};

#[leptos::component]
pub fn DefaultLayout(children: Children) -> impl leptos::IntoView {
    let children = children();
    view! {
        <div
            class="grid grid-cols-12 mb-0 pb-0 w-[100vw] gap-x-2"
            style="height:100vh; overflow-y: hidden; margin-bottom:0 dark:bg-gray-900"
        >
            <main class="col-span-12 py-4 px-4 overflow-y-scroll h-screen">{children}</main>
        </div>
    }
}
