use filesync_icons::arrow_left_right_icon::ArrowLeftRightIconSolid;
use filesync_icons::home_icon::HomeIcon;
use filesync_icons::settings_icon::SettingsIconOutline;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{view, IntoView};

#[leptos::component]
pub fn BottomNavigationRoute<F>(
    label: &'static str,
    href: &'static str,
    icon: F,
) -> impl leptos::IntoView
where
    F: IntoView,
{
    view! {
        <a
            href=href
            class="text-gray-500 flex flex-col items-center  hover:text-app p-0 m-0  w-full rounded-lg"
        >
            {icon}
            <span class="sr-only">{label}</span>
        </a>
    }
}

#[leptos::component]

pub fn BottomNavigation() -> impl leptos::IntoView {
    let settings_icon = SettingsIconOutline();
    let transfer_icon = ArrowLeftRightIconSolid();
    let home_icon = HomeIcon();

    view! {
        <nav class="flex items-center justify-between">
            <BottomNavigationRoute label="home" href="/" icon=home_icon />
            <BottomNavigationRoute label="share" href="/share" icon=transfer_icon />
            <BottomNavigationRoute label="settings" href="/settings" icon=settings_icon />

        </nav>
    }
}
