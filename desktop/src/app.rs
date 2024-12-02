use crate::layout::desktop_layout::DesktopLayout;
use crate::views::about::AboutUI;
use crate::views::history::HistoryUI;
use crate::views::settings::SettingsUi;
use crate::views::share::ShareUI;
use crate::views::transfer::TransferUI;
use leptos::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn DesktopApplication() -> impl leptos::IntoView {
    view! {
        <Router>
            <DesktopLayout>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=TransferUI />
                    <Route path=path!("/about") view=AboutUI />
                    <Route path=path!("/settings") view=SettingsUi />
                    <Route path=path!("/share") view=ShareUI />
                    <Route path=path!("/about") view=HistoryUI />
                </Routes>
            </DesktopLayout>
        </Router>
    }
}
