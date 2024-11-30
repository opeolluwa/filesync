use tauri::{Config, Runtime};

#[tauri::command]
pub async fn get_app_config<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<Config, String> {
    let app_config = app.config();

    Ok(app_config.to_owned())
}
