use crate::net::NetworkAccessStatus;

// create hotspot on the linux operating system using nmcli
// @copyright Adeoye adefemi 2023
use super::AccessPointInterface;
use std::process::{Command, Output};
use std::str;

/// create hotspot on linux
pub fn create_ap() -> Result<AccessPointInterface, AccessPointInterface> {
    // get the network interface e.g wlan0, wlo1 ...
    let Some(network_interface) = get_network_interface().ok() else {
        return Err(AccessPointInterface {
            status: Some(NetworkAccessStatus::Error),
            message: Some(String::from("Failed to create Wifi hotspot")),
            ..Default::default()
        });
    };

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

    // create new access point config
    let access_point = AccessPointInterface::new(&network_gateway[0]);
    // destructure the ssid, password and gateway
    let AccessPointInterface {
        ssid,
        password,
        gateway,
        ..
    } = access_point;

    // Execute 'nmcli' commands to create a hotspot
    let create_ap = Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("hotspot")
        .arg("ifname")
        .arg(&network_interface) // Replace 'wlan0' with the appropriate network interface name
        .arg("con-name")
        .arg("my-hotspot") // Replace 'my-hotspot' with the desired connection name
        .arg("ssid")
        .arg(&ssid) //the desired SSID name
        .arg("password")
        .arg(&password) // Replace 'MyPassword' with the desired password
        .output()
        .expect("Failed to execute 'nmcli' command.");

    // Check if the command was successful
    if create_ap.status.success() {
        Ok(AccessPointInterface {
            ssid,
            password,
            gateway,
            status: Some(NetworkAccessStatus::Created),
            message: Some(String::from("Wifi hotspot created successfully")),
        })
    } else {
        let error_msg = String::from_utf8_lossy(&create_ap.stderr);
        Err(AccessPointInterface {
            status: Some(NetworkAccessStatus::Error),
            message: Some(format!("Failed to create hotspot: {}", error_msg)),
            ..Default::default()
        })
    }
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
fn get_network_interface() -> std::io::Result<String> {
    let output = Command::new("nmcli").args(["device", "show"]).output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            if line.contains("GENERAL.DEVICE")
            /* && line.contains("TYPE") */
            {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if let Some(interface) = parts.get(1) {
                    return Ok(interface.to_string());
                }
            }
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing nmcli: {}", stderr);
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to get network interface",
    ))
}
