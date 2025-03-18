use filesync_icons::arrow_left_right_icon::ArrowLeftRightIconSolid;
use filesync_icons::cog::SettingsIconOutline;
use filesync_icons::menu_icon::MenuIcon;
use filesync_icons::platform::AndroidLogo;
use filesync_icons::platform::LinuxLogo;
use filesync_icons::platform::MacOsLogo;
use filesync_icons::platform::WindowsPlatformLogo;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::IntoAttribute;
use leptos::prelude::{ClassAttribute, ElementChild, RwSignal, Set};
use leptos::{view, IntoView};
use thaw::{DrawerBody, DrawerPosition, OverlayDrawer};

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

      let platform_logo_class_rules = "dark:bg-gray-700 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200  p-4 rounded-xl shadow hover:shadow-none cursor-pointer dark:hover:bg-gray-700/50 size-10";

    let android = AndroidLogo();
    let macOs = MacOsLogo();
    let windows = WindowsPlatformLogo();
    let linux = LinuxLogo();
    
    let select_action= view! {
         <div class="text-center  ">
            <div>
                <h1 class="font-medium leading-2 text-2xl text-gray-700 dark:text-gray-400 ">
                    "What would you like to do?"
                </h1>
                <p class="text-base dark:text-gray-500 ">"Do you want to send or receive files?"</p>
                <div class="flex justify-center gap-x-5 items-center mt-8">
                    <button
                        class="flex flex-col items-center  "

                    >
                        <div class="dark:bg-gray-700 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200  p-4 rounded-xl shadow hover:shadow-none cursor-pointer dark:hover:bg-gray-700/50">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="size-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M8.25 6.75 12 3m0 0 3.75 3.75M12 3v18"
                                />
                            </svg>

                        </div>
                        <p class="mt-2 dark:text-gray-500 ">Send File</p>
                    </button>
                    <div class="flex flex-col items-center ">

                        <button
                            class=" dark:bg-gray-700 dark:hover:bg-gray-700/50 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200 p-4 rounded-xl shadow hover:shadow-none cursor-pointer"

                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="size-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M15.75 17.25 12 21m0 0-3.75-3.75M12 21V3"
                                />
                            </svg>
                        </button>
                        <p class="mt-2 dark:text-gray-500 ">Receive File</p>
                    </div>
                </div>
            </div>
        </div>
    };
    let open = RwSignal::new(false);
    let position = RwSignal::new(DrawerPosition::Bottom);


    let select_pltform = view! {

            <div class="text-center flex flex-col align-center justify-center items-center ">
                <div>
                    <h1 class="font-medium leading-2 text-2xl text-gray-700 dark:text-gray-400">
                        "Select platform of the other device"
                    </h1>

                    <div class="flex justify-center items-center gap-x-5 mt-8  ">
                        <button
                            class=platform_logo_class_rules
                          
                        >
                            {android}
                        </button>
                        <button class=platform_logo_class_rules>{macOs}</button>
                        <button class=platform_logo_class_rules>{windows}</button>
                        <button class=platform_logo_class_rules>{linux}</button>
                    </div>
                </div>
            </div>
    };
    view! {
          <OverlayDrawer open position>
        <DrawerBody class="flex flex-col items-center justify-center">
          {select_action}
        </DrawerBody>
    </OverlayDrawer>

        <nav class="flex  items-center justify-between">
            <BottomNavigationRoute label="home" href="/" icon=MenuIcon() />
            <BottomNavigationRoute label="share" href="/share" icon=transfer_icon on:click=move |_|     open.set(true) />

            <BottomNavigationRoute label="settings" href="/settings" icon=settings_icon />

        </nav>




    }
}
