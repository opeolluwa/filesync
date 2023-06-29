// use crate::utils::CommandData;
// use image::ImageEncoder::;
use qrcode::{EcLevel, QrCode, Version};
use qrcode_generator::QrCodeEcc;

fn generate_wifi_qr_code(ssid: &str, password: &str) -> QrCode {
    let wifi_data = format!("WIFI:S:{};T:WPA;P:{};;", ssid, password);
    QrCode::with_version(wifi_data.as_bytes(), Version::Normal(2), EcLevel::L).unwrap()
}



pub fn generate_connect_qr_code(ssid: &str, password: &str) /* -> CommandData<String>  */{
   
let result: Vec<Vec<bool>> = qrcode_generator::to_matrix("Hello world!", QrCodeEcc::Low).unwrap();

println!("{:?}", result);

  /*   let qr_code = generate_wifi_qr_code(ssid, password);
    let data_uri = qr_code_to_data_uri(&qr_code);

    CommandData::ok("Connection QR code successfully generated", data_uri); */
    // pr!("QR code data URI: {}", data_uri);
}
