use crate::raw::messages::digital_radar_data::GenericDataBlock;
use crate::result::Result;
use nexrad_model::data::{MomentData, Radial, RadialStatus};
use std::io::{Read, Seek};

const MILLIS_PER_DAY: i64 = 86_400_000;

/// Decodes a digital radar data message type 31 from the provided reader.
pub fn decode_digital_radar_data<R: Read + Seek>(reader: &mut R) -> Result<Radial> {
    let message = crate::raw::messages::digital_radar_data::decode_digital_radar_data(reader)?;

    let map_data_block = |data_block: Option<GenericDataBlock>| -> Option<MomentData> {
        data_block.map(|data_block| {
            MomentData::from_fixed_point(
                data_block.header.scale,
                data_block.header.offset,
                data_block.encoded_data,
            )
        })
    };

    Ok(Radial::new(
        message.header.date as i64 * MILLIS_PER_DAY + message.header.time as i64,
        message.header.azimuth_number,
        message.header.azimuth_angle,
        match message.header.azimuth_resolution_spacing {
            1 => 0.5,
            _ => 1.0,
        },
        match message.header.radial_status {
            0 => RadialStatus::ElevationStart,
            1 => RadialStatus::IntermediateRadialData,
            2 => RadialStatus::ElevationEnd,
            3 => RadialStatus::VolumeScanStart,
            4 => RadialStatus::VolumeScanEnd,
            _ => RadialStatus::ElevationStartVCPFinal,
        },
        message.header.elevation_angle,
        map_data_block(message.reflectivity_data_block),
        map_data_block(message.velocity_data_block),
        map_data_block(message.spectrum_width_data_block),
        map_data_block(message.differential_reflectivity_data_block),
        map_data_block(message.differential_phase_data_block),
        map_data_block(message.correlation_coefficient_data_block),
        map_data_block(message.specific_diff_phase_data_block),
    ))
}
