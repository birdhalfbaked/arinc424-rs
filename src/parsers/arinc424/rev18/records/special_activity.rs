
use crate::parsers::arinc424::rev18::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
use crate::parsers::arinc424::rev18::definitions::*;
pub(super) struct SpecialActivityRecords;
impl SpecialActivityRecords {
    const CONTINUATION_COLUMN: usize = 22;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::SpecialActivityAreaPrimary(
                SpecialActivityAreaPrimaryRecord::parse(input)?,
            ))
        } else {
            Err(RecordParseError {
                message: "Invalid continuation record application type".to_string(),
            })
        }
    }
}

/// Parse the communications frequency from the input bytes
/// since we need to manually construct the frequency based on the frequency unit
fn parse_communications_frequency<'a>(
    input: &'a [u8],
) -> Result<RecordField<'a, CommunicationsFrequency>, RecordParseError> {
    let frequency_unit = FrequencyUnits::from_bytes(&input[45..46])?.ok_or(RecordParseError {
        message: "Invalid frequency units".to_string(),
    })?;
    let frequency_bytes = &input[86..92];
    let frequency = match frequency_unit {
        FrequencyUnits::HF => Some(CommunicationsFrequency::HighFrequency(
            HighFrequencyCommunicationsFrequency::from_bytes(frequency_bytes)?.ok_or(
                RecordParseError {
                    message: "Invalid transmit frequency".to_string(),
                },
            )?,
        )),
        FrequencyUnits::VHFNonStandardSpacing
        | FrequencyUnits::VHF8_33KHzSpacing
        | FrequencyUnits::VHF25KHzSpacing
        | FrequencyUnits::VHF50KHzSpacing
        | FrequencyUnits::VHF100KHzSpacing => Some(CommunicationsFrequency::VeryHighFrequency(
            VeryHighFrequencyCommunicationsFrequency::from_bytes(frequency_bytes)?.ok_or(
                RecordParseError {
                    message: "Invalid transmit frequency".to_string(),
                },
            )?,
        )),
        FrequencyUnits::UHF => Some(CommunicationsFrequency::UltraHighFrequency(
            UltraHighFrequencyCommunicationsFrequency::from_bytes(frequency_bytes)?.ok_or(
                RecordParseError {
                    message: "Invalid transmit frequency".to_string(),
                },
            )?,
        )),
        FrequencyUnits::DigitalService => None,
        _ => {
            return Err(RecordParseError {
                message: "Invalid frequency units".to_string(),
            });
        }
    };
    Ok(RecordField {
        raw_bytes: frequency_bytes,
        value: frequency,
    })
}

/// 4.1.33.1 Special Activity Area Primary Record
#[derive(Debug)]
pub struct SpecialActivityAreaPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub area_type: RecordField<'a, SpecialActivityType>,
    pub area_identifier: RecordField<'a, SpecialActivityAreaIdentifier>,
    pub area_icao_code: RecordField<'a, IcaoCode>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub latitude: RecordField<'a, Latitude>,
    pub longitude: RecordField<'a, Longitude>,
    pub area_size: RecordField<'a, SpecialActivityAreaSize>,
    pub comm_frequenc_units: RecordField<'a, FrequencyUnits>,
    pub upper_limit: RecordField<'a, LowerUpperLimit>,
    pub limit_unit_indicator: RecordField<'a, AirspaceLimitUnitIndicator>,
    pub area_volume: RecordField<'a, SpecialActivityAreaVolume>,
    pub operating_times: RecordField<'a, SpecialActivityTimes>,
    pub public_military_indicator: RecordField<'a, PublicMilitaryIndicator>,
    pub controlling_agency: RecordField<'a, ControllingAgency>,
    pub communication_type: RecordField<'a, CommunicationsType>,
    pub communications_frequency: RecordField<'a, CommunicationsFrequency>,
    pub area_name: RecordField<'a, RestrictiveAirspaceName>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> SpecialActivityAreaPrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        let communications_frequency = parse_communications_frequency(input)?;
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            area_type:                    RecordField::from_bytes(input, 7, 1)?,
            area_identifier:              RecordField::from_bytes(input, 8, 6)?,
            area_icao_code:               RecordField::from_bytes(input, 14, 2)?,
            airport_identifier:           RecordField::from_bytes(input, 16, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            latitude:                     RecordField::from_bytes(input, 24, 9)?,
            longitude:                    RecordField::from_bytes(input, 33, 10)?,
            area_size:                    RecordField::from_bytes(input, 43, 3)?,
            comm_frequenc_units:          RecordField::from_bytes(input, 46, 1)?,
            upper_limit:                  RecordField::from_bytes(input, 47, 5)?,
            limit_unit_indicator:         RecordField::from_bytes(input, 52, 1)?,
            area_volume:                  RecordField::from_bytes(input, 53, 1)?,
            operating_times:              RecordField::from_bytes(input, 54, 3)?,
            public_military_indicator:    RecordField::from_bytes(input, 57, 1)?,
            controlling_agency:           RecordField::from_bytes(input, 59, 25)?,
            communication_type:           RecordField::from_bytes(input, 84, 3)?,
            communications_frequency:     communications_frequency,
            area_name:                    RecordField::from_bytes(input, 94, 30)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
