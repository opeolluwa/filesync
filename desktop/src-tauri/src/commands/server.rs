use embedded_server::config::EmbeddedServerConfig;
use tauri::{Manager, Runtime};

use shared::state::AppState;

#[tauri::command]
pub async fn extract_connection<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<EmbeddedServerConfig, String> {
    let data = app.state::<AppState>();
    let AppState { server_config } = data.inner().clone();
    Ok(server_config)
}
