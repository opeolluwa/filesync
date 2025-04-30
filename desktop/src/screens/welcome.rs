use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos_qr::QrCode;

#[leptos::component]
pub fn WelcomeScreen() -> impl leptos::IntoView {
    view! {
        <div class="text-center flex flex-col align-center justify-center items-center h-[90%]">
                <div class="w-[150px] h-[150px] block mx-auto mb-2 rounded-md bg-blend-multiply border-[1px] ">
                    <QrCode
                        data="wifi_creds"
                        ecl=leptos_qr::ECL::Q
                        shape=leptos_qr::Shape::Square
                        bg_color="#f5f7f7"
                        fg_color="#0c0c0c"
                    />
                </div>

                <p class="font-medium leading-2  text-gray-700 dark:text-gray-400 mt-3 ">
                    "Scan QR code to pair mobile device"
                </p>
                <p class="text-base hidden dark:text-gray-500 text-center w-2/3 mx-auto  ">
                    "Open the mobile companion app and follow prompt to scan QR code"
                </p>

        </div>
    }
}
