use crate::components::bottom_navigation::BottomNavigation;
use filesync_icons::dots::DotVertical;
use filesync_icons::menu_icon::MenuIcon;
use leptos::prelude::{CollectView, ElementChild, Get};
use leptos::{
    component,
    prelude::{ClassAttribute, RwSignal},
    view,
};
use leptos_router::path;
use thaw::{ConfigProvider, Tab, TabList};
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
    let selected_value = RwSignal::new(String::from("history"));

    let active_tab = move || selected_value.get();

    let tab_class_rule = "text-sm ";
    let tabs = vec![
        TabConfig::new("History"),
        TabConfig::new("Apps"),
        TabConfig::new("Pictures"),
        TabConfig::new("Downloads"),
        TabConfig::new("Videos"),
    ];

    view! {
            <header class="bg-app fixed px-4 text-white left-0 top-0 right-0 w-full pt-4 ">

                <div class="flex items-center justify-between py-1 hidden">
                    <button class="size-4">
                        <DotVertical />
                    </button>
                    <button class="size-4">
             <MenuIcon/>
                    </button>
                </div>
                <div class="flex justify-between  pt-4 pb-2 overflow-scroll ">
                    {tabs
                        .into_iter()
                        .map(|tab| {
                            view! { <button >{tab.name}</button> }
                        })
                        .collect_view()}

                </div>
            </header>

            <main class="px-4  pt-20 h-screen overflow-y-scroll overflow-x-hidden">
            {active_tab}


    
            </main>

 

             <footer class="bg-gray-50 border-t border-1 border-gray-100/50  fixed text-gray-500 w-full left-0 right-0 z-50 border-gray-200 border-[0.25px]  bottom-0 py-2">
                <BottomNavigation />
            </footer>
     



        }
}
