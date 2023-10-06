use crate::utils::shell::execute_shell_command;

/**
 * see available network on linux OS
 * this is achieved using a nmcli
 * see https://askubuntu.com/questions/567006/how-can-i-display-the-list-of-available-wi-fi-networks
 */

pub fn scan_wifi() {
    let command = "nmcli dev wifi list";
    let available_networks = execute_shell_command(command);
    println!("{:#?}", available_networks);
}
