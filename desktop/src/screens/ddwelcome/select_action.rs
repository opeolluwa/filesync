use leptos::{view, IntoView};

use crate::screens::welcome::default::DefaultScreen;

#[leptos::component]
pub fn TransferScreen() -> impl IntoView {
    view! { <DefaultScreen /> }
}
