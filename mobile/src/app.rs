use leptos::*;
use leptos_router::{components::*, path};

use crate::mobile_application_layout::default_layout::DefaultLayout;
use crate::mobile_application_views::about::AboutUI;
use crate::mobile_application_views::history::HistoryUI;
use crate::mobile_application_views::home::HomeUI;
use crate::mobile_application_views::settings::SettingsUi;
use crate::mobile_application_views::share::ShareUI;

#[component]
pub fn MobileApplication() -> impl leptos::IntoView {
    view! {
        <Router>
            <DefaultLayout>
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/") view=HomeUI />
                    <Route path=path!("/about") view=AboutUI />
                    <Route path=path!("/settings") view=SettingsUi />
                    <Route path=path!("/share") view=ShareUI />
                    <Route path=path!("/about") view=HistoryUI />
                </Routes>
            </DefaultLayout>
        </Router>
    }
}
