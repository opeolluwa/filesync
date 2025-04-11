use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos_qr::QrCode;

#[leptos::component]
pub fn WelcomeScreen() -> impl leptos::IntoView {
    view! {
        <div class="text-center flex flex-col align-center justify-center items-center h-[90%]">
                <div class="w-[175px] h-[175px] block mx-auto mb-2 rounded-xl bg-blend-multiply border-[1px] border-gray-400 p-2">
                    <QrCode
                        data="wifi_creds"
                        ecl=leptos_qr::ECL::Q
                        shape=leptos_qr::Shape::Square
                        bg_color="#f5f7f7"
                        fg_color="#0c0c0c"
                    />
                </div>

                <h1 class="font-medium leading-2 text-2xl text-gray-700 dark:text-gray-400 ">
                    "Scan QR code to pair mobile device"
                </h1>
                <p class="text-base dark:text-gray-500 text-center w-2/3 mx-auto  ">
                    "Open the mobile companion app and follow prompt to scan QR code"
                </p>

        </div>
    }
}
