use crate::utils::generator::{generate_passkey, generate_random_digits};
use tauri_bindgen::wifi_bindgen::WifiCredentials;

#[tauri::command]
pub fn generate_android_wifi_credentials() -> WifiCredentials {
    let ssid = generate_random_digits();
    let passkey = generate_passkey();

    WifiCredentials { ssid, passkey }
}
