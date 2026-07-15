use crate::parsers::arinc424::rev23::records::record::ARINCRecord;

use crate::parsers::arinc424::rev23::definitions::*;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{
    Arinc424RecordSpec, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct EnrouteAirwayRestrictionRecords;
impl EnrouteAirwayRestrictionRecords {
    const CONTINUATION_COLUMN: usize = 18;
    const CONTINUATION_APPLICATION_COLUMN: usize = 19;
    const RESTRICTION_TYPE_COLUMN: usize = 16;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        match RestrictionRecordType::from_bytes(
            &input[Self::RESTRICTION_TYPE_COLUMN - 1..Self::RESTRICTION_TYPE_COLUMN + 1],
        )? {
            Some(RestrictionRecordType::AltitudeExclusion) => {
                if is_primary_record(input, Self::CONTINUATION_COLUMN) {
                    Ok(
                        ARINCRecord::EnrouteAirwayRestrictionAltitudeExclusionPrimary(
                            EnrouteAirwayRestrictionAltitudeExclusionPrimaryRecord::parse(input)?,
                        ),
                    )
                } else {
                    match ContinuationRecordApplicationType::from_bytes(
                        &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                            ..Self::CONTINUATION_APPLICATION_COLUMN],
                    )? {
                        Some(ContinuationRecordApplicationType::PrimaryRecordExtension) => {
                            Ok(ARINCRecord::EnrouteAirwayRestrictionAltitudeExclusionPrimaryExtensionContinuation(
                                EnrouteAirwayRestrictionAltitudeExclusionPrimaryExtensionContinuationRecord::parse(input)?,
                            ))
                        }
                        Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                            Ok(ARINCRecord::EnrouteAirwayRestrictionAltitudeExclusionFormattedTimeContinuation(
                                EnrouteAirwayRestrictionAltitudeExclusionFormattedTimeContinuationRecord::parse(input)?,
                            ))
                        }
                        Some(ContinuationRecordApplicationType::NarrativeTimeOfOperationsContinuation) => {
                            Ok(ARINCRecord::EnrouteAirwayRestrictionAltitudeExclusionNarrativeTimeContinuation(
                                EnrouteAirwayRestrictionAltitudeExclusionNarrativeTimeContinuationRecord::parse(input)?,
                            ))
                        }
                        _ => Err(RecordParseError::new("Invalid continuation record application type".to_string(), Some(String::from_utf8_lossy(input).into_owned()))),
                    }
                }
            }
            Some(RestrictionRecordType::NoteRestriction) => {
                if is_primary_record(input, Self::CONTINUATION_COLUMN) {
                    Ok(ARINCRecord::EnrouteAirwayRestrictionNoteRestrictionPrimary(
                        EnrouteAirwayRestrictionNoteRestrictionPrimaryRecord::parse(input)?,
                    ))
                } else {
                    match ContinuationRecordApplicationType::from_bytes(
                        &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                            ..Self::CONTINUATION_APPLICATION_COLUMN],
                    )? {
                        Some(ContinuationRecordApplicationType::StandardContinuation) => Ok(
                            ARINCRecord::EnrouteAirwayRestrictionNoteRestrictionContinuation(
                                EnrouteAirwayRestrictionNoteRestrictionContinuationRecord::parse(
                                    input,
                                )?,
                            ),
                        ),
                        _ => Err(RecordParseError::new(
                            "Invalid continuation record application type".to_string(),
                            Some(String::from_utf8_lossy(input).into_owned()),
                        )),
                    }
                }
            }
            Some(RestrictionRecordType::SeasonalRestriction) => {
                if is_primary_record(input, Self::CONTINUATION_COLUMN) {
                    Ok(ARINCRecord::EnrouteAirwayRestrictionSeasonalClosurePrimary(
                        EnrouteAirwayRestrictionSeasonalClosurePrimaryRecord::parse(input)?,
                    ))
                } else {
                    match ContinuationRecordApplicationType::from_bytes(
                        &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                            ..Self::CONTINUATION_APPLICATION_COLUMN],
                    )? {
                        Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                            Ok(ARINCRecord::EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuation(
                                EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuationRecord::parse(input)?,
                            ))
                        }
                        Some(ContinuationRecordApplicationType::NarrativeTimeOfOperationsContinuation) => {
                            Ok(ARINCRecord::EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuation(
                                EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuationRecord::parse(input)?,
                            ))
                        }
                        _ => Err(RecordParseError::new("Invalid continuation record application type".to_string(), Some(String::from_utf8_lossy(input).into_owned())))
                    }
                }
            }
            Some(RestrictionRecordType::CruisingTableReplacement) => {
                if is_primary_record(input, Self::CONTINUATION_COLUMN) {
                    Ok(
                        ARINCRecord::EnrouteAirwayRestrictionCruisingTableReplacementPrimary(
                            EnrouteAirwayRestrictionCruisingTableReplacementPrimaryRecord::parse(
                                input,
                            )?,
                        ),
                    )
                } else {
                    match ContinuationRecordApplicationType::from_bytes(
                        &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                            ..Self::CONTINUATION_APPLICATION_COLUMN],
                    )? {
                        Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                            Ok(ARINCRecord::EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuation(
                                EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuationRecord::parse(input)?,
                            ))
                        }
                        Some(ContinuationRecordApplicationType::NarrativeTimeOfOperationsContinuation) => {
                            Ok(ARINCRecord::EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuation(
                                EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuationRecord::parse(input)?,
                            ))
                        }
                        _ => Err(RecordParseError::new("Invalid continuation record application type".to_string(), Some(String::from_utf8_lossy(input).into_owned()))),
                    }
                }
            }
            _ => Err(RecordParseError::new(
                "Invalid restriction record type".to_string(),
                Some(String::from_utf8_lossy(input).into_owned()),
            )),
        }
    }
}

/// 4.1.21.A.1 Enroute Airway Restriction Altitude Exclusion Primary Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionAltitudeExclusionPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub start_fix_identifier: RecordField<'a, FixIdentifier>,
    pub start_fix_icao_code: RecordField<'a, IcaoCode>,
    pub start_fix_section_code: RecordField<'a, Section>,
    pub start_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub end_fix_identifier: RecordField<'a, FixIdentifier>,
    pub end_fix_icao_code: RecordField<'a, IcaoCode>,
    pub end_fix_section_code: RecordField<'a, Section>,
    pub end_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub start_date: RecordField<'a, AirwayRestrictionStartEndDate>,
    pub end_date: RecordField<'a, AirwayRestrictionStartEndDate>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub exclusion_indicator: RecordField<'a, AltitudeExclusionIndicator>,
    pub units_of_altitude: RecordField<'a, AirwayRestrictionAltitudeUnit>,
    pub restriction_altitude_1: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_1_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_2: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_2_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_3: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_3_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_4: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_4_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_5: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_5_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_6: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_6_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_7: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_7_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionAltitudeExclusionPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionAltitudeExclusionPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            subsection:                               RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                         RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:                   RecordField::from_bytes(input, 13, 3)?,
            restriction_type:                         RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:               RecordField::from_bytes(input, 18, 1)?,
            start_fix_identifier:                     RecordField::from_bytes(input, 19, 5)?,
            start_fix_icao_code:                      RecordField::from_bytes(input, 24, 2)?,
            start_fix_section_code:                   RecordField::from_bytes(input, 26, 1)?,
            start_fix_subsection_code:                RecordField::from_bytes(input, 27, 1)?,
            end_fix_identifier:                       RecordField::from_bytes(input, 28, 5)?,
            end_fix_icao_code:                        RecordField::from_bytes(input, 33, 2)?,
            end_fix_section_code:                     RecordField::from_bytes(input, 35, 1)?,
            end_fix_subsection_code:                  RecordField::from_bytes(input, 36, 1)?,
            start_date:                               RecordField::from_bytes(input, 38, 7)?,
            end_date:                                 RecordField::from_bytes(input, 45, 7)?,
            time_code:                                RecordField::from_bytes(input, 52, 1)?,
            exclusion_indicator:                      RecordField::from_bytes(input, 94, 1)?,
            units_of_altitude:                        RecordField::from_bytes(input, 95, 1)?,
            restriction_altitude_1:                   RecordField::from_bytes(input, 96, 3)?,
            restriction_altitude_1_block_indicator:   RecordField::from_bytes(input, 99, 1)?,
            restriction_altitude_2:                   RecordField::from_bytes(input, 100, 3)?,
            restriction_altitude_2_block_indicator:   RecordField::from_bytes(input, 103, 1)?,
            restriction_altitude_3:                   RecordField::from_bytes(input, 104, 3)?,
            restriction_altitude_3_block_indicator:   RecordField::from_bytes(input, 107, 1)?,
            restriction_altitude_4:                   RecordField::from_bytes(input, 108, 3)?,
            restriction_altitude_4_block_indicator:   RecordField::from_bytes(input, 111, 1)?,
            restriction_altitude_5:                   RecordField::from_bytes(input, 112, 3)?,
            restriction_altitude_5_block_indicator:   RecordField::from_bytes(input, 115, 1)?,
            restriction_altitude_6:                   RecordField::from_bytes(input, 116, 3)?,
            restriction_altitude_6_block_indicator:   RecordField::from_bytes(input, 119, 1)?,
            restriction_altitude_7:                   RecordField::from_bytes(input, 120, 3)?,
            restriction_altitude_7_block_indicator:   RecordField::from_bytes(input, 123, 1)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                               RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.A.2 Enroute Airway Restriction Altitude Exclusion Primary Extension Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionAltitudeExclusionPrimaryExtensionContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub restriction_altitude_1: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_1_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_2: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_2_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_3: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_3_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_4: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_4_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_5: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_5_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_6: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_6_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub restriction_altitude_7: RecordField<'a, AirwayRestrictionAltitude>,
    pub restriction_altitude_7_block_indicator: RecordField<'a, BlockAltitudeIndicator>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionAltitudeExclusionPrimaryExtensionContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionAltitudeExclusionPrimaryExtensionContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            subsection:                               RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                         RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:                   RecordField::from_bytes(input, 13, 3)?,
            restriction_type:                         RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:               RecordField::from_bytes(input, 18, 1)?,
            application_type:                         RecordField::from_bytes(input, 19, 1)?,
            restriction_altitude_1:                   RecordField::from_bytes(input, 96, 3)?,
            restriction_altitude_1_block_indicator:   RecordField::from_bytes(input, 99, 1)?,
            restriction_altitude_2:                   RecordField::from_bytes(input, 100, 3)?,
            restriction_altitude_2_block_indicator:   RecordField::from_bytes(input, 103, 1)?,
            restriction_altitude_3:                   RecordField::from_bytes(input, 104, 3)?,
            restriction_altitude_3_block_indicator:   RecordField::from_bytes(input, 107, 1)?,
            restriction_altitude_4:                   RecordField::from_bytes(input, 108, 3)?,
            restriction_altitude_4_block_indicator:   RecordField::from_bytes(input, 111, 1)?,
            restriction_altitude_5:                   RecordField::from_bytes(input, 112, 3)?,
            restriction_altitude_5_block_indicator:   RecordField::from_bytes(input, 115, 1)?,
            restriction_altitude_6:                   RecordField::from_bytes(input, 116, 3)?,
            restriction_altitude_6_block_indicator:   RecordField::from_bytes(input, 119, 1)?,
            restriction_altitude_7:                   RecordField::from_bytes(input, 120, 3)?,
            restriction_altitude_7_block_indicator:   RecordField::from_bytes(input, 123, 1)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                               RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.A.3 Enroute Airway Restriction Altitude Exclusion Formatted Time Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionAltitudeExclusionFormattedTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_code: RecordField<'a, ContinuationRecordTimeCode>,
    pub time_indicator: RecordField<'a, TimeIndicator>,
    pub time_of_operation_1: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_2: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_3: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_4: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_5: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_6: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_7: RecordField<'a, TimeOfOperation>,
    pub timezone: RecordField<'a, Timezone>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionAltitudeExclusionFormattedTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionAltitudeExclusionFormattedTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_identifier:             RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:       RecordField::from_bytes(input, 13, 3)?,
            restriction_type:             RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 18, 1)?,
            application_type:             RecordField::from_bytes(input, 19, 1)?,
            time_code:                    RecordField::from_bytes(input, 20, 1)?,
            time_indicator:               RecordField::from_bytes(input, 22, 1)?,
            time_of_operation_1:          RecordField::from_bytes(input, 23, 10)?,
            time_of_operation_2:          RecordField::from_bytes(input, 33, 10)?,
            time_of_operation_3:          RecordField::from_bytes(input, 43, 10)?,
            time_of_operation_4:          RecordField::from_bytes(input, 53, 10)?,
            time_of_operation_5:          RecordField::from_bytes(input, 63, 10)?,
            time_of_operation_6:          RecordField::from_bytes(input, 73, 10)?,
            time_of_operation_7:          RecordField::from_bytes(input, 83, 10)?,
            timezone:                     RecordField::from_bytes(input, 93, 3)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.A.4 Enroute Airway Restriction Altitude Exclusion Narrative Time Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionAltitudeExclusionNarrativeTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub narrative_time: RecordField<'a, TimeNarrative>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionAltitudeExclusionNarrativeTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionAltitudeExclusionNarrativeTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_identifier:             RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:       RecordField::from_bytes(input, 13, 3)?,
            restriction_type:             RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 18, 1)?,
            application_type:             RecordField::from_bytes(input, 19, 1)?,
            narrative_time:               RecordField::from_bytes(input, 23, 97)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.B.1 Enroute Airway Restriction Note Restriction Primary Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionNoteRestrictionPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub start_fix_identifier: RecordField<'a, FixIdentifier>,
    pub start_fix_icao_code: RecordField<'a, IcaoCode>,
    pub start_fix_section_code: RecordField<'a, Section>,
    pub start_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub end_fix_identifier: RecordField<'a, FixIdentifier>,
    pub end_fix_icao_code: RecordField<'a, IcaoCode>,
    pub end_fix_section_code: RecordField<'a, Section>,
    pub end_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub start_date: RecordField<'a, AirwayRestrictionStartEndDate>,
    pub end_date: RecordField<'a, AirwayRestrictionStartEndDate>,
    pub restriction_notes: RecordField<'a, AirwayRestrictionNotes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionNoteRestrictionPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionNoteRestrictionPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            subsection:                               RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                         RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:                   RecordField::from_bytes(input, 13, 3)?,
            restriction_type:                         RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:               RecordField::from_bytes(input, 18, 1)?,
            start_fix_identifier:                     RecordField::from_bytes(input, 19, 5)?,
            start_fix_icao_code:                      RecordField::from_bytes(input, 24, 2)?,
            start_fix_section_code:                   RecordField::from_bytes(input, 26, 1)?,
            start_fix_subsection_code:                RecordField::from_bytes(input, 27, 1)?,
            end_fix_identifier:                       RecordField::from_bytes(input, 28, 5)?,
            end_fix_icao_code:                        RecordField::from_bytes(input, 33, 2)?,
            end_fix_section_code:                     RecordField::from_bytes(input, 35, 1)?,
            end_fix_subsection_code:                  RecordField::from_bytes(input, 36, 1)?,
            start_date:                               RecordField::from_bytes(input, 38, 7)?,
            end_date:                                 RecordField::from_bytes(input, 45, 7)?,
            restriction_notes:                        RecordField::from_bytes(input, 52, 69)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                               RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.B.2 Enroute Airway Restriction Note Restriction Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionNoteRestrictionContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub restriction_notes: RecordField<'a, AirwayRestrictionNotes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionNoteRestrictionContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionNoteRestrictionContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            subsection:                               RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                         RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:                   RecordField::from_bytes(input, 13, 3)?,
            restriction_type:                         RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:               RecordField::from_bytes(input, 18, 1)?,
            restriction_notes:                        RecordField::from_bytes(input, 52, 69)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                               RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.C.1 Enroute Airway Restriction Seasonal Closure Primary Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionSeasonalClosurePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub start_fix_identifier: RecordField<'a, FixIdentifier>,
    pub start_fix_icao_code: RecordField<'a, IcaoCode>,
    pub start_fix_section_code: RecordField<'a, Section>,
    pub start_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub end_fix_identifier: RecordField<'a, FixIdentifier>,
    pub end_fix_icao_code: RecordField<'a, IcaoCode>,
    pub end_fix_section_code: RecordField<'a, Section>,
    pub end_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub start_date: RecordField<'a, AirwayRestrictionStartEndDate>,
    pub end_date: RecordField<'a, AirwayRestrictionStartEndDate>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionSeasonalClosurePrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionSeasonalClosurePrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            subsection:                               RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                         RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:                   RecordField::from_bytes(input, 13, 3)?,
            restriction_type:                         RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:               RecordField::from_bytes(input, 18, 1)?,
            start_fix_identifier:                     RecordField::from_bytes(input, 19, 5)?,
            start_fix_icao_code:                      RecordField::from_bytes(input, 24, 2)?,
            start_fix_section_code:                   RecordField::from_bytes(input, 26, 1)?,
            start_fix_subsection_code:                RecordField::from_bytes(input, 27, 1)?,
            end_fix_identifier:                       RecordField::from_bytes(input, 28, 5)?,
            end_fix_icao_code:                        RecordField::from_bytes(input, 33, 2)?,
            end_fix_section_code:                     RecordField::from_bytes(input, 35, 1)?,
            end_fix_subsection_code:                  RecordField::from_bytes(input, 36, 1)?,
            start_date:                               RecordField::from_bytes(input, 38, 7)?,
            end_date:                                 RecordField::from_bytes(input, 45, 7)?,
            time_code:                                RecordField::from_bytes(input, 52, 1)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                               RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.C.3 Enroute Airway Restriction Seasonal Closure Formatted Time Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_code: RecordField<'a, ContinuationRecordTimeCode>,
    pub time_indicator: RecordField<'a, TimeIndicator>,
    pub time_of_operation_1: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_2: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_3: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_4: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_5: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_6: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_7: RecordField<'a, TimeOfOperation>,
    pub timezone: RecordField<'a, Timezone>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_identifier:             RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:       RecordField::from_bytes(input, 13, 3)?,
            restriction_type:             RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 18, 1)?,
            application_type:             RecordField::from_bytes(input, 19, 1)?,
            time_code:                    RecordField::from_bytes(input, 20, 1)?,
            time_indicator:               RecordField::from_bytes(input, 22, 1)?,
            time_of_operation_1:          RecordField::from_bytes(input, 23, 10)?,
            time_of_operation_2:          RecordField::from_bytes(input, 33, 10)?,
            time_of_operation_3:          RecordField::from_bytes(input, 43, 10)?,
            time_of_operation_4:          RecordField::from_bytes(input, 53, 10)?,
            time_of_operation_5:          RecordField::from_bytes(input, 63, 10)?,
            time_of_operation_6:          RecordField::from_bytes(input, 73, 10)?,
            time_of_operation_7:          RecordField::from_bytes(input, 83, 10)?,
            timezone:                     RecordField::from_bytes(input, 93, 3)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.C.4 Enroute Airway Restriction Seasonal Closure Narrative Time Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub narrative_time: RecordField<'a, TimeNarrative>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_identifier:             RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:       RecordField::from_bytes(input, 13, 3)?,
            restriction_type:             RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 18, 1)?,
            application_type:             RecordField::from_bytes(input, 19, 1)?,
            narrative_time:               RecordField::from_bytes(input, 23, 97)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.D.1 Enroute Airway Restriction Cruising Table Replacement Primary Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionCruisingTableReplacementPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub start_fix_identifier: RecordField<'a, FixIdentifier>,
    pub start_fix_icao_code: RecordField<'a, IcaoCode>,
    pub start_fix_section_code: RecordField<'a, Section>,
    pub start_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub end_fix_identifier: RecordField<'a, FixIdentifier>,
    pub end_fix_icao_code: RecordField<'a, IcaoCode>,
    pub end_fix_section_code: RecordField<'a, Section>,
    pub end_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub start_date: RecordField<'a, AirwayRestrictionStartEndDate>,
    pub end_date: RecordField<'a, AirwayRestrictionStartEndDate>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub cruising_table_identifier: RecordField<'a, CruiseTableIdentifier>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionCruisingTableReplacementPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionCruisingTableReplacementPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            subsection:                               RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                         RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:                   RecordField::from_bytes(input, 13, 3)?,
            restriction_type:                         RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:               RecordField::from_bytes(input, 18, 1)?,
            start_fix_identifier:                     RecordField::from_bytes(input, 19, 5)?,
            start_fix_icao_code:                      RecordField::from_bytes(input, 24, 2)?,
            start_fix_section_code:                   RecordField::from_bytes(input, 26, 1)?,
            start_fix_subsection_code:                RecordField::from_bytes(input, 27, 1)?,
            end_fix_identifier:                       RecordField::from_bytes(input, 28, 5)?,
            end_fix_icao_code:                        RecordField::from_bytes(input, 33, 2)?,
            end_fix_section_code:                     RecordField::from_bytes(input, 35, 1)?,
            end_fix_subsection_code:                  RecordField::from_bytes(input, 36, 1)?,
            start_date:                               RecordField::from_bytes(input, 38, 7)?,
            end_date:                                 RecordField::from_bytes(input, 45, 7)?,
            time_code:                                RecordField::from_bytes(input, 52, 1)?,
            cruising_table_identifier:                RecordField::from_bytes(input, 94, 2)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                               RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.D.2 Enroute Airway Restriction Cruising Table Replacement Formatted Time Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_code: RecordField<'a, ContinuationRecordTimeCode>,
    pub time_indicator: RecordField<'a, TimeIndicator>,
    pub time_of_operation_1: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_2: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_3: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_4: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_5: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_6: RecordField<'a, TimeOfOperation>,
    pub time_of_operation_7: RecordField<'a, TimeOfOperation>,
    pub timezone: RecordField<'a, Timezone>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_identifier:             RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:       RecordField::from_bytes(input, 13, 3)?,
            restriction_type:             RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 18, 1)?,
            application_type:             RecordField::from_bytes(input, 19, 1)?,
            time_code:                    RecordField::from_bytes(input, 20, 1)?,
            time_indicator:               RecordField::from_bytes(input, 22, 1)?,
            time_of_operation_1:          RecordField::from_bytes(input, 23, 10)?,
            time_of_operation_2:          RecordField::from_bytes(input, 33, 10)?,
            time_of_operation_3:          RecordField::from_bytes(input, 43, 10)?,
            time_of_operation_4:          RecordField::from_bytes(input, 53, 10)?,
            time_of_operation_5:          RecordField::from_bytes(input, 63, 10)?,
            time_of_operation_6:          RecordField::from_bytes(input, 73, 10)?,
            time_of_operation_7:          RecordField::from_bytes(input, 83, 10)?,
            timezone:                     RecordField::from_bytes(input, 93, 3)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.1.21.D.3 Enroute Airway Restriction Cruising Table Replacement Narrative Time Continuation Record
#[derive(Debug)]
pub struct EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub restriction_identifier: RecordField<'a, AirwayRestrictionIdentifier>,
    pub restriction_type: RecordField<'a, RestrictionRecordType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub narrative_time: RecordField<'a, TimeNarrative>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_identifier:             RecordField::from_bytes(input, 7, 5)?,
            restriction_identifier:       RecordField::from_bytes(input, 13, 3)?,
            restriction_type:             RecordField::from_bytes(input, 16, 2)?,
            continuation_record_number:   RecordField::from_bytes(input, 18, 1)?,
            application_type:             RecordField::from_bytes(input, 19, 1)?,
            narrative_time:               RecordField::from_bytes(input, 23, 97)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}
