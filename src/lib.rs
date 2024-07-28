use thiserror::Error as ThisError;

pub mod message {
    use std::io::Read;

    pub fn digital_radar_data<R: Read>(_reader: &mut R) -> crate::Result<nexrad_model::data::Radial> {
        todo!()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {}
