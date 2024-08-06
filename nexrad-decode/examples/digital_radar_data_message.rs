//!
//! examples/digital_radar_data_message
//!
//! This example loads and decodes a digital radar data message (type 31) from a file before
//! printing its contents.
//!
//! Usage: cargo run --example digital_radar_data_message
//!

use nexrad_decode::messages::digital_radar_data::decode_digital_radar_data;

const DIGITAL_RADAR_DATA_MESSAGE: &[u8] = include_bytes!("data/digital_radar_data_message");

fn main() {
    let mut reader = std::io::Cursor::new(DIGITAL_RADAR_DATA_MESSAGE);

    let message = decode_digital_radar_data(&mut reader).unwrap();
    println!("Decoded digital radar data message: {:?}", message);

    #[cfg(feature = "nexrad-model")]
    println!("Decoded message radial model: {:?}", message.radial());
}
