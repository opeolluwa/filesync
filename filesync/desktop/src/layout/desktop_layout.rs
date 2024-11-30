use leptos::{view, Children};
use crate::components::side_navigation::nav::SideNavigation;

#[leptos::component]
pub fn DesktopLayout(children: Children) -> impl leptos::IntoView {
    let children = children();
    view! {
        <div
            class="grid grid-cols-12 mb-0 pb-0 w-[100vw] gap-x-2"
            style="height:100vh; overflow-y: hidden; margin-bottom:0 dark:bg-gray-900"
        >
            <nav
                class="col-span-1 bg-[rgba(249,250,254,255)] dark:bg-gray-900/50 px-[2px] fixed  text-gray-600 pt-10 "
                style="height: calc(100vh-200px); overflowY: hidden; position: relative"
            >
            <SideNavigation/>
            </nav>

            <main class="col-span-11 pt-10 px-10   overflow-y-scroll">{children}</main>
        </div>
    }
}
