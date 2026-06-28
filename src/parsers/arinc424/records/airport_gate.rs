use crate::parsers::arinc424::definitions::*;
use crate::parsers::arinc424::fields::ParseableField;
use crate::parsers::arinc424::records::record::{
    ARINCRecord, RecordField, RecordParseError, is_primary_record,
};
pub(super) struct AirportGateRecords;
impl AirportGateRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::AirportGatePrimary(
                AirportGatePrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN
                    ..Self::CONTINUATION_APPLICATION_COLUMN + 1],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::AirportGateContinuation(
                        AirportGateContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError {
                    message: "Invalid continuation record application type".to_string(),
                }),
            }
        }
    }
}

#[derive(Debug)]
pub struct AirportGatePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub gate_identifier: RecordField<'a, GateIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub gate_latitude: RecordField<'a, Latitude>,
    pub gate_longitude: RecordField<'a, Longitude>,
    pub name: RecordField<'a, Name>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> AirportGatePrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:           RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            gate_identifier:              RecordField::from_bytes(input, 14, 5)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            gate_latitude:                RecordField::from_bytes(input, 33, 9)?,
            gate_longitude:               RecordField::from_bytes(input, 42, 10)?,
            name:                         RecordField::from_bytes(input, 99, 25)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

#[derive(Debug)]
pub struct AirportGateContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub gate_identifier: RecordField<'a, GateIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> AirportGateContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:           RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            gate_identifier:              RecordField::from_bytes(input, 14, 5)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            application_type:             RecordField::from_bytes(input, 23, 1)?,
            notes:                        RecordField::from_bytes(input, 24, 69)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
