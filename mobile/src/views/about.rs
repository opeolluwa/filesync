use leptos::{create_signal, leptos_dom::logging::console_log, spawn_local, view};
use tauri_wasm_bindgen::api::invoke::invoke_tauri_command_without_args;
use thaw::{Flex, FlexAlign, FlexJustify, Image, Text};

#[leptos::component]
pub fn AboutUI() -> impl leptos::IntoView {
    let (app_name, _set_app_name) = create_signal("Filesync");
    let (app_version, _set_app_version) = create_signal("0.7.9");

    spawn_local(async move {
        let app_config = invoke_tauri_command_without_args("get_app_config").await;
        console_log(&app_config.as_string().unwrap());
    });

    view! {
        <Flex vertical=true align=FlexAlign::Center justify=FlexJustify::Center class="my-6">
            <Image src="/images/app-icon.png" class="w-[50px] block mb-5" />
            <Text class="text-center text-medium text-2xl  text-medium text-xl capitalize -mb-2">
                {app_name} {" "} {app_version}
            </Text>
            <div class="small text-gray text-dark text-center flex gap-x-2 cursor pointer">
                <a href="https://github.com/opeolluwa/filesync">
                    "https://github.com/opeolluwa/filesync"
                </a>
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    class="size-4"
                >
                    <path d="M12.232 4.232a2.5 2.5 0 0 1 3.536 3.536l-1.225 1.224a.75.75 0 0 0 1.061 1.06l1.224-1.224a4 4 0 0 0-5.656-5.656l-3 3a4 4 0 0 0 .225 5.865.75.75 0 0 0 .977-1.138 2.5 2.5 0 0 1-.142-3.667l3-3Z" />
                    <path d="M11.603 7.963a.75.75 0 0 0-.977 1.138 2.5 2.5 0 0 1 .142 3.667l-3 3a2.5 2.5 0 0 1-3.536-3.536l1.225-1.224a.75.75 0 0 0-1.061-1.06l-1.224 1.224a4 4 0 1 0 5.656 5.656l3-3a4 4 0 0 0-.225-5.865Z" />
                </svg>

            </div>
        </Flex>
    }
}
