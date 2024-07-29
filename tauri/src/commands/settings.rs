use crate::pkg::CommandData;
use crate::{database::Settings, state::State};
/** this module is responsible for handling the user settings  and application informatin */

#[tauri::command(async)]
pub async fn get_settings() -> Result<CommandData<Settings>, CommandData<()>> {
    let settings = Settings::fetch().await;
    Ok(CommandData::ok("settings successfully fetched", settings))
}

#[tauri::command(async)]
pub async fn update_settings(settings: Settings) -> Result<CommandData<Settings>, CommandData<()>> {
    let settings = settings.save().await.unwrap();
    Ok(CommandData::ok("settings successfully saved", settings))
}

#[tauri::command(async)]
pub async fn get_application_data() -> Result<CommandData<State>, CommandData<()>> {
    let app_data = State::collect().await;
    Ok(CommandData::ok("settings successfully saved", app_data))
}
