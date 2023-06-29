use std::{fmt, net::Ipv4Addr};

use battery::units::time::*;
use battery::Manager;
use mockall::predicate::*;
use mockall::*;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use sys_info;

use super::compute_file_size;
use crate::net::ip_manager;
use crate::SERVER_PORT;

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
        let system_info = DefaultSystemInfoGetter;
        Self::new_with_sys_info_getter(system_info)
    }
    pub fn new_with_sys_info_getter<T: GetSystemInformation>(system_info: T) -> Self {
        let port = *SERVER_PORT;
        let system_name = match sys_info::hostname() {
            Ok(name) => name,
            Err(_) => String::from("guest computer"),
        };
        let ip_address = match ip_manager::autodetect_ip_address() {
            Ok(ip) => ip.parse::<Ipv4Addr>(),
            Err(_) => Ok(Ipv4Addr::from([0, 0, 0, 0])),
        };

        // Get the used memory information

        let disk_info = system_info.get_disk_info();

        let available_disk = disk_info.free;
        let used_disk = disk_info.total - disk_info.free;

        let remaining_time = match system_info.remaining_battery_time() {
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

pub struct DefaultSystemInfoGetter;
#[automock]
pub trait GetSystemInformation {
    fn get_disk_info(&self) -> sys_info::DiskInfo;
    fn remaining_battery_time(&self) -> Option<u64>;
}

impl GetSystemInformation for DefaultSystemInfoGetter {
    fn get_disk_info(&self) -> sys_info::DiskInfo {
        match sys_info::disk_info() {
            Ok(info) => info,
            Err(e) => {
                println!("Failed to get disk information: {:?}", e);
                sys_info::DiskInfo { total: 0, free: 0 }
            }
        }
    }

    fn remaining_battery_time(&self) -> Option<u64> {
        match Manager::new()
            .expect("Failed to get battery manager")
            .batteries()
            .expect("Failed to get batteries")
            .enumerate()
            .next()
            .unwrap()
            .1
        {
            Ok(battery) => battery.time_to_empty()?.get::<second>().to_u64(),
            Err(e) => {
                println!("Failed to get the battery information.\n{:?}", e);
                None
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

    fn set_disk_info(mock_item: &mut MockGetSystemInformation, total: u64, free: u64) {
        mock_item
            .expect_get_disk_info()
            .returning(move || sys_info::DiskInfo { total, free });
    }
    fn set_remaining_battery_time(mock: &mut MockGetSystemInformation, remaining: Option<u64>) {
        mock.expect_remaining_battery_time()
            .returning(move || remaining);
    }
    fn get_system_info(system_info: MockGetSystemInformation) -> SystemInformation {
        SystemInformation::new_with_sys_info_getter(system_info)
    }
    fn available_disk_should_be(target: String, result: String) {
        assert_eq!(target, result);
    }
    fn remaining_battery_time_should_be(target: Option<String>, result: Option<String>) {
        assert_eq!(target, result)
    }
    #[test]
    fn mock_disk_info_is_available() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 1024, 128);
        set_remaining_battery_time(&mut mock, None);
        let result = get_system_info(mock);
        available_disk_should_be(format!("{} B", 128), result.available_disk);
        available_disk_should_be(format!("{} B", 1024 - 128), result.used_disk);
    }
    #[test]
    fn mock_remaining_battery_time_only_seconds() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 0, 0);
        set_remaining_battery_time(&mut mock, Some(12));
        let result = get_system_info(mock);
        remaining_battery_time_should_be(Some(format!("00:00:12")), result.remaining_time);
    }
    #[test]
    fn mock_remaining_battery_time_only_minutes() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 0, 0);
        set_remaining_battery_time(&mut mock, Some(720));
        let result = get_system_info(mock);
        remaining_battery_time_should_be(Some(format!("00:12:00")), result.remaining_time);
    }
    #[test]
    fn mock_remaining_battery_time_only_hours() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 0, 0);
        set_remaining_battery_time(&mut mock, Some(12 * 60 * 60));
        let result = get_system_info(mock);
        remaining_battery_time_should_be(Some(format!("12:00:00")), result.remaining_time);
    }
    #[test]
    fn mock_remaining_battery_time() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 0, 0);
        set_remaining_battery_time(&mut mock, Some(12 * 60 * 60 + 12 * 60 + 12));
        let result = get_system_info(mock);
        remaining_battery_time_should_be(Some(format!("12:12:12")), result.remaining_time);
    }
}
