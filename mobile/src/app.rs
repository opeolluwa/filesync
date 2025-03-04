use filesync_icons::chevron::ChevronUpDownIcon;
use filesync_icons::dots::DotVertical;
use filesync_icons::scan_qr_icon::ScanQrIcon;
use leptos::prelude::{CollectView, ElementChild, Get};
use leptos::{
    component,
    prelude::{ClassAttribute, RwSignal},
    view,
};
use leptos_router::path;

use thaw::{Tab, TabList};

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
    let selected_value = RwSignal::new(String::from("Apps"));

    let tabs = vec![
        TabConfig::new("History"),
        TabConfig::new("Apps"),
        TabConfig::new("Pictures"),
        TabConfig::new("Downloads"),
        TabConfig::new("Videos"),
    ];

    view! {
        <header class="bg-app px-4 text-white">

            <div class="flex items-center justify-between hidden">
                <button class="size-4">
                    <DotVertical />
                </button>
              <button class="size-4">
                    <ScanQrIcon />
                </button>

            </div>
            <TabList
                selected_value
                class="flex justify-between  pt-6 pb-2 overflow-scroll "
            >
                {tabs
                    .into_iter()
                    .map(|tab| {
                        view! { <Tab value=tab.value>{tab.name}</Tab> }
                    })
                    .collect_view()}

            </TabList>
        </header>

        <main class="px-4 pt-5">
            {
            match selected_value.get() {
                val if val == "app".to_string() => view! { "app" },
                _ => "heheh",
            }}
        </main>

        <button class="fab">
            <ChevronUpDownIcon />
        </button>
    }
}
