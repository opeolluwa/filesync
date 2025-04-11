use leptos::prelude::CustomAttribute;
use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute};
use leptos::view;
use leptos_qr::QrCode;

use js_bindgen::navigate::change_location_to;

use crate::routes::{RECEIVE_ROUTE, SELECT_PLATFORM_ROUTE};

#[leptos::component]
pub fn DefaultScreen() -> impl leptos::IntoView {
    let app_action_css_rule ="dark:bg-gray-700 dark:hover:bg-gray-700/50 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app-600 transition-all duration-200 p-4 rounded-xl shadow hover:shadow-none cursor-pointer  flex flex-col ites-center justify-center";

    view! {
        <div class="text-center flex flex-col align-center justify-center items-center h-[90%]">
            <div>
                <div class="w-[175px] h-[175px]">
                    <QrCode
                        data="wifi_creds"
                        ecl=leptos_qr::ECL::Q
                        shape=leptos_qr::Shape::Square
                        fg_color="#f7f7f7"
                        bg_color="#1d232a"
                    />
                </div>

                <h1 class="font-medium leading-2 text-2xl text-gray-700 dark:text-gray-400 ">
                    "Scan QR code to pair mobile device"
                </h1>
                <p class="text-base dark:text-gray-500 ">
                    "Open the mobile companion app and follow prompt to scan QR code"
                </p>

            </div>
        </div>
    }
}
