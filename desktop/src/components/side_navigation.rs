use filesync_icons::{
    arrow_left_right_icon::ArrowLeftRightIconSolid, home_icon::HomeIcon,
  
    share_icon::ShareIconSolid,
};
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{view, IntoView};

#[leptos::component]
pub fn SideNavigationRoute<F>(
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
            class="text-gray-500 flex flex-col justify-center items-center dark:hover:bg-gray-700/40 hover:bg-app-50 hover:text-app p-3 w-full rounded-lg"
        >
            {icon}
            <span class="sr-only">{label}</span>
        </a>
    }
}

#[leptos::component]

pub fn SideNavigation() -> impl leptos::IntoView {

    let transfer_icon = ArrowLeftRightIconSolid();
    let share_icon = ShareIconSolid();
    let home_icon = HomeIcon();

    view! {
        <div class="flex flex-col relative items-center justify-center gap-y-4 px-2">
            <SideNavigationRoute label="transfer" href="/home" icon=home_icon />
            <SideNavigationRoute label="transfer" href="/" icon=transfer_icon />
            <SideNavigationRoute label="share" href="/share" icon=share_icon />
        </div>
    }
}
