use leptos::view;
use thaw::{Upload, UploadDragger};

#[leptos::component]
pub fn ShareScreen() -> impl leptos::IntoView {
    view! {
        <Upload multiple=true>
            <UploadDragger class="w-full h-full dark:bg-gray-900/40">

                "Click or drag a file to this area to upload"

            </UploadDragger>
        </Upload>
    }
}
