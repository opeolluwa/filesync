use crate::components::toolbar::Toolbar;
use filesync_icons::chevron::ChevronUpDownIcon;
use filesync_icons::dots::DotVertical;
use filesync_icons::menu_icon::MenuIcon;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::{CollectView, ElementChild, Get};
use leptos::{
    component,
    prelude::{ClassAttribute, RwSignal},
    view,
};
use leptos_router::path;
use tauri_wasm_bindgen::plugins::barcode::scan_barcode;
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
            <header class="bg-app px-4 text-white fixed left-0 top-0 right-0 w-full pt-4">

                <div class="flex items-center justify-between py-1 hidden">
                    <button class="size-4">
                        <DotVertical />
                    </button>
                    <button class="size-4">
             <MenuIcon/>
                    </button>
                </div>
                <TabList selected_value class="flex justify-between  pt-4 pb-2 overflow-scroll ">
                    {tabs
                        .into_iter()
                        .map(|tab| {
                            view! { <Tab value=tab.value class=tab_class_rule>{tab.name}</Tab> }
                        })
                        .collect_view()}

                </TabList>
            </header>

            <main class="px-4 pt-20">
            {active_tab}

       <div id="qr-reader" style="width:100%; padding-top: 800px;"></div>
    <div id="qr-reader-results"></div>
            </main>

            <button class="fab flex flex-col items-center justify-center size-6"  id="qr-trigger">
                <ChevronUpDownIcon />
            </button>

             <footer class="w-[80%] hidden  mx-auto rounded-full fixed bottom-10 left-0 right-0 z-50 border-gray-200 border-[0.25px]  shadow-xl py-0">
                <Toolbar />
            </footer>
      <script src="https://unpkg.com/html5-qrcode"></script>

      <script>
        var resultContainer = document.getElementById("qr-reader-results");
          var lastResult, countResults = 0;

        //   function onScanSuccess(decodedText, decodedResult) {
            // if (decodedText !== lastResult) {
            //   ++countResults;
            //   lastResult = decodedText;
              // Handle on success condition with the decoded message.
            //   console.log(`Scan result ${decodedText}`, decodedResult);
            // }
        //   }

        //   var html5QrcodeScanner = new Html5QrcodeScanner(
            // "qr-reader", { fps: 10, qrbox: 250 });
        //   html5QrcodeScanner.render(onScanSuccess);
      </script>


        }
}
