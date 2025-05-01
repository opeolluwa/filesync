use leptos::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use thaw::ConfigProvider;

// use crate::screens::about::AboutScreen;
use crate::screens::history::HistoryScreen;
use crate::screens::settings::SettingsScreen;
use crate::screens::share::ShareScreen;
// use crate::screens::welcome::choose_platform::SelectPlatformScreen;
// use crate::screens::welcome::receive::ReceiveScreen;
// use crate::screens::welcome::select_action::TransferScreen;
// use crate::screens::welcome::send::SendScreen;
use crate::screens::welcome::WelcomeScreen;
use crate::{layouts::default_layout::DefaultLayout, screens::home::HomeScreen};

#[component]
pub fn App() -> impl leptos::IntoView {
    view! {
        <ConfigProvider class="dark:bg-[#1d232a]">
            <Router>
                <DefaultLayout>
                    <Routes transition=true fallback=|| "Not found.">
                        <Route path=path!("/") view=WelcomeScreen />
                        <Route path=path!("/home") view=HomeScreen />
                        // <Route path=path!("/platform") view=SelectPlatformScreen />
                        // <Route path=path!("/send") view=SendScreen />
                        // <Route path=path!("/receive") view=ReceiveScreen />
                        // <Route path=path!("/about") view=AboutScreen />
                        <Route path=path!("/settings") view=SettingsScreen />
                        <Route path=path!("/share") view=ShareScreen />
                        <Route path=path!("/history") view=HistoryScreen />
                    </Routes>

                </DefaultLayout>
            </Router>
        </ConfigProvider>
    }
}
