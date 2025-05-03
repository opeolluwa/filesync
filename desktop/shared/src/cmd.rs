
use crate::state::AppState;
use crate::config::EmbeddedServerConfig;
use tauri_interop::host_usage;

host_usage! {
    use tauri::{Manager, Runtime};
}

#[tauri_interop::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri_interop::command]
// pub async fn extract_connection(
//    state: TauriState<AppState>
// ) -> Result<EmbeddedServerConfig, String> {
//     let data = state::<AppState>();
//     let AppState { server_config } = data.inner().clone();
//     Ok(server_config)
// }

#[tauri_interop::command]
pub async fn extract_connection<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<EmbeddedServerConfig, String> {
    let data = app.state::<AppState>();
    let AppState { server_config } = data.inner().clone();
    Ok(server_config)
}

tauri_interop::collect_commands!();
