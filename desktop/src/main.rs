mod app;

mod components;
mod layouts;
mod routes;
mod screens;

use app::*;

use leptos::*;
use mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
