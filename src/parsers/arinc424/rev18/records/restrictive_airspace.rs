use crate::parsers::arinc424::rev18::definitions::*;
use crate::parsers::arinc424::rev18::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
pub(super) struct RestrictiveAirspaceRecords;
impl RestrictiveAirspaceRecords {
    const CONTINUATION_COLUMN: usize = 25;
    const CONTINUATION_APPLICATION_COLUMN: usize = 26;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::RestrictiveAirspacePrimary(
                RestrictiveAirspacePrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                _ => Err(RecordParseError {
                    message: "Invalid continuation record application type".to_string(),
                }),
            }
        }
    }
}

/// 4.1.18.1 Restrictive Airspace Primary Record
#[derive(Debug)]
pub struct RestrictiveAirspacePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub airspace_icao_code: RecordField<'a, IcaoCode>,
    pub restrictive_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restrictive_airspace_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub level: RecordField<'a, Level>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub notam: RecordField<'a, NotamFlag>,
    pub uav: RecordField<'a, UnmannedAerialVehicleOnly>,
    pub boundary_via: RecordField<'a, BoundaryVia>,
    pub latitude: RecordField<'a, Latitude>,
    pub longitude: RecordField<'a, Longitude>,
    pub arc_origin_latitude: RecordField<'a, Latitude>,
    pub arc_origin_longitude: RecordField<'a, Longitude>,
    pub arc_distance: RecordField<'a, ArcDistance>,
    pub arc_bearing: RecordField<'a, ArcBearing>,
    pub lower_limit: RecordField<'a, LowerUpperLimit>,
    pub lower_limit_unit: RecordField<'a, AirspaceLimitUnitIndicator>,
    pub upper_limit: RecordField<'a, LowerUpperLimit>,
    pub upper_limit_unit: RecordField<'a, AirspaceLimitUnitIndicator>,
    pub airspace_name: RecordField<'a, RestrictiveAirspaceName>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> RestrictiveAirspacePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            airspace_icao_code:                 RecordField::from_bytes(input, 7, 2)?,
            restrictive_type:                   RecordField::from_bytes(input, 9, 1)?,
            restrictive_airspace_designation:   RecordField::from_bytes(input, 10, 10)?,
            multiple_code:                      RecordField::from_bytes(input, 20, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 21, 4)?,
            continuation_record_number:         RecordField::from_bytes(input, 25, 1)?,
            level:                              RecordField::from_bytes(input, 26, 1)?,
            time_code:                          RecordField::from_bytes(input, 27, 1)?,
            notam:                              RecordField::from_bytes(input, 28, 1)?,
            uav:                                RecordField::from_bytes(input, 29, 1)?,
            boundary_via:                       RecordField::from_bytes(input, 31, 2)?,
            latitude:                           RecordField::from_bytes(input, 33, 9)?,
            longitude:                          RecordField::from_bytes(input, 42, 10)?,
            arc_origin_latitude:                RecordField::from_bytes(input, 52, 9)?,
            arc_origin_longitude:               RecordField::from_bytes(input, 61, 10)?,
            arc_distance:                       RecordField::from_bytes(input, 71, 4)?,
            arc_bearing:                        RecordField::from_bytes(input, 75, 4)?,
            lower_limit:                        RecordField::from_bytes(input, 82, 5)?,
            lower_limit_unit:                   RecordField::from_bytes(input, 87, 1)?,
            upper_limit:                        RecordField::from_bytes(input, 88, 5)?,
            upper_limit_unit:                   RecordField::from_bytes(input, 93, 1)?,
            airspace_name:                      RecordField::from_bytes(input, 94, 30)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
