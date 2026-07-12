use crate::parsers::arinc424::rev18::definitions::*;
use crate::parsers::arinc424::rev18::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError};
pub(super) struct HelicopterCompanyRouteRecords;
impl HelicopterCompanyRouteRecords {
    const VIA_COLUMN: usize = 50;
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if let Some(via_code) =
            CompanyRouteVIACode::from_bytes(&input[Self::VIA_COLUMN - 1..Self::VIA_COLUMN + 2])?
        {
            Ok(match via_code {
                CompanyRouteVIACode::SID
                | CompanyRouteVIACode::SIDEnrouteTransition
                | CompanyRouteVIACode::SIDRunwayTransition => {
                    ARINCRecord::HelicopterCompanySIDRoutePrimary(
                        HelicopterCompanySIDRoutePrimaryRecord::parse(input)?,
                    )
                }
                CompanyRouteVIACode::STARProfileDescent
                | CompanyRouteVIACode::STARProfileDescentEnrouteTransition
                | CompanyRouteVIACode::STARProfileDescentRunwayTransition => {
                    ARINCRecord::HelicopterCompanySTARRoutePrimary(
                        HelicopterCompanySTARRoutePrimaryRecord::parse(input)?,
                    )
                }
                CompanyRouteVIACode::ApproachRoute | CompanyRouteVIACode::ApproachTransition => {
                    ARINCRecord::HelicopterCompanyApproachRoutePrimary(
                        HelicopterCompanyApproachRoutePrimaryRecord::parse(input)?,
                    )
                }
                CompanyRouteVIACode::DesignatedAirway => {
                    ARINCRecord::HelicopterCompanyAirwayRoutePrimary(
                        HelicopterCompanyAirwayRoutePrimaryRecord::parse(input)?,
                    )
                }
                _ => ARINCRecord::HelicopterCompanyGeneralRoutePrimary(
                    HelicopterCompanyGeneralRoutePrimaryRecord::parse(input)?,
                ),
            })
        } else {
            Err(RecordParseError {
                message: "Invalid company route VIA code".to_string(),
            })
        }
    }
}

/// 4.2.7.1(A) Helicopter Company SID Route Primary Record
#[derive(Debug)]
pub struct HelicopterCompanySIDRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub sid_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub sid_section_code: RecordField<'a, Section>,
    pub sid_subsection_code: RecordField<'a, GenericSubsection>,
    pub sid_route_type: RecordField<'a, SIDRouteType>,
    pub sid_route_type_qualifier_1: RecordField<'a, RouteTypeQualifier1>,
    pub sid_route_type_qualifier_2: RecordField<'a, RouteTypeQualifier2>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, CompanyRouteToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_helipad_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport_heliport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_heliport_icao_code: RecordField<'a, IcaoCode>,
    pub terminal_alternate_airport_heliport_section: RecordField<'a, Section>,
    pub terminal_alternate_airport_heliport_subsection: RecordField<'a, GenericSubsection>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport_heliport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterCompanySIDRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(HelicopterCompanySIDRoutePrimaryRecord {
            record_type:                                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                               RecordField::from_bytes(input, 2,3)?,
            section:                                          RecordField::from_bytes(input, 5, 1)?,
            subsection:                                       RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                                  RecordField::from_bytes(input, 7, 5)?,
            from_helipad_identifier:                          RecordField::from_bytes(input, 12, 5)?,
            from_icao_code:                                   RecordField::from_bytes(input, 18, 2)?,
            from_section_code:                                RecordField::from_bytes(input, 20, 1)?,
            from_subsection_code:                             RecordField::from_bytes(input, 21, 1)?,
            to_identifier:                                    RecordField::from_bytes(input, 22, 5)?,
            to_helipad_identifier:                            RecordField::from_bytes(input, 27, 5)?,
            to_icao_code:                                     RecordField::from_bytes(input, 33, 2)?,
            to_section_code:                                  RecordField::from_bytes(input, 35, 1)?,
            to_subsection_code:                               RecordField::from_bytes(input, 36, 1)?,
            company_route_id:                                 RecordField::from_bytes(input, 37, 10)?,
            sequence_number:                                  RecordField::from_bytes(input, 47, 3)?,
            via_code:                                         RecordField::from_bytes(input, 50, 3)?,
            sid_identifier:                                   RecordField::from_bytes(input, 53, 6)?,
            sid_section_code:                                 RecordField::from_bytes(input, 59, 1)?,
            sid_subsection_code:                              RecordField::from_bytes(input, 60, 1)?,
            sid_route_type:                                   RecordField::from_bytes(input, 61, 1)?,
            sid_route_type_qualifier_1:                       RecordField::from_bytes(input, 62, 1)?,
            sid_route_type_qualifier_2:                       RecordField::from_bytes(input, 63, 1)?,
            area:                                             RecordField::from_bytes(input, 64, 3)?,
            to_fix:                                           RecordField::from_bytes(input, 67, 6)?,
            to_fix_icao_code:                                 RecordField::from_bytes(input, 73, 2)?,
            to_fix_section:                                   RecordField::from_bytes(input, 75, 1)?,
            to_fix_subsection:                                RecordField::from_bytes(input, 76, 1)?,
            runway_helipad_transition:                        RecordField::from_bytes(input, 77, 5)?,
            enroute_transition:                               RecordField::from_bytes(input, 82, 5)?,
            cruise_altitude:                                  RecordField::from_bytes(input, 88, 5)?,
            terminal_alternate_airport_heliport:              RecordField::from_bytes(input, 93, 4)?,
            terminal_alternate_airport_heliport_icao_code:    RecordField::from_bytes(input, 97, 2)?,
            terminal_alternate_airport_heliport_section:      RecordField::from_bytes(input, 99, 1)?,
            terminal_alternate_airport_heliport_subsection:   RecordField::from_bytes(input, 100, 1)?,
            alternate_distance:                               RecordField::from_bytes(input, 101, 4)?,
            cost_index:                                       RecordField::from_bytes(input, 105, 3)?,
            enroute_alternate_airport_heliport:               RecordField::from_bytes(input, 108, 4)?,
            file_record_number:                               RecordField::from_bytes(input, 124,5)?,
            cycle_date:                                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.2.7.1(B) Helicopter Company STAR Route Primary Record
#[derive(Debug)]
pub struct HelicopterCompanySTARRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub star_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub star_section_code: RecordField<'a, Section>,
    pub star_subsection_code: RecordField<'a, GenericSubsection>,
    pub star_route_type: RecordField<'a, STARRouteType>,
    pub star_route_type_qualifier_1: RecordField<'a, RouteTypeQualifier1>,
    pub star_route_type_qualifier_2: RecordField<'a, RouteTypeQualifier2>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, CompanyRouteToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_helipad_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport_heliport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_heliport_icao_code: RecordField<'a, IcaoCode>,
    pub terminal_alternate_airport_heliport_section: RecordField<'a, Section>,
    pub terminal_alternate_airport_heliport_subsection: RecordField<'a, GenericSubsection>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport_heliport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterCompanySTARRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(HelicopterCompanySTARRoutePrimaryRecord {
            record_type:                                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                               RecordField::from_bytes(input, 2,3)?,
            section:                                          RecordField::from_bytes(input, 5, 1)?,
            subsection:                                       RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                                  RecordField::from_bytes(input, 7, 5)?,
            from_helipad_identifier:                          RecordField::from_bytes(input, 12, 5)?,
            from_icao_code:                                   RecordField::from_bytes(input, 18, 2)?,
            from_section_code:                                RecordField::from_bytes(input, 20, 1)?,
            from_subsection_code:                             RecordField::from_bytes(input, 21, 1)?,
            to_identifier:                                    RecordField::from_bytes(input, 22, 5)?,
            to_helipad_identifier:                            RecordField::from_bytes(input, 27, 5)?,
            to_icao_code:                                     RecordField::from_bytes(input, 33, 2)?,
            to_section_code:                                  RecordField::from_bytes(input, 35, 1)?,
            to_subsection_code:                               RecordField::from_bytes(input, 36, 1)?,
            company_route_id:                                 RecordField::from_bytes(input, 37, 10)?,
            sequence_number:                                  RecordField::from_bytes(input, 47, 3)?,
            via_code:                                         RecordField::from_bytes(input, 50, 3)?,
            star_identifier:                                  RecordField::from_bytes(input, 53, 6)?,
            star_section_code:                                RecordField::from_bytes(input, 59, 1)?,
            star_subsection_code:                             RecordField::from_bytes(input, 60, 1)?,
            star_route_type:                                  RecordField::from_bytes(input, 61, 1)?,
            star_route_type_qualifier_1:                      RecordField::from_bytes(input, 62, 1)?,
            star_route_type_qualifier_2:                      RecordField::from_bytes(input, 63, 1)?,
            area:                                             RecordField::from_bytes(input, 64, 3)?,
            to_fix:                                           RecordField::from_bytes(input, 67, 6)?,
            to_fix_icao_code:                                 RecordField::from_bytes(input, 73, 2)?,
            to_fix_section:                                   RecordField::from_bytes(input, 75, 1)?,
            to_fix_subsection:                                RecordField::from_bytes(input, 76, 1)?,
            runway_helipad_transition:                        RecordField::from_bytes(input, 77, 5)?,
            enroute_transition:                               RecordField::from_bytes(input, 82, 5)?,
            cruise_altitude:                                  RecordField::from_bytes(input, 88, 5)?,
            terminal_alternate_airport_heliport:              RecordField::from_bytes(input, 93, 4)?,
            terminal_alternate_airport_heliport_icao_code:    RecordField::from_bytes(input, 97, 2)?,
            terminal_alternate_airport_heliport_section:      RecordField::from_bytes(input, 99, 1)?,
            terminal_alternate_airport_heliport_subsection:   RecordField::from_bytes(input, 100, 1)?,
            alternate_distance:                               RecordField::from_bytes(input, 101, 4)?,
            cost_index:                                       RecordField::from_bytes(input, 105, 3)?,
            enroute_alternate_airport_heliport:               RecordField::from_bytes(input, 108, 4)?,
            file_record_number:                               RecordField::from_bytes(input, 124,5)?,
            cycle_date:                                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.2.7.1(C) Helicopter Company Approach Route Primary Record
#[derive(Debug)]
pub struct HelicopterCompanyApproachRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub approach_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub approach_section_code: RecordField<'a, Section>,
    pub approach_subsection_code: RecordField<'a, GenericSubsection>,
    pub approach_route_type: RecordField<'a, AirportHeliportApproachRouteType>,
    pub approach_route_type_qualifier_1: RecordField<'a, RouteTypeQualifier1>,
    pub approach_route_type_qualifier_2: RecordField<'a, RouteTypeQualifier2>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, CompanyRouteToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_helipad_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport_heliport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_heliport_icao_code: RecordField<'a, IcaoCode>,
    pub terminal_alternate_airport_heliport_section: RecordField<'a, Section>,
    pub terminal_alternate_airport_heliport_subsection: RecordField<'a, GenericSubsection>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport_heliport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterCompanyApproachRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(HelicopterCompanyApproachRoutePrimaryRecord {
            record_type:                                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                               RecordField::from_bytes(input, 2,3)?,
            section:                                          RecordField::from_bytes(input, 5, 1)?,
            subsection:                                       RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                                  RecordField::from_bytes(input, 7, 5)?,
            from_helipad_identifier:                          RecordField::from_bytes(input, 12, 5)?,
            from_icao_code:                                   RecordField::from_bytes(input, 18, 2)?,
            from_section_code:                                RecordField::from_bytes(input, 20, 1)?,
            from_subsection_code:                             RecordField::from_bytes(input, 21, 1)?,
            to_identifier:                                    RecordField::from_bytes(input, 22, 5)?,
            to_helipad_identifier:                            RecordField::from_bytes(input, 27, 5)?,
            to_icao_code:                                     RecordField::from_bytes(input, 33, 2)?,
            to_section_code:                                  RecordField::from_bytes(input, 35, 1)?,
            to_subsection_code:                               RecordField::from_bytes(input, 36, 1)?,
            company_route_id:                                 RecordField::from_bytes(input, 37, 10)?,
            sequence_number:                                  RecordField::from_bytes(input, 47, 3)?,
            via_code:                                         RecordField::from_bytes(input, 50, 3)?,
            approach_identifier:                              RecordField::from_bytes(input, 53, 6)?,
            approach_section_code:                            RecordField::from_bytes(input, 59, 1)?,
            approach_subsection_code:                         RecordField::from_bytes(input, 60, 1)?,
            approach_route_type:                              RecordField::from_bytes(input, 61, 1)?,
            approach_route_type_qualifier_1:                  RecordField::from_bytes(input, 62, 1)?,
            approach_route_type_qualifier_2:                  RecordField::from_bytes(input, 63, 1)?,
            area:                                             RecordField::from_bytes(input, 64, 3)?,
            to_fix:                                           RecordField::from_bytes(input, 67, 6)?,
            to_fix_icao_code:                                 RecordField::from_bytes(input, 73, 2)?,
            to_fix_section:                                   RecordField::from_bytes(input, 75, 1)?,
            to_fix_subsection:                                RecordField::from_bytes(input, 76, 1)?,
            runway_helipad_transition:                        RecordField::from_bytes(input, 77, 5)?,
            enroute_transition:                               RecordField::from_bytes(input, 82, 5)?,
            cruise_altitude:                                  RecordField::from_bytes(input, 88, 5)?,
            terminal_alternate_airport_heliport:              RecordField::from_bytes(input, 93, 4)?,
            terminal_alternate_airport_heliport_icao_code:    RecordField::from_bytes(input, 97, 2)?,
            terminal_alternate_airport_heliport_section:      RecordField::from_bytes(input, 99, 1)?,
            terminal_alternate_airport_heliport_subsection:   RecordField::from_bytes(input, 100, 1)?,
            alternate_distance:                               RecordField::from_bytes(input, 101, 4)?,
            cost_index:                                       RecordField::from_bytes(input, 105, 3)?,
            enroute_alternate_airport_heliport:               RecordField::from_bytes(input, 108, 4)?,
            file_record_number:                               RecordField::from_bytes(input, 124,5)?,
            cycle_date:                                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.2.7.1(D) Helicopter Company Airway Route Primary Record
#[derive(Debug)]
pub struct HelicopterCompanyAirwayRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub airway_identifier: RecordField<'a, SidStarApproachAirwayIdentifier>,
    pub airway_section_code: RecordField<'a, Section>,
    pub airway_subsection_code: RecordField<'a, GenericSubsection>,
    pub airway_route_type: RecordField<'a, EnrouteAirwayRouteType>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, CompanyRouteToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_helipad_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport_heliport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_heliport_icao_code: RecordField<'a, IcaoCode>,
    pub terminal_alternate_airport_heliport_section: RecordField<'a, Section>,
    pub terminal_alternate_airport_heliport_subsection: RecordField<'a, GenericSubsection>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport_heliport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterCompanyAirwayRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(HelicopterCompanyAirwayRoutePrimaryRecord {
            record_type:                                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                               RecordField::from_bytes(input, 2,3)?,
            section:                                          RecordField::from_bytes(input, 5, 1)?,
            subsection:                                       RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                                  RecordField::from_bytes(input, 7, 5)?,
            from_helipad_identifier:                          RecordField::from_bytes(input, 12, 5)?,
            from_icao_code:                                   RecordField::from_bytes(input, 18, 2)?,
            from_section_code:                                RecordField::from_bytes(input, 20, 1)?,
            from_subsection_code:                             RecordField::from_bytes(input, 21, 1)?,
            to_identifier:                                    RecordField::from_bytes(input, 22, 5)?,
            to_helipad_identifier:                            RecordField::from_bytes(input, 27, 5)?,
            to_icao_code:                                     RecordField::from_bytes(input, 33, 2)?,
            to_section_code:                                  RecordField::from_bytes(input, 35, 1)?,
            to_subsection_code:                               RecordField::from_bytes(input, 36, 1)?,
            company_route_id:                                 RecordField::from_bytes(input, 37, 10)?,
            sequence_number:                                  RecordField::from_bytes(input, 47, 3)?,
            via_code:                                         RecordField::from_bytes(input, 50, 3)?,
            airway_identifier:                                RecordField::from_bytes(input, 53, 6)?,
            airway_section_code:                              RecordField::from_bytes(input, 59, 1)?,
            airway_subsection_code:                           RecordField::from_bytes(input, 60, 1)?,
            airway_route_type:                                RecordField::from_bytes(input, 61, 1)?,
            area:                                             RecordField::from_bytes(input, 64, 3)?,
            to_fix:                                           RecordField::from_bytes(input, 67, 6)?,
            to_fix_icao_code:                                 RecordField::from_bytes(input, 73, 2)?,
            to_fix_section:                                   RecordField::from_bytes(input, 75, 1)?,
            to_fix_subsection:                                RecordField::from_bytes(input, 76, 1)?,
            runway_helipad_transition:                        RecordField::from_bytes(input, 77, 5)?,
            enroute_transition:                               RecordField::from_bytes(input, 82, 5)?,
            cruise_altitude:                                  RecordField::from_bytes(input, 88, 5)?,
            terminal_alternate_airport_heliport:              RecordField::from_bytes(input, 93, 4)?,
            terminal_alternate_airport_heliport_icao_code:    RecordField::from_bytes(input, 97, 2)?,
            terminal_alternate_airport_heliport_section:      RecordField::from_bytes(input, 99, 1)?,
            terminal_alternate_airport_heliport_subsection:   RecordField::from_bytes(input, 100, 1)?,
            alternate_distance:                               RecordField::from_bytes(input, 101, 4)?,
            cost_index:                                       RecordField::from_bytes(input, 105, 3)?,
            enroute_alternate_airport_heliport:               RecordField::from_bytes(input, 108, 4)?,
            file_record_number:                               RecordField::from_bytes(input, 124,5)?,
            cycle_date:                                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.2.7.1(E) Helicopter Company General Route Primary Record
///
/// Note: This is meant to be a catchall for ALT/DIR/INT/PRE via codes
#[derive(Debug)]
pub struct HelicopterCompanyGeneralRoutePrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub from_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub from_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub from_icao_code: RecordField<'a, IcaoCode>,
    pub from_section_code: RecordField<'a, Section>,
    pub from_subsection_code: RecordField<'a, GenericSubsection>,
    pub to_identifier: RecordField<'a, FromToAirportHeliportFix>,
    pub to_helipad_identifier: RecordField<'a, PadIdentifier>,
    pub to_icao_code: RecordField<'a, IcaoCode>,
    pub to_section_code: RecordField<'a, Section>,
    pub to_subsection_code: RecordField<'a, GenericSubsection>,
    pub company_route_id: RecordField<'a, CompanyRouteIdent>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    pub via_code: RecordField<'a, CompanyRouteVIACode>,
    pub area: RecordField<'a, CustomerAreaCode>,
    pub to_fix: RecordField<'a, CompanyRouteToFix>,
    pub to_fix_icao_code: RecordField<'a, IcaoCode>,
    pub to_fix_section: RecordField<'a, Section>,
    pub to_fix_subsection: RecordField<'a, GenericSubsection>,
    pub runway_helipad_transition: RecordField<'a, RunwayTransition>,
    pub enroute_transition: RecordField<'a, EnrouteTransition>,
    pub cruise_altitude: RecordField<'a, CruiseAltitude>,
    pub terminal_alternate_airport_heliport: RecordField<'a, TerminalAlternateAirport>,
    pub terminal_alternate_airport_heliport_icao_code: RecordField<'a, IcaoCode>,
    pub terminal_alternate_airport_heliport_section: RecordField<'a, Section>,
    pub terminal_alternate_airport_heliport_subsection: RecordField<'a, GenericSubsection>,
    pub alternate_distance: RecordField<'a, AlternateDistance>,
    pub cost_index: RecordField<'a, CostIndex>,
    pub enroute_alternate_airport_heliport: RecordField<'a, EnrouteAlternateAirportHeliport>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterCompanyGeneralRoutePrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(HelicopterCompanyGeneralRoutePrimaryRecord {
            record_type:                                      RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                               RecordField::from_bytes(input, 2,3)?,
            section:                                          RecordField::from_bytes(input, 5, 1)?,
            subsection:                                       RecordField::from_bytes(input, 6, 1)?,
            from_identifier:                                  RecordField::from_bytes(input, 7, 5)?,
            from_helipad_identifier:                          RecordField::from_bytes(input, 12, 5)?,
            from_icao_code:                                   RecordField::from_bytes(input, 18, 2)?,
            from_section_code:                                RecordField::from_bytes(input, 20, 1)?,
            from_subsection_code:                             RecordField::from_bytes(input, 21, 1)?,
            to_identifier:                                    RecordField::from_bytes(input, 22, 5)?,
            to_helipad_identifier:                            RecordField::from_bytes(input, 27, 5)?,
            to_icao_code:                                     RecordField::from_bytes(input, 33, 2)?,
            to_section_code:                                  RecordField::from_bytes(input, 35, 1)?,
            to_subsection_code:                               RecordField::from_bytes(input, 36, 1)?,
            company_route_id:                                 RecordField::from_bytes(input, 37, 10)?,
            sequence_number:                                  RecordField::from_bytes(input, 47, 3)?,
            via_code:                                         RecordField::from_bytes(input, 50, 3)?,
            area:                                             RecordField::from_bytes(input, 64, 3)?,
            to_fix:                                           RecordField::from_bytes(input, 67, 6)?,
            to_fix_icao_code:                                 RecordField::from_bytes(input, 73, 2)?,
            to_fix_section:                                   RecordField::from_bytes(input, 75, 1)?,
            to_fix_subsection:                                RecordField::from_bytes(input, 76, 1)?,
            runway_helipad_transition:                        RecordField::from_bytes(input, 77, 5)?,
            enroute_transition:                               RecordField::from_bytes(input, 82, 5)?,
            cruise_altitude:                                  RecordField::from_bytes(input, 88, 5)?,
            terminal_alternate_airport_heliport:              RecordField::from_bytes(input, 93, 4)?,
            terminal_alternate_airport_heliport_icao_code:    RecordField::from_bytes(input, 97, 2)?,
            terminal_alternate_airport_heliport_section:      RecordField::from_bytes(input, 99, 1)?,
            terminal_alternate_airport_heliport_subsection:   RecordField::from_bytes(input, 100, 1)?,
            alternate_distance:                               RecordField::from_bytes(input, 101, 4)?,
            cost_index:                                       RecordField::from_bytes(input, 105, 3)?,
            enroute_alternate_airport_heliport:               RecordField::from_bytes(input, 108, 4)?,
            file_record_number:                               RecordField::from_bytes(input, 124,5)?,
            cycle_date:                                       RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
