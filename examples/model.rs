//!
//! examples/model
//!
//! This example loads and decodes an archive header, message header, and digital radar data file
//! into the model data structures defined in `nexrad-model`.
//!
//! Usage: cargo run --example model
//!

#![cfg(all(feature = "nexrad-model"))]

use nexrad_decode::model::decode_digital_radar_data;
use std::fs;

fn main() {
    decode_digital_radar_data_message_example();
}

fn decode_digital_radar_data_message_example() {
    let file = fs::read("examples/data/digital_radar_data_message").expect("file exists");
    let mut reader = std::io::Cursor::new(file.as_slice());
    let digital_radar_data = decode_digital_radar_data(&mut reader).unwrap();
    println!(
        "Decoded digital radar data message: {:?}",
        digital_radar_data
    );
}
