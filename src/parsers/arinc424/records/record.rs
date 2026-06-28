//! # ARINC 424 Records
//! This module contains the record types for the ARINC 424 data.
//! Records are the top-level container for the ARINC 424 data.
//! They are used to group together related data into a single entity.
//!

use crate::parsers::arinc424::fields::{BLANK, FieldParseError, ParseableField};
use crate::parsers::arinc424::records::airport::*;
use crate::parsers::arinc424::records::airport_approach::*;
use crate::parsers::arinc424::records::airport_comms::*;
use crate::parsers::arinc424::records::airport_gate::*;
use crate::parsers::arinc424::records::airport_helipad::*;
use crate::parsers::arinc424::records::airport_msa::*;
use crate::parsers::arinc424::records::airport_sbas::*;
use crate::parsers::arinc424::records::airport_sid::*;
use crate::parsers::arinc424::records::airport_star::*;
use crate::parsers::arinc424::records::airport_taa::*;
use crate::parsers::arinc424::records::airway_marker::*;
use crate::parsers::arinc424::records::alternate::*;
use crate::parsers::arinc424::records::atn::*;
use crate::parsers::arinc424::records::communication_type_translation::*;
use crate::parsers::arinc424::records::company_route::*;
use crate::parsers::arinc424::records::controlled_airspace::*;
use crate::parsers::arinc424::records::cruising_table::*;
use crate::parsers::arinc424::records::enroute_airway::*;
use crate::parsers::arinc424::records::enroute_airway_restriction::*;
use crate::parsers::arinc424::records::enroute_comms::*;
use crate::parsers::arinc424::records::enroute_waypoint::*;
use crate::parsers::arinc424::records::fir_uir::*;
use crate::parsers::arinc424::records::flight_planning_data::*;
use crate::parsers::arinc424::records::gbas_path_point::*;
use crate::parsers::arinc424::records::geo_ref_table::*;
use crate::parsers::arinc424::records::gls::*;
use crate::parsers::arinc424::records::grid_mora::*;
use crate::parsers::arinc424::records::helicopter_company_route::*;
use crate::parsers::arinc424::records::helicopter_sbas::*;
use crate::parsers::arinc424::records::heliport::*;
use crate::parsers::arinc424::records::heliport_approach::*;
use crate::parsers::arinc424::records::heliport_comms::*;
use crate::parsers::arinc424::records::heliport_helipad::*;
use crate::parsers::arinc424::records::heliport_msa::*;
use crate::parsers::arinc424::records::heliport_sid::*;
use crate::parsers::arinc424::records::heliport_star::*;
use crate::parsers::arinc424::records::heliport_taa::*;
use crate::parsers::arinc424::records::heliport_terminal_waypoint::*;
use crate::parsers::arinc424::records::holding_pattern::*;
use crate::parsers::arinc424::records::localizer_glideslope::*;
use crate::parsers::arinc424::records::localizer_marker::*;
use crate::parsers::arinc424::records::mls::*;
use crate::parsers::arinc424::records::ndb_navaid::*;
use crate::parsers::arinc424::records::preferred_route::*;
use crate::parsers::arinc424::records::restrictive_airspace::*;
use crate::parsers::arinc424::records::runway::*;
use crate::parsers::arinc424::records::special_activity::*;
use crate::parsers::arinc424::records::tacan_navaid::*;
use crate::parsers::arinc424::records::terminal_waypoint::*;
use crate::parsers::arinc424::records::vhf_navaid::*;

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

#[derive(Debug)]
pub struct RecordField<'a, T> {
    pub raw_bytes: &'a [u8],
    pub value: Option<T>,
}
impl<'a, T: ParseableField> RecordField<'a, T> {
    pub fn from_bytes(
        input: &'a [u8],
        column: usize,
        length: usize,
    ) -> Result<Self, FieldParseError> {
        // to make it 1:1 with the spec, let's use 1-indexed columns
        let value = T::from_bytes(&input[column - 1..column - 1 + length])?;
        Ok(Self {
            raw_bytes: &input[column - 1..column - 1 + length],
            value,
        })
    }
}

/// ARINC 424 Record Sum Type for all possible record types.
#[derive(Debug)]
pub enum ARINCRecord<'a> {
    VHFNavaidPrimary(VHFNavaidPrimaryRecord<'a>),
    VHFNavaidContinuation(VHFNavaidContinuationRecord<'a>),
    VHFNavaidSimulationContinuation(VHFNavaidSimulationContinuationRecord<'a>),
    VHFNavaidFlightPlanningContinuation(VHFNavaidFlightPlanningContinuationRecord<'a>),
    VHFNavaidLimitationContinuation(VHFNavaidLimitationContinuationRecord<'a>),

    NDBNavaidPrimary(NDBNavaidPrimaryRecord<'a>),
    NDBNavaidContinuation(NDBNavaidContinuationRecord<'a>),
    NDBNavaidSimulationContinuation(NDBNavaidSimulationContinuationRecord<'a>),
    NDBNavaidFlightPlanningContinuation(NDBNavaidFlightPlanningContinuationRecord<'a>),

    EnrouteWaypointPrimary(EnrouteWaypointPrimaryRecord<'a>),
    EnrouteWaypointContinuation(EnrouteWaypointContinuationRecord<'a>),
    EnrouteWaypointFlightPlanningContinuation(EnrouteWaypointFlightPlanningContinuationRecord<'a>),

    TerminalWaypointPrimary(TerminalWaypointPrimaryRecord<'a>),
    TerminalWaypointContinuation(TerminalWaypointContinuationRecord<'a>),
    TerminalWaypointFlightPlanningContinuation(
        TerminalWaypointFlightPlanningContinuationRecord<'a>,
    ),

    HoldingPatternPrimary(HoldingPatternPrimaryRecord<'a>),
    HoldingPatternContinuation(HoldingPatternContinuationRecord<'a>),
    HoldingPatternPrimaryExtensionContinuation(
        HoldingPatternPrimaryExtensionContinuationRecord<'a>,
    ),

    EnrouteAirwayPrimary(EnrouteAirwayPrimaryRecord<'a>),
    EnrouteAirwayContinuation(EnrouteAirwayContinuationRecord<'a>),
    EnrouteAirwayFlightPlanningContinuation(EnrouteAirwayFlightPlanningContinuationRecord<'a>),

    AirportPrimary(AirportPrimaryRecord<'a>),
    AirportContinuation(AirportContinuationRecord<'a>),
    AirportFlightPlanningContinuation(AirportFlightPlanningContinuationRecord<'a>),

    AirportGatePrimary(AirportGatePrimaryRecord<'a>),
    AirportGateContinuation(AirportGateContinuationRecord<'a>),

    AirportSIDPrimary(AirportSIDPrimaryRecord<'a>),
    AirportSIDPrimaryExtensionContinuation(AirportSIDPrimaryExtensionContinuationRecord<'a>),
    AirportSIDFlightPlanningContinuation(AirportSIDFlightPlanningContinuationRecord<'a>),
    AirportSIDProcedureNameContinuation(AirportSIDProcedureNameContinuationRecord<'a>),

    AirportSTARPrimary(AirportSTARPrimaryRecord<'a>),
    AirportSTARPrimaryExtensionContinuation(AirportSTARPrimaryExtensionContinuationRecord<'a>),
    AirportSTARFlightPlanningContinuation(AirportSTARFlightPlanningContinuationRecord<'a>),
    AirportSTARProcedureNameContinuation(AirportSTARProcedureNameContinuationRecord<'a>),

    AirportApproachMSACenterFixPrimary(AirportApproachMSACenterFixPrimaryRecord<'a>),
    AirportApproachTAAPrimary(AirportApproachTAAPrimaryRecord<'a>),
    AirportApproachPrimaryExtensionContinuation(
        AirportApproachPrimaryExtensionContinuationRecord<'a>,
    ),
    AirportApproachFlightPlanningContinuation(AirportApproachFlightPlanningContinuationRecord<'a>),
    AirportApproachProcedureNameContinuation(AirportApproachProcedureNameContinuationRecord<'a>),
    AirportApproachProcedureDataContinuation(AirportApproachProcedureDataContinuationRecord<'a>),

    RunwayPrimary(RunwayPrimaryRecord<'a>),
    RunwayContinuation(RunwayContinuationRecord<'a>),
    RunwaySimulationContinuation(RunwaySimulationContinuationRecord<'a>),

    AirportLocalizerGlideslopePrimary(AirportLocalizerGlideslopePrimaryRecord<'a>),
    HeliportLocalizerGlideslopePrimary(HeliportLocalizerGlideslopePrimaryRecord<'a>),
    LocalizerGlideslopeContinuation(LocalizerGlideslopeContinuationRecord<'a>),
    LocalizerGlideslopeSimulationContinuation(LocalizerGlideslopeSimulationContinuationRecord<'a>),

    CompanySIDRoutePrimary(CompanySIDRoutePrimaryRecord<'a>),
    CompanySTARRoutePrimary(CompanySTARRoutePrimaryRecord<'a>),
    CompanyApproachRoutePrimary(CompanyApproachRoutePrimaryRecord<'a>),
    CompanyAirwayRoutePrimary(CompanyAirwayRoutePrimaryRecord<'a>),
    CompanyGeneralRoutePrimary(CompanyGeneralRoutePrimaryRecord<'a>),

    AirportLocalizerMarkerPrimary(AirportLocalizerMarkerPrimaryRecord<'a>),
    HeliportLocalizerMarkerPrimary(HeliportLocalizerMarkerPrimaryRecord<'a>),
    AirportLocalizerMarkerContinuation(AirportLocalizerMarkerContinuationRecord<'a>),
    HeliportLocalizerMarkerContinuation(HeliportLocalizerMarkerContinuationRecord<'a>),

    AirportCommsPrimary(AirportCommsPrimaryRecord<'a>),
    AirportCommsPrimaryExtensionContinuation(AirportCommsPrimaryExtensionContinuationRecord<'a>),
    AirportCommsSectorNarrativeContinuation(AirportCommsSectorNarrativeContinuationRecord<'a>),
    AirportCommsFormattedTimeContinuation(AirportCommsFormattedTimeContinuationRecord<'a>),
    AirportCommsNarrativeTimeContinuation(AirportCommsNarrativeTimeContinuationRecord<'a>),
    AirportCommsAdditionalSectorizationContinuation(
        AirportCommsAdditionalSectorizationContinuationRecord<'a>,
    ),

    AirwaysMarkerPrimary(AirwaysMarkerPrimaryRecord<'a>),
    AirwaysMarkerContinuation(AirwaysMarkerContinuationRecord<'a>),

    CruisingTablePrimary(CruisingTablePrimaryRecord<'a>),

    FIRUIRPrimary(FIRUIRPrimaryRecord<'a>),
    FIRUIRContinuation(FIRUIRContinuationRecord<'a>),

    RestrictiveAirspacePrimary(RestrictiveAirspacePrimaryRecord<'a>),
    RestrictiveAirspaceFormattedTimeContinuation(
        RestrictiveAirspaceFormattedTimeContinuationRecord<'a>,
    ),
    RestrictiveAirspaceNarrativeTimeContinuation(
        RestrictiveAirspaceNarrativeTimeContinuationRecord<'a>,
    ),
    RestrictiveAirspaceControllingAgencyContinuation(
        RestrictiveAirspaceControllingAgencyContinuationRecord<'a>,
    ),

    GridMORAPrimary(GridMORAPrimaryRecord<'a>),

    AirportMSAPrimary(AirportMSAPrimaryRecord<'a>),
    AirportMSAPrimaryExtensionContinuation(AirportMSAPrimaryExtensionContinuationRecord<'a>),
    AirportMSAContinuation(AirportMSAContinuationRecord<'a>),

    EnrouteAirwayRestrictionAltitudeExclusionPrimary(
        EnrouteAirwayRestrictionAltitudeExclusionPrimaryRecord<'a>,
    ),
    EnrouteAirwayRestrictionAltitudeExclusionPrimaryExtensionContinuation(
        EnrouteAirwayRestrictionAltitudeExclusionPrimaryExtensionContinuationRecord<'a>,
    ),
    EnrouteAirwayRestrictionAltitudeExclusionFormattedTimeContinuation(
        EnrouteAirwayRestrictionAltitudeExclusionFormattedTimeContinuationRecord<'a>,
    ),
    EnrouteAirwayRestrictionAltitudeExclusionNarrativeTimeContinuation(
        EnrouteAirwayRestrictionAltitudeExclusionNarrativeTimeContinuationRecord<'a>,
    ),

    EnrouteAirwayRestrictionNoteRestrictionPrimary(
        EnrouteAirwayRestrictionNoteRestrictionPrimaryRecord<'a>,
    ),
    EnrouteAirwayRestrictionNoteRestrictionContinuation(
        EnrouteAirwayRestrictionNoteRestrictionContinuationRecord<'a>,
    ),

    EnrouteAirwayRestrictionSeasonalClosurePrimary(
        EnrouteAirwayRestrictionSeasonalClosurePrimaryRecord<'a>,
    ),
    EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuation(
        EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuationRecord<'a>,
    ),
    EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuation(
        EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuationRecord<'a>,
    ),

    EnrouteAirwayRestrictionCruisingTableReplacementPrimary(
        EnrouteAirwayRestrictionCruisingTableReplacementPrimaryRecord<'a>,
    ),
    EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuation(
        EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuationRecord<'a>,
    ),
    EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuation(
        EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuationRecord<'a>,
    ),

    MLSPrimary(MLSPrimaryRecord<'a>),
    MLSContinuation(MLSContinuationRecord<'a>),

    EnrouteCommsPrimary(EnrouteCommsPrimaryRecord<'a>),
    EnrouteCommsPrimaryExtensionContinuation(EnrouteCommsPrimaryExtensionContinuationRecord<'a>),
    EnrouteCommsFormattedTimeContinuation(EnrouteCommsFormattedTimeContinuationRecord<'a>),
    EnrouteCommsNarrativeTimeContinuation(EnrouteCommsNarrativeTimeContinuationRecord<'a>),

    PreferredSIDRoutePrimary(PreferredSIDRoutePrimaryRecord<'a>),
    PreferredSTARRoutePrimary(PreferredSTARRoutePrimaryRecord<'a>),
    PreferredAirwayRoutePrimary(PreferredAirwayRoutePrimaryRecord<'a>),
    PreferredGeneralRoutePrimary(PreferredGeneralRoutePrimaryRecord<'a>),
    PreferredRouteFormattedTimeContinuation(PreferredRouteFormattedTimeContinuationRecord<'a>),
    PreferredRouteContinuation(PreferredRouteContinuationRecord<'a>),
    PreferredRouteNarrativeTimeContinuation(PreferredRouteNarrativeTimeContinuationRecord<'a>),

    ControlledAirspacePrimary(ControlledAirspacePrimaryRecord<'a>),
    ControlledAirspaceFormattedTimeContinuation(
        ControlledAirspaceFormattedTimeContinuationRecord<'a>,
    ),
    ControlledAirspacePrimaryExtensionContinuation(
        ControlledAirspacePrimaryExtensionContinuationRecord<'a>,
    ),
    ControlledAirspaceNarrativeTimeContinuation(
        ControlledAirspaceNarrativeTimeContinuationRecord<'a>,
    ),
    ControlledAirspaceControllingAgencyContinuation(
        ControlledAirspaceControllingAgencyContinuationRecord<'a>,
    ),

    GeographicalReferenceTablePrimary(GeographicalReferenceTablePrimaryRecord<'a>),
    GeographicalReferenceTableContinuation(GeographicalReferenceTableContinuationRecord<'a>),

    FlightPlanningSIDSTARDataPrimary(FlightPlanningSIDSTARDataPrimaryRecord<'a>),
    FlightPlanningSIDSTARDataPrimaryExtensionContinuation(
        FlightPlanningSIDSTARPrimaryExtensionContinuationRecord<'a>,
    ),
    FlightPlanningSIDSTARDataFormattedTimeContinuation(
        FlightPlanningSIDSTARFormattedTimeContinuationRecord<'a>,
    ),
    FlightPlanningSIDSTARDataNarrativeTimeContinuation(
        FlightPlanningSIDSTARNarrativeTimeContinuationRecord<'a>,
    ),
    FlightPlanningApproachDataPrimary(FlightPlanningApproachDataPrimaryRecord<'a>),
    FlightPlanningApproachDataPrimaryExtensionContinuation(
        FlightPlanningApproachPrimaryExtensionContinuationRecord<'a>,
    ),
    FlightPlanningApproachDataFormattedTimeContinuation(
        FlightPlanningApproachFormattedTimeContinuationRecord<'a>,
    ),
    FlightPlanningApproachDataNarrativeTimeContinuation(
        FlightPlanningApproachNarrativeTimeContinuationRecord<'a>,
    ),

    AirportRunwaySBASPathPointPrimary(AirportRunwaySBASPathPointPrimaryRecord<'a>),
    AirportFinalApproachCourseAsRunwaySBASPathPointPrimary(
        AirportFinalApproachCourseAsRunwaySBASPathPointPrimaryRecord<'a>,
    ),
    AirportRunwaySBASPathPointContinuation(AirportRunwaySBASPathPointContinuationRecord<'a>),
    AirportFinalApproachCourseAsRunwaySBASPathPointContinuation(
        AirportFinalApproachCourseAsRunwaySBASPathPointContinuationRecord<'a>,
    ),

    AirportGLSPrimary(AirportGLSPrimaryRecord<'a>),
    AirportGLSContinuation(AirportGLSContinuationRecord<'a>),
    HeliportGLSPrimary(HeliportGLSPrimaryRecord<'a>),
    HeliportGLSContinuation(HeliportGLSContinuationRecord<'a>),

    AlternatePrimary(AlternatePrimaryRecord<'a>),

    AirportTAAPrimary(AirportTAAPrimaryRecord<'a>),
    AirportTAAContinuation(AirportTAAContinuationRecord<'a>),

    TACANOnlyNavaidPrimary(TACANOnlyNavaidPrimaryRecord<'a>),
    TACANOnlyNavaidContinuation(TACANOnlyNavaidContinuationRecord<'a>),
    TACANOnlyNavaidSimulationContinuation(TACANOnlyNavaidSimulationContinuationRecord<'a>),
    TACANOnlyNavaidFlightPlanningContinuation(TACANOnlyNavaidFlightPlanningContinuationRecord<'a>),
    TACANOnlyNavaidLimitationContinuation(TACANOnlyNavaidLimitationContinuationRecord<'a>),

    SpecialActivityAreaPrimary(SpecialActivityAreaPrimaryRecord<'a>),

    CommunicationTypeTranslationPrimary(CommunicationTypeTranslationPrimaryRecord<'a>),

    AirportRunwayGBASPathPointPrimary(AirportRunwayGBASPathPointPrimaryRecord<'a>),
    AirportFinalApproachCourseAsRunwayGBASPathPointPrimary(
        AirportFinalApproachCourseAsRunwayGBASPathPointPrimaryRecord<'a>,
    ),
    AirportRunwayGBASPathPointContinuation(AirportRunwayGBASPathPointContinuationRecord<'a>),
    AirportFinalApproachCourseAsRunwayGBASPathPointContinuation(
        AirportFinalApproachCourseAsRunwayGBASPathPointContinuationRecord<'a>,
    ),

    AirportHelipadPrimary(HelipadPrimaryRecord<'a>),

    ATNDataPrimary(ATNDataPrimaryRecord<'a>),

    HeliportPrimary(HeliportPrimaryRecord<'a>),
    HeliportContinuation(HeliportContinuationRecord<'a>),
    HeliportFlightPlanningContinuation(HeliportFlightPlanningContinuationRecord<'a>),

    HeliportTerminalWaypointPrimary(HeliportTerminalWaypointPrimaryRecord<'a>),
    HeliportTerminalWaypointContinuation(HeliportTerminalWaypointContinuationRecord<'a>),
    HeliportTerminalWaypointFlightPlanningContinuation(
        HeliportTerminalWaypointFlightPlanningContinuationRecord<'a>,
    ),

    HeliportSIDPrimary(HeliportSIDPrimaryRecord<'a>),
    HeliportSIDPrimaryExtensionContinuation(HeliportSIDPrimaryExtensionContinuationRecord<'a>),
    HeliportSIDFlightPlanningContinuation(HeliportSIDFlightPlanningContinuationRecord<'a>),
    HeliportSIDProcedureNameContinuation(HeliportSIDProcedureNameContinuationRecord<'a>),

    HeliportSTARPrimary(HeliportSTARPrimaryRecord<'a>),
    HeliportSTARPrimaryExtensionContinuation(HeliportSTARPrimaryExtensionContinuationRecord<'a>),
    HeliportSTARFlightPlanningContinuation(HeliportSTARFlightPlanningContinuationRecord<'a>),
    HeliportSTARProcedureNameContinuation(HeliportSTARProcedureNameContinuationRecord<'a>),

    HeliportApproachMSACenterFixPrimary(HeliportApproachMSACenterFixPrimaryRecord<'a>),
    HeliportApproachTAAPrimary(HeliportApproachTAAPrimaryRecord<'a>),
    HeliportApproachPrimaryExtensionContinuation(
        HeliportApproachPrimaryExtensionContinuationRecord<'a>,
    ),
    HeliportApproachFlightPlanningContinuation(
        HeliportApproachFlightPlanningContinuationRecord<'a>,
    ),
    HeliportApproachProcedureNameContinuation(HeliportApproachProcedureNameContinuationRecord<'a>),
    HeliportApproachProcedureDataContinuation(HeliportApproachProcedureDataContinuationRecord<'a>),

    HeliportMSAPrimary(HeliportMSAPrimaryRecord<'a>),
    HeliportMSAPrimaryExtensionContinuation(HeliportMSAPrimaryExtensionContinuationRecord<'a>),
    HeliportMSAContinuation(HeliportMSAContinuationRecord<'a>),

    HeliportCommsPrimary(HeliportCommsPrimaryRecord<'a>),
    HeliportCommsPrimaryExtensionContinuation(HeliportCommsPrimaryExtensionContinuationRecord<'a>),
    HeliportCommsSectorNarrativeContinuation(HeliportCommsSectorNarrativeContinuationRecord<'a>),
    HeliportCommsFormattedTimeContinuation(HeliportCommsFormattedTimeContinuationRecord<'a>),
    HeliportCommsNarrativeTimeContinuation(HeliportCommsNarrativeTimeContinuationRecord<'a>),
    HeliportCommsAdditionalSectorizationContinuation(
        HeliportCommsAdditionalSectorizationContinuationRecord<'a>,
    ),

    HeliportTAAPrimary(HeliportTAAPrimaryRecord<'a>),
    HeliportTAAContinuation(HeliportTAAContinuationRecord<'a>),

    HelicopterCompanySIDRoutePrimary(HelicopterCompanySIDRoutePrimaryRecord<'a>),
    HelicopterCompanySTARRoutePrimary(HelicopterCompanySTARRoutePrimaryRecord<'a>),
    HelicopterCompanyApproachRoutePrimary(HelicopterCompanyApproachRoutePrimaryRecord<'a>),
    HelicopterCompanyAirwayRoutePrimary(HelicopterCompanyAirwayRoutePrimaryRecord<'a>),
    HelicopterCompanyGeneralRoutePrimary(HelicopterCompanyGeneralRoutePrimaryRecord<'a>),

    HelicopterRunwaySBASPathPointPrimary(HelicopterRunwaySBASPathPointPrimaryRecord<'a>),
    HelicopterFinalApproachCourseAsRunwaySBASPathPointPrimary(
        HelicopterFinalApproachCourseAsRunwaySBASPathPointPrimaryRecord<'a>,
    ),
    HelicopterRunwaySBASPathPointContinuation(HelicopterRunwaySBASPathPointContinuationRecord<'a>),
    HelicopterFinalApproachCourseAsRunwaySBASPathPointContinuation(
        HelicopterFinalApproachCourseAsRunwaySBASPathPointContinuationRecord<'a>,
    ),

    HeliportHelipadPrimary(HeliportHelipadPrimaryRecord<'a>),
}

impl<'a> ARINCRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        match input[4..6] {
            [b'A', b'S'] => GridMORARecords::parse(input),
            [b'D', BLANK] => VHFNavaidRecords::parse(input),
            [b'D', b'B'] => NDBNavaidRecords::parse(input),
            [b'D', b'T'] => TACANOnlyNavaidRecords::parse(input),
            [b'E', b'A'] => EnrouteWaypointRecords::parse(input),
            [b'E', b'M'] => AirwaysMarkerRecords::parse(input),
            [b'E', b'P'] => HoldingPatternRecords::parse(input),
            [b'E', b'R'] => EnrouteAirwayRecords::parse(input),
            [b'E', b'S'] => SpecialActivityRecords::parse(input),
            [b'E', b'T'] => PreferredRouteRecords::parse(input),
            [b'E', b'U'] => EnrouteAirwayRestrictionRecords::parse(input),
            [b'E', b'V'] => EnrouteCommsRecords::parse(input),
            [b'H', b'A'] => HeliportRecords::parse(input),
            [b'H', b'C'] => HeliportTerminalWaypointRecords::parse(input),
            [b'H', b'D'] => HeliportSIDRecords::parse(input),
            [b'H', b'E'] => HeliportSTARRecords::parse(input),
            [b'H', b'F'] => HeliportApproachRecords::parse(input),
            [b'H', b'H'] => HeliportHelipadRecords::parse(input),
            [b'H', b'K'] => HeliportTAARecords::parse(input),
            [b'H', b'S'] => HeliportMSARecords::parse(input),
            [b'H', b'P'] => HelicopterSBASRecords::parse(input),
            [b'H', b'V'] => HeliportCommsRecords::parse(input),
            [b'P', b'A'] => AirportRecords::parse(input),
            [b'P', b'B'] => AirportGateRecords::parse(input),
            [b'P', b'C'] => TerminalWaypointRecords::parse(input),
            [b'P', b'D'] => AirportSIDRecords::parse(input),
            [b'P', b'E'] => AirportSTARRecords::parse(input),
            [b'P', b'F'] => AirportApproachRecords::parse(input),
            [b'P', b'G'] => RunwayRecords::parse(input),
            [b'P', b'H'] => AirportHelipadRecords::parse(input),
            [b'P', b'I'] => LocalizerGlideslopeRecords::parse(input),
            [b'P', b'K'] => AirportTAARecords::parse(input),
            [b'P', b'L'] => MLSRecords::parse(input),
            [b'P', b'M'] => LocalizerMarkerRecords::parse(input),
            [b'P', b'N'] => NDBNavaidRecords::parse(input),
            [b'P', b'P'] => AirportSBASRecords::parse(input),
            [b'P', b'Q'] => AirportGBASRecords::parse(input),
            [b'P', b'R'] => FlightPlanningDataRecords::parse(input),
            [b'P', b'S'] => AirportMSARecords::parse(input),
            [b'P', b'T'] => GLSRecords::parse(input),
            [b'P', b'V'] => AirportCommsRecords::parse(input),
            [b'R', BLANK] => CompanyRouteRecords::parse(input),
            [b'R', b'A'] => AlternateRecords::parse(input),
            [b'R', b'H'] => HelicopterCompanyRouteRecords::parse(input),
            [b'T', b'C'] => CruisingTableRecords::parse(input),
            [b'T', b'G'] => GeographicalReferenceTableRecords::parse(input),
            [b'T', b'L'] => ATNRecords::parse(input),
            [b'T', b'V'] => CommunicationTypeTranslationRecords::parse(input),
            [b'U', b'C'] => ControlledAirspaceRecords::parse(input),
            [b'U', b'F'] => FIRUIRRecords::parse(input),
            [b'U', b'R'] => RestrictiveAirspaceRecords::parse(input),
            _ => Err(RecordParseError {
                message: "Invalid record type".to_string(),
            }),
        }
    }
}

// Layout dispatch helpers
pub(super) fn is_primary_record(input: &[u8], continuation_column: usize) -> bool {
    matches!(input[continuation_column - 1], b'0' | b'1' | BLANK)
}
