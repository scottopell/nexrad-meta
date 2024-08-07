pub mod clutter_filter_map;
pub mod digital_radar_data;
pub mod message_header;
pub mod rda_status_data;

mod definitions;
mod primitive_aliases;

mod message_type;
pub use message_type::MessageType;

mod message;
pub use message::{Message, MessageWithHeader};

use crate::messages::message_header::MessageHeader;
use crate::result::Result;
use crate::util::deserialize;
use std::io::Read;

/// Decodes a message header from the provided reader.
pub fn decode_message_header<R: Read>(reader: &mut R) -> Result<MessageHeader> {
    deserialize(reader)
}
