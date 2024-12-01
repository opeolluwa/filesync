use leptos::view;
use thaw::{Upload, UploadDragger};

#[leptos::component]
pub fn ShareUI() -> impl leptos::IntoView {
    view! {
        <Upload multiple=true>
            <UploadDragger>
                <div class="w-full h-full dark:bg-gray-900/40">
                    "Click or drag a file to this area to upload"
                </div>
            </UploadDragger>
        </Upload>
    }
}
