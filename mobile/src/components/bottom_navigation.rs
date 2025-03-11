use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{view, IntoView};

use filesync_icons::arrow_left_right_icon::ArrowLeftRightIconSolid;
use filesync_icons::cog::SettingsIconOutline;
use filesync_icons::history_icon::HistoryIcon;
use filesync_icons::home_icon::HomeIcon;

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
        <a href=href class="flex flex-col items-center p-0 m-0  w-full rounded-lg">
            {icon}
            <span class="text-sm capitalize">{label}</span>
        </a>
    }
}

#[leptos::component]

pub fn BottomNavigation() -> impl leptos::IntoView {
    let settings_icon = SettingsIconOutline();
    let transfer_icon = ArrowLeftRightIconSolid();
    let home_icon = HomeIcon();
    let history_icon = HistoryIcon();

    view! {
        <nav class="flex items-center justify-between bottom-nav">
            <BottomNavigationRoute label="home" href="/" icon=home_icon />
            <BottomNavigationRoute label="share" href="/share" icon=transfer_icon />
            <BottomNavigationRoute label="history" href="/share" icon=history_icon />
            <BottomNavigationRoute label="settings" href="/settings" icon=settings_icon />

        </nav>
    }
}
