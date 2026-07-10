use crate::parsers::arinc424::definitions::*;
use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
pub(super) struct EnrouteCommsRecords;
impl EnrouteCommsRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::EnrouteCommsPrimary(
                EnrouteCommsPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::PrimaryRecordExtension) => {
                    Ok(ARINCRecord::EnrouteCommsPrimaryExtensionContinuation(
                        EnrouteCommsPrimaryExtensionContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                    Ok(ARINCRecord::EnrouteCommsFormattedTimeContinuation(
                        EnrouteCommsFormattedTimeContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::NarrativeTimeOfOperationsContinuation) => {
                    Ok(ARINCRecord::EnrouteCommsNarrativeTimeContinuation(
                        EnrouteCommsNarrativeTimeContinuationRecord::parse(input)?,
                    ))
                }
                _ => {
                    return Err(RecordParseError {
                        message: "Invalid continuation record application type".to_string(),
                    });
                }
            }
        }
    }
}

/// Parse the communications frequency from the input bytes
/// since we need to manually construct the frequency based on the frequency unit
fn parse_communications_frequency<'a>(
    input: &'a [u8],
) -> Result<
    (
        RecordField<'a, CommunicationsFrequency>,
        RecordField<'a, CommunicationsFrequency>,
    ),
    RecordParseError,
> {
    let frequency_unit = FrequencyUnits::from_bytes(&input[39..40])?.ok_or(RecordParseError {
        message: "Invalid frequency units".to_string(),
    })?;
    let transmit_frequency_bytes = &input[25..32];
    let receive_frequency_bytes = &input[32..39];
    let transmit_frequency = match frequency_unit {
        FrequencyUnits::HF => Some(CommunicationsFrequency::HighFrequency(
            HighFrequencyCommunicationsFrequency::from_bytes(transmit_frequency_bytes)?.ok_or(
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
            VeryHighFrequencyCommunicationsFrequency::from_bytes(transmit_frequency_bytes)?.ok_or(
                RecordParseError {
                    message: "Invalid transmit frequency".to_string(),
                },
            )?,
        )),
        FrequencyUnits::UHF => Some(CommunicationsFrequency::UltraHighFrequency(
            UltraHighFrequencyCommunicationsFrequency::from_bytes(transmit_frequency_bytes)?
                .ok_or(RecordParseError {
                    message: "Invalid transmit frequency".to_string(),
                })?,
        )),
        FrequencyUnits::DigitalService => None,
        _ => {
            return Err(RecordParseError {
                message: "Invalid frequency units".to_string(),
            });
        }
    };
    let receive_frequency =
        match frequency_unit {
            FrequencyUnits::HF => Some(CommunicationsFrequency::HighFrequency(
                HighFrequencyCommunicationsFrequency::from_bytes(receive_frequency_bytes)?.ok_or(
                    RecordParseError {
                        message: "Invalid receive frequency".to_string(),
                    },
                )?,
            )),
            FrequencyUnits::VHFNonStandardSpacing
            | FrequencyUnits::VHF8_33KHzSpacing
            | FrequencyUnits::VHF25KHzSpacing
            | FrequencyUnits::VHF50KHzSpacing
            | FrequencyUnits::VHF100KHzSpacing => Some(CommunicationsFrequency::VeryHighFrequency(
                VeryHighFrequencyCommunicationsFrequency::from_bytes(receive_frequency_bytes)?
                    .ok_or(RecordParseError {
                        message: "Invalid transmit frequency".to_string(),
                    })?,
            )),
            FrequencyUnits::UHF => Some(CommunicationsFrequency::UltraHighFrequency(
                UltraHighFrequencyCommunicationsFrequency::from_bytes(receive_frequency_bytes)?
                    .ok_or(RecordParseError {
                        message: "Invalid transmit frequency".to_string(),
                    })?,
            )),
            FrequencyUnits::DigitalService => None,
            _ => {
                return Err(RecordParseError {
                    message: "Invalid frequency units".to_string(),
                });
            }
        };
    Ok((
        RecordField {
            raw_bytes: transmit_frequency_bytes,
            value: transmit_frequency,
        },
        RecordField {
            raw_bytes: receive_frequency_bytes,
            value: receive_frequency,
        },
    ))
}

// 4.1.23.1 Enroute Communications Primary Record
#[derive(Debug)]
pub struct EnrouteCommsPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_rdo_identifier: RecordField<'a, FirRdoIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub communications_class: RecordField<'a, CommunicationsClass>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub communication_types: RecordField<'a, CommunicationsType>,
    pub transmit_frequency: RecordField<'a, CommunicationsFrequency>,
    pub receive_frequency: RecordField<'a, CommunicationsFrequency>,
    pub frequency_units: RecordField<'a, FrequencyUnits>,
    pub radar_class: RecordField<'a, Radar>,
    pub is_h24: RecordField<'a, H24Indicator>,
    pub callsigns: RecordField<'a, Callsign>,
    pub position_narrative: RecordField<'a, PositionNarrative>,
    pub latitude: RecordField<'a, Latitude>,
    pub longitude: RecordField<'a, Longitude>,
    pub service_indicator: RecordField<'a, AirportHeliportCommunicationsServiceIndicator1>,
    pub modulation: RecordField<'a, Modulation>,
    pub signal_emission: RecordField<'a, SignalEmission>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub communication_altitude_1: RecordField<'a, CommunicationsAltitude>,
    pub communication_altitude_2: RecordField<'a, CommunicationsAltitude>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> EnrouteCommsPrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        let (transmit_frequency, receive_frequency) = parse_communications_frequency(input)?;
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            fir_rdo_identifier:           RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:              RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:            RecordField::from_bytes(input, 15, 1)?,
            communications_class:         RecordField::from_bytes(input, 16, 4)?,
            sequence_number:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            communication_types:          RecordField::from_bytes(input, 23, 3)?,
            transmit_frequency:           transmit_frequency,
            receive_frequency:            receive_frequency,
            frequency_units:              RecordField::from_bytes(input, 40, 1)?,
            radar_class:                  RecordField::from_bytes(input, 41, 1)?,
            is_h24:                       RecordField::from_bytes(input, 42, 1)?,
            callsigns:                    RecordField::from_bytes(input, 43, 25)?,
            position_narrative:           RecordField::from_bytes(input, 68, 25)?,
            latitude:                     RecordField::from_bytes(input, 93, 9)?,
            longitude:                    RecordField::from_bytes(input, 102, 10)?,
            service_indicator:            RecordField::from_bytes(input, 112, 3)?,
            modulation:                   RecordField::from_bytes(input, 115, 1)?,
            signal_emission:              RecordField::from_bytes(input, 116, 1)?,
            altitude_description:         RecordField::from_bytes(input, 117, 1)?,
            communication_altitude_1:     RecordField::from_bytes(input, 118, 3)?,
            communication_altitude_2:     RecordField::from_bytes(input, 121, 3)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

// 4.1.23.2 Enroute Communications Primary Extension Continuation Record
#[derive(Debug)]
pub struct EnrouteCommsPrimaryExtensionContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_rdo_identifier: RecordField<'a, FirRdoIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub communications_class: RecordField<'a, CommunicationsClass>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub remote_facility_identifier: RecordField<'a, RemoteFacility>,
    pub remote_facility_icao_code: RecordField<'a, IcaoCode>,
    pub remote_facility_section: RecordField<'a, Section>,
    pub remote_facility_subsection: RecordField<'a, GenericSubsection>,
    pub transmitter_site_mag_variation: RecordField<'a, MagneticVariation>,
    pub transmitter_site_elevation: RecordField<'a, FacilityElevation>,
    pub assigned_sector_name: RecordField<'a, AssignedSectorName>,
    pub time_code: RecordField<'a, ContinuationRecordTimeCode>,
    pub notam: RecordField<'a, NotamFlag>,
    pub level: RecordField<'a, Level>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> EnrouteCommsPrimaryExtensionContinuationRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            fir_rdo_identifier:                 RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:                    RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:                  RecordField::from_bytes(input, 15, 1)?,
            communications_class:               RecordField::from_bytes(input, 16, 4)?,
            sequence_number:                    RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            application_type:                   RecordField::from_bytes(input, 23, 1)?,
            remote_facility_identifier:         RecordField::from_bytes(input, 24, 4)?,
            remote_facility_icao_code:          RecordField::from_bytes(input, 28, 2)?,
            remote_facility_section:            RecordField::from_bytes(input, 30, 1)?,
            remote_facility_subsection:         RecordField::from_bytes(input, 31, 1)?,
            transmitter_site_mag_variation:     RecordField::from_bytes(input, 32, 5)?,
            transmitter_site_elevation:         RecordField::from_bytes(input, 37, 5)?,
            assigned_sector_name:               RecordField::from_bytes(input, 42, 25)?,
            time_code:                          RecordField::from_bytes(input, 67, 1)?,
            notam:                              RecordField::from_bytes(input, 68, 1)?,
            level:                              RecordField::from_bytes(input, 69, 1)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

// 4.1.23.3 Enroute Communications Formatted Time Continuation Record
#[derive(Debug)]
pub struct EnrouteCommsFormattedTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_rdo_identifier: RecordField<'a, FirRdoIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub communications_class: RecordField<'a, CommunicationsClass>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_code: RecordField<'a, ContinuationRecordTimeCode>,
    pub notam: RecordField<'a, NotamFlag>,
    pub time_indicator: RecordField<'a, TimeIndicator>,
    pub timezone: RecordField<'a, Timezone>,
    pub time_of_operation_1: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_2: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_3: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_4: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_5: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_6: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_7: RecordField<'a, TimeOfOperation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> EnrouteCommsFormattedTimeContinuationRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            fir_rdo_identifier:           RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:              RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:            RecordField::from_bytes(input, 15, 1)?,
            communications_class:         RecordField::from_bytes(input, 16, 4)?,
            sequence_number:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            application_type:             RecordField::from_bytes(input, 23, 1)?,
            time_code:                    RecordField::from_bytes(input, 24, 1)?,
            notam:                        RecordField::from_bytes(input, 25, 1)?,
            time_indicator:               RecordField::from_bytes(input, 26, 1)?,
            timezone:                     RecordField::from_bytes(input, 27, 3)?,
            time_of_operation_1:          RecordField::from_bytes(input, 50, 10)?,
            time_of_operation_2:          RecordField::from_bytes(input, 60, 10)?,
            time_of_operation_3:          RecordField::from_bytes(input, 70, 10)?,
            time_of_operation_4:          RecordField::from_bytes(input, 80, 10)?,
            time_of_operation_5:          RecordField::from_bytes(input, 90, 10)?,
            time_of_operation_6:          RecordField::from_bytes(input, 100, 10)?,
            time_of_operation_7:          RecordField::from_bytes(input, 110, 10)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

// 4.1.23.4 Enroute Communications Narrative Time Continuation Record
#[derive(Debug)]
pub struct EnrouteCommsNarrativeTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub fir_rdo_identifier: RecordField<'a, FirRdoIdentifier>,
    pub fir_uir_address: RecordField<'a, FirUirAddress>,
    pub fir_uir_indicator: RecordField<'a, FirUirIndicator>,
    pub communications_class: RecordField<'a, CommunicationsClass>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub narrative_time: RecordField<'a, TimeNarrative>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> EnrouteCommsNarrativeTimeContinuationRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            fir_rdo_identifier:           RecordField::from_bytes(input, 7, 4)?,
            fir_uir_address:              RecordField::from_bytes(input, 11, 4)?,
            fir_uir_indicator:            RecordField::from_bytes(input, 15, 1)?,
            communications_class:         RecordField::from_bytes(input, 16, 4)?,
            sequence_number:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            application_type:             RecordField::from_bytes(input, 23, 1)?,
            narrative_time:               RecordField::from_bytes(input, 24, 100)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
