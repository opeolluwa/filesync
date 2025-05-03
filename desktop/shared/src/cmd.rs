#[cfg(feature = "config")]
use crate::config::EmbeddedServerConfig;

#[cfg(feature = "cmd")]
#[tauri_interop::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(feature = "config")]
#[tauri_interop::command]
pub async fn extract_connection() -> Result<EmbeddedServerConfig, String> {
    Ok(EmbeddedServerConfig::default())
}

tauri_interop::collect_commands!();
