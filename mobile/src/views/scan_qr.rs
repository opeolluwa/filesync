use leptos::prelude::{ClassAttribute, CollectView, ElementChild, RwSignal};
use leptos::view;
use thaw::{DrawerBody, DrawerPosition, OverlayDrawer};

#[leptos::component]
pub fn ScanQrCodeUI() -> impl leptos::IntoView {
    let open = RwSignal::new(true);
    let setup_instructions = vec![
        "create Wifi Hotspot",
        "Initiate connection on Filesync Desktop",
        "Scan Qr Code on desktop to begin",
    ];
    view! {
        <div class="flex flex-col">
        // scan to connect
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
