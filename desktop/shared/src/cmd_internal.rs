use embedded_server::config::EmbeddedServerConfig;
#[tauri_interop::command]
pub async fn extract_connection<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<EmbeddedServerConfig, String> {
    let data = app.state::<AppState>();
    let AppState { server_config } = data.inner().clone();
    Ok(server_config)
}