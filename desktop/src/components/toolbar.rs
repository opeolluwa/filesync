use filesync_icons::chevron::ChevronUpDownIcon;
use filesync_icons::cog::SettingsIconOutline;
use filesync_icons::download_icon::DownloadIcon;
use filesync_icons::home_icon::HomeIcon;
use filesync_icons::upload_icon::UploadIcon;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{view, IntoView};

use crate::routes::{HISTORY_ROUTE, HOME_ROUTE, RECEIVE_ROUTE, SEND_ROUTE, SETTINGS_ROUTE};

#[leptos::component]
pub fn ToolbarItem<F>(
    label: &'static str,
    href: &'static str,
    icon: F,
    #[prop(optional)] class: &'static str,
) -> impl leptos::IntoView
where
    F: IntoView + 'static,
{
    view! {
        <a
            href=href
            class=format!(
                "text-gray-500 flex flex-col justify-center items-center dark:hover:bg-gray-700/40 hover:bg-app-50/50 hover:text-app  w-full py-3 ripple-effect ripple-app-500  border-r-gray-600 border-r border-r-[0.75px] {}",
                class,
            )
        >
            {icon}
            <span class="sr-only">{label}</span>
        </a>
    }
}

#[leptos::component]
pub fn Toolbar() -> impl leptos::IntoView {
    let home_icon = HomeIcon();
    let settings_icon = SettingsIconOutline();
    let upload_icon = UploadIcon();
    let download_icon = DownloadIcon();
    let history_icon = ChevronUpDownIcon();

    view! {
        <div class="flex items-center justify-center">
            <ToolbarItem label="Home" href=HOME_ROUTE icon=home_icon class="rounded-l-full" />
            <ToolbarItem label="Send" href=SEND_ROUTE icon=upload_icon />
            <ToolbarItem label="Receive" href=RECEIVE_ROUTE icon=download_icon />
            <ToolbarItem label="Settings" href=SETTINGS_ROUTE icon=settings_icon />
            <ToolbarItem
                label="history"
                href=HISTORY_ROUTE
                icon=history_icon
                class="rounded-r-full border-none"
            />
        </div>
    }
}
