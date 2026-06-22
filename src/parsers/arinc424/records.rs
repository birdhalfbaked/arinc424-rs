//! # ARINC 424 Records
//! This module contains the record types for the ARINC 424 data.
//! Records are the top-level container for the ARINC 424 data.
//! They are used to group together related data into a single entity.
//!

use crate::parsers::arinc424::definitions::*;
use crate::parsers::arinc424::fields::{BLANK, FieldParseError};
#[derive(Debug)]
pub struct RecordParseError {
    pub message: String,
}
impl From<FieldParseError> for RecordParseError {
    fn from(error: FieldParseError) -> Self {
        Self {
            message: format!("Record parse error: {}", error.message),
        }
    }
}

/// Required field is used to represent a field that is required to be present in the record.
fn required_field<T>(value: Result<Option<T>, FieldParseError>) -> Result<T, FieldParseError> {
    match value {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(FieldParseError {
            message: "Required field is missing".to_string(),
        }),
        Err(error) => Err(error),
    }
}

/// Placeholder field is used to represent a field that is not yet implemented.
#[deprecated(note = "Replace this field with the appropriate record field type")]
#[derive(Debug)]
pub struct PlaceholderField {}
impl PlaceholderField {
    pub fn from_bytes(_bytes: &[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {})
    }
}

/// ARINC 424 Record Sum Type for all possible record types.
#[derive(Debug)]
pub enum ARINCRecord {
    VHFNavaidPrimary(VHFNavaidPrimaryRecord),
    VHFNavaidContinuation,
    VHFNavaidSimulationContinuation,
    VHFNavaidFlightPlanningContinuation,
    VHFNavaidLimitationContinuation,

    NDBNavaidPrimary,
    NDBNavaidContinuation,
    NDBNavaidSimulationContinuation,
    NDBNavaidFlightPlanningContinuation,
    NDBNavaidLimitationContinuation,

    WaypointPrimary,
    WaypointContinuation,
    WaypointSimulationContinuation,

    HoldingPatternPrimary,
    HoldingPatternContinuation,
    HoldingPatternPrimaryExtensionContinuation,

    EnrouteAirwaysPrimary,
    EnrouteAirwaysContinuation,
    EnrouteAirwaysFlightPlanningContinuation,

    AirportPrimary,
    AirportContinuation,
    AirportFlightPlanningContinuation,

    AirportGatePrimary,
    AirportGateContinuation,

    AirportSIDSTARApproachPrimary,
    AirportSIDSTARApproachPrimaryExtensionContinuation,
    AirportSIDSTARApproachFlightPlanningContinuation,
    AirportProcedureDataContinuation,
    AirportSIDSTARApproachProcedureNameContinuation,

    RunwayPrimary,
    RunwayContinuation,
    RunwaySimulationContinuation,

    LocalizerGlideslopePrimary,
    LocalizerGlideslopeContinuation,
    LocalizerGlideslopeSimulationContinuation,

    CompanyRoutePrimary,

    LocalizerMarkerPrimary,
    LocalizerMarkerContinuation,

    AirportCommsPrimary,
    AirportCommsPrimaryExtensionContinuation,
    AirportCommsSectorNarrativeContinuation,
    AirportCommsFormattedTimeContinuation,
    AirportCommsNarrativeTimeContinuation,
    AirportCommsAdditionalSectorizationContinuation,

    AirwaysMarkerPrimary,
    AirwaysMarkerContinuation,

    CruisingTablePrimary,

    FIRUIRPrimary,
    FIRUIRContinuation,

    RestrictiveAirspacePrimary,
    RestrictiveAirspaceFormattedTimeContinuation,
    RestrictiveAirspaceNarrativeTimeContinuation,

    GridMORAPrimary,

    AirportMSAPrimary,
    AirportMSAPrimaryExtensionContinuation,
    AirportMSAContinuation,

    EnrouteAirwayRestrictionAltitudePrimary,
    EnrouteAirwayRestrictionAltitudeContinuation,
    EnrouteAirwayRestrictionFormattedTimeContinuation,
    EnrouteAirwayRestrictionNarrativeTimeContinuation,

    EnrouteAirwayRestrictionNoteRestrictionPrimary,
    EnrouteAirwayRestrictionNoteRestrictionContinuation,

    EnrouteAirwayRestrictionSeasonalClosurePrimary,
    EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuation,
    EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuation,

    EnrouteAirwayRestrictionCruisingTableReplacementPrimary,
    EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuation,
    EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuation,

    MLSPrimary,
    MLSContinuation,

    EnrouteCommsPrimary,
    EnrouteCommsPrimaryExtensionContinuation,
    EnrouteCommsFormattedTimeContinuation,
    EnrouteCommsNarrativeTimeContinuation,

    PreferredRoutePrimary,
    PreferredRouteFormattedTimeContinuation,
    PreferredRouteNotesContinuation,
    PreferredRouteNarrativeTimeContinuation,

    ControlledAirspacePrimary,
    ControlledAirspaceFormattedTimeContinuation,
    ControlledAirspacePrimaryExtension,
    ControlledAirspaceNarrativeTimeContinuation,
    ControlledAirspaceControllingAgencyContinuation,

    GeographicalReferenceTablePrimary,
    GeographicalReferenceTableContinuation,

    FlightPlanningPrimary,
    FlightPlanningPrimaryExtensionContinuation,
    FlightPlanningFormattedTimeContinuation,
    FlightPlanningNarrativeTimeContinuation,

    SBASPathPointPrimary,
    SBASPathPointContinuation,

    GLSPrimary,
    GLSContinuation,

    AlternatePrimary,

    TAAPrimary,
    TAAContinuation,

    TACANOnlyNavaidPrimary,
    TACANOnlyNavaidContinuation,
    TACANOnlyNavaidSimulationContinuation,
    TACANOnlyNavaidFlightPlanningContinuation,
    TACANOnlyNavaidLimitationContinuation,

    SpecialActivityAreaPrimary,

    CommunicationTypeTranslationPrimary,

    GBASPathPointPrimary,
    GBASPathPointContinuation,

    AirportHelipadPrimary,

    ATNDataPrimary,

    HeliportPrimary,
    HeliportContinuation,
    HeliportFlightPlanningContinuation,

    HeliportTerminalWaypointPrimary,
    HeliportTerminalWaypointContinuation,
    HeliportTerminalWaypointFlightPlanningContinuation,

    HeliportSIDSTARApproachPrimary,
    HeliportSIDSTARApproachPrimaryExtensionContinuation,
    HeliportSIDSTARApproachFlightPlanningContinuation,
    HeliportProcedureDataContinuation,
    HeliportSIDSTARApproachProcedureNameContinuation,

    HeliportMSAPrimary,
    HelportMSAPrimaryExtensionContinuation,
    HeliportMSAContinuation,

    HeliportCommsPrimary,
    HeliportCommsPrimaryExtensionContinuation,
    HeliportCommsSectorNarrativeContinuation,
    HeliportCommsFormattedTimeContinuation,
    HeliportCommsNarrativeTimeContinuation,
    HeliportCommsAdditionalSectorizationContinuation,

    HeliportTAAPrimary,
    HeliportTAAContinuation,

    HelicopterCompanyRoutePrimary,

    HelicopterSBASPathPointPrimary,
    HelicopterSBASPathPointContinuation,

    HeliportHelipadPrimary,
}

impl ARINCRecord {
    pub fn parse(input: &[u8]) -> Result<Self, RecordParseError> {
        match input[4..6] {
            [b'D', subcode] => Ok(match subcode {
                BLANK => ARINCRecord::VHFNavaidPrimary(VHFNavaidPrimaryRecord::parse(input)?),
                _ => {
                    return Err(RecordParseError {
                        message: "Invalid record type".to_string(),
                    });
                }
            }),
            _ => Err(RecordParseError {
                message: "Invalid record type".to_string(),
            }),
        }
    }
}

/// 4.1.2.1 - VHFNavaid Primary Record
#[derive(Debug)]
pub struct VHFNavaidPrimaryRecord {
    pub record_type: RecordType,
    pub customer_area_code: Option<CustomerAreaCode>,
    pub section: Section,
    pub subsection: NavaidSubsection,
    pub airport_icao_identifier: Option<AirportHeliportIdentifier>,
    pub airport_icao_code: Option<IcaoCode>,
    pub vor_identifier: Option<VORNDBIdentifier>,
    pub vor_icao_code: Option<IcaoCode>,
    pub continuation_record_number: ContinuationRecordNumber,
    pub vor_frequency: Option<VORNDBFrequency>,
    pub navaid_class: VHFNavaidClass,
    pub vor_latitude: Option<Latitude>,
    pub vor_longitude: Option<Longitude>,
    pub dme_identifier: Option<DMEIdentifier>,
    pub dme_latitude: Option<Latitude>,
    pub dme_longitude: Option<Longitude>,
    pub station_declination: PlaceholderField,
    pub dme_elevation: PlaceholderField,
    pub navaid_usable_range: PlaceholderField,
    pub ils_dme_bias: PlaceholderField,
    pub frequency_protection: PlaceholderField,
    pub datum_code: PlaceholderField,
    pub vor_name: PlaceholderField,
    pub vfr_checkpoint_flag: PlaceholderField,
    pub vor_range_power: PlaceholderField,
    pub expanded_dme_service_volume: PlaceholderField,
    pub route_inappropriate_dme: PlaceholderField,
    pub dme_operational_service_volume: PlaceholderField,
    pub file_record_number: FileRecordNumber,
    pub cycle_date: CycleDate,
}

impl VHFNavaidPrimaryRecord {
    pub fn parse(input: &[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type: required_field(RecordType::from_bytes(&input[0..1]))?,
            customer_area_code: CustomerAreaCode::from_bytes(&input[1..4])?,
            section: required_field(Section::from_bytes(&input[4..5]))?,
            subsection: required_field(NavaidSubsection::from_bytes(&input[5..6]))?,
            airport_icao_identifier: AirportHeliportIdentifier::from_bytes(&input[6..10])?,
            airport_icao_code: IcaoCode::from_bytes(&input[10..12])?,
            vor_identifier: VORNDBIdentifier::from_bytes(&input[13..17])?,
            vor_icao_code: IcaoCode::from_bytes(&input[19..21])?,
            continuation_record_number: required_field(ContinuationRecordNumber::from_bytes(
                &input[21..22],
            ))?,
            vor_frequency: VORNDBFrequency::from_bytes(&input[22..27])?,
            navaid_class: required_field(VHFNavaidClass::from_bytes(&input[27..32]))?,
            vor_latitude: Latitude::from_bytes(&input[32..41])?,
            vor_longitude: Longitude::from_bytes(&input[41..51])?,
            dme_identifier: DMEIdentifier::from_bytes(&input[51..55])?,
            dme_latitude: Latitude::from_bytes(&input[55..64])?,
            dme_longitude: Longitude::from_bytes(&input[64..74])?,
            station_declination: PlaceholderField::from_bytes(&input[74..79])?,
            dme_elevation: PlaceholderField::from_bytes(&input[79..84])?,
            navaid_usable_range: PlaceholderField::from_bytes(&input[84..85])?,
            ils_dme_bias: PlaceholderField::from_bytes(&input[85..87])?,
            frequency_protection: PlaceholderField::from_bytes(&input[87..90])?,
            datum_code: PlaceholderField::from_bytes(&input[90..93])?,
            vor_name: PlaceholderField::from_bytes(&input[93..118])?,
            vfr_checkpoint_flag: PlaceholderField::from_bytes(&input[118..119])?,
            vor_range_power: PlaceholderField::from_bytes(&input[119..120])?,
            expanded_dme_service_volume: PlaceholderField::from_bytes(&input[120..121])?,
            route_inappropriate_dme: PlaceholderField::from_bytes(&input[121..122])?,
            dme_operational_service_volume: PlaceholderField::from_bytes(&input[122..123])?,
            file_record_number: required_field(FileRecordNumber::from_bytes(&input[123..128]))?,
            cycle_date: required_field(CycleDate::from_bytes(&input[128..132]))?,
        })
    }
}

#[test]
pub fn test_vhf_navaid_primary_record() {
    let input = b"SUSAD KFATK2 IRPW  K2011130 ITWN                   IRPWN36471081W119435663E0130003470     NARFRESNO YOSEMITE INTL          261851713";
    let record = VHFNavaidPrimaryRecord::parse(input).unwrap();
    // replace this later, just want to see it mapping correctly
    println!("{:?}", record);
}
