use local_ip_address::local_ip;
use std::net::Ipv4Addr;

use crate::{
    network_manager::ip_manager,
    utils::{system_info::SystemInformation},
    SERVER_PORT,
};

use pkg::CommandData;

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
    let ip_address = ip_manager::autodetect_ip_address()
        .ok()
        .unwrap_or(String::from("0.0.0.0")) // use error catching in the frontend to validate only non"0.0.0.0. ip address
        .parse::<Ipv4Addr>()
        .unwrap();
    format!("{ip_address}:{port:?}", port = *SERVER_PORT)
}

// #[cfg(target_os = "android")]
// #[tauri::command]
// pub fn get_ip_address() -> String {
//     let ip_address = ip_manager::autodetect_ip_address()
//         .ok()
//         .unwrap_or(String::from("0.0.0.0")) // use error catching in the frontend to validate only non"0.0.0.0. ip address
//         .parse::<Ipv4Addr>()
//         .unwrap();
//     format!("{ip_address}:{port:?}", port = *SERVER_PORT)
// }
/// get the system information
/// tis include
/// - the uptime
/// the free memory
/// the port on which the cor server run
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
