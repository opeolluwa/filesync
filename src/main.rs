mod app;
mod platform;
use app::*;

use leptos::*;
use mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App /> }
    })
}
