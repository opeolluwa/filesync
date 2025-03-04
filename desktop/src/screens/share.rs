use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};
use leptos::view;
use thaw::{FileList, Upload, UploadDragger};

#[leptos::component]
pub fn ShareScreen() -> impl leptos::IntoView {
    let custom_request = move |file_list: FileList| {
        let _len = file_list.length();
        println!("heheh")
    };

    view! {
        <Upload
            multiple=true
            custom_request
            class="w-full border-[2px] border-app-200/50 dark:border-gray-600/50 h-full flex justify-center items-center dark:bg-gray-900/40 rounded-lg border-dashed border"
        >
            <UploadDragger class="flex flex-col  justify-center items-center h-inherit size-full border-app/20 border-2  w-[89vw] h-[93vh]">
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
    }
}
