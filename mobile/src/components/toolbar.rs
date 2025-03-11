use filesync_icons::cloud::{CloudDownloadIconSolid, CloudUploadIconSolid};
use filesync_icons::cog::CogSolid;
use filesync_icons::home_icon::HomeIconSolid;
use filesync_icons::scan_qr_icon::ScanQrIcon;
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
                "text-gray-500 flex flex-col justify-center items-center dark:hover:bg-gray-700/40 hover:bg-app-50/50 hover:text-app  p-2 ripple-effect ripple-app-500   {}",
                class,
            )
        >
         <span class="size-5">   {icon} </span>
            <span class="sr-only">{label}</span>
        </a>
    }
}

#[leptos::component]
pub fn Toolbar() -> impl leptos::IntoView {
    let home_icon = HomeIconSolid();
    let settings_icon = CogSolid();
    let upload_icon = CloudUploadIconSolid();
    let download_icon = CloudDownloadIconSolid();
    let scan_icon = ScanQrIcon();

    view! {
        <div class="flex items-center justify-evenly py-2">
            <ToolbarItem label="Home" href=HOME_ROUTE icon=home_icon class="rounded-l-full" />
            <ToolbarItem label="Send" href=SEND_ROUTE icon=upload_icon />
            <ToolbarItem label="Receive" href=RECEIVE_ROUTE icon=download_icon />
            <ToolbarItem label="Settings" href=SETTINGS_ROUTE icon=settings_icon />
            <ToolbarItem
                label="history"
                href=HISTORY_ROUTE
                icon=scan_icon
                class="rounded-r-full border-none"
            />
        </div>
    }
}
