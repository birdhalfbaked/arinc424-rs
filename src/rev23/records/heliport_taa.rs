use crate::rev23::records::record::ARINCRecord;

use crate::rev23::definitions::*;
use crate::types::fields::ParseableField;
use crate::types::records::{
    Arinc424RecordSpec, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct HeliportTAARecords;
impl HeliportTAARecords {
    const CONTINUATION_COLUMN: usize = 30;
    const CONTINUATION_APPLICATION_COLUMN: usize = 31;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportTAAPrimary(
                HeliportTAAPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::HeliportTAAContinuation(
                        HeliportTAAContinuationRecord::parse(input)?,
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

/// 4.2.6.1 Heliport TAA Primary Record
#[derive(Debug)]
pub struct HeliportTAAPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub taa_waypoint: RecordField<'a, TaaWaypoint>,
    pub taa_waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub taa_waypoint_section: RecordField<'a, Section>,
    pub taa_waypoint_subsection: RecordField<'a, GenericSubsection>,
    pub taa_fix_sector_identifier: RecordField<'a, TaaSectorIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub sector_1_bearing: RecordField<'a, SectorBearing>,
    pub sector_1_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_1_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_1_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub sector_2_bearing: RecordField<'a, SectorBearing>,
    pub sector_2_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_2_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_2_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub sector_3_bearing: RecordField<'a, SectorBearing>,
    pub sector_3_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_3_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_3_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub sector_4_bearing: RecordField<'a, SectorBearing>,
    pub sector_4_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_4_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_4_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub sector_5_bearing: RecordField<'a, SectorBearing>,
    pub sector_5_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_5_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_5_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub bearing_reference_waypoint: RecordField<'a, TaaSectorBearingReferenceWaypoint>,
    pub bearing_reference_waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub bearing_reference_waypoint_section: RecordField<'a, Section>,
    pub bearing_reference_waypoint_subsection: RecordField<'a, GenericSubsection>,
    pub aircraft_category_or_type: RecordField<'a, ProcedureDesignAircraftCategoryOrType>,
    pub approach_route_qualifier_1: RecordField<'a, AirportHeliportApproachRouteTypeQualifier1>,
    pub approach_route_qualifier_2: RecordField<'a, AirportHeliportApproachRouteTypeQualifier2>,
    pub magnetic_true_indicator: RecordField<'a, MagneticTrueIndicator>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportTAAPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportTAAPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                             RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                      RecordField::from_bytes(input, 2, 3)?,
            section:                                 RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                     RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                      RecordField::from_bytes(input, 11, 2)?,
            subsection:                              RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                     RecordField::from_bytes(input, 14, 6)?,
            taa_waypoint:                            RecordField::from_bytes(input, 20, 5)?,
            taa_waypoint_icao_code:                  RecordField::from_bytes(input, 25, 2)?,
            taa_waypoint_section:                    RecordField::from_bytes(input, 27, 1)?,
            taa_waypoint_subsection:                 RecordField::from_bytes(input, 28, 1)?,
            taa_fix_sector_identifier:               RecordField::from_bytes(input, 29, 1)?,
            continuation_record_number:              RecordField::from_bytes(input, 30, 1)?,
            sector_1_bearing:                        RecordField::from_bytes(input, 33, 6)?,
            sector_1_minimum_altitude:               RecordField::from_bytes(input, 39, 3)?,
            sector_1_radius:                         RecordField::from_bytes(input, 42, 4)?,
            sector_1_procedure_turn_indicator:       RecordField::from_bytes(input, 46, 1)?,
            sector_2_bearing:                        RecordField::from_bytes(input, 47, 6)?,
            sector_2_minimum_altitude:               RecordField::from_bytes(input, 53, 3)?,
            sector_2_radius:                         RecordField::from_bytes(input, 56, 4)?,
            sector_2_procedure_turn_indicator:       RecordField::from_bytes(input, 60, 1)?,
            sector_3_bearing:                        RecordField::from_bytes(input, 61, 6)?,
            sector_3_minimum_altitude:               RecordField::from_bytes(input, 67, 3)?,
            sector_3_radius:                         RecordField::from_bytes(input, 70, 4)?,
            sector_3_procedure_turn_indicator:       RecordField::from_bytes(input, 74, 1)?,
            sector_4_bearing:                        RecordField::from_bytes(input, 75, 6)?,
            sector_4_minimum_altitude:               RecordField::from_bytes(input, 81, 3)?,
            sector_4_radius:                         RecordField::from_bytes(input, 84, 4)?,
            sector_4_procedure_turn_indicator:       RecordField::from_bytes(input, 88, 1)?,
            sector_5_bearing:                        RecordField::from_bytes(input, 89, 6)?,
            sector_5_minimum_altitude:               RecordField::from_bytes(input, 95, 3)?,
            sector_5_radius:                         RecordField::from_bytes(input, 98, 4)?,
            sector_5_procedure_turn_indicator:       RecordField::from_bytes(input, 102, 1)?,
            bearing_reference_waypoint:              RecordField::from_bytes(input, 103, 5)?,
            bearing_reference_waypoint_icao_code:    RecordField::from_bytes(input, 108, 2)?,
            bearing_reference_waypoint_section:      RecordField::from_bytes(input, 110, 1)?,
            bearing_reference_waypoint_subsection:   RecordField::from_bytes(input, 111, 1)?,
            aircraft_category_or_type:               RecordField::from_bytes(input, 117, 1)?,
            approach_route_qualifier_1:              RecordField::from_bytes(input, 118, 1)?,
            approach_route_qualifier_2:              RecordField::from_bytes(input, 119, 1)?,
            magnetic_true_indicator:                 RecordField::from_bytes(input, 120, 1)?,
            file_record_number:                      RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                              RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.2.6.2 Heliport TAA Continuation Record
#[derive(Debug)]
pub struct HeliportTAAContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub taa_waypoint: RecordField<'a, TaaWaypoint>,
    pub taa_waypoint_icao_code: RecordField<'a, IcaoCode>,
    pub taa_waypoint_section: RecordField<'a, Section>,
    pub taa_waypoint_subsection: RecordField<'a, GenericSubsection>,
    pub taa_fix_sector_identifier: RecordField<'a, TaaSectorIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub sector_1_bearing: RecordField<'a, SectorBearing>,
    pub sector_1_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_1_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_1_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub sector_2_bearing: RecordField<'a, SectorBearing>,
    pub sector_2_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_2_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_2_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub sector_3_bearing: RecordField<'a, SectorBearing>,
    pub sector_3_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_3_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_3_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub sector_4_bearing: RecordField<'a, SectorBearing>,
    pub sector_4_minimum_altitude: RecordField<'a, SectorAltitude>,
    pub sector_4_radius: RecordField<'a, TaaSectorRadius>,
    pub sector_4_procedure_turn_indicator: RecordField<'a, ProcedureTurn>,
    pub notes: RecordField<'a, Notes>,
    pub aircraft_category_or_type: RecordField<'a, ProcedureDesignAircraftCategoryOrType>,
    pub approach_route_qualifier_1: RecordField<'a, AirportHeliportApproachRouteTypeQualifier1>,
    pub approach_route_qualifier_2: RecordField<'a, AirportHeliportApproachRouteTypeQualifier2>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportTAAContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportTAAContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                             RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                      RecordField::from_bytes(input, 2, 3)?,
            section:                                 RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                     RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                      RecordField::from_bytes(input, 11, 2)?,
            subsection:                              RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                     RecordField::from_bytes(input, 14, 6)?,
            taa_waypoint:                            RecordField::from_bytes(input, 20, 5)?,
            taa_waypoint_icao_code:                  RecordField::from_bytes(input, 25, 2)?,
            taa_waypoint_section:                    RecordField::from_bytes(input, 27, 1)?,
            taa_waypoint_subsection:                 RecordField::from_bytes(input, 28, 1)?,
            taa_fix_sector_identifier:               RecordField::from_bytes(input, 29, 1)?,
            continuation_record_number:              RecordField::from_bytes(input, 30, 1)?,
            application_type:                        RecordField::from_bytes(input, 31, 1)?,
            sector_1_bearing:                        RecordField::from_bytes(input, 33, 6)?,
            sector_1_minimum_altitude:               RecordField::from_bytes(input, 39, 3)?,
            sector_1_radius:                         RecordField::from_bytes(input, 42, 4)?,
            sector_1_procedure_turn_indicator:       RecordField::from_bytes(input, 46, 1)?,
            sector_2_bearing:                        RecordField::from_bytes(input, 47, 6)?,
            sector_2_minimum_altitude:               RecordField::from_bytes(input, 53, 3)?,
            sector_2_radius:                         RecordField::from_bytes(input, 56, 4)?,
            sector_2_procedure_turn_indicator:       RecordField::from_bytes(input, 60, 1)?,
            sector_3_bearing:                        RecordField::from_bytes(input, 61, 6)?,
            sector_3_minimum_altitude:               RecordField::from_bytes(input, 67, 3)?,
            sector_3_radius:                         RecordField::from_bytes(input, 70, 4)?,
            sector_3_procedure_turn_indicator:       RecordField::from_bytes(input, 74, 1)?,
            sector_4_bearing:                        RecordField::from_bytes(input, 75, 6)?,
            sector_4_minimum_altitude:               RecordField::from_bytes(input, 81, 3)?,
            sector_4_radius:                         RecordField::from_bytes(input, 84, 4)?,
            sector_4_procedure_turn_indicator:       RecordField::from_bytes(input, 88, 1)?,
            notes:                                   RecordField::from_bytes(input, 89, 21)?,
            aircraft_category_or_type:               RecordField::from_bytes(input, 117, 1)?,
            approach_route_qualifier_1:              RecordField::from_bytes(input, 118, 1)?,
            approach_route_qualifier_2:              RecordField::from_bytes(input, 119, 1)?,
            file_record_number:                      RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                              RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}
