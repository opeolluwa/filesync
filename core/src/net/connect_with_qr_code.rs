
// TODO: generate the qr code and return data URI
#[tauri::command]
pub fn generate_qr_code(ssid: &str, password: &str) -> String {
    format!("WIFI:S:{};T:WPA;P:{};;", ssid, password)
}
