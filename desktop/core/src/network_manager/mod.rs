#[allow(unused)]
use std::fmt;

use crate::utils::CommandData;
use serde::{Deserialize, Serialize};

pub mod ip_manager;

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(target_os = "macos", path = "mac.rs")]
#[cfg_attr(target_os = "windows", path = "windows.rs")]
pub mod network;

/// the Network config trait to be implemented independently fi=or windows, mac and linux
pub trait WifiHotspotConfigBuilder {
    /// create new hotspot
    fn create() -> Self;
    /// connect to a wifi
    fn connect() -> CommandData<bool>;
    /// scan for network, return an array of network names / ssid
    fn scan() -> Vec<String>;
    /// terminate the wifi hot spot
    fn stop();
    /// refresh  the network hotspot or wifi
    fn refresh();
}
// pub mod network_scanner;
/// the network interface type contains the
/// - gateway ex 192.168.0.0.1
/// - username ex sillicone
/// - password ex myPassword
/// - (optional) QR code to scan to connect
/// - (optional) the network card frequency, e.g 2.4GHz or 5Ghz
/// it will be used to communicate with the application interface
#[derive(Debug, Serialize, Deserialize)]
pub struct WifiHotspotConfig {
    /// the network broadcast ip address eg 192.16.8.0.1
    gateway: String,
    /// the network name (generated)
    ssid: String,
    /// the network password
    password: String,
    /// the status of the network
    status: NetworkAccessStatus,
    /// success or error message
    message: String,
}

/// network was successfully created or there is an error
#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkAccessStatus {
    Created,
    Error,
}

impl WifiHotspotConfig {}

impl fmt::Display for WifiHotspotConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
            gateway,
            ssid:{},
            password:{},
            status:{:?},
            message: {:?},
        ",
            self.ssid, self.password, self.status, self.message
        )
    }
}
