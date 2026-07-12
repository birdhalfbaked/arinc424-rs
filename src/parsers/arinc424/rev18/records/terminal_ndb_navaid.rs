use crate::parsers::arinc424::rev18::definitions::*;
use crate::parsers::arinc424::rev18::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
pub(super) struct TerminalNDBNavaidRecords;
impl TerminalNDBNavaidRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::TerminalNDBNavaidPrimary(
                TerminalNDBNavaidPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::TerminalNDBNavaidContinuation(
                        TerminalNDBNavaidContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::SimulationContinuation) => {
                    Ok(ARINCRecord::TerminalNDBNavaidSimulationContinuation(
                        TerminalNDBNavaidSimulationContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::FlightPlanningContinuation) => {
                    Ok(ARINCRecord::TerminalNDBNavaidFlightPlanningContinuation(
                        TerminalNDBNavaidFlightPlanningContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError {
                    message: "Invalid continuation record application type".to_string(),
                }),
            }
        }
    }
}

/// 4.1.3.1(B) Terminal NDB Navaid Primary Record
#[derive(Debug)]
pub struct TerminalNDBNavaidPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub ndb_identifier: RecordField<'a, VORNDBIdentifier>,
    pub ndb_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub ndb_frequency: RecordField<'a, NDBFrequency>,
    pub ndb_class: RecordField<'a, NDBNavaidClass>,
    pub ndb_latitude: RecordField<'a, Latitude>,
    pub ndb_longitude: RecordField<'a, Longitude>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub vfr_checkpoint_flag: RecordField<'a, VFRCheckpointFlag>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub ndb_name: RecordField<'a, NameOfFacility>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> TerminalNDBNavaidPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            subsection:                     RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:        RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:              RecordField::from_bytes(input, 11, 2)?,
            ndb_identifier:                 RecordField::from_bytes(input, 14, 4)?,
            ndb_icao_code:                  RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
            ndb_frequency:                  RecordField::from_bytes(input, 23, 5)?,
            ndb_class:                      RecordField::from_bytes(input, 28, 5)?,
            ndb_latitude:                   RecordField::from_bytes(input, 33, 9)?,
            ndb_longitude:                  RecordField::from_bytes(input, 42, 10)?,
            magnetic_variation:             RecordField::from_bytes(input, 75, 5)?,
            vfr_checkpoint_flag:            RecordField::from_bytes(input, 80, 1)?,
            datum_code:                     RecordField::from_bytes(input, 91, 3)?,
            ndb_name:                       RecordField::from_bytes(input, 94, 30)?,
            file_record_number:             RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                     RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.3.2 NDB Navaid Continuation Record
#[derive(Debug)]
pub struct TerminalNDBNavaidContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub ndb_identifier: RecordField<'a, VORNDBIdentifier>,
    pub ndb_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> TerminalNDBNavaidContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:         RecordField::from_bytes(input, 2, 3)?,
            section:                    RecordField::from_bytes(input, 5, 1)?,
            subsection:                 RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:    RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:          RecordField::from_bytes(input, 11, 2)?,
            ndb_identifier:             RecordField::from_bytes(input, 14, 4)?,
            ndb_icao_code:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number: RecordField::from_bytes(input, 22, 1)?,
            application_type:           RecordField::from_bytes(input, 23, 1)?,
            notes:                      RecordField::from_bytes(input, 24, 69)?,
            file_record_number:         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.3.3 NDB Navaid Simulation Continuation Record
#[derive(Debug)]
pub struct TerminalNDBNavaidSimulationContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub ndb_identifier: RecordField<'a, VORNDBIdentifier>,
    pub ndb_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub facility_characteristics: RecordField<'a, FacilityCharacteristics>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> TerminalNDBNavaidSimulationContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:         RecordField::from_bytes(input, 2, 3)?,
            section:                    RecordField::from_bytes(input, 5, 1)?,
            subsection:                 RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:    RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:          RecordField::from_bytes(input, 11, 2)?,
            ndb_identifier:             RecordField::from_bytes(input, 14, 4)?,
            ndb_icao_code:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number: RecordField::from_bytes(input, 22, 1)?,
            application_type:           RecordField::from_bytes(input, 23, 1)?,
            facility_characteristics:   RecordField::from_bytes(input, 28, 5)?,
            facility_elevation:         RecordField::from_bytes(input, 80, 5)?,
            file_record_number:         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.3.4 NDB Navaid Flight Planning Continuation Record
#[derive(Debug)]
pub struct TerminalNDBNavaidFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, AirportSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub ndb_identifier: RecordField<'a, VORNDBIdentifier>,
    pub ndb_icao_code: RecordField<'a, IcaoCode>,
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
impl<'a> TerminalNDBNavaidFlightPlanningContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:            RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            ndb_identifier:                     RecordField::from_bytes(input, 14, 4)?,
            ndb_icao_code:                      RecordField::from_bytes(input, 20, 2)?,
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
