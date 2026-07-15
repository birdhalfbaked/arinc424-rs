use crate::parsers::arinc424::rev23::definitions::*;

use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{
    Arinc424RecordSpec, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct EnrouteAirwayRecords;
impl EnrouteAirwayRecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::EnrouteAirwayPrimary(
                EnrouteAirwayPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::EnrouteAirwayContinuation(
                        EnrouteAirwayContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::FlightPlanningContinuation) => {
                    Ok(ARINCRecord::EnrouteAirwayFlightPlanningContinuation(
                        EnrouteAirwayFlightPlanningContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError::new(
                    "Invalid continuation record application type".to_string(),
                    Some(String::from_utf8_lossy(input).into_owned()),
                )),
            }
        }
    }
}

#[derive(Debug)]
pub struct EnrouteAirwayPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub waypoint_description_code: RecordField<'a, WaypointDescriptionCode>,
    pub boundary_code: RecordField<'a, BoundaryCode>,
    pub route_type: RecordField<'a, EnrouteAirwayRouteType>,
    pub level: RecordField<'a, Level>,
    pub direction_restriction: RecordField<'a, EnrouteDirectionalRestriction>,
    pub cruise_table_indicator: RecordField<'a, CruiseTableIdentifier>,
    pub has_restrictions: RecordField<'a, EnrouteAirwayRestrictionFlag>,
    pub recommended_navaid: RecordField<'a, RecommendedNavaid>,
    pub recommended_navaid_icao_code: RecordField<'a, IcaoCode>,
    pub rnp: RecordField<'a, RequiredNavigationPerformance>,
    pub recommended_navaid_section_code: RecordField<'a, Section>,
    pub recommended_navaid_subsection_code: RecordField<'a, NavaidSubsection>,
    pub outbound_course_unit: RecordField<'a, MagneticTrueIndicator>,
    pub theta: RecordField<'a, Theta>,
    pub rho: RecordField<'a, Rho>,
    pub outbound_course: RecordField<'a, OutboundCourse>,
    pub route_distance_from: RecordField<'a, RouteDistanceFrom>,
    pub inbound_course: RecordField<'a, InboundCourse>,
    pub inbound_course_unit: RecordField<'a, MagneticTrueIndicator>,
    pub minimum_altitude_1: RecordField<'a, MinimumAltitude>,
    pub minimum_altitude_2: RecordField<'a, MinimumAltitude>,
    pub maximum_altitude_1: RecordField<'a, MaximumAltitude>,
    pub fix_radius_transition: RecordField<'a, FixedRadiusTransitionIndicator>,
    pub vertical_scale_factor: RecordField<'a, VerticalScaleFactor>,
    pub rvsm_minimum_level: RecordField<'a, RVSMMinimumLevel>,
    pub vsf_rvsm_maximum_level: RecordField<'a, RVSMMaximumLevel>,
    pub maximum_altitude_2: RecordField<'a, MaximumAltitude>,
    pub route_qualifier1: RecordField<'a, EnrouteAirwayRouteTypeQualifier1>,
    pub route_qualifier2: RecordField<'a, EnrouteAirwayRouteTypeQualifier2>,
    pub route_qualifier3: RecordField<'a, EnrouteAirwayRouteTypeQualifier3>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayPrimaryRecord"
    }

    fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                   RecordField::from_bytes(input, 2, 3)?,
            section:                              RecordField::from_bytes(input, 5, 1)?,
            subsection:                           RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                     RecordField::from_bytes(input, 14, 5)?,
            sequence_number:                      RecordField::from_bytes(input, 26, 4)?,
            fix_identifier:                       RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                        RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                     RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                  RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:           RecordField::from_bytes(input, 39, 1)?,
            waypoint_description_code:            RecordField::from_bytes(input, 40, 4)?,
            boundary_code:                        RecordField::from_bytes(input, 44, 1)?,
            route_type:                           RecordField::from_bytes(input, 45, 1)?,
            level:                                RecordField::from_bytes(input, 46, 1)?,
            direction_restriction:                RecordField::from_bytes(input, 47, 1)?,
            cruise_table_indicator:               RecordField::from_bytes(input, 48, 2)?,
            has_restrictions:                     RecordField::from_bytes(input, 50, 1)?,
            recommended_navaid:                   RecordField::from_bytes(input, 51, 4)?,
            recommended_navaid_icao_code:         RecordField::from_bytes(input, 55, 2)?,
            rnp:                                  RecordField::from_bytes(input, 57, 3)?,
            recommended_navaid_section_code:      RecordField::from_bytes(input, 60, 1)?,
            recommended_navaid_subsection_code:   RecordField::from_bytes(input, 61, 1)?,
            outbound_course_unit:                 RecordField::from_bytes(input, 62, 1)?,
            theta:                                RecordField::from_bytes(input, 63, 4)?,
            rho:                                  RecordField::from_bytes(input, 67, 4)?,
            outbound_course:                      RecordField::from_bytes(input, 71, 4)?,
            route_distance_from:                  RecordField::from_bytes(input, 75, 4)?,
            inbound_course:                       RecordField::from_bytes(input, 79, 4)?,
            inbound_course_unit:                  RecordField::from_bytes(input, 83, 1)?,
            minimum_altitude_1:                   RecordField::from_bytes(input, 84, 5)?,
            minimum_altitude_2:                   RecordField::from_bytes(input, 89, 5)?,
            maximum_altitude_1:                   RecordField::from_bytes(input, 94, 5)?,
            fix_radius_transition:                RecordField::from_bytes(input, 99, 3)?,
            vertical_scale_factor:                RecordField::from_bytes(input, 102, 3)?,
            rvsm_minimum_level:                   RecordField::from_bytes(input, 105, 3)?,
            vsf_rvsm_maximum_level:               RecordField::from_bytes(input, 108, 3)?,
            maximum_altitude_2:                   RecordField::from_bytes(input, 116, 5)?,
            route_qualifier1:                     RecordField::from_bytes(input, 121, 1)?,
            route_qualifier2:                     RecordField::from_bytes(input, 122, 1)?,
            route_qualifier3:                     RecordField::from_bytes(input, 123, 1)?,
            file_record_number:                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct EnrouteAirwayContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                   RecordField::from_bytes(input, 2, 3)?,
            section:                              RecordField::from_bytes(input, 5, 1)?,
            subsection:                           RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                     RecordField::from_bytes(input, 14, 5)?,
            sequence_number:                      RecordField::from_bytes(input, 26, 4)?,
            fix_identifier:                       RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                        RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                     RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                  RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:           RecordField::from_bytes(input, 39, 1)?,
            application_type:                     RecordField::from_bytes(input, 40, 1)?,
            notes:                                RecordField::from_bytes(input, 41, 69)?,
            file_record_number:                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct EnrouteAirwayFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub route_identifier: RecordField<'a, EnrouteRouteIdentifier>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    // restricted airspace block 1
    pub restricted_airspace_1_icao_code: RecordField<'a, IcaoCode>,
    pub restricted_airspace_1_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restricted_airspace_1_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub restricted_airspace_1_multiple_code: RecordField<'a, MultipleCode>,
    // restricted airspace block 2
    pub restricted_airspace_2_icao_code: RecordField<'a, IcaoCode>,
    pub restricted_airspace_2_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restricted_airspace_2_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub restricted_airspace_2_multiple_code: RecordField<'a, MultipleCode>,
    // restricted airspace block 3
    pub restricted_airspace_3_icao_code: RecordField<'a, IcaoCode>,
    pub restricted_airspace_3_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restricted_airspace_3_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub restricted_airspace_3_multiple_code: RecordField<'a, MultipleCode>,
    // restricted airspace block 4
    pub restricted_airspace_4_icao_code: RecordField<'a, IcaoCode>,
    pub restricted_airspace_4_type: RecordField<'a, RestrictiveAirspaceType>,
    pub restricted_airspace_4_designation: RecordField<'a, RestrictiveAirspaceDesignation>,
    pub restricted_airspace_4_multiple_code: RecordField<'a, MultipleCode>,
    pub restricted_airspace_link_continuation: RecordField<'a, RestrictiveAirspaceLinkContinuation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for EnrouteAirwayFlightPlanningContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "EnrouteAirwayFlightPlanningContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self{
            record_type:                             RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                      RecordField::from_bytes(input, 2, 3)?,
            section:                                 RecordField::from_bytes(input, 5, 1)?,
            subsection:                              RecordField::from_bytes(input, 6, 1)?,
            route_identifier:                        RecordField::from_bytes(input, 14, 5)?,
            sequence_number:                         RecordField::from_bytes(input, 26, 4)?,
            fix_identifier:                          RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                           RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                        RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                     RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:              RecordField::from_bytes(input, 39, 1)?,
            application_type:                        RecordField::from_bytes(input, 40, 1)?,
            restricted_airspace_1_icao_code:         RecordField::from_bytes(input, 67, 2)?,
            restricted_airspace_1_type:              RecordField::from_bytes(input, 69, 1)?,
            restricted_airspace_1_designation:       RecordField::from_bytes(input, 70, 10)?,
            restricted_airspace_1_multiple_code:     RecordField::from_bytes(input, 80, 1)?,
            restricted_airspace_2_icao_code:         RecordField::from_bytes(input, 81, 2)?,
            restricted_airspace_2_type:              RecordField::from_bytes(input, 83, 1)?,
            restricted_airspace_2_designation:       RecordField::from_bytes(input, 84, 10)?,
            restricted_airspace_2_multiple_code:     RecordField::from_bytes(input, 94, 1)?,
            restricted_airspace_3_icao_code:         RecordField::from_bytes(input, 95, 2)?,
            restricted_airspace_3_type:              RecordField::from_bytes(input, 97, 1)?,
            restricted_airspace_3_designation:       RecordField::from_bytes(input, 98, 10)?,
            restricted_airspace_3_multiple_code:     RecordField::from_bytes(input, 108, 1)?,
            restricted_airspace_4_icao_code:         RecordField::from_bytes(input, 109, 2)?,
            restricted_airspace_4_type:              RecordField::from_bytes(input, 111, 1)?,
            restricted_airspace_4_designation:       RecordField::from_bytes(input, 112, 10)?,
            restricted_airspace_4_multiple_code:     RecordField::from_bytes(input, 122, 1)?,
            restricted_airspace_link_continuation:   RecordField::from_bytes(input, 123, 1)?,
            file_record_number:                      RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                              RecordField::from_bytes(input, 129, 4)?,
       })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}
