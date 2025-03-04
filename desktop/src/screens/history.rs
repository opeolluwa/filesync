use leptos::{view, IntoView};
use thaw::Text;

use crate::layouts::desktop_layout::DesktopLayout;

#[leptos::component]
pub fn HistoryScreen() -> impl IntoView {
    view! {
        <DesktopLayout>

            <Text class="font-medium leading-2 text-xl text-gray-700 dark:text-gray-400">
                Transfer history
            </Text>

        </DesktopLayout>
    }
}
