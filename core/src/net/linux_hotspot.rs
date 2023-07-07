use super::AccessPointInterface;
use crate::net::NetworkAccessStatus;
use std::process::{Command, Output};
use std::str;

/// create hotspot on linux
pub fn create_hotspot() -> Result<AccessPointInterface, AccessPointInterface> {
    // get the network gate way ex DNS Configuration: Some(["192.168.100.121"])
    let output = Command::new("nmcli")
        .args(["dev", "show"])
        .output()
        .expect("Failed to execute nmcli");

    let Some(network_gateway) = parse_dns_config(&output) else {
        return Err(AccessPointInterface{
            status: Some(NetworkAccessStatus::Error),
            message: Some(String::from("Failed to create Wifi hotspot")),
            ..Default::default()
        });
    };
    println!("network gateway {:#?}", network_gateway);

    // create new access point config
    let access_point = AccessPointInterface::new(&network_gateway[0]);
    // destructure the ssid, password, and gateway
    let AccessPointInterface {
        ssid,
        password,
        gateway,
        ..
    } = access_point;

    // get the network interface e.g wlan0, wlo1 ...
    let Some(network_interfaces) = get_network_interface().ok() else {
            return Err(AccessPointInterface {
                status: Some(NetworkAccessStatus::Error),
                message: Some(String::from("Failed to create Wifi hotspot")),
                ..Default::default()
            });
        };
    println!("network interface {:#?}", network_interfaces);
    // iterate through available interfaces and create a network with whichever is available
    if let Some(network_interface) = network_interfaces.into_iter().next() {
        // Execute 'nmcli' commands to create a hotspot
        let create_wifi_hotspot_cli = Command::new("nmcli")
            .arg("dev")
            .arg("wifi")
            .arg("hotspot")
            .arg("ifname")
            .arg(&network_interface) // Replace 'wlan0' with the appropriate network interface name
            .arg("con-name")
            .arg("send-file-access-point") // Replace 'my-hotspot' with the desired connection name
            .arg("ssid")
            .arg(&ssid) //the desired SSID name
            .arg("password")
            .arg(&password) // Replace 'MyPassword' with the desired password
            .output()
            .expect("Failed to execute 'nmcli' command.");

        // Check if the command was successful
        if create_wifi_hotspot_cli.status.success() {
            return Ok(AccessPointInterface {
                ssid,
                password,
                gateway,
                status: Some(NetworkAccessStatus::Created),
                message: Some(String::from("Wifi hotspot created successfully")),
            });
            // break;
        } else {
            let error_msg = String::from_utf8_lossy(&create_wifi_hotspot_cli.stderr);
            return Err(AccessPointInterface {
                status: Some(NetworkAccessStatus::Error),
                message: Some(format!("Failed to create hotspot: {}", error_msg)),
                ..Default::default()
            });
        }
    }
    Err(AccessPointInterface {
        status: Some(NetworkAccessStatus::Error),
        message: Some(String::from("Failed to create Wifi hotspot")),
        ..Default::default()
    })

    /*  // get the network interface e.g wlan0, wlo1 ...
        let Some(network_interfaces) = get_network_interface().ok() else {
            return Err(AccessPointInterface {
                status: Some(NetworkAccessStatus::Error),
                message: Some(String::from("Failed to create Wifi hotspot")),
                ..Default::default()
            });
        };
        println!("network interface {:?}", network_interfaces);
        // get the network gate way ex DNS Configuration: Some(["192.168.100.121"])
        let output = Command::new("nmcli")
            .args(["dev", "show"])
            .output()
            .expect("Failed to execute nmcli");

        let Some(network_gateway) = parse_dns_config(&output) else {
        return Err(AccessPointInterface{
            status: Some(NetworkAccessStatus::Error),
            message: Some(String::from("Failed to create Wifi hotspot")),
            ..Default::default()
        });
    };

        println!("network gateway {:?}", network_gateway);

        // create new access point config
        let access_point = AccessPointInterface::new(&network_gateway[0]);
        // destructure the ssid, password and gateway
        let AccessPointInterface {
            ssid,
            password,
            gateway,
            ..
        } = access_point;

        //iterate through available interfaces and create network with whichever is available
        for network_interface in network_interfaces {
            // Execute 'nmcli' commands to create a hotspot
            let  = Command::new("nmcli")
                .arg("dev")
                .arg("wifi")
                .arg("hotspot")
                .arg("ifname")
                .arg(&network_interface) // Replace 'wlan0' with the appropriate network interface name
                // .arg("con-name")
                // .arg("my-hotspot") // Replace 'my-hotspot' with the desired connection name
                .arg("ssid")
                .arg(&ssid) //the desired SSID name
                .arg("password")
                .arg(&password) // Replace 'MyPassword' with the desired password
                .output()
                .expect("Failed to execute 'nmcli' command.");

            // Check if the command was successful
            if .status.success() {
                return Ok(AccessPointInterface {
                    ssid,
                    password,
                    gateway,
                    status: Some(NetworkAccessStatus::Created),
                    message: Some(String::from("Wifi hotspot created successfully")),
                })
                // break;
            } else {
                let error_msg = String::from_utf8_lossy(&.stderr);
                return Err(AccessPointInterface {
                    status: Some(NetworkAccessStatus::Error),
                    message: Some(format!("Failed to create hotspot: {}", error_msg)),
                    ..Default::default()
                });
            }
        }
        Ok(()); */
}

/// get the network gate way DNS Configuration: Some(["192.168.100.121"])
fn parse_dns_config(output: &Output) -> Option<Vec<String>> {
    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
    let mut dns_config = Vec::new();

    for line in stdout.lines() {
        // println!("{:?}",line);
        if line.contains("IP4.DNS") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(dns_value) = parts.get(1) {
                dns_config.push(dns_value.to_string());
            }
        }
    }

    if dns_config.is_empty() {
        None
    } else {
        Some(dns_config) // DNS Configuration: Some(["192.168.100.121"])
    }
}

/// get the network interface
fn get_network_interface() -> std::io::Result<Vec<String>> {
    let output = Command::new("nmcli").args(["device", "show"]).output()?;

    let mut network_interfaces: Vec<String> = Vec::new();
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            if line.contains("GENERAL.DEVICE")
            /* && line.contains("TYPE") */
            {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if let Some(interface) = parts.get(1) {
                    // return Ok(interface.to_string());
                    network_interfaces.push(interface.to_string());
                }
            }
        }
        return Ok(network_interfaces); // [wlo1, wlan0, docker0 ...]
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing nmcli: {}", stderr);
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to get network interface",
    ))
}

pub fn turn_off_hotspot() {
    Command::new("nmcli")
        .args(["r", "wifi", "off"])
        .output()
        .expect("Failed to execute nmcli");
}
