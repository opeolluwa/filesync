use crate::views::about::AboutUI;
use crate::views::history::HistoryUI;
use crate::views::settings::SettingsUi;
use crate::views::share::ShareUI;
use crate::views::transfer::TransferUI;
use crate::{layout::desktop_layout::DesktopLayout, views::home::HomeUI};
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
                    <Route path=path!("/home") view=HomeUI />
                    <Route path=path!("/about") view=AboutUI />
                    <Route path=path!("/settings") view=SettingsUi />
                    <Route path=path!("/share") view=ShareUI />
                    <Route path=path!("/history") view=HistoryUI />
                </Routes>
            </DesktopLayout>
        </Router>
    }
}
