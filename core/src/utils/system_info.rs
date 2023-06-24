use std::{fmt, net::Ipv4Addr};

use battery::units::time::*;
use battery::Manager;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use sys_info;

use crate::SERVER_PORT;

use super::{compute_file_size, ip_manager::autodetect_ip_address};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MockSystemInformation {
    /// the current user name eg - drizzle
    pub system_name: String,
    /// available store
    pub available_disk: String,
    /// used store
    pub used_disk: String,
    /// the port on which the core server runs
    port: u128,
    /// the battery remaining time
    /// could be none if the battery is charging
    pub remaining_time: Option<String>,
    /// the system ip address
    pub ip_address: Ipv4Addr,
    /// the server base URL constructed form the ip address and port
    pub server_base_url: String,
}

impl std::default::Default for MockSystemInformation {
    fn default() -> Self {
        Self {
            system_name: String::from(""),
            available_disk: String::from(""),
            used_disk: String::from(""),
            remaining_time: None,
            port: 0,
            ip_address: Ipv4Addr::from([0, 0, 0, 0]),
            server_base_url: String::from(""),
        }
    }
}

/// system information construct
/// accepts the system name name and returns an instance of the struct with the remaining values constructed internally

impl MockSystemInformation {
    pub fn new(disk_total: Option<u64>, disk_free: Option<u64>, r_time: Option<u64>) -> Self {
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
        let mut disk_info: Result<sys_info::DiskInfo, sys_info::Error> = match disk_total {
            Some(total) => match disk_free {
                Some(free) => Ok(sys_info::DiskInfo { total, free }),
                None => Err(sys_info::Error::UnsupportedSystem),
            },
            None => Err(sys_info::Error::UnsupportedSystem),
        };

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

        let remaining_time = match r_time {
            Some(mut seconds) => {
                let remaining_hours = seconds / 3600;
                seconds %= 3600;
                let remaining_minutes = seconds / 60;
                seconds %= 60;
                let remaining_seconds = seconds;
                Some(format!(
                    "{:02}:{:02}:{:02}",
                    remaining_hours, remaining_minutes, remaining_seconds
                ))
            }
            None => None,
        };

        Self {
            system_name,
            available_disk: compute_file_size(available_disk as u128),
            used_disk: compute_file_size(used_disk as u128),
            port: port.into(),
            ip_address: ip_address.clone().unwrap(),
            server_base_url: format!("http://{}:{}", ip_address.unwrap(), port),
            remaining_time: remaining_time,
        }
    }
}

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
    /// the battery remaining time
    /// could be none if the battery is charging
    pub remaining_time: Option<String>,
    /// the system ip address
    pub ip_address: Ipv4Addr,
    /// the server base URL constructed form the ip address and port
    pub server_base_url: String,
}

impl std::default::Default for SystemInformation {
    fn default() -> Self {
        Self {
            system_name: String::from(""),
            available_disk: String::from(""),
            used_disk: String::from(""),
            remaining_time: None,
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

        let remaining_time = match Self::remaining_battery_time() {
            Some(mut seconds) => {
                let remaining_hours = seconds / 3600;
                seconds %= 3600;
                let remaining_minutes = seconds / 60;
                seconds %= 60;
                let remaining_seconds = seconds;
                Some(format!(
                    "{:02}:{:02}:{:02}",
                    remaining_hours, remaining_minutes, remaining_seconds
                ))
            }
            None => None,
        };

        Self {
            system_name,
            available_disk: compute_file_size(available_disk as u128),
            used_disk: compute_file_size(used_disk as u128),
            port: port.into(),
            ip_address: ip_address.clone().unwrap(),
            server_base_url: format!("http://{}:{}", ip_address.unwrap(), port),
            remaining_time: remaining_time,
        }
    }
    fn get_disk_info() -> Result<sys_info::DiskInfo, sys_info::Error> {
        sys_info::disk_info()
    }
    fn remaining_battery_time() -> Option<u64> {
        match Manager::new()
            .expect("Failed to get battery manager")
            .batteries()
            .expect("Failed to get batteries")
            .enumerate()
            .nth(0)
            .unwrap()
            .1
        {
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
            match &self.remaining_time {
                Some(time) => time,
                None => "None",
            },
            self.ip_address,
            self.server_base_url
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn disk_info_should_be(available_disk: u64, used_disk: u64) -> (String, String) {
        (format!("{} B", available_disk), format!("{} B", used_disk))
    }
    #[test]
    fn test_disk_info_available() {
        let target = disk_info_should_be(128, 1024 - 128);
        let result = MockSystemInformation::new(Some(1024), Some(128), None);
        assert_eq!((result.available_disk, result.used_disk), target);
    }
    #[test]
    fn test_disk_info_unavailable() {
        let target = disk_info_should_be(0, 0);
        let result = MockSystemInformation::new(None, None, None);
        assert_eq!((result.available_disk, result.used_disk), target);
    }
    #[test]
    fn test_disk_info_total_unavailable() {
        let target = disk_info_should_be(0, 0);
        let result = MockSystemInformation::new(None, Some(128), None);
        assert_eq!((result.available_disk, result.used_disk), target);
    }
    #[test]
    fn test_disk_info_free_unavailable() {
        let target = disk_info_should_be(0, 0);
        let result = MockSystemInformation::new(Some(128), None, None);
        assert_eq!((result.available_disk, result.used_disk), target);
    }
    #[test]
    fn test_battery_remaining_time_seconds() {
        let target: Option<String> = Some(format!("00:00:12"));
        let result = MockSystemInformation::new(None, None, Some(12));
        assert_eq!(result.remaining_time, target);
    }
    #[test]
    fn test_battery_remaining_time_minures() {
        let target: Option<String> = Some(format!("00:12:00"));
        let result = MockSystemInformation::new(None, None, Some(720));
        assert_eq!(result.remaining_time, target);
    }
    #[test]
    fn test_battery_remaining_time_hours() {
        let target: Option<String> = Some(format!("12:00:00"));
        let result = MockSystemInformation::new(None, None, Some(43200));
        assert_eq!(result.remaining_time, target);
    }
    #[test]
    fn test_battery_remaining_time_hours_minutes_and_secons() {
        let target: Option<String> = Some(format!("01:01:01"));
        let result = MockSystemInformation::new(None, None, Some(3661));
        assert_eq!(result.remaining_time, target);
    }
    #[test]
    fn test_battery_remaining_time_exceed_99_hours() {
        let target: Option<String> = Some(format!("100:00:00"));
        let result = MockSystemInformation::new(None, None, Some(360000));
        assert_eq!(result.remaining_time, target);
    }
    #[test]
    fn test_battery_is_charging() {
        let target: Option<String> = None;
        let result = MockSystemInformation::new(None, None, None);
        assert_eq!(result.remaining_time, target);
    }
}
