/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate pnet_datalink;
extern crate std;

/// Returns IP address of host, excluding localhost, or None if none found.
pub fn autodetect_ip_address() -> Result<String, ()> {
    pnet_datalink::interfaces()
        .into_iter()
        .filter(|iface| !iface.is_loopback() && iface.is_up())
        .flat_map(|iface| iface.ips)
        .find(|ip_network| ip_network.is_ipv4())
        .map(|ip_network| ip_network.ip().to_string())
        .ok_or(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autodetect_ip_address_no_loopback() {
        // When: retrieving a current IP address of the machine
        let ip = autodetect_ip_address().unwrap();

        // Then: it is not a loopback ("localhost") IP
        assert_ne!("127.0.0.1", ip);
        assert_ne!("::1", ip);
    }

    #[test]
    fn test_autodetect_ip_address_correct_format() {
        // When retrieving a current IP address of the machine
        let ip = autodetect_ip_address().unwrap();

        // Then: it is a IPv4 address in dotted notation without netmask
        let numbers_strs: Vec<&str> = ip.split('.').collect();
        assert_eq!(
            4,
            numbers_strs.len(),
            "Dotted IPv4 address notation should have 4 numbers"
        );
        for number_str in numbers_strs {
            let number_int = number_str
                .parse::<u8>()
                .expect("expected a number between 0 and 255");
            assert_eq!(
                number_int.to_string(),
                number_str,
                "expected no zero padding"
            );
        }
    }
}
