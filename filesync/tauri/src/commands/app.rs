use tauri::{Config, Runtime};

#[tauri::command]
pub async fn get_app_config<R: Runtime>(app: tauri::AppHandle<R>) -> Config {
    let app_config = app.config();
    app_config.to_owned()
}
