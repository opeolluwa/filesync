use crate::{database::Settings, utils::CommandData};

/** this module is responsible for handling the user settings */

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
