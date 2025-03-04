use serde::{Deserialize, Serialize};

use crate::utils::generator::{generate_passkey, generate_random_digits};

#[derive(Serialize, Deserialize, Debug)]
pub struct WifiCredentials {
    ssid: u16,
    passkey: String,
}
#[tauri::command]
pub fn generate_android_wifi_credentials() -> WifiCredentials {
    let ssid = generate_random_digits();
    let passkey = generate_passkey();

    WifiCredentials { ssid, passkey }
}
