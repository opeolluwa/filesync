use std::{fmt, net::Ipv4Addr};

use serde::{Deserialize, Serialize};
use sys_info;

use crate::SERVER_PORT;

use super::{compute_file_size, ip_manager::autodetect_ip_address};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInformation {
    /// the current user name eg - drizzle
    pub system_name: String,
    /// available store
    pub free_memory: String,
    /// the port on which the core server runs
    port: u128,
    /// the uptime e.g 2 hours     
    pub uptime: String,
    /// the system ip address
    pub ip_address: Ipv4Addr,
    /// the server base URL constructed form the ip address and port
    pub server_base_url: String,
}

impl std::default::Default for SystemInformation {
    fn default() -> Self {
        Self {
            system_name: String::from(""),
            free_memory: String::from(""),
            uptime: String::from(""),
            port: 0,
            ip_address: Ipv4Addr::from([0, 0, 0, 0]),
            server_base_url: String::from(""),
        }
    }
}

/// system information construct
/// accepts the system name name and returns an instance of the struct with the remaining values constructed internally

impl SystemInformation {
    pub fn new() -> Self {
        let port = *SERVER_PORT;
        let system_name = match sys_info::hostname() {
            Ok(name) => name,
            Err(_) => String::from("guest computer"),
        };
        let ip_address = match autodetect_ip_address() {
            Ok(ip) => ip.parse::<Ipv4Addr>(),
            Err(_) => Ok(Ipv4Addr::from([0, 0, 0, 0])),
        };

        let free_memory = match sys_info::mem_info() {
            Ok(memory_information) => memory_information.avail,
            Err(error_message) => {
                println!("error getting system memory due to {:?}", error_message);
                0
            }
        };

        let uptime = match uptime_lib::get() {
            Ok(uptime) => uptime.as_secs_f64(),
            Err(err) => {
                println!("couldn't get available uptime due to {:?}", err);
                0.0
            }
        };

        Self {
            system_name,
            free_memory: compute_file_size(free_memory as u128),
            port: port.into(),
            ip_address: ip_address.clone().unwrap(),
            server_base_url: format!("http://{}:{}", ip_address.unwrap(), port),
            uptime: format!(
                "{time_in_minutes:.2} minutes",
                time_in_minutes = (uptime / 600.0)
            ),
        }
    }
}

//impl display for system information type
impl fmt::Display for SystemInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "system_name: {},
            free_memory: {},
            port: {},
            uptime: {}
            ip_address {:?}
            server_base_url {:?}
            ",
            self.system_name,
            self.free_memory,
            self.port,
            self.uptime,
            self.ip_address,
            self.server_base_url
        )
    }
}
