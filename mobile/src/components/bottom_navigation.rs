use filesync_icons::menu_icon::MenuIcon;
use leptos::prelude::{ClassAttribute, ElementChild, RwSignal, Set};
use leptos::{view, IntoView};
use leptos::prelude::AddAnyAttr;
use filesync_icons::arrow_left_right_icon::ArrowLeftRightIconSolid;
use filesync_icons::cog::SettingsIconOutline;
use thaw::{
    Button, ButtonAppearance,  DrawerBody, DrawerHeader, DrawerHeaderTitle, DrawerHeaderTitleAction, DrawerPosition, OverlayDrawer
};
use leptos::prelude::IntoAttribute;

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
            <small class=" font-medium mt-[2px] capitalize ">{label}</small>
        </a>
    }
}

#[leptos::component]

pub fn BottomNavigation() -> impl leptos::IntoView {
    let settings_icon = SettingsIconOutline();
    let transfer_icon = ArrowLeftRightIconSolid();

    let open = RwSignal::new(false);
    let position = RwSignal::new(DrawerPosition::Top);

    let open_f = move |new_position: DrawerPosition| {
        // Note: Since `show` changes are made in real time,
        // please put it in front of `show.set(true)` when changing `placement`.
        position.set(new_position);
        open.set(true);
    };

    view! {
        <nav class="flex  items-center justify-between">
            <BottomNavigationRoute label="home" href="/" icon=MenuIcon() />
            <BottomNavigationRoute label="share" href="/share" icon=transfer_icon on:click=move |_| open_f(DrawerPosition::Bottom) />
            // <BottomNavigationRoute label="history" href="/share" icon=history_icon />
            <BottomNavigationRoute label="settings" href="/settings" icon=settings_icon />

        </nav>



        <OverlayDrawer open position>
        <DrawerHeader>
          <DrawerHeaderTitle>
            <DrawerHeaderTitleAction slot>
                 <Button
                    appearance=ButtonAppearance::Subtle
                    on_click=move |_| open.set(false)
                >
                    "x"
                </Button>
            </DrawerHeaderTitleAction>
            "Default Drawer"
          </DrawerHeaderTitle>
        </DrawerHeader>
        <DrawerBody>
          <p>"Drawer content"</p>
        </DrawerBody>
    </OverlayDrawer>
    }
}
