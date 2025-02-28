use leptos::{view, IntoView};
use thaw::Text;
#[leptos::component]
pub fn HistoryScreen() -> impl IntoView {
    view! {
        <Text class="font-medium leading-2 text-xl text-gray-700 dark:text-gray-400">
            Transfer history
        </Text>
    }
}
