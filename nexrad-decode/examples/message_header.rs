//!
//! examples/message_header
//!
//! This example loads and decodes a message header from a file before printing its contents.
//!
//! Usage: cargo run --example message_header
//!

use nexrad_decode::messages::decode_message_header;

const MESSAGE_HEADER: &[u8] = include_bytes!("data/message_header");

fn main() {
    let mut reader = std::io::Cursor::new(MESSAGE_HEADER);

    let decoded_msg_header = decode_message_header(&mut reader).unwrap();
    println!("Decoded message header: {:?}", decoded_msg_header);
}
