use embedded_server::config::EmbeddedServerConfig;
use tauri::{Manager, Runtime};

use crate::state::AppState;

#[tauri_interop::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri_interop::command]
pub async fn extract_connection<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<EmbeddedServerConfig, String> {
    let data = app.state::<AppState>();
    let AppState { server_config } = data.inner().clone();
    Ok(server_config)
}

tauri_interop::collect_commands!();
