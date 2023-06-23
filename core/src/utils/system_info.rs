use std::{fmt, net::Ipv4Addr};

use battery::units::time::*;
use battery::{Battery, Manager};
use num_traits::cast::ToPrimitive;
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
    pub available_disk: String,
    /// used store
    pub used_disk: String,
    /// the port on which the core server runs
    port: u128,
    /// the remaining_time e.g 2 hours     
    pub remaining_time: String,
    /// the system ip address
    pub ip_address: Ipv4Addr,
    /// the server base URL constructed form the ip address and port
    pub server_base_url: String,
    /// the battery state
    pub battery_is_charging: bool,
}

impl std::default::Default for SystemInformation {
    fn default() -> Self {
        Self {
            system_name: String::from(""),
            available_disk: String::from(""),
            used_disk: String::from(""),
            remaining_time: String::from(""),
            port: 0,
            ip_address: Ipv4Addr::from([0, 0, 0, 0]),
            server_base_url: String::from(""),
            battery_is_charging: false,
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

        // Get the used memory information
        let disk_info = Self::get_disk_info();

        let mut available_disk = 0;
        let mut used_disk = 0;
        match disk_info {
            Ok(info) => {
                available_disk = info.free;
                used_disk = info.total - info.free;
            }
            Err(_) => {
                println!("Failed to get the disk information");
            }
        };

        let charging_battery = Self::battery_is_discharging();
        let mut remaining_seconds = 0;
        let mut remaining_minutes = 0;
        let mut remaining_hours = 0;
        match Self::remaining_battery_time() {
            Some(mut seconds) => {
                remaining_hours = seconds / 3600;
                seconds %= 3600;
                remaining_minutes = seconds / 60;
                seconds %= 60;
                remaining_seconds = seconds;
            }
            None => (),
        }

        Self {
            system_name,
            available_disk: compute_file_size(available_disk as u128),
            used_disk: compute_file_size(used_disk as u128),
            port: port.into(),
            ip_address: ip_address.clone().unwrap(),
            server_base_url: format!("http://{}:{}", ip_address.unwrap(), port),
            remaining_time: format!(
                "{hour:02}:{minute:02}:{second:02}",
                hour = remaining_hours,
                minute = remaining_minutes,
                second = remaining_seconds
            ),
            battery_is_charging: charging_battery,
        }
    }

    fn get_disk_info() -> Result<sys_info::DiskInfo, sys_info::Error> {
        sys_info::disk_info()
    }
    fn get_battery_info() -> Result<Battery, battery::Error> {
        Manager::new()?.batteries()?.enumerate().nth(0).unwrap().1
    }
    fn battery_is_discharging() -> bool {
        match Self::get_battery_info() {
            Ok(battery) => {
                if battery.state() == battery::State::Discharging {
                    return false;
                } else {
                    return true;
                }
            }
            Err(e) => {
                println!("Failed to get the battery information.\n{:?}", e);
                return false;
            }
        }
    }
    fn remaining_battery_time() -> Option<u64> {
        match Self::get_battery_info() {
            Ok(battery) => return battery.time_to_empty()?.get::<second>().to_u64(),
            Err(e) => {
                println!("Failed to get the battery information.\n{:?}", e);
                return None;
            }
        }
    }
}

//impl display for system information type
impl fmt::Display for SystemInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "system_name: {},
            available_disk: {},
            used_disk: {},
            port: {},
            remaining_time: {}
            ip_address {:?}
            server_base_url {:?}
            ",
            self.system_name,
            self.available_disk,
            self.used_disk,
            self.port,
            self.remaining_time,
            self.ip_address,
            self.server_base_url
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    #[test]
    fn test_battery_discharging() {
        let target = (128, 1024 - 128);
        let mut result = MockSystemInformation::new();
        result.expect_get_disk_info().returning(sys_info::DiskInfo {
            total: 1024,
            free: 128,
        });
        assert_eq!((result.available_disk, result.used_disk), target);
    }

    #[test]
    fn test_battery_remaining_time() {
        let target = "00:00:12";
        let result = SystemInformation::new();
        assert_eq!(result.remaining_time, target);
    }
}
