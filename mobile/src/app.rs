use filesync_icons::chevron::ChevronUpDownIcon;
use filesync_icons::dots::DotVertical;
use filesync_icons::menu_icon::MenuIcon;
use filesync_icons::scan_qr_icon::ScanQrIcon;
use leptos::prelude::{CollectView, ElementChild, Get};
use leptos::{
    component,
    prelude::{ClassAttribute, RwSignal},
    view,
};
use leptos_router::path;

use thaw::{Tab, TabList};

use crate::components::toolbar::Toolbar;

struct TabConfig {
    name: String,
    value: String,
}

impl TabConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: name.to_lowercase().to_string(),
        }
    }
}
#[component]
pub fn MobileApplication() -> impl leptos::IntoView {
    let selected_value = RwSignal::new(String::from(""));

    let tab_class_rule = "text-sm ";
    let tabs = vec![
        TabConfig::new("History"),
        TabConfig::new("Apps"),
        TabConfig::new("Pictures"),
        TabConfig::new("Downloads"),
        TabConfig::new("Videos"),
    ];

    view! {
        <header class="bg-app px-4 text-white fixed left-0 top-0 right-0 w-full pt-3">

            <div class="flex items-center justify-between py-1 hidden">
                <button class="size-4">
                    <DotVertical />
                </button>
                <button class="size-4">
         <MenuIcon/>
                </button>
            </div>
            <TabList selected_value class="flex justify-between  pt-6 pb-2 overflow-scroll ">
                {tabs
                    .into_iter()
                    .map(|tab| {
                        view! { <Tab value=tab.value class=tab_class_rule>{tab.name}</Tab> }
                    })
                    .collect_view()}

            </TabList>
        </header>

        <main class="px-4 pt-5">
        {selected_value.get()}

        </main>

        <button class="fab hidden">
            <ChevronUpDownIcon />
        </button>

         <footer class="w-[80%]  mx-auto rounded-full fixed bottom-10 left-0 right-0 z-50 border-gray-200 border-[0.25px]  shadow-xl py-0">
            <Toolbar />
        </footer>
    }
}
