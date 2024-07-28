//!
//! examples/raw
//!
//! This example loads and decodes an archive header, message header, and digital radar data file
//! into the raw data structures defined in the `nexrad-decode::raw` module.
//!
//! Usage: cargo run --example raw
//!

use nexrad_decode::raw::decode_archive2_header;
use nexrad_decode::raw::messages::decode_message_header;
use nexrad_decode::raw::messages::digital_radar_data::decode_digital_radar_data;
use std::fs;

fn main() {
    decode_archive2_header_example();
    decode_message_header_example();
    decode_digital_radar_data_message_example();
}

fn decode_archive2_header_example() {
    let file = fs::read("examples/data/archive2_header").expect("file exists");
    let mut reader = std::io::Cursor::new(file.as_slice());
    let archive2_header = decode_archive2_header(&mut reader).unwrap();
    println!("Decoded Archive2 header: {:?}", archive2_header);
}

fn decode_message_header_example() {
    let file = fs::read("examples/data/message_header").expect("file exists");
    let mut reader = std::io::Cursor::new(file.as_slice());
    let message_header = decode_message_header(&mut reader).unwrap();
    println!("Decoded message header: {:?}", message_header);
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
