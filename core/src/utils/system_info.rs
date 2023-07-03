use std::{fmt, net::Ipv4Addr};

use battery::units::time::*;
use battery::Manager;
use mockall::predicate::*;
use mockall::*;
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Command;
use sys_info;
use sysinfo::{DiskExt, System, SystemExt};

use super::compute_file_size;
use crate::net::ip_manager;
use crate::SERVER_PORT;

#[derive(Debug, Deserialize, Serialize)]
struct MyDisk {
    type_: String,
    device_name: String,
    file_system: Vec<u8>,
    mount_point: String,
    total_space: u64,
    available_space: u64,
    is_removable: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Drives {
    array_of_drives: Vec<MyDisk>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInformation {
    /// the current user name eg - drizzle
    pub system_name: String,
    /// the disks
    pub disk: Drives,
    /// available store
    pub available_disk: String,
    /// used space
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
            disk: Drives {
                array_of_drives: Vec::new(),
            },
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
        let disk = match system_info.get_disk_info() {
            Ok(drives) => drives,
            Err(_) => Drives {
                array_of_drives: vec![],
            },
        };

        let available_disk: String = compute_file_size(
            disk.array_of_drives
                .iter()
                .filter(|disk| disk.type_ == "HDD" || disk.type_ == "SSD")
                .map(|disk| disk.available_space)
                .sum::<u64>() as u128,
        );
        let used_disk: String = compute_file_size(
            disk.array_of_drives
                .iter()
                .filter(|disk| disk.type_ == "HDD" || disk.type_ == "SSD")
                .map(|disk| disk.total_space - disk.available_space)
                .sum::<u64>() as u128,
        );

        Self {
            system_name,
            disk,
            available_disk,
            used_disk,
            port: port.into(),
            ip_address: ip_address.clone().unwrap(),
            server_base_url: format!("http://{}:{}", ip_address.unwrap(), port),
            remaining_time,
        }
    }
}

pub struct DefaultSystemInfoGetter;
#[automock]
pub trait GetSystemInformation {
    fn get_disk_info(&self) -> Result<Drives, String>;
    fn remaining_battery_time(&self) -> Option<u64>;
}

impl GetSystemInformation for DefaultSystemInfoGetter {
    fn get_disk_info(&self) -> Result<Drives, String> {
        let mut array_of_drives = Vec::new();
        let mut system = System::new_all();
        let mut hashset: HashSet<String> = HashSet::new();
        system.refresh_all();
        for disk in system.disks() {
            if hashset.insert(disk.name().to_string_lossy().to_string()) {
                array_of_drives.push(MyDisk {
                    type_: match disk.kind() {
                        sysinfo::DiskKind::HDD => format!("HDD"),
                        sysinfo::DiskKind::SSD => format!("SSD"),
                        _ => format!("Removeable disk"),
                    },
                    device_name: disk.name().to_string_lossy().to_string(),
                    file_system: disk.file_system().to_vec(),
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    total_space: disk.total_space(),
                    available_space: disk.available_space(),
                    is_removable: disk.is_removable(),
                });
            }
        }
        Ok(Drives { array_of_drives })
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "system_name: {},
            disk: {:?},
            available space: {},
            used space: {},
            port: {},
            remaining_time: {}
            ip_address {:?}
            server_base_url {:?}
            ",
            self.system_name,
            self.disk,
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
        mock_item.expect_get_disk_info().returning(move || {
            Ok(Drives {
                array_of_drives: vec![MyDisk {
                    type_: "HDD".to_string(),
                    device_name: "".to_string(),
                    file_system: Vec::<u8>::new(),
                    mount_point: "".to_string(),
                    total_space: total,
                    available_space: free,
                    is_removable: false,
                }],
            })
        });
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
    fn mock_disk_info_is_used() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 1024, 128);
        set_remaining_battery_time(&mut mock, None);
        let result = get_system_info(mock);
        available_disk_should_be(format!("{} B", 1024 - 128), result.used_disk);
    }
    #[test]
    fn mock_disk_info_is_available() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 1024, 128);
        set_remaining_battery_time(&mut mock, None);
        let result = get_system_info(mock);
        available_disk_should_be(format!("{} B", 128), result.available_disk);
    }
    #[test]
    fn mock_remaining_battery_time_only_seconds() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 0, 0);
        set_remaining_battery_time(&mut mock, Some(12));
        let result = get_system_info(mock);
        remaining_battery_time_should_be(Some("00:00:12".to_string()), result.remaining_time);
    }
    #[test]
    fn mock_remaining_battery_time_only_minutes() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 0, 0);
        set_remaining_battery_time(&mut mock, Some(720));
        let result = get_system_info(mock);
        remaining_battery_time_should_be(Some("00:12:00".to_string()), result.remaining_time);
    }
    #[test]
    fn mock_remaining_battery_time_only_hours() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 0, 0);
        set_remaining_battery_time(&mut mock, Some(12 * 60 * 60));
        let result = get_system_info(mock);
        remaining_battery_time_should_be(Some("12:00:00".to_string()), result.remaining_time);
    }
    #[test]
    fn mock_remaining_battery_time() {
        let mut mock = MockGetSystemInformation::new();
        set_disk_info(&mut mock, 0, 0);
        set_remaining_battery_time(&mut mock, Some(12 * 60 * 60 + 12 * 60 + 12));
        let result = get_system_info(mock);
        remaining_battery_time_should_be(Some("12:12:12".to_string()), result.remaining_time);
    }
}
