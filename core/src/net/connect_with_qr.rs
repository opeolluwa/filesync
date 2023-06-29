use crate::utils::CommandData;
use image::png::PngEncoder;
use image::{DynamicImage, Rgba};
use qrcode::{EcLevel, QrCode, Version};
use std::io::Cursor;

fn generate_wifi_qr_code(ssid: &str, password: &str) -> QrCode {
    let wifi_data = format!("WIFI:S:{};T:WPA;P:{};;", ssid, password);
    QrCode::with_version(wifi_data.as_bytes(), Version::Auto, EcLevel::L).unwrap()
}

fn qr_code_to_data_uri(qr_code: &QrCode) -> String {
    let image = qr_code.render::<Rgba<u8>>().quiet_zone(true).build();
    let mut buffer = Vec::new();
    let encoder = PngEncoder::new(&mut buffer);
    encoder
        .encode(
            &image.into_bytes(),
            image.width(),
            image.height(),
            image.color(),
        )
        .unwrap();
    let data_uri = format!("data:image/png;base64,{}", base64::encode(&buffer));
    data_uri
}

pub fn generate_connect_qr_code(ssid: &str, password: &str) -> CommandData<String> {
    let ssid = "Your WiFi SSID";
    let password = "Your WiFi password";

    let qr_code = generate_wifi_qr_code(ssid, password);
    let data_uri = qr_code_to_data_uri(&qr_code);

    CommandData::ok("Connection QR code successfully generated", data_uri);
    // pr!("QR code data URI: {}", data_uri);
}
