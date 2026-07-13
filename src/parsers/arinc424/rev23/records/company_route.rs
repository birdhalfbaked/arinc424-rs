use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError};
use crate::parsers::arinc424::rev23::definitions::*;
pub(super) struct CompanyRouteRecords;
impl CompanyRouteRecords {
    const VIA_COLUMN: usize = 40;
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if let Some(via_code) =
            CompanyRouteVIACode::from_bytes(&input[Self::VIA_COLUMN - 1..Self::VIA_COLUMN + 2])?
        {
            Ok(match via_code {
                CompanyRouteVIACode::SID
                | CompanyRouteVIACode::SIDEnrouteTransition
                | CompanyRouteVIACode::SIDRunwayTransition => {
                    ARINCRecord::CompanySIDRoutePrimary(CompanySIDRoutePrimaryRecord::parse(input)?)
                }
                CompanyRouteVIACode::STARProfileDescent
                | CompanyRouteVIACode::STARProfileDescentEnrouteTransition
                | CompanyRouteVIACode::STARProfileDescentRunwayTransition => {
                    ARINCRecord::CompanySTARRoutePrimary(CompanySTARRoutePrimaryRecord::parse(
                        input,
                    )?)
                }
                CompanyRouteVIACode::ApproachRoute | CompanyRouteVIACode::ApproachTransition => {
                    ARINCRecord::CompanyApproachRoutePrimary(
                        CompanyApproachRoutePrimaryRecord::parse(input)?,
                    )
                }
                CompanyRouteVIACode::DesignatedAirway => ARINCRecord::CompanyAirwayRoutePrimary(
                    CompanyAirwayRoutePrimaryRecord::parse(input)?,
                ),
                _ => ARINCRecord::CompanyGeneralRoutePrimary(
                    CompanyGeneralRoutePrimaryRecord::parse(input)?,
                ),
            })
        } else {
            Err(RecordParseError::new("Invalid company route VIA code".to_string(), Some(String::from_utf8_lossy(input).into_owned())))
        }
    }
}

/// 4.1.12.1(A) Company SID Route Primary Record
#[derive(Debug)]
pub struct CompanySIDRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub sid_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, ToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_icao_code: RecordField<'a, IcaoCode>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub sid_route_type: RecordField<'a, SIDRouteType>,
    pub sid_route_type_qualifier_1: RecordField<'a, AirportHeliportSIDRouteTypeQualifier1>,
    pub sid_route_type_qualifier_2: RecordField<'a, AirportHeliportSIDRouteTypeQualifier2>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> CompanySIDRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(CompanySIDRoutePrimaryRecord {
            record_type:                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                     RecordField::from_bytes(input, 2,3)?,
            section:                                RecordField::from_bytes(input, 5, 1)?,
            subsection:                             RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                        RecordField::from_bytes(input, 7, 5)?,
            from_icao_code:                         RecordField::from_bytes(input, 13, 2)?,
            from_section_code:                      RecordField::from_bytes(input, 15, 1)?,
            from_subsection_code:                   RecordField::from_bytes(input, 16, 1)?,
            to_identifier:                          RecordField::from_bytes(input, 17, 5)?,
            to_icao_code:                           RecordField::from_bytes(input, 23, 2)?,
            to_section_code:                        RecordField::from_bytes(input, 25, 1)?,
            to_subsection_code:                     RecordField::from_bytes(input, 26, 1)?,
            company_route_id:                       RecordField::from_bytes(input, 27, 10)?,
            sequence_number:                        RecordField::from_bytes(input, 37, 3)?,
            via_code:                               RecordField::from_bytes(input, 40, 3)?,
            sid_identifier:                         RecordField::from_bytes(input, 43, 6)?,
            area:                                   RecordField::from_bytes(input, 49, 3)?,
            to_fix:                                 RecordField::from_bytes(input, 52, 6)?,
            to_fix_icao_code:                       RecordField::from_bytes(input, 58, 2)?,
            to_fix_section:                         RecordField::from_bytes(input, 60, 1)?,
            to_fix_subsection:                      RecordField::from_bytes(input, 61, 1)?,
            runway_transition:                      RecordField::from_bytes(input, 62, 5)?,
            enroute_transition:                     RecordField::from_bytes(input, 67, 5)?,
            cruise_altitude:                        RecordField::from_bytes(input, 73, 5)?,
            terminal_alternate_airport:             RecordField::from_bytes(input, 78, 4)?,
            terminal_alternate_airport_icao_code:   RecordField::from_bytes(input, 82, 2)?,
            alternate_distance:                     RecordField::from_bytes(input, 84, 4)?,
            cost_index:                             RecordField::from_bytes(input, 88, 3)?,
            enroute_alternate_airport:              RecordField::from_bytes(input, 91, 4)?,
            sid_route_type:                         RecordField::from_bytes(input, 95, 1)?,
            sid_route_type_qualifier_1:             RecordField::from_bytes(input, 96, 1)?,
            sid_route_type_qualifier_2:             RecordField::from_bytes(input, 97, 1)?,
            file_record_number:                     RecordField::from_bytes(input, 124,5)?,
            cycle_date:                             RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.12.1(B) Company STAR Route Primary Record
#[derive(Debug)]
pub struct CompanySTARRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub star_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, ToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_icao_code: RecordField<'a, IcaoCode>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub star_route_type: RecordField<'a, STARRouteType>,
    pub star_route_type_qualifier_1: RecordField<'a, AirportHeliportSTARRouteTypeQualifier1>,
    pub star_route_type_qualifier_2: RecordField<'a, AirportHeliportSTARRouteTypeQualifier2>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> CompanySTARRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(CompanySTARRoutePrimaryRecord {
            record_type:                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                     RecordField::from_bytes(input, 2,3)?,
            section:                                RecordField::from_bytes(input, 5, 1)?,
            subsection:                             RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                        RecordField::from_bytes(input, 7, 5)?,
            from_icao_code:                         RecordField::from_bytes(input, 13, 2)?,
            from_section_code:                      RecordField::from_bytes(input, 15, 1)?,
            from_subsection_code:                   RecordField::from_bytes(input, 16, 1)?,
            to_identifier:                          RecordField::from_bytes(input, 17, 5)?,
            to_icao_code:                           RecordField::from_bytes(input, 23, 2)?,
            to_section_code:                        RecordField::from_bytes(input, 25, 1)?,
            to_subsection_code:                     RecordField::from_bytes(input, 26, 1)?,
            company_route_id:                       RecordField::from_bytes(input, 27, 10)?,
            sequence_number:                        RecordField::from_bytes(input, 37, 3)?,
            via_code:                               RecordField::from_bytes(input, 40, 3)?,
            star_identifier:                        RecordField::from_bytes(input, 43, 6)?,
            area:                                   RecordField::from_bytes(input, 49, 3)?,
            to_fix:                                 RecordField::from_bytes(input, 52, 6)?,
            to_fix_icao_code:                       RecordField::from_bytes(input, 58, 2)?,
            to_fix_section:                         RecordField::from_bytes(input, 60, 1)?,
            to_fix_subsection:                      RecordField::from_bytes(input, 61, 1)?,
            runway_transition:                      RecordField::from_bytes(input, 62, 5)?,
            enroute_transition:                     RecordField::from_bytes(input, 67, 5)?,
            cruise_altitude:                        RecordField::from_bytes(input, 73, 5)?,
            terminal_alternate_airport:             RecordField::from_bytes(input, 78, 4)?,
            terminal_alternate_airport_icao_code:   RecordField::from_bytes(input, 82, 2)?,
            alternate_distance:                     RecordField::from_bytes(input, 84, 4)?,
            cost_index:                             RecordField::from_bytes(input, 88, 3)?,
            enroute_alternate_airport:              RecordField::from_bytes(input, 91, 4)?,
            star_route_type:                        RecordField::from_bytes(input, 95, 1)?,
            star_route_type_qualifier_1:            RecordField::from_bytes(input, 96, 1)?,
            star_route_type_qualifier_2:            RecordField::from_bytes(input, 97, 1)?,
            file_record_number:                     RecordField::from_bytes(input, 124,5)?,
            cycle_date:                             RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.12.1(C) Company Approach Route Primary Record
#[derive(Debug)]
pub struct CompanyApproachRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub approach_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, ToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_icao_code: RecordField<'a, IcaoCode>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub approach_route_type: RecordField<'a, AirportHeliportApproachRouteType>,
    pub approach_route_type_qualifier_1:
        RecordField<'a, AirportHeliportApproachRouteTypeQualifier1>,
    pub approach_route_type_qualifier_2:
        RecordField<'a, AirportHeliportApproachRouteTypeQualifier2>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> CompanyApproachRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(CompanyApproachRoutePrimaryRecord {
            record_type:                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                     RecordField::from_bytes(input, 2,3)?,
            section:                                RecordField::from_bytes(input, 5, 1)?,
            subsection:                             RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                        RecordField::from_bytes(input, 7, 5)?,
            from_icao_code:                         RecordField::from_bytes(input, 13, 2)?,
            from_section_code:                      RecordField::from_bytes(input, 15, 1)?,
            from_subsection_code:                   RecordField::from_bytes(input, 16, 1)?,
            to_identifier:                          RecordField::from_bytes(input, 17, 5)?,
            to_icao_code:                           RecordField::from_bytes(input, 23, 2)?,
            to_section_code:                        RecordField::from_bytes(input, 25, 1)?,
            to_subsection_code:                     RecordField::from_bytes(input, 26, 1)?,
            company_route_id:                       RecordField::from_bytes(input, 27, 10)?,
            sequence_number:                        RecordField::from_bytes(input, 37, 3)?,
            via_code:                               RecordField::from_bytes(input, 40, 3)?,
            approach_identifier:                    RecordField::from_bytes(input, 43, 6)?,
            area:                                   RecordField::from_bytes(input, 49, 3)?,
            to_fix:                                 RecordField::from_bytes(input, 52, 6)?,
            to_fix_icao_code:                       RecordField::from_bytes(input, 58, 2)?,
            to_fix_section:                         RecordField::from_bytes(input, 60, 1)?,
            to_fix_subsection:                      RecordField::from_bytes(input, 61, 1)?,
            runway_transition:                      RecordField::from_bytes(input, 62, 5)?,
            enroute_transition:                     RecordField::from_bytes(input, 67, 5)?,
            cruise_altitude:                        RecordField::from_bytes(input, 73, 5)?,
            terminal_alternate_airport:             RecordField::from_bytes(input, 78, 4)?,
            terminal_alternate_airport_icao_code:   RecordField::from_bytes(input, 82, 2)?,
            alternate_distance:                     RecordField::from_bytes(input, 84, 4)?,
            cost_index:                             RecordField::from_bytes(input, 88, 3)?,
            enroute_alternate_airport:              RecordField::from_bytes(input, 91, 4)?,
            approach_route_type:                    RecordField::from_bytes(input, 95, 1)?,
            approach_route_type_qualifier_1:        RecordField::from_bytes(input, 96, 1)?,
            approach_route_type_qualifier_2:        RecordField::from_bytes(input, 97, 1)?,
            file_record_number:                     RecordField::from_bytes(input, 124,5)?,
            cycle_date:                             RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.12.1(D) Company Airway Route Primary Record
#[derive(Debug)]
pub struct CompanyAirwayRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub airway_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, ToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_icao_code: RecordField<'a, IcaoCode>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub airway_route_type: RecordField<'a, EnrouteAirwayRouteType>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> CompanyAirwayRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(CompanyAirwayRoutePrimaryRecord {
            record_type:                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                     RecordField::from_bytes(input, 2,3)?,
            section:                                RecordField::from_bytes(input, 5, 1)?,
            subsection:                             RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                        RecordField::from_bytes(input, 7, 5)?,
            from_icao_code:                         RecordField::from_bytes(input, 13, 2)?,
            from_section_code:                      RecordField::from_bytes(input, 15, 1)?,
            from_subsection_code:                   RecordField::from_bytes(input, 16, 1)?,
            to_identifier:                          RecordField::from_bytes(input, 17, 5)?,
            to_icao_code:                           RecordField::from_bytes(input, 23, 2)?,
            to_section_code:                        RecordField::from_bytes(input, 25, 1)?,
            to_subsection_code:                     RecordField::from_bytes(input, 26, 1)?,
            company_route_id:                       RecordField::from_bytes(input, 27, 10)?,
            sequence_number:                        RecordField::from_bytes(input, 37, 3)?,
            via_code:                               RecordField::from_bytes(input, 40, 3)?,
            airway_identifier:                      RecordField::from_bytes(input, 43, 6)?,
            area:                                   RecordField::from_bytes(input, 49, 3)?,
            to_fix:                                 RecordField::from_bytes(input, 52, 6)?,
            to_fix_icao_code:                       RecordField::from_bytes(input, 58, 2)?,
            to_fix_section:                         RecordField::from_bytes(input, 60, 1)?,
            to_fix_subsection:                      RecordField::from_bytes(input, 61, 1)?,
            runway_transition:                      RecordField::from_bytes(input, 62, 5)?,
            enroute_transition:                     RecordField::from_bytes(input, 67, 5)?,
            cruise_altitude:                        RecordField::from_bytes(input, 73, 5)?,
            terminal_alternate_airport:             RecordField::from_bytes(input, 78, 4)?,
            terminal_alternate_airport_icao_code:   RecordField::from_bytes(input, 82, 2)?,
            alternate_distance:                     RecordField::from_bytes(input, 84, 4)?,
            cost_index:                             RecordField::from_bytes(input, 88, 3)?,
            enroute_alternate_airport:              RecordField::from_bytes(input, 91, 4)?,
            airway_route_type:                      RecordField::from_bytes(input, 95, 1)?,
            file_record_number:                     RecordField::from_bytes(input, 124,5)?,
            cycle_date:                             RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.12.1(E) Company General Route Primary Record
///
/// Note: This is meant to be a catchall for ALT/DIR/INT/PRE via codes
#[derive(Debug)]
pub struct CompanyGeneralRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, ToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_icao_code: RecordField<'a, IcaoCode>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> CompanyGeneralRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(CompanyGeneralRoutePrimaryRecord {
            record_type:                            RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                     RecordField::from_bytes(input, 2,3)?,
            section:                                RecordField::from_bytes(input, 5, 1)?,
            subsection:                             RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                        RecordField::from_bytes(input, 7, 5)?,
            from_icao_code:                         RecordField::from_bytes(input, 13, 2)?,
            from_section_code:                      RecordField::from_bytes(input, 15, 1)?,
            from_subsection_code:                   RecordField::from_bytes(input, 16, 1)?,
            to_identifier:                          RecordField::from_bytes(input, 17, 5)?,
            to_icao_code:                           RecordField::from_bytes(input, 23, 2)?,
            to_section_code:                        RecordField::from_bytes(input, 25, 1)?,
            to_subsection_code:                     RecordField::from_bytes(input, 26, 1)?,
            company_route_id:                       RecordField::from_bytes(input, 27, 10)?,
            sequence_number:                        RecordField::from_bytes(input, 37, 3)?,
            via_code:                               RecordField::from_bytes(input, 40, 3)?,
            area:                                   RecordField::from_bytes(input, 49, 3)?,
            to_fix:                                 RecordField::from_bytes(input, 52, 6)?,
            to_fix_icao_code:                       RecordField::from_bytes(input, 58, 2)?,
            to_fix_section:                         RecordField::from_bytes(input, 60, 1)?,
            to_fix_subsection:                      RecordField::from_bytes(input, 61, 1)?,
            runway_transition:                      RecordField::from_bytes(input, 62, 5)?,
            enroute_transition:                     RecordField::from_bytes(input, 67, 5)?,
            cruise_altitude:                        RecordField::from_bytes(input, 73, 5)?,
            terminal_alternate_airport:             RecordField::from_bytes(input, 78, 4)?,
            terminal_alternate_airport_icao_code:   RecordField::from_bytes(input, 82, 2)?,
            alternate_distance:                     RecordField::from_bytes(input, 84, 4)?,
            cost_index:                             RecordField::from_bytes(input, 88, 3)?,
            enroute_alternate_airport:              RecordField::from_bytes(input, 91, 4)?,
            file_record_number:                     RecordField::from_bytes(input, 124,5)?,
            cycle_date:                             RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
