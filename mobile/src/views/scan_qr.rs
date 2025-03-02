use leptos::prelude::{ClassAttribute, CollectView, ElementChild, RwSignal};
use leptos::view;
use thaw::Image;
use thaw::{DrawerBody, DrawerPosition, OverlayDrawer};

#[leptos::component]
pub fn ScanQrCodeUI() -> impl leptos::IntoView {
    let open = RwSignal::new(false);
    let setup_instructions = vec![
        "create Wifi Hotspot",
        "Initiate connection on Filesync Desktop",
        "Scan Qr Code on desktop to begin",
    ];
    let folders = vec!["filesync", "audio", "video", "images", "document", "more"];

    view! {
        <div class="flex flex-col gap-y-5">

            // folders
            <div class="grid grid-cols-2 gap-4 gap-x-6 items-center text-gray-600">
                {folders
                    .into_iter()
                    .map(|folder_name| {
                        view! {
                            <div class="col-span-1 my-5 flex flex-col items-center justify-center gap-y-1 bg-app-50/50 py-4 rounded-xl">
                                <Image src="/assets/folder-icon.png" class="w-[60px] block" />
                                <span class="capitalize font-medium">{folder_name}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
        <OverlayDrawer open position=DrawerPosition::Bottom>
            <DrawerBody class="rounded-t-lg relative">
                <h1 class="text-xl font-medium">Peer device</h1>
                <ol class="list-decimal mt-6 hidden">
                    {setup_instructions
                        .into_iter()
                        .map(|list_item| {
                            view! {
                                <li class="py-2 text-gray-400 text-sm leading-1 first-letter:capitalize">
                                    {list_item}
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
                <button class="btn w-full  text-white bg-app text-center absolute bottom-2 left-0 right-0 ">
                    Continue
                </button>
            </DrawerBody>
        </OverlayDrawer>
    }
}
