//!
//! Decoder and struct definitions for NEXRAD Level II data structures as defined by NOAA's WSR-88D
//! Interface Control Document for Archive II.
//!
//! These structure definitions should match the ICD 2620010H as of build 19.0.
//!

mod archive2_header;
pub use archive2_header::Archive2Header;

mod archive_2_file;
pub use archive_2_file::Archive2File;

pub mod messages;

mod util;

use crate::raw::util::deserialize;
use crate::result::Result;
use std::io::Read;

/// Decodes an Archive II header from the provided reader.
pub fn decode_archive2_header<R: Read>(reader: &mut R) -> Result<Archive2Header> {
    deserialize(reader)
}
