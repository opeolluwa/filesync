use crate::screens::about::AboutScreen;
use crate::screens::history::HistoryScreen;
use crate::screens::settings::SettingsScreen;
use crate::screens::share::ShareScreen;
use crate::screens::welcome::receive::ReceiveScreen;
use crate::screens::welcome::select_action::TransferScreen;
use crate::screens::welcome::send::SendScreen;
use crate::{layout::desktop_layout::DesktopLayout, screens::home::HomeScreen};
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
                    <Route path=path!("/") view=TransferScreen />
                    <Route path=path!("/send") view=SendScreen />
                    <Route path=path!("/receive") view=ReceiveScreen />
                    <Route path=path!("/home") view=HomeScreen />
                    <Route path=path!("/about") view=AboutScreen />
                    <Route path=path!("/settings") view=SettingsScreen />
                    <Route path=path!("/share") view=ShareScreen />
                    <Route path=path!("/history") view=HistoryScreen />
                </Routes>
            </DesktopLayout>
        </Router>
    }
}
