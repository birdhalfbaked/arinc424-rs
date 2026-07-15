use crate::rev23::records::record::ARINCRecord;

use crate::rev23::definitions::*;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct HeliportHelipadRecords;
impl HeliportHelipadRecords {
    const CONTINUATION_COLUMN: usize = 22;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportHelipadPrimary(
                HeliportHelipadPrimaryRecord::parse(input)?,
            ))
        } else {
            Err(RecordParseError::new(
                "Invalid record type".to_string(),
                Some(String::from_utf8_lossy(input).into_owned()),
            ))
        }
    }
}

fn parse_pad_dimensions<'a>(
    input: &'a [u8],
) -> Result<
    (
        RecordField<'a, PadDimensions>,
        RecordField<'a, PadDimensions>,
        RecordField<'a, PadDimensions>,
    ),
    RecordParseError,
> {
    let shape_bytes = &input[22..23];
    let tlof_dimension_bytes = &input[23..30];
    let fato_dimension_bytes = &input[70..77];
    let safety_area_dimension_bytes = &input[78..85];

    let tlof_dimension_value = match HelipadShape::from_bytes(shape_bytes)? {
        Some(HelipadShape::Circle) => {
            if let Some(circular_pad_dimensions) =
                CircularPadDimensions::from_bytes(tlof_dimension_bytes)?
            {
                Some(PadDimensions::Circular(circular_pad_dimensions))
            } else {
                None
            }
        }

        Some(HelipadShape::Rectangular) | Some(HelipadShape::Runway) => {
            if let Some(rectangular_pad_dimensions) =
                RectangularPadDimensions::from_bytes(tlof_dimension_bytes)?
            {
                Some(PadDimensions::Rectangular(rectangular_pad_dimensions))
            } else {
                None
            }
        }
        Some(HelipadShape::Undefined) => None,
        None => {
            return Err(RecordParseError::new(
                "Invalid helipad shape".to_string(),
                Some(String::from_utf8_lossy(input).into_owned()),
            ));
        }
    };
    let fato_dimension_value = match HelipadShape::from_bytes(shape_bytes)? {
        Some(HelipadShape::Circle) => {
            if let Some(circular_pad_dimensions) =
                CircularPadDimensions::from_bytes(fato_dimension_bytes)?
            {
                Some(PadDimensions::Circular(circular_pad_dimensions))
            } else {
                None
            }
        }

        Some(HelipadShape::Rectangular) | Some(HelipadShape::Runway) => {
            if let Some(rectangular_pad_dimensions) =
                RectangularPadDimensions::from_bytes(fato_dimension_bytes)?
            {
                Some(PadDimensions::Rectangular(rectangular_pad_dimensions))
            } else {
                None
            }
        }
        Some(HelipadShape::Undefined) => None,
        None => {
            return Err(RecordParseError::new(
                "Invalid helipad shape".to_string(),
                Some(String::from_utf8_lossy(input).into_owned()),
            ));
        }
    };
    let safety_area_dimension_value = match HelipadShape::from_bytes(shape_bytes)? {
        Some(HelipadShape::Circle) => {
            if let Some(circular_pad_dimensions) =
                CircularPadDimensions::from_bytes(safety_area_dimension_bytes)?
            {
                Some(PadDimensions::Circular(circular_pad_dimensions))
            } else {
                None
            }
        }

        Some(HelipadShape::Rectangular) | Some(HelipadShape::Runway) => {
            if let Some(rectangular_pad_dimensions) =
                RectangularPadDimensions::from_bytes(safety_area_dimension_bytes)?
            {
                Some(PadDimensions::Rectangular(rectangular_pad_dimensions))
            } else {
                None
            }
        }
        Some(HelipadShape::Undefined) => None,
        None => {
            return Err(RecordParseError::new(
                "Invalid helipad shape".to_string(),
                Some(String::from_utf8_lossy(input).into_owned()),
            ));
        }
    };

    Ok((
        RecordField {
            raw_bytes: tlof_dimension_bytes,
            value: tlof_dimension_value,
            start_column: 23,
            end_column: 30,
        },
        RecordField {
            raw_bytes: fato_dimension_bytes,
            value: fato_dimension_value,
            start_column: 70,
            end_column: 77,
        },
        RecordField {
            raw_bytes: safety_area_dimension_bytes,
            value: safety_area_dimension_value,
            start_column: 78,
            end_column: 85,
        },
    ))
}

/// 4.2.36.1 Heliport Helipad Primary Record
#[derive(Debug)]
pub struct HeliportHelipadPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub helipad_identifier: RecordField<'a, PadIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub helipad_shape: RecordField<'a, HelipadShape>,
    pub helipad_tlof_dimension: RecordField<'a, PadDimensions>,
    pub helipad_coordinates_source: RecordField<'a, GovernmentSource>,
    pub helipad_latitude: RecordField<'a, Latitude>,
    pub helipad_longitude: RecordField<'a, Longitude>,
    pub helipad_surface_code: RecordField<'a, RunwaySurfaceCode>,
    pub helipad_surface_type: RecordField<'a, SurfaceType>,
    pub max_allowable_helicopter_weight: RecordField<'a, MaximumAllowableHelicopterWeight>,
    pub max_rotor_diameter: RecordField<'a, HelipadMaximumRotorDiameter>,
    pub helipad_type: RecordField<'a, HelipadType>,
    pub helipad_elevation_type: RecordField<'a, ElevationType>,
    pub helipad_elevation: RecordField<'a, LandingThresholdElevation>,
    pub helipad_fato_dimension: RecordField<'a, PadDimensions>,
    pub safety_area_dimension: RecordField<'a, PadDimensions>,
    pub helipad_orientation: RecordField<'a, HeliportOrientation>,
    pub helipad_identifier_orientation: RecordField<'a, HeliportIdentifierOrientation>,
    pub preferred_approach_bearing_1: RecordField<'a, PreferredApproachBearing>,
    pub preferred_approach_bearing_2: RecordField<'a, PreferredApproachBearing>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportHelipadPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportHelipadPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {

        let (tlof_dimensions, fato_dimensions, safety_area_dimensions) = parse_pad_dimensions(input)?;

        Ok(Self {
            record_type:                       RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                RecordField::from_bytes(input, 2, 3)?,
            section:                           RecordField::from_bytes(input, 5, 1)?,
            airport_heliport_identifier:       RecordField::from_bytes(input, 7, 4)?,
            airport_heliport_icao_code:        RecordField::from_bytes(input, 11, 2)?,
            subsection:                        RecordField::from_bytes(input, 13, 1)?,
            helipad_identifier:                RecordField::from_bytes(input, 14, 5)?,
            continuation_record_number:        RecordField::from_bytes(input, 22, 1)?,
            helipad_shape:                     RecordField::from_bytes(input, 23, 1)?,
            helipad_tlof_dimension:            tlof_dimensions,
            helipad_coordinates_source:        RecordField::from_bytes(input, 32, 1)?,
            helipad_latitude:                  RecordField::from_bytes(input, 33, 9)?,
            helipad_longitude:                 RecordField::from_bytes(input, 42, 10)?,
            helipad_surface_code:              RecordField::from_bytes(input, 52, 1)?,
            helipad_surface_type:              RecordField::from_bytes(input, 53, 4)?,
            max_allowable_helicopter_weight:   RecordField::from_bytes(input, 57, 3)?,
            max_rotor_diameter:                RecordField::from_bytes(input, 60, 1)?,
            helipad_type:                      RecordField::from_bytes(input, 61, 3)?,
            helipad_elevation_type:            RecordField::from_bytes(input, 64, 1)?,
            helipad_elevation:                 RecordField::from_bytes(input, 65, 1)?,
            helipad_fato_dimension:            fato_dimensions,
            safety_area_dimension:             safety_area_dimensions,
            helipad_orientation:               RecordField::from_bytes(input, 87, 5)?,
            helipad_identifier_orientation:    RecordField::from_bytes(input, 92, 5)?,
            preferred_approach_bearing_1:      RecordField::from_bytes(input, 97, 100)?,
            preferred_approach_bearing_2:      RecordField::from_bytes(input, 101, 4)?,
            file_record_number:                RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                        RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}
