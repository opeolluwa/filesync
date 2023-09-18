use std::net::Ipv4Addr;

use crate::{
    utils::{system_info::SystemInformation, CommandData},
    wifi::ip_manager,
    SERVER_PORT,
};

// TODO: generate the qr code and return data URI
#[tauri::command]
pub fn generate_qr_code(ssid: &str, password: &str) -> String {
    format!("WIFI:S:{};T:WPA;P:{};;", ssid, password)
}

// give connection details of the application core server
// get the ip address of the machine
#[tauri::command]
pub fn get_ip_address() -> String {
    let ip_address = ip_manager::autodetect_ip_address()
        .ok()
        .unwrap_or(String::from("0.0.0.0")) // use error catching in the frontend to validate only non"0.0.0.0. ip address
        .parse::<Ipv4Addr>()
        .unwrap();
    format!("{ip_address}:{port:?}", port = *SERVER_PORT)
}

/// get the system information
/// tis include
/// - the uptime
/// the free memory
/// the port on which the cor server run
#[tauri::command]
pub fn get_system_information() -> CommandData<SystemInformation> {
    CommandData::ok("connected system information ", SystemInformation::new())
}
