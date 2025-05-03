use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::view;
use leptos_qr::QrCode;

#[leptos::component]
pub fn WelcomeScreen() -> impl leptos::IntoView {
    let (greet_msg, set_greet_msg) = signal(String::new());

    // let gree = move |_|{
    spawn_local(async move {
        let result = call_greet("alex").await;
        set_greet_msg.set(result);
    });
    // };

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
                          <p  class="font-medium leading-2  text-gray-700 dark:text-gray-400 mt-3 "> hey { move || greet_msg.get() }</p>

               <button  class="font-medium leading-2  text-gray-700 dark:text-gray-400 mt-3 " > refresh  </button>

        </div>
    }
}

async fn call_greet(name: &str) -> String {
    shared::cmd::greet(&name).await
}
