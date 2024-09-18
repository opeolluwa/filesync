use local_ip_address::local_ip;
use std::net::Ipv4Addr;

use crate::config::CONFIG;
use crate::utils::system_info::SystemInformation;

use crate::pkg::CommandData;

// TODO: generate the qr code and return data URI
#[tauri::command]
pub fn generate_qr_code(ssid: &str, password: &str) -> String {
    format!("WIFI:S:{};T:WPA;P:{};;", ssid, password)
}

// give connection details of the application core server
// get the ip address of the machine
// for desktop
// #[cfg(not(target_os = "android"))]
#[tauri::command]
pub fn get_ip_address() -> String {
    let ip_address = crate::commands::network::autodetect_ip_address()
        .ok()
        .unwrap_or(Ipv4Addr::UNSPECIFIED.to_string());
    format!("{ip_address}:{port:?}", port = CONFIG.server_port)
}

#[tauri::command]
pub fn get_system_information() -> CommandData<SystemInformation> {
    CommandData::ok("connected system information ", SystemInformation::new())
}

#[tauri::command]
pub fn is_connected_to_wifi() -> CommandData<bool> {
    // the app would have a local ip address if it is connected to a network
    // else it would crash, this is leveraged to check the network status
    let has_ip_addr = local_ip().ok();
    if has_ip_addr.is_none() {
        return CommandData::ok("wifi status", false);
    }
    CommandData::ok("server address", true)
}
