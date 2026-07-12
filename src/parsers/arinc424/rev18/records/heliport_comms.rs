use crate::parsers::arinc424::rev18::definitions::*;
use crate::parsers::arinc424::rev18::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
pub(super) struct HeliportCommsRecords;
impl HeliportCommsRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportCommsPrimary(
                HeliportCommsPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::PrimaryRecordExtension) => {
                    Ok(ARINCRecord::HeliportCommsPrimaryExtensionContinuation(
                        HeliportCommsPrimaryExtensionContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::SectorNarrativeContinuation) => {
                    Ok(ARINCRecord::HeliportCommsSectorNarrativeContinuation(
                        HeliportCommsSectorNarrativeContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::AdditionalSectorizationContinuation) => Ok(
                    ARINCRecord::HeliportCommsAdditionalSectorizationContinuation(
                        HeliportCommsAdditionalSectorizationContinuationRecord::parse(input)?,
                    ),
                ),
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

// 4.2.5.1 Heliport Communications Primary Record
#[derive(Debug)]
pub struct HeliportCommsPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
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
    pub multi_sector_indicator: RecordField<'a, MultiSectorIndicator>,
    pub sectorization: RecordField<'a, CommunicationsSectorization>,
    pub sector_facility: RecordField<'a, SectorFacility>,
    pub sector_facility_icao_code: RecordField<'a, IcaoCode>,
    pub sector_facility_section: RecordField<'a, Section>,
    pub sector_facility_subsection: RecordField<'a, GenericSubsection>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub communication_altitude_1: RecordField<'a, CommunicationsAltitude>,
    pub communication_altitude_2: RecordField<'a, CommunicationsAltitude>,
    pub distance_description: RecordField<'a, DistanceDescription>,
    pub communication_distance: RecordField<'a, CommunicationsDistance>,
    pub transmitter_latitude: RecordField<'a, Latitude>,
    pub transmitter_longitude: RecordField<'a, Longitude>,
    pub service_indicator: RecordField<'a, AirportHeliportCommunicationsServiceIndicator1>,
    pub modulation: RecordField<'a, Modulation>,
    pub signal_emission: RecordField<'a, SignalEmission>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub notam: RecordField<'a, NotamFlag>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HeliportCommsPrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {

        let (transmit_frequency, receive_frequency) = parse_communications_frequency(input)?;

        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:          RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:           RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
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
            multi_sector_indicator:       RecordField::from_bytes(input, 68, 1)?,
            sectorization:                RecordField::from_bytes(input, 69, 6)?,
            sector_facility:              RecordField::from_bytes(input, 75, 4)?,
            sector_facility_icao_code:    RecordField::from_bytes(input, 79, 2)?,
            sector_facility_section:      RecordField::from_bytes(input, 81, 1)?,
            sector_facility_subsection:   RecordField::from_bytes(input, 82, 1)?,
            altitude_description:         RecordField::from_bytes(input, 83, 1)?,
            communication_altitude_1:     RecordField::from_bytes(input, 84, 3)?,
            communication_altitude_2:     RecordField::from_bytes(input, 87, 3)?,
            distance_description:         RecordField::from_bytes(input, 90, 1)?,
            communication_distance:       RecordField::from_bytes(input, 91, 2)?,
            transmitter_latitude:         RecordField::from_bytes(input, 93, 9)?,
            transmitter_longitude:        RecordField::from_bytes(input, 102, 10)?,
            service_indicator:            RecordField::from_bytes(input, 112, 3)?,
            modulation:                   RecordField::from_bytes(input, 115, 1)?,
            signal_emission:              RecordField::from_bytes(input, 116, 2)?,
            time_code:                    RecordField::from_bytes(input, 117, 1)?,
            notam:                        RecordField::from_bytes(input, 118, 1)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

// 4.2.5.2 Heliport Communications Primary Extension Continuation Record
#[derive(Debug)]
pub struct HeliportCommsPrimaryExtensionContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub communications_class: RecordField<'a, CommunicationsClass>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub remote_facility: RecordField<'a, RemoteFacility>,
    pub remote_facility_icao_code: RecordField<'a, IcaoCode>,
    pub remote_facility_section: RecordField<'a, Section>,
    pub remote_facility_subsection: RecordField<'a, GenericSubsection>,
    pub transmitter_site_mag_variation: RecordField<'a, MagneticVariation>,
    pub transmitter_site_elevation: RecordField<'a, FacilityElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HeliportCommsPrimaryExtensionContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:               RecordField::from_bytes(input, 2, 3)?,
            section:                          RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:               RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                RecordField::from_bytes(input, 11, 2)?,
            subsection:                       RecordField::from_bytes(input, 13, 1)?,
            communications_class:             RecordField::from_bytes(input, 16, 4)?,
            sequence_number:                  RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:       RecordField::from_bytes(input, 22, 1)?,
            application_type:                 RecordField::from_bytes(input, 23, 1)?,
            remote_facility:                  RecordField::from_bytes(input, 24, 4)?,
            remote_facility_icao_code:        RecordField::from_bytes(input, 28, 2)?,
            remote_facility_section:          RecordField::from_bytes(input, 30, 1)?,
            remote_facility_subsection:       RecordField::from_bytes(input, 31, 1)?,
            transmitter_site_mag_variation:   RecordField::from_bytes(input, 32, 5)?,
            transmitter_site_elevation:       RecordField::from_bytes(input, 37, 5)?,
            file_record_number:               RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

// 4.2.5.3 Heliport Communications Sector Narrative Continuation Record
#[derive(Debug)]
pub struct HeliportCommsSectorNarrativeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub communications_class: RecordField<'a, CommunicationsClass>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub sectorization_narrative: RecordField<'a, SectorizationNarrative>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HeliportCommsSectorNarrativeContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:               RecordField::from_bytes(input, 2, 3)?,
            section:                          RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:               RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                RecordField::from_bytes(input, 11, 2)?,
            subsection:                       RecordField::from_bytes(input, 13, 1)?,
            communications_class:             RecordField::from_bytes(input, 16, 4)?,
            sequence_number:                  RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:       RecordField::from_bytes(input, 22, 1)?,
            application_type:                 RecordField::from_bytes(input, 23, 1)?,
            sectorization_narrative:          RecordField::from_bytes(input, 24, 60)?,
            file_record_number:               RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

// 4.2.5.6 Heliport Communications Additional Sectorization Continuation Record
#[derive(Debug)]
pub struct HeliportCommsAdditionalSectorizationContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub communications_class: RecordField<'a, CommunicationsClass>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub sectorization_1: RecordField<'a, CommunicationsSectorization>,
    pub sectorization_1_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub sectorization_1_altitude_1: RecordField<'a, CommunicationsAltitude>,
    pub sectorization_1_altitude_2: RecordField<'a, CommunicationsAltitude>,
    pub sectorization_2: RecordField<'a, CommunicationsSectorization>,
    pub sectorization_2_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub sectorization_2_altitude_1: RecordField<'a, CommunicationsAltitude>,
    pub sectorization_2_altitude_2: RecordField<'a, CommunicationsAltitude>,
    pub sectorization_3: RecordField<'a, CommunicationsSectorization>,
    pub sectorization_3_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub sectorization_3_altitude_1: RecordField<'a, CommunicationsAltitude>,
    pub sectorization_3_altitude_2: RecordField<'a, CommunicationsAltitude>,
    pub sectorization_4: RecordField<'a, CommunicationsSectorization>,
    pub sectorization_4_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub sectorization_4_altitude_1: RecordField<'a, CommunicationsAltitude>,
    pub sectorization_4_altitude_2: RecordField<'a, CommunicationsAltitude>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HeliportCommsAdditionalSectorizationContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                     RecordField::from_bytes(input, 2, 3)?,
            section:                                RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                     RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                      RecordField::from_bytes(input, 11, 2)?,
            subsection:                             RecordField::from_bytes(input, 13, 1)?,
            communications_class:                   RecordField::from_bytes(input, 16, 4)?,
            sequence_number:                        RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:             RecordField::from_bytes(input, 22, 1)?,
            application_type:                       RecordField::from_bytes(input, 23, 1)?,
            sectorization_1:                        RecordField::from_bytes(input, 24, 6)?,
            sectorization_1_altitude_description:   RecordField::from_bytes(input, 30, 1)?,
            sectorization_1_altitude_1:             RecordField::from_bytes(input, 31, 3)?,
            sectorization_1_altitude_2:             RecordField::from_bytes(input, 34, 3)?,
            sectorization_2:                        RecordField::from_bytes(input, 37, 6)?,
            sectorization_2_altitude_description:   RecordField::from_bytes(input, 43, 1)?,
            sectorization_2_altitude_1:             RecordField::from_bytes(input, 44, 3)?,
            sectorization_2_altitude_2:             RecordField::from_bytes(input, 47, 3)?,
            sectorization_3:                        RecordField::from_bytes(input, 50, 6)?,
            sectorization_3_altitude_description:   RecordField::from_bytes(input, 56, 1)?,
            sectorization_3_altitude_1:             RecordField::from_bytes(input, 57, 3)?,
            sectorization_3_altitude_2:             RecordField::from_bytes(input, 60, 3)?,
            sectorization_4:                        RecordField::from_bytes(input, 63, 6)?,
            sectorization_4_altitude_description:   RecordField::from_bytes(input, 69, 1)?,
            sectorization_4_altitude_1:             RecordField::from_bytes(input, 70, 3)?,
            sectorization_4_altitude_2:             RecordField::from_bytes(input, 73, 3)?,
            file_record_number:                     RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                             RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
