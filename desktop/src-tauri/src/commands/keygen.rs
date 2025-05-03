#[derive(Debug, Serialize, Deserialize)]
pub struct WifiCredentials {
    pub ssid: u16,
    pub passkey: String,
}
use serde::{Deserialize, Serialize};

use crate::utils::generator::{generate_passkey, generate_random_digits};

#[tauri::command]
pub fn generate_android_wifi_credentials() -> WifiCredentials {
    let ssid = generate_random_digits();
    let passkey = generate_passkey();

    WifiCredentials { ssid, passkey }
}
