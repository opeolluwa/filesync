use leptos::*;
use leptos_router::{Route, Router, Routes};

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
                <Routes>
                    <Route path="/" view=ScanQrCodeUI />
                    <Route path="/about" view=AboutUI />
                    <Route path="/settings" view=SettingsUi />
                    <Route path="/share" view=ShareUI />
                    <Route path="/about" view=HistoryUI />
                </Routes>
            </Router>
        </DefaultLayout>
    }
}
