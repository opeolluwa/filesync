use std::fmt;

use crate::utils::system_info::SystemInformation;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

pub mod create_ap;
pub mod ip_manager;
pub mod linux_hotspot;
pub mod mac_hotspot;
pub mod wifi_scanner;
pub mod windows_hotspot;
pub mod connect_with_qrx;
/// the network interface type contains the
/// - gateway ex 192.168.0.0.1
/// - username ex sillicone
/// - password ex myPassword
/// - (optional) QR code to scan to connect
/// - (optional) the network card frequency, e.g 2.4GHz or 5Ghz
/// it will be used to communicate with the application interface
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccessPointInterface {
    /// the network broadcast ip address
    gateway: String,
    /// the network name (generates)
    ssid: String,
    /// the network password
    password: String,
    /// the status of the network
    status: Option<NetworkAccessStatus>,
    /// message
    message: Option<String>,
}

/// network was successfully creatd or there is an error
#[derive(Debug, Serialize, Deserialize,)]
pub enum NetworkAccessStatus {
    Created,
    Error,
}

impl AccessPointInterface {
    pub fn new(gateway: &str) -> Self {
        let SystemInformation {
            system_name: ssid, ..
        } = SystemInformation::new(); // use the system name
        let password = nanoid!(8); // generate username and password
        let gateway = gateway.to_string();

        Self {
            gateway,
            ssid,
            password,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn ok(gateway: &str) -> Self {
        let SystemInformation {
            system_name: ssid, ..
        } = SystemInformation::new(); // use the system name
        let password = nanoid!(8); // generate username and password
        let gateway = gateway.to_string();
        let message = Some(String::from("Wifi hotspot created successfully"));

        Self {
            gateway,
            ssid,
            password,
            status: Some(NetworkAccessStatus::Created),
            message,
        }
    }

    pub fn err() -> Self {
        let message = Some(String::from("Failed to create Wifi hotspot"));

        Self {
            status: Some(NetworkAccessStatus::Created),
            message,
            ..Default::default()
        }
    }
}

// impl std::default::Default for AccessPointInterface {
//     fn default() -> Self {
//         Self {
//             gateway: String::from("0.0.0.0"),
//             ..Default::default()
//         }
//     }
// }

impl fmt::Display for AccessPointInterface {
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
