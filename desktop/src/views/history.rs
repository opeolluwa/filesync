use leptos::view;
use thaw::Text;
#[leptos::component]
pub fn HistoryUI() -> impl leptos::IntoView {
    view! {
        <Text class="font-medium leading-2 text-xl text-gray-700 dark:text-gray-400">
            Transfer history
        </Text>
    }
}
