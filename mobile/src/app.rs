use leptos::*;
use leptos_router::{components::*, path};

use crate::layout::default_layout::DefaultLayout;
use crate::views::about::AboutUI;
use crate::views::history::HistoryUI;
use crate::views::scan_qr::ScanQrCodeUI;
use crate::views::settings::SettingsUi;
use crate::views::share::ShareUI;

#[component]
pub fn MobileApplication() -> impl IntoView {
    view! {
        <DefaultLayout>
            <Router>
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/") view=ScanQrCodeUI />
                    <Route path=path!("/about") view=AboutUI />
                    <Route path=path!("/settings") view=SettingsUi />
                    <Route path=path!("/share") view=ShareUI />
                    <Route path=path!("/about") view=HistoryUI />
                </Routes>
            </Router>
        </DefaultLayout>
    }
}
