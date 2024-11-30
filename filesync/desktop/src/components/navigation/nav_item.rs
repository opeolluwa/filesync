use leptos::{view, IntoView};
use leptos_remix_icon::Icon;

#[leptos::component]
pub fn NavItem(route: String, label: String, icon_class: &'static str) -> impl IntoView {
    view! {
        <a href=route>
            <Icon icon=icon_class class="text-gray-500" />
            <span class="sr-only">{label}</span>
        </a>
    }
}
