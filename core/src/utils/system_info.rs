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
    pub available_memory: String,
    /// used store
    pub used_memory: String,
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
            available_memory: String::from(""),
            used_memory: String::from(""),
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

        /// Get the used memory information
        let mem_info = self.get_mem_info();

        /*
            Get the available memory information
            The available memory is the amount of memory that can be used by the process.
            It takes into account the amount of memory that can be made available by reclaiming various resources,
            such as page caches, that can be used by applications if necessary.
        */
        let available_memory = match mem_info {
            Ok(memory_information) => memory_information.avail,
            Err(error_message) => {
                println!("error getting system memory due to {:?}", error_message);
                0
            }
        };

        /*
            Get the used memory information
            The used memory is the amount of memory that is currently being used by the process.
            Substract the amount of free memory to get it.
        */
        let used_memory = match mem_info {
            Ok(memory_information) => memory_information.total - memory_information.free,
            Err(error_message) => {
                println!("error getting system memory due to {:?}", error_message);
                0
            }
        };

        /*
            Get the uptime information in seconds
        */
        let uptime = match self.get_uptime() {
            Ok(uptime) => uptime.as_secs_f64(),
            Err(err) => {
                println!("couldn't get available uptime due to {:?}", err);
                0.0
            }
        };

        Self {
            system_name,
            available_memory: compute_file_size(available_memory as u128),
            used_memory: compute_file_size(used_memory as u128),
            port: port.into(),
            ip_address: ip_address.clone().unwrap(),
            server_base_url: format!("http://{}:{}", ip_address.unwrap(), port),
            uptime: format!(
                "device uptime in {time_in_hours:u32} hour{multi_hour} {time_in_minutes:u32} minute{multi_minute}",
                time_in_hours = (uptime / 3600.0).floor() as u32,
                multi_hour = if time_in_hours > 1 { "s" } else { "" },
                time_in_minutes = ((uptime - time_in_hours * 3600.0) / 60.0).floor() as u32,
                multi_minute = if time_in_minutes > 1 { "s" } else { "" },
            ),
        }
    }
    fn get_uptime(&self) -> Result<Duration, String> {
        uptime_lib::get()
    }
    fn get_mem_info(&self) -> Result<sys_info::MemInfo, Error> {
        sys_info::mem_info()
    }
}

//impl display for system information type
impl fmt::Display for SystemInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "system_name: {},
            available_memory: {},
            used_memory: {},
            port: {},
            uptime: {}
            ip_address {:?}
            server_base_url {:?}
            ",
            self.system_name,
            self.available_memory,
            self.used_memory,
            self.port,
            self.uptime,
            self.ip_address,
            self.server_base_url
        )
    }
}
