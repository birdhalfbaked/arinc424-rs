use crate::parsers::arinc424::definitions::*;
use crate::parsers::arinc424::fields::ParseableField;
use crate::parsers::arinc424::records::record::{
    ARINCRecord, RecordField, RecordParseError, is_primary_record,
};
pub(super) struct HeliportSTARRecords;
impl HeliportSTARRecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportSTARPrimary(
                HeliportSTARPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::HeliportSTARPrimaryExtensionContinuation(
                        HeliportSTARPrimaryExtensionContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::FlightPlanningContinuation) => {
                    Ok(ARINCRecord::HeliportSTARFlightPlanningContinuation(
                        HeliportSTARFlightPlanningContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::AirportSIDSTARApproachProcedureNameContinuation) => {
                    Ok(ARINCRecord::HeliportSTARProcedureNameContinuation(
                        HeliportSTARProcedureNameContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError {
                    message: "Invalid continuation record application type".to_string(),
                }),
            }
        }
    }
}

/// 4.2.3.1(A) Heliport SID Primary Record
#[derive(Debug)]
pub struct HeliportSTARPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub star_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub route_type: RecordField<'a, STARRouteType>,
    pub transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub aircraft_category_or_type: RecordField<'a, ProcedureDesignAircraftCategoryOrType>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub waypoint_description: RecordField<'a, WaypointDescriptionCode>,
    pub turn_direction: RecordField<'a, TurnDirection>,
    pub rnp: RecordField<'a, RequiredNavigationPerformance>,
    pub path_and_termination: RecordField<'a, PathAndTermination>,
    pub turn_direction_valid: RecordField<'a, TurnDirectionValid>,
    pub recommended_navaid: RecordField<'a, RecommendedNavaid>,
    pub recommended_navaid_icao_code: RecordField<'a, IcaoCode>,
    pub arc_radius: RecordField<'a, ArcRadius>,
    pub theta: RecordField<'a, Theta>,
    pub rho: RecordField<'a, Rho>,
    pub course: RecordField<'a, OutboundCourse>,
    pub route_or_holding_distance: RecordField<'a, RouteDistanceFrom>,
    pub recommended_navaid_section_code: RecordField<'a, Section>,
    pub recommended_navaid_subsection_code: RecordField<'a, NavaidSubsection>,
    pub leg_direction_indicator: RecordField<'a, HoldingPatternCourseReversalLegIndicator>,
    pub altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub altitude_atc_indicator: RecordField<'a, AtcIndicator>,
    pub altitude_1: RecordField<'a, MinimumAltitude>,
    pub altitude_2: RecordField<'a, MinimumAltitude>,
    pub transition_altitude: RecordField<'a, TransitionAltitudeLevel>,
    pub speed_limit: RecordField<'a, SpeedLimit>,
    pub vertical_angle: RecordField<'a, VerticalAngle>,
    pub center_fix: RecordField<'a, CenterFix>,
    pub multiple_code: RecordField<'a, MultipleCode>,
    pub center_fix_icao_code: RecordField<'a, IcaoCode>,
    pub center_fix_section_code: RecordField<'a, Section>,
    pub center_fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub gnss_fms_indication: RecordField<'a, GNSSFMSIndicator>,
    pub speed_limit_description: RecordField<'a, SpeedLimitDescription>,
    pub route_qualifier_1: RecordField<'a, AirportHeliportSTARRouteTypeQualifier1>,
    pub route_qualifier_2: RecordField<'a, AirportHeliportSTARRouteTypeQualifier2>,
    pub route_qualifier_3: RecordField<'a, AirportHeliportSTARRouteTypeQualifier3>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl <'a> HeliportSTARPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                   RecordField::from_bytes(input, 2, 4)?,
            section:                              RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                  RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                   RecordField::from_bytes(input, 11, 2)?,
            subsection:                           RecordField::from_bytes(input, 13, 1)?,
            star_identifier:                      RecordField::from_bytes(input, 14, 6)?,
            route_type:                           RecordField::from_bytes(input, 20, 1)?,
            transition_identifier:                RecordField::from_bytes(input, 21, 5)?,
            aircraft_category_or_type:            RecordField::from_bytes(input, 26, 1)?,
            sequence_number:                      RecordField::from_bytes(input, 27, 3)?,
            fix_identifier:                       RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                        RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                     RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                  RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:           RecordField::from_bytes(input, 39, 1)?,
            waypoint_description:                 RecordField::from_bytes(input, 40, 4)?,
            turn_direction:                       RecordField::from_bytes(input, 44, 1)?,
            rnp:                                  RecordField::from_bytes(input, 45, 3)?,
            path_and_termination:                 RecordField::from_bytes(input, 48, 2)?,
            turn_direction_valid:                 RecordField::from_bytes(input, 50, 1)?,
            recommended_navaid:                   RecordField::from_bytes(input, 51, 4)?,
            recommended_navaid_icao_code:         RecordField::from_bytes(input, 55, 2)?,
            arc_radius:                           RecordField::from_bytes(input, 57, 6)?,
            theta:                                RecordField::from_bytes(input, 63, 4)?,
            rho:                                  RecordField::from_bytes(input, 67, 4)?,
            course:                               RecordField::from_bytes(input, 71, 4)?,
            route_or_holding_distance:            RecordField::from_bytes(input, 75, 4)?,
            recommended_navaid_section_code:      RecordField::from_bytes(input, 79, 1)?,
            recommended_navaid_subsection_code:   RecordField::from_bytes(input, 80, 1)?,
            leg_direction_indicator:              RecordField::from_bytes(input, 81, 1)?,
            altitude_description:                 RecordField::from_bytes(input, 83, 1)?,
            altitude_atc_indicator:               RecordField::from_bytes(input, 84, 1)?,
            altitude_1:                           RecordField::from_bytes(input, 85, 5)?,
            altitude_2:                           RecordField::from_bytes(input, 90, 5)?,
            transition_altitude:                  RecordField::from_bytes(input, 95, 5)?,
            speed_limit:                          RecordField::from_bytes(input, 100, 3)?,
            vertical_angle:                       RecordField::from_bytes(input, 103, 4)?,
            center_fix:                           RecordField::from_bytes(input, 107, 5)?,
            multiple_code:                        RecordField::from_bytes(input, 112, 1)?,
            center_fix_icao_code:                 RecordField::from_bytes(input, 113, 2)?,
            center_fix_section_code:              RecordField::from_bytes(input, 115, 1)?,
            center_fix_subsection_code:           RecordField::from_bytes(input, 116, 1)?,
            gnss_fms_indication:                  RecordField::from_bytes(input, 117, 1)?,
            speed_limit_description:              RecordField::from_bytes(input, 118, 1)?,
            route_qualifier_1:                    RecordField::from_bytes(input, 119, 1)?,
            route_qualifier_2:                    RecordField::from_bytes(input, 120, 1)?,
            route_qualifier_3:                    RecordField::from_bytes(input, 121, 1)?,
            file_record_number:                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                           RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.2.3.2(A) Heliport SID Primary Extension Continuation Record
#[derive(Debug)]
pub struct HeliportSTARPrimaryExtensionContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub star_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub route_type: RecordField<'a, SIDRouteType>,
    pub transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub aircraft_category_or_type: RecordField<'a, ProcedureDesignAircraftCategoryOrType>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub procedure_tch: RecordField<'a, ThresholdCrossingHeight>,
    pub magnetic_variation: RecordField<'a, ProcedureDesignMagneticVariation>,
    pub magnetic_variation_unit: RecordField<'a, ProcedureDesignMagneticVariationIndicator>,
    pub referenced_fix_1_identifier: RecordField<'a, FixIdentifier>,
    pub referenced_fix_1_icao_code: RecordField<'a, IcaoCode>,
    pub referenced_fix_1_section_code: RecordField<'a, Section>,
    pub referenced_fix_1_subsection_code: RecordField<'a, GenericSubsection>,
    pub referenced_fix_2_identifier: RecordField<'a, FixIdentifier>,
    pub referenced_fix_2_icao_code: RecordField<'a, IcaoCode>,
    pub referenced_fix_2_section_code: RecordField<'a, Section>,
    pub referenced_fix_2_subsection_code: RecordField<'a, GenericSubsection>,
    pub referenced_fix_3_identifier: RecordField<'a, FixIdentifier>,
    pub referenced_fix_3_icao_code: RecordField<'a, IcaoCode>,
    pub referenced_fix_3_section_code: RecordField<'a, Section>,
    pub referenced_fix_3_subsection_code: RecordField<'a, GenericSubsection>,
    pub referenced_fix_4_identifier: RecordField<'a, FixIdentifier>,
    pub referenced_fix_4_icao_code: RecordField<'a, IcaoCode>,
    pub referenced_fix_4_section_code: RecordField<'a, Section>,
    pub referenced_fix_4_subsection_code: RecordField<'a, GenericSubsection>,
    pub cat_a_radii: RecordField<'a, CirclingCategoryDistance>,
    pub special_indicator: RecordField<'a, SpecialProcedureIndicator>,
    pub military_indicator: RecordField<'a, TerminalProcedureForMilitaryIndicator>,
    pub vertical_scale_factor: RecordField<'a, VerticalScaleFactor>,
    pub route_qualifier_1: RecordField<'a, AirportHeliportSTARRouteTypeQualifier1>,
    pub route_qualifier_2: RecordField<'a, AirportHeliportSTARRouteTypeQualifier2>,
    pub route_qualifier_3: RecordField<'a, AirportHeliportSTARRouteTypeQualifier3>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl <'a> HeliportSTARPrimaryExtensionContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 4)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                 RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            star_identifier:                    RecordField::from_bytes(input, 14, 6)?,
            route_type:                         RecordField::from_bytes(input, 20, 1)?,
            transition_identifier:              RecordField::from_bytes(input, 21, 5)?,
            aircraft_category_or_type:          RecordField::from_bytes(input, 26, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 27, 3)?,
            fix_identifier:                     RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                      RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                   RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:         RecordField::from_bytes(input, 39, 1)?,
            application_type:                   RecordField::from_bytes(input, 40, 1)?,
            procedure_tch:                      RecordField::from_bytes(input, 41, 3)?,
            magnetic_variation:                 RecordField::from_bytes(input, 61, 5)?,
            magnetic_variation_unit:            RecordField::from_bytes(input, 66, 1)?,
            referenced_fix_1_identifier:        RecordField::from_bytes(input, 67, 5)?,
            referenced_fix_1_icao_code:         RecordField::from_bytes(input, 72, 2)?,
            referenced_fix_1_section_code:      RecordField::from_bytes(input, 74, 1)?,
            referenced_fix_1_subsection_code:   RecordField::from_bytes(input, 75, 1)?,
            referenced_fix_2_identifier:        RecordField::from_bytes(input, 76, 5)?,
            referenced_fix_2_icao_code:         RecordField::from_bytes(input, 81, 2)?,
            referenced_fix_2_section_code:      RecordField::from_bytes(input, 83, 1)?,
            referenced_fix_2_subsection_code:   RecordField::from_bytes(input, 84, 1)?,
            referenced_fix_3_identifier:        RecordField::from_bytes(input, 85, 5)?,
            referenced_fix_3_icao_code:         RecordField::from_bytes(input, 90, 2)?,
            referenced_fix_3_section_code:      RecordField::from_bytes(input, 92, 1)?,
            referenced_fix_3_subsection_code:   RecordField::from_bytes(input, 93, 1)?,
            referenced_fix_4_identifier:        RecordField::from_bytes(input, 94, 5)?,
            referenced_fix_4_icao_code:         RecordField::from_bytes(input, 99, 2)?,
            referenced_fix_4_section_code:      RecordField::from_bytes(input, 101, 1)?,
            referenced_fix_4_subsection_code:   RecordField::from_bytes(input, 102, 1)?,
            cat_a_radii:                        RecordField::from_bytes(input, 103, 2)?,
            special_indicator:                  RecordField::from_bytes(input, 111, 1)?,
            military_indicator:                 RecordField::from_bytes(input, 113, 1)?,
            vertical_scale_factor:              RecordField::from_bytes(input, 116, 3)?,
            route_qualifier_1:                  RecordField::from_bytes(input, 119, 1)?,
            route_qualifier_2:                  RecordField::from_bytes(input, 120, 1)?,
            route_qualifier_3:                  RecordField::from_bytes(input, 121, 1)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

#[derive(Debug)]
pub struct HeliportSTARFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub star_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub route_type: RecordField<'a, SIDRouteType>,
    pub transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub aircraft_category_or_type: RecordField<'a, ProcedureDesignAircraftCategoryOrType>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub atc_assigned_only: RecordField<'a, AtcAssignedOnly>,
    pub leg_distance: RecordField<'a, TerminalProcedureFlightPlanningLegDistance>,
    pub route_qualifier_1: RecordField<'a, AirportHeliportSTARRouteTypeQualifier1>,
    pub route_qualifier_2: RecordField<'a, AirportHeliportSTARRouteTypeQualifier2>,
    pub route_qualifier_3: RecordField<'a, AirportHeliportSTARRouteTypeQualifier3>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl <'a> HeliportSTARFlightPlanningContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 4)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                 RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            star_identifier:                    RecordField::from_bytes(input, 14, 6)?,
            route_type:                         RecordField::from_bytes(input, 20, 1)?,
            transition_identifier:              RecordField::from_bytes(input, 21, 5)?,
            aircraft_category_or_type:          RecordField::from_bytes(input, 26, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 27, 3)?,
            fix_identifier:                     RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                      RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                   RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:         RecordField::from_bytes(input, 39, 1)?,
            application_type:                   RecordField::from_bytes(input, 40, 1)?,
            atc_assigned_only:                  RecordField::from_bytes(input, 41, 1)?,
            leg_distance:                       RecordField::from_bytes(input, 75, 4)?,
            route_qualifier_1:                  RecordField::from_bytes(input, 119, 1)?,
            route_qualifier_2:                  RecordField::from_bytes(input, 120, 1)?,
            route_qualifier_3:                  RecordField::from_bytes(input, 121, 1)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

#[derive(Debug)]
pub struct HeliportSTARProcedureNameContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub star_identifier: RecordField<'a, SidStarRouteIdentifier>,
    pub route_type: RecordField<'a, SIDRouteType>,
    pub transition_identifier: RecordField<'a, TransitionIdentifier>,
    pub aircraft_category_or_type: RecordField<'a, ProcedureDesignAircraftCategoryOrType>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub name: RecordField<'a, ProcedureName>,
    pub route_qualifier_1: RecordField<'a, AirportHeliportSTARRouteTypeQualifier1>,
    pub route_qualifier_2: RecordField<'a, AirportHeliportSTARRouteTypeQualifier2>,
    pub route_qualifier_3: RecordField<'a, AirportHeliportSTARRouteTypeQualifier3>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl <'a> HeliportSTARProcedureNameContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 4)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                 RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            star_identifier:                    RecordField::from_bytes(input, 14, 6)?,
            route_type:                         RecordField::from_bytes(input, 20, 1)?,
            transition_identifier:              RecordField::from_bytes(input, 21, 5)?,
            aircraft_category_or_type:          RecordField::from_bytes(input, 26, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 27, 3)?,
            fix_identifier:                     RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                      RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                   RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:         RecordField::from_bytes(input, 39, 1)?,
            application_type:                   RecordField::from_bytes(input, 40, 1)?,
            name:                               RecordField::from_bytes(input, 41, 78)?,
            route_qualifier_1:                  RecordField::from_bytes(input, 119, 1)?,
            route_qualifier_2:                  RecordField::from_bytes(input, 120, 1)?,
            route_qualifier_3:                  RecordField::from_bytes(input, 121, 1)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
