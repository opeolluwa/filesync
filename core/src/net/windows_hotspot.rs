use std::process::Command;
#[allow(dead_code)]
pub fn create_ap() {
    // Set up the hotspot name and password
    let hotspot_name = "MyHotspot";
    let hotspot_password = "MyPassword123";

    // Execute the netsh commands to create the hotspot
    let output = Command::new("netsh")
        .args([
            "wlan",
            "set",
            "hostednetwork",
            "mode=allow",
            &("ssid=".to_owned() + hotspot_name),
            &("key=".to_owned() + hotspot_password),
        ])
        .output()
        .expect("Failed to execute 'netsh' command.");

    // Check the command execution result
    if output.status.success() {
        println!("Hotspot created successfully.");
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("Failed to create hotspot: {}", error_message);
    }
}
