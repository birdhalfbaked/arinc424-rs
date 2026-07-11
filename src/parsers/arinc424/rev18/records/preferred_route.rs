
use crate::parsers::arinc424::rev18::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
use crate::parsers::arinc424::rev18::definitions::*;
pub(super) struct PreferredRouteRecords;
impl PreferredRouteRecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;
    const VIA_COLUMN: usize = 49;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            match PreferredRouteVIACode::from_bytes(
                &input[Self::VIA_COLUMN - 1..Self::VIA_COLUMN + 3],
            )? {
                Some(PreferredRouteVIACode::SID) => Ok(ARINCRecord::PreferredSIDRoutePrimary(
                    PreferredSIDRoutePrimaryRecord::parse(input)?,
                )),
                Some(PreferredRouteVIACode::STARProfileDescent) => {
                    Ok(ARINCRecord::PreferredSTARRoutePrimary(
                        PreferredSTARRoutePrimaryRecord::parse(input)?,
                    ))
                }
                Some(PreferredRouteVIACode::DesignatedAirway) => {
                    Ok(ARINCRecord::PreferredAirwayRoutePrimary(
                        PreferredAirwayRoutePrimaryRecord::parse(input)?,
                    ))
                }
                Some(_) => Ok(ARINCRecord::PreferredGeneralRoutePrimary(
                    PreferredGeneralRoutePrimaryRecord::parse(input)?,
                )),
                None => Err(RecordParseError {
                    message: "Invalid preferred route VIA code".to_string(),
                }),
            }
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation) => {
                    Ok(ARINCRecord::PreferredRouteFormattedTimeContinuation(
                        PreferredRouteFormattedTimeContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::PreferredRouteContinuation(
                        PreferredRouteContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::NarrativeTimeOfOperationsContinuation) => {
                    Ok(ARINCRecord::PreferredRouteNarrativeTimeContinuation(
                        PreferredRouteNarrativeTimeContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError {
                    message: "Invalid continuation record application type".to_string(),
                }),
            }
        }
    }
}

/// 4.1.24.1(A) Preferred SID Route Primary Record
#[derive(Debug)]
pub struct PreferredSIDRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub to_fix_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section_code: RecordField<'a, Section>,
    pub to_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub sid_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub level: RecordField<'a, Level>,
    pub route_type: RecordField<'a, PreferredRouteRouteType>,
    pub initial_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub initial_point_icao_code: RecordField<'a, IcaoCode>,
    pub initial_point_section_code: RecordField<'a, Section>,
    pub initial_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub terminus_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub terminus_point_icao_code: RecordField<'a, IcaoCode>,
    pub terminus_point_section_code: RecordField<'a, Section>,
    pub terminus_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub minimum_altitude: RecordField<'a, MinimumAltitude>,
    pub maximum_altitude: RecordField<'a, MaximumAltitude>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub aircraft_use_group: RecordField<'a, AircraftUseGroupIndicator>,
    pub direction_restriction: RecordField<'a, PreferredRouteDirectionalRestriction>,
    pub alitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_1: RecordField<'a, MinimumAltitude>,
    pub altitude_2: RecordField<'a, MinimumAltitude>,
    pub sid_route_type: RecordField<'a, SIDRouteType>,
    pub sid_route_type_qualifier_1: RecordField<'a, AirportHeliportSIDRouteTypeQualifier1>,
    pub sid_route_type_qualifier_2: RecordField<'a, AirportHeliportSIDRouteTypeQualifier2>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> PreferredSIDRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredSIDRoutePrimaryRecord {
            record_type:                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:               RecordField::from_bytes(input, 2, 3)?,
            section:                          RecordField::from_bytes(input, 5, 1)?,
            subsection:                       RecordField::from_bytes(input, 6, 1)?,
            route_id:                         RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:              RecordField::from_bytes(input, 24, 2)?,
            sequence_number:                  RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:       RecordField::from_bytes(input, 39, 1)?,
            to_fix_identifier:                RecordField::from_bytes(input, 40, 5)?,
            to_fix_icao_code:                 RecordField::from_bytes(input, 45, 2)?,
            to_fix_section_code:              RecordField::from_bytes(input, 47, 1)?,
            to_fix_subsection_code:           RecordField::from_bytes(input, 48, 1)?,
            via_code:                         RecordField::from_bytes(input, 49, 3)?,
            sid_identifier:                   RecordField::from_bytes(input, 52, 6)?,
            area:                             RecordField::from_bytes(input, 58, 3)?,
            level:                            RecordField::from_bytes(input, 61, 1)?,
            route_type:                       RecordField::from_bytes(input, 62, 1)?,
            initial_point:                    RecordField::from_bytes(input, 63, 5)?,
            initial_point_icao_code:          RecordField::from_bytes(input, 68, 2)?,
            initial_point_section_code:       RecordField::from_bytes(input, 70, 1)?,
            initial_point_subsection_code:    RecordField::from_bytes(input, 71, 1)?,
            terminus_point:                   RecordField::from_bytes(input, 72, 5)?,
            terminus_point_icao_code:         RecordField::from_bytes(input, 77, 2)?,
            terminus_point_section_code:      RecordField::from_bytes(input, 79, 1)?,
            terminus_point_subsection_code:   RecordField::from_bytes(input, 80, 1)?,
            minimum_altitude:                 RecordField::from_bytes(input, 81, 5)?,
            maximum_altitude:                 RecordField::from_bytes(input, 86, 5)?,
            time_code:                        RecordField::from_bytes(input, 91, 1)?,
            aircraft_use_group:               RecordField::from_bytes(input, 92, 2)?,
            direction_restriction:            RecordField::from_bytes(input, 94, 1)?,
            alitude_description:              RecordField::from_bytes(input, 95, 1)?,
            altitude_1:                       RecordField::from_bytes(input, 96, 5)?,
            altitude_2:                       RecordField::from_bytes(input, 101, 5)?,
            sid_route_type:                   RecordField::from_bytes(input, 106, 1)?,
            sid_route_type_qualifier_1:       RecordField::from_bytes(input, 107, 1)?,
            sid_route_type_qualifier_2:       RecordField::from_bytes(input, 108, 1)?,
            file_record_number:               RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.24.1(B) Preferred STAR Route Primary Record
#[derive(Debug)]
pub struct PreferredSTARRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub to_fix_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section_code: RecordField<'a, Section>,
    pub to_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub star_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub level: RecordField<'a, Level>,
    pub route_type: RecordField<'a, PreferredRouteRouteType>,
    pub initial_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub initial_point_icao_code: RecordField<'a, IcaoCode>,
    pub initial_point_section_code: RecordField<'a, Section>,
    pub initial_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub terminus_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub terminus_point_icao_code: RecordField<'a, IcaoCode>,
    pub terminus_point_section_code: RecordField<'a, Section>,
    pub terminus_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub minimum_altitude: RecordField<'a, MinimumAltitude>,
    pub maximum_altitude: RecordField<'a, MaximumAltitude>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub aircraft_use_group: RecordField<'a, AircraftUseGroup>,
    pub direction_restriction: RecordField<'a, PreferredRouteDirectionalRestriction>,
    pub alitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_1: RecordField<'a, MinimumAltitude>,
    pub altitude_2: RecordField<'a, MinimumAltitude>,
    pub star_route_type: RecordField<'a, STARRouteType>,
    pub star_route_type_qualifier_1: RecordField<'a, AirportHeliportSTARRouteTypeQualifier1>,
    pub star_route_type_qualifier_2: RecordField<'a, AirportHeliportSTARRouteTypeQualifier2>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> PreferredSTARRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredSTARRoutePrimaryRecord {
            record_type:                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:               RecordField::from_bytes(input, 2, 3)?,
            section:                          RecordField::from_bytes(input, 5, 1)?,
            subsection:                       RecordField::from_bytes(input, 6, 1)?,
            route_id:                         RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:              RecordField::from_bytes(input, 24, 2)?,
            sequence_number:                  RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:       RecordField::from_bytes(input, 39, 1)?,
            to_fix_identifier:                RecordField::from_bytes(input, 40, 5)?,
            to_fix_icao_code:                 RecordField::from_bytes(input, 45, 2)?,
            to_fix_section_code:              RecordField::from_bytes(input, 47, 1)?,
            to_fix_subsection_code:           RecordField::from_bytes(input, 48, 1)?,
            via_code:                         RecordField::from_bytes(input, 49, 3)?,
            star_identifier:                  RecordField::from_bytes(input, 52, 6)?,
            area:                             RecordField::from_bytes(input, 58, 3)?,
            level:                            RecordField::from_bytes(input, 61, 1)?,
            route_type:                       RecordField::from_bytes(input, 62, 1)?,
            initial_point:                    RecordField::from_bytes(input, 63, 5)?,
            initial_point_icao_code:          RecordField::from_bytes(input, 68, 2)?,
            initial_point_section_code:       RecordField::from_bytes(input, 70, 1)?,
            initial_point_subsection_code:    RecordField::from_bytes(input, 71, 1)?,
            terminus_point:                   RecordField::from_bytes(input, 72, 5)?,
            terminus_point_icao_code:         RecordField::from_bytes(input, 77, 2)?,
            terminus_point_section_code:      RecordField::from_bytes(input, 79, 1)?,
            terminus_point_subsection_code:   RecordField::from_bytes(input, 80, 1)?,
            minimum_altitude:                 RecordField::from_bytes(input, 81, 5)?,
            maximum_altitude:                 RecordField::from_bytes(input, 86, 5)?,
            time_code:                        RecordField::from_bytes(input, 91, 1)?,
            aircraft_use_group:               RecordField::from_bytes(input, 92, 2)?,
            direction_restriction:            RecordField::from_bytes(input, 94, 1)?,
            alitude_description:              RecordField::from_bytes(input, 95, 1)?,
            altitude_1:                       RecordField::from_bytes(input, 96, 5)?,
            altitude_2:                       RecordField::from_bytes(input, 101, 5)?,
            star_route_type:                  RecordField::from_bytes(input, 106, 1)?,
            star_route_type_qualifier_1:      RecordField::from_bytes(input, 107, 1)?,
            star_route_type_qualifier_2:      RecordField::from_bytes(input, 108, 1)?,
            file_record_number:               RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.24.1(C) Preferred Airway Route Primary Record
#[derive(Debug)]
pub struct PreferredAirwayRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub to_fix_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section_code: RecordField<'a, Section>,
    pub to_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub airway_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub level: RecordField<'a, Level>,
    pub route_type: RecordField<'a, PreferredRouteRouteType>,
    pub initial_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub initial_point_icao_code: RecordField<'a, IcaoCode>,
    pub initial_point_section_code: RecordField<'a, Section>,
    pub initial_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub terminus_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub terminus_point_icao_code: RecordField<'a, IcaoCode>,
    pub terminus_point_section_code: RecordField<'a, Section>,
    pub terminus_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub minimum_altitude: RecordField<'a, MinimumAltitude>,
    pub maximum_altitude: RecordField<'a, MaximumAltitude>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub aircraft_use_group: RecordField<'a, AircraftUseGroup>,
    pub direction_restriction: RecordField<'a, PreferredRouteDirectionalRestriction>,
    pub alitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_1: RecordField<'a, MinimumAltitude>,
    pub altitude_2: RecordField<'a, MinimumAltitude>,
    pub airway_route_type: RecordField<'a, EnrouteAirwayRouteType>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> PreferredAirwayRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredAirwayRoutePrimaryRecord {
            record_type:                       RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                RecordField::from_bytes(input, 2, 3)?,
            section:                           RecordField::from_bytes(input, 5, 1)?,
            subsection:                        RecordField::from_bytes(input, 6, 1)?,
            route_id:                          RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:               RecordField::from_bytes(input, 24, 2)?,
            sequence_number:                   RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:        RecordField::from_bytes(input, 39, 1)?,
            to_fix_identifier:                 RecordField::from_bytes(input, 40, 5)?,
            to_fix_icao_code:                  RecordField::from_bytes(input, 45, 2)?,
            to_fix_section_code:               RecordField::from_bytes(input, 47, 1)?,
            to_fix_subsection_code:            RecordField::from_bytes(input, 48, 1)?,
            via_code:                          RecordField::from_bytes(input, 49, 3)?,
            airway_identifier:                 RecordField::from_bytes(input, 52, 6)?,
            area:                              RecordField::from_bytes(input, 58, 3)?,
            level:                             RecordField::from_bytes(input, 61, 1)?,
            route_type:                        RecordField::from_bytes(input, 62, 1)?,
            initial_point:                     RecordField::from_bytes(input, 63, 5)?,
            initial_point_icao_code:           RecordField::from_bytes(input, 68, 2)?,
            initial_point_section_code:        RecordField::from_bytes(input, 70, 1)?,
            initial_point_subsection_code:     RecordField::from_bytes(input, 71, 1)?,
            terminus_point:                    RecordField::from_bytes(input, 72, 5)?,
            terminus_point_icao_code:          RecordField::from_bytes(input, 77, 2)?,
            terminus_point_section_code:       RecordField::from_bytes(input, 79, 1)?,
            terminus_point_subsection_code:    RecordField::from_bytes(input, 80, 1)?,
            minimum_altitude:                  RecordField::from_bytes(input, 81, 5)?,
            maximum_altitude:                  RecordField::from_bytes(input, 86, 5)?,
            time_code:                         RecordField::from_bytes(input, 91, 1)?,
            aircraft_use_group:                RecordField::from_bytes(input, 92, 2)?,
            direction_restriction:             RecordField::from_bytes(input, 94, 1)?,
            alitude_description:               RecordField::from_bytes(input, 95, 1)?,
            altitude_1:                        RecordField::from_bytes(input, 96, 5)?,
            altitude_2:                        RecordField::from_bytes(input, 101, 5)?,
            airway_route_type:                 RecordField::from_bytes(input, 106, 1)?,
            file_record_number:                RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                        RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.24.1(D) Preferred General Route Primary Record
#[derive(Debug)]
pub struct PreferredGeneralRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub to_fix_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section_code: RecordField<'a, Section>,
    pub to_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub level: RecordField<'a, Level>,
    pub route_type: RecordField<'a, PreferredRouteRouteType>,
    pub initial_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub initial_point_icao_code: RecordField<'a, IcaoCode>,
    pub initial_point_section_code: RecordField<'a, Section>,
    pub initial_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub terminus_point: RecordField<'a, InitialTerminusFixOrAirport>,
    pub terminus_point_icao_code: RecordField<'a, IcaoCode>,
    pub terminus_point_section_code: RecordField<'a, Section>,
    pub terminus_point_subsection_code: RecordField<'a, GenericSubsection>,
    pub minimum_altitude: RecordField<'a, MinimumAltitude>,
    pub maximum_altitude: RecordField<'a, MaximumAltitude>,
    pub time_code: RecordField<'a, PrimaryRecordTimeCode>,
    pub aircraft_use_group: RecordField<'a, AircraftUseGroup>,
    pub direction_restriction: RecordField<'a, PreferredRouteDirectionalRestriction>,
    pub alitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_1: RecordField<'a, MinimumAltitude>,
    pub altitude_2: RecordField<'a, MinimumAltitude>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> PreferredGeneralRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredGeneralRoutePrimaryRecord {
            record_type:                       RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                RecordField::from_bytes(input, 2, 3)?,
            section:                           RecordField::from_bytes(input, 5, 1)?,
            subsection:                        RecordField::from_bytes(input, 6, 1)?,
            route_id:                          RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:               RecordField::from_bytes(input, 24, 2)?,
            sequence_number:                   RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:        RecordField::from_bytes(input, 39, 1)?,
            to_fix_identifier:                 RecordField::from_bytes(input, 40, 5)?,
            to_fix_icao_code:                  RecordField::from_bytes(input, 45, 2)?,
            to_fix_section_code:               RecordField::from_bytes(input, 47, 1)?,
            to_fix_subsection_code:            RecordField::from_bytes(input, 48, 1)?,
            via_code:                          RecordField::from_bytes(input, 49, 3)?,
            area:                              RecordField::from_bytes(input, 58, 3)?,
            level:                             RecordField::from_bytes(input, 61, 1)?,
            route_type:                        RecordField::from_bytes(input, 62, 1)?,
            initial_point:                     RecordField::from_bytes(input, 63, 5)?,
            initial_point_icao_code:           RecordField::from_bytes(input, 68, 2)?,
            initial_point_section_code:        RecordField::from_bytes(input, 70, 1)?,
            initial_point_subsection_code:     RecordField::from_bytes(input, 71, 1)?,
            terminus_point:                    RecordField::from_bytes(input, 72, 5)?,
            terminus_point_icao_code:          RecordField::from_bytes(input, 77, 2)?,
            terminus_point_section_code:       RecordField::from_bytes(input, 79, 1)?,
            terminus_point_subsection_code:    RecordField::from_bytes(input, 80, 1)?,
            minimum_altitude:                  RecordField::from_bytes(input, 81, 5)?,
            maximum_altitude:                  RecordField::from_bytes(input, 86, 5)?,
            time_code:                         RecordField::from_bytes(input, 91, 1)?,
            aircraft_use_group:                RecordField::from_bytes(input, 92, 2)?,
            direction_restriction:             RecordField::from_bytes(input, 94, 1)?,
            alitude_description:               RecordField::from_bytes(input, 95, 1)?,
            altitude_1:                        RecordField::from_bytes(input, 96, 5)?,
            altitude_2:                        RecordField::from_bytes(input, 101, 5)?,
            file_record_number:                RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                        RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.24.2 Preferred Route Formatted Time Continuation Record
#[derive(Debug)]
pub struct PreferredRouteFormattedTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
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
impl<'a> PreferredRouteFormattedTimeContinuationRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredRouteFormattedTimeContinuationRecord {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_id:                     RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:          RecordField::from_bytes(input, 24, 2)?,
            sequence_number:              RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:   RecordField::from_bytes(input, 39, 1)?,
            application_type:             RecordField::from_bytes(input, 40, 1)?,
            time_code:                    RecordField::from_bytes(input, 41, 1)?,
            time_indicator:               RecordField::from_bytes(input, 42, 1)?,
            time_of_operation_1:          RecordField::from_bytes(input, 43, 10)?,
            time_of_operation_2:          RecordField::from_bytes(input, 53, 10)?,
            time_of_operation_3:          RecordField::from_bytes(input, 63, 10)?,
            time_of_operation_4:          RecordField::from_bytes(input, 73, 10)?,
            time_of_operation_5:          RecordField::from_bytes(input, 83, 10)?,
            time_of_operation_6:          RecordField::from_bytes(input, 93, 10)?,
            time_of_operation_7:          RecordField::from_bytes(input, 103, 10)?,
            timezone:                     RecordField::from_bytes(input, 113, 3)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.24.2 Preferred Route Continuation Record
#[derive(Debug)]
pub struct PreferredRouteContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> PreferredRouteContinuationRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredRouteContinuationRecord {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_id:                     RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:          RecordField::from_bytes(input, 24, 2)?,
            sequence_number:              RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:   RecordField::from_bytes(input, 39, 1)?,
            application_type:             RecordField::from_bytes(input, 40, 1)?,
            notes:                        RecordField::from_bytes(input, 41, 69)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.24.4 Preferred Route Narrative Time Continuation Record
#[derive(Debug)]
pub struct PreferredRouteNarrativeTimeContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub route_id: RecordField<'a, PreferredRouteIdentifier>,
    pub route_use_indicator: RecordField<'a, PreferredRouteUseIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub time_narrative: RecordField<'a, TimeNarrative>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> PreferredRouteNarrativeTimeContinuationRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(PreferredRouteNarrativeTimeContinuationRecord {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            route_id:                     RecordField::from_bytes(input, 14, 10)?,
            route_use_indicator:          RecordField::from_bytes(input, 24, 2)?,
            sequence_number:              RecordField::from_bytes(input, 26, 4)?,
            continuation_record_number:   RecordField::from_bytes(input, 39, 1)?,
            application_type:             RecordField::from_bytes(input, 40, 1)?,
            time_narrative:               RecordField::from_bytes(input, 41, 83)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
