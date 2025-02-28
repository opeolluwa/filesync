use leptos::prelude::{ClassAttribute, CollectView, ElementChild};
use leptos::view;
use leptos_qr::QrCode;
use thaw::Text;

#[leptos::component]
pub fn SenderScreen() -> impl leptos::IntoView {
    let send_steps = [
        "Create Wi-fi hostpot on yor phone",
        "Connect your Laptop the phone hotspot",
        "Open Filesync mobile",
        "Initialize a receive action",
        "Scan Qr code below",
    ];

    view! {
          <div class="pl-3">
              <Text class="font-medium leading-2 text-xl text-gray-700 dark:text-gray-400">
                  Connect mobile
              </Text>
            <div >
    <ol class="list-decimal list-inside pl-2 ml-4 mt-4 ">
                  {send_steps.map(|step| view! { <li>{step}</li> }).collect_view()}
              </ol>
            </div>
              <div class="w-48 h-48 hidden ">
                  <QrCode
                      data="Hello, World!"
                      ecl=leptos_qr::ECL::Q
                      shape=leptos_qr::Shape::RoundedSquare
                      fg_color="#111111"
                      bg_color="#dddddd"
                  />
              </div>
          </div>
      }
}
