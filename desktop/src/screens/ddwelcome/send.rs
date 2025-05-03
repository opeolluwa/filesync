use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};
use leptos::view;
use thaw::{FileList, Upload, UploadDragger};
use web_sys::console;

use js_bindgen::navigate::change_location_to;

use crate::layouts::welcome_screen_layout::WelcomeScreenLayout;

#[leptos::component]
pub fn SendScreen() -> impl leptos::IntoView {
    let custom_request = move |file_list: FileList| {
        let total_number_of_files = file_list.length();

        //TODO: use parallel processing
        for index in 0..=total_number_of_files {
            let file_name = file_list.item(index).unwrap().name();
            let _web_sysfile_blob: js_sys::Promise = file_list.item(index).unwrap().array_buffer();
            change_location_to("/home");
            console::log_1(&file_name.into());
        }

        console::log_1(&"Hello using web-sys".into());
        console::log_1(&file_list.into());
    };

    view! {
        <WelcomeScreenLayout label="Send">
            <Upload
                multiple=true
                custom_request
                class="w-full border-[2px] border-app-200/50 dark:border-gray-600/50 h-full flex justify-center items-center dark:bg-gray-900/20 rounded border-dashed border"
            >
                <UploadDragger class="flex flex-col  justify-center items-center h-inherit size-full border-app/20 border-2  w-[89vw] h-[80vh]">
                    <div class="flex flex-col justify-center items-center text-gray-400">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width=".5"
                            stroke="currentColor"
                            class="size-32"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M3.75 9.776c.112-.017.227-.026.344-.026h15.812c.117 0 .232.009.344.026m-16.5 0a2.25 2.25 0 0 0-1.883 2.542l.857 6a2.25 2.25 0 0 0 2.227 1.932H19.05a2.25 2.25 0 0 0 2.227-1.932l.857-6a2.25 2.25 0 0 0-1.883-2.542m-16.5 0V6A2.25 2.25 0 0 1 6 3.75h3.879a1.5 1.5 0 0 1 1.06.44l2.122 2.12a1.5 1.5 0 0 0 1.06.44H18A2.25 2.25 0 0 1 20.25 9v.776"
                            />
                        </svg>

                        <p class="mt-4 leading">Drag and drop your files here</p>
                    </div>
                </UploadDragger>
            </Upload>
        </WelcomeScreenLayout>
    }

    // let send_steps = [
    //     "Create Wi-fi hostpot on yor phone",
    //     "Connect your Laptop the phone hotspot",
    //     "Open Filesync mobile",
    //     "Initialize a receive action",
    //     "Scan Qr code below",
    // ];

    // view! {
    //   <WelcomeScreenLayout action=shared::r#enum::TransferAction::Send>
    //     <div class="pl-3">
    //         <Text class="font-medium leading-2 text-xl text-gray-700 dark:text-gray-400">
    //             Connect mobile
    //         </Text>
    //         <div>
    //             <ol class="list-decimal list-inside pl-2 ml-4 mt-4 ">
    //                 {send_steps.map(|step| view! { <li>{step}</li> }).collect_view()}
    //             </ol>
    //         </div>
    //         <div class="w-48 h-48 hidden ">
    //             <QrCode
    //                 data="Hello, World!"
    //                 ecl=leptos_qr::ECL::Q
    //                 shape=leptos_qr::Shape::RoundedSquare
    //                 fg_color="#111111"
    //                 bg_color="#dddddd"
    //             />
    //         </div>
    //     </div>
    //     </WelcomeScreenLayout>
    // }
}
