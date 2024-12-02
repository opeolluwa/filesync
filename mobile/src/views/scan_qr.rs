use leptos::view;
use thaw::{Drawer,  Text};
#[leptos::component]
pub fn ScanQrCodeUI() -> impl leptos::IntoView {
    // view! {
    //     <div>
          

    //     </div>
    // }

    let open = RwSignal::new(false);
let position = RwSignal::new(DrawerPosition::Top);

let open_f = move |new_position: DrawerPosition| {
    // Note: Since `show` changes are made in real time,
    // please put it in front of `show.set(true)` when changing `placement`.
    position.set(new_position);
    open.set(true);
};

view! {
    <ButtonGroup>
        <Button on_click=move |_| open_f(DrawerPosition::Top)>"Top"</Button>
        <Button on_click=move |_| open_f(DrawerPosition::Right)>"Right"</Button>
        <Button on_click=move |_| open_f(DrawerPosition::Bottom)>"Bottom"</Button>
        <Button on_click=move |_| open_f(DrawerPosition::Left)>"Left"</Button>
    </ButtonGroup>
    <OverlayDrawer open position>
        <DrawerHeader>
            <DrawerHeaderTitle>
                <DrawerHeaderTitleAction slot>
                    <Button appearance=ButtonAppearance::Subtle on_click=move |_| open.set(false)>
                        "x"
                    </Button>
                </DrawerHeaderTitleAction>
                "Default Drawer"
            </DrawerHeaderTitle>
        </DrawerHeader>
        <DrawerBody>
            <p>"Drawer content"</p>
        </DrawerBody>
    </OverlayDrawer>
}

}
