
use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
use crate::parsers::arinc424::rev23::definitions::*;
pub(super) struct TerminalWaypointRecords;
impl TerminalWaypointRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::TerminalWaypointPrimary(
                TerminalWaypointPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::TerminalWaypointContinuation(
                        TerminalWaypointContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::FlightPlanningContinuation) => {
                    Ok(ARINCRecord::TerminalWaypointFlightPlanningContinuation(
                        TerminalWaypointFlightPlanningContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError {
                    message: "Invalid continuation record application type".to_string(),
                }),
            }
        }
    }
}

/// 4.1.4.1(B) Terminal Waypoint Primary Record
#[derive(Debug)]
pub struct TerminalWaypointPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub waypoint_identifier: RecordField<'a, FixIdentifier>,
    pub waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub waypoint_type: RecordField<'a, WaypointType>,
    pub waypoint_usage: RecordField<'a, WaypointUsage>,
    pub waypoint_latitude: RecordField<'a, Latitude>,
    pub waypoint_longitude: RecordField<'a, Longitude>,
    pub dynamic_magnetic_variation: RecordField<'a, MagneticVariation>,
    pub vfr_checkpoint_flag: RecordField<'a, VFRCheckpointFlag>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub name_format_indicator: RecordField<'a, NameFormat>,
    pub waypoint_name_description: RecordField<'a, WaypointNameDescription>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> TerminalWaypointPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            region_code:                    RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:               RecordField::from_bytes(input, 11, 2)?,
            subsection:                     RecordField::from_bytes(input, 13, 1)?,
            waypoint_identifier:            RecordField::from_bytes(input, 14, 5)?,
            waypoint_icao_code:             RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
            waypoint_type:                  RecordField::from_bytes(input, 27, 3)?,
            waypoint_usage:                 RecordField::from_bytes(input, 31, 1)?,
            waypoint_latitude:              RecordField::from_bytes(input, 33, 9)?,
            waypoint_longitude:             RecordField::from_bytes(input, 42, 10)?,
            dynamic_magnetic_variation:     RecordField::from_bytes(input, 75, 5)?,
            vfr_checkpoint_flag:            RecordField::from_bytes(input, 80, 1)?,
            datum_code:                     RecordField::from_bytes(input, 85, 3)?,
            name_format_indicator:          RecordField::from_bytes(input, 96, 3)?,
            waypoint_name_description:      RecordField::from_bytes(input, 99, 25)?,
            file_record_number:             RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                     RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.4.2(B) Terminal Waypoint Continuation Record
#[derive(Debug)]
pub struct TerminalWaypointContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub waypoint_identifier: RecordField<'a, FixIdentifier>,
    pub waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> TerminalWaypointContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            region_code:                    RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:               RecordField::from_bytes(input, 11, 2)?,
            subsection:                     RecordField::from_bytes(input, 13, 1)?,
            waypoint_identifier:            RecordField::from_bytes(input, 14, 5)?,
            waypoint_icao_code:             RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
            application_type:               RecordField::from_bytes(input, 23, 1)?,
            notes:                          RecordField::from_bytes(input, 24, 69)?,
            file_record_number:             RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                     RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.4.3(B) Terminal Waypoint Flight Planning Continuation Record
#[derive(Debug)]
pub struct TerminalWaypointFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub waypoint_identifier: RecordField<'a, FixIdentifier>,
    pub waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub fir_identifier: RecordField<'a, FirUirIdentifier>,
    pub uir_identifier: RecordField<'a, FirUirIdentifier>,
    pub fir_fra_entry_point: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_exit_point: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_arrival_transition: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_departure_transition: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_intermediate_point: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_terminal_holding_point: RecordField<'a, FIRFRATransitionType>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> TerminalWaypointFlightPlanningContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            region_code:                        RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:                   RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            waypoint_identifier:                RecordField::from_bytes(input, 14, 5)?,
            waypoint_icao_code:                 RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            application_type:                   RecordField::from_bytes(input, 23, 1)?,
            fir_identifier:                     RecordField::from_bytes(input, 24, 4)?,
            uir_identifier:                     RecordField::from_bytes(input, 28, 4)?,
            fir_fra_entry_point:                RecordField::from_bytes(input, 44, 1)?,
            fir_fra_exit_point:                 RecordField::from_bytes(input, 45, 1)?,
            fir_fra_arrival_transition:         RecordField::from_bytes(input, 46, 1)?,
            fir_fra_departure_transition:       RecordField::from_bytes(input, 47, 1)?,
            fir_fra_intermediate_point:         RecordField::from_bytes(input, 48, 1)?,
            fir_fra_terminal_holding_point:     RecordField::from_bytes(input, 49, 1)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
