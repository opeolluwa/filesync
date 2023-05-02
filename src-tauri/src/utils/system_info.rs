use std::fmt;

use serde::{Deserialize, Serialize};
use sys_info;

use crate::SERVER_PORT;

use super::compute_file_size;

#[derive(Debug, Default, Serialize, Deserialize)]
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

        let free_memory = match sys_info::mem_info() {
            Ok(memory_information) => memory_information.avail,
            Err(error_message) => {
                println!("error getting system memory due to {:?}", error_message);
                0 as u64
            }
        };

        let uptime = match uptime_lib::get() {
            Ok(uptime) => uptime.as_secs_f64(),
            Err(err) => {
                println!("couldn't get available uptime due to {:?}", err);
                0.0 as f64
            }
        };

        Self {
            system_name: system_name.into(),
            free_memory: compute_file_size(free_memory as u128),
            port: port.into(),
            uptime: format!(
                "{time_in_minutes:.2} minutes",
                time_in_minutes = (uptime / 60.0)
            ),
        }
    }
}

//impl display for system information typpe
impl fmt::Display for SystemInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "system_name: {},
            free_memory: {},
            port: {},
            uptime: {}",
            self.system_name, self.free_memory, self.port, self.uptime
        )
    }
}
