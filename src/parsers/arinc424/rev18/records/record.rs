//! # ARINC 424 Records
//! This module contains the record types for the ARINC 424 data.
//! Records are the top-level container for the ARINC 424 data.
//! They are used to group together related data into a single entity.
//!

use crate::parsers::arinc424::rev18::records::*;
use crate::parsers::arinc424::types::fields::BLANK;
use crate::parsers::arinc424::types::records::RecordParseError;

/// ARINC 424 Record Sum Type for all possible record types.
#[derive(Debug)]
pub enum ARINCRecord<'a> {
    VHFNavaidPrimary(VHFNavaidPrimaryRecord<'a>),
    VHFNavaidContinuation(VHFNavaidContinuationRecord<'a>),
    VHFNavaidSimulationContinuation(VHFNavaidSimulationContinuationRecord<'a>),
    VHFNavaidFlightPlanningContinuation(VHFNavaidFlightPlanningContinuationRecord<'a>),
    VHFNavaidChangedDataContinuation(VHFNavaidChangedDataContinuationRecord<'a>),
    VHFNavaidLimitationContinuation(VHFNavaidLimitationContinuationRecord<'a>),

    NDBNavaidPrimary(NDBNavaidPrimaryRecord<'a>),
    NDBNavaidContinuation(NDBNavaidContinuationRecord<'a>),
    NDBNavaidSimulationContinuation(NDBNavaidSimulationContinuationRecord<'a>),
    NDBNavaidFlightPlanningContinuation(NDBNavaidFlightPlanningContinuationRecord<'a>),
    NDBNavaidChangedDataContinuation(NDBNavaidChangedDataContinuationRecord<'a>),

    TerminalNDBNavaidPrimary(TerminalNDBNavaidPrimaryRecord<'a>),
    TerminalNDBNavaidContinuation(TerminalNDBNavaidContinuationRecord<'a>),
    TerminalNDBNavaidSimulationContinuation(TerminalNDBNavaidSimulationContinuationRecord<'a>),
    TerminalNDBNavaidFlightPlanningContinuation(
        TerminalNDBNavaidFlightPlanningContinuationRecord<'a>,
    ),
    TerminalNDBNavaidChangedDataContinuation(TerminalNDBNavaidChangedDataContinuationRecord<'a>),

    EnrouteWaypointPrimary(EnrouteWaypointPrimaryRecord<'a>),
    EnrouteWaypointContinuation(EnrouteWaypointContinuationRecord<'a>),
    EnrouteWaypointFlightPlanningContinuation(EnrouteWaypointFlightPlanningContinuationRecord<'a>),
    EnrouteWaypointChangedDataContinuation(EnrouteWaypointChangedDataContinuationRecord<'a>),

    TerminalWaypointPrimary(TerminalWaypointPrimaryRecord<'a>),
    TerminalWaypointContinuation(TerminalWaypointContinuationRecord<'a>),
    TerminalWaypointFlightPlanningContinuation(
        TerminalWaypointFlightPlanningContinuationRecord<'a>,
    ),
    TerminalWaypointChangedDataContinuation(TerminalWaypointChangedDataContinuationRecord<'a>),

    HoldingPatternPrimary(HoldingPatternPrimaryRecord<'a>),
    HoldingPatternContinuation(HoldingPatternContinuationRecord<'a>),

    EnrouteAirwayPrimary(EnrouteAirwayPrimaryRecord<'a>),
    EnrouteAirwayContinuation(EnrouteAirwayContinuationRecord<'a>),
    EnrouteAirwayFlightPlanningContinuation(EnrouteAirwayFlightPlanningContinuationRecord<'a>),
    EnrouteAirwayChangedDataContinuation(EnrouteAirwayChangedDataContinuationRecord<'a>),

    AirportPrimary(AirportPrimaryRecord<'a>),
    AirportContinuation(AirportContinuationRecord<'a>),
    AirportFlightPlanningContinuation(AirportFlightPlanningContinuationRecord<'a>),
    AirportChangedDataContinuation(AirportChangedDataContinuationRecord<'a>),

    AirportGatePrimary(AirportGatePrimaryRecord<'a>),
    AirportGateContinuation(AirportGateContinuationRecord<'a>),

    AirportSIDPrimary(AirportSIDPrimaryRecord<'a>),
    AirportSIDPrimaryExtensionContinuation(AirportSIDPrimaryExtensionContinuationRecord<'a>),
    AirportSIDFlightPlanningContinuation(AirportSIDFlightPlanningContinuationRecord<'a>),
    AirportSIDChangedDataContinuation(AirportSIDChangedDataContinuationRecord<'a>),

    AirportSTARPrimary(AirportSTARPrimaryRecord<'a>),
    AirportSTARPrimaryExtensionContinuation(AirportSTARPrimaryExtensionContinuationRecord<'a>),
    AirportSTARFlightPlanningContinuation(AirportSTARFlightPlanningContinuationRecord<'a>),
    AirportSTARChangedDataContinuation(AirportSTARChangedDataContinuationRecord<'a>),

    AirportApproachMSACenterFixPrimary(AirportApproachMSACenterFixPrimaryRecord<'a>),
    AirportApproachTAAPrimary(AirportApproachTAAPrimaryRecord<'a>),
    AirportApproachPrimaryExtensionContinuation(
        AirportApproachPrimaryExtensionContinuationRecord<'a>,
    ),
    AirportApproachFlightPlanningContinuation(AirportApproachFlightPlanningContinuationRecord<'a>),
    AirportApproachChangedDataContinuation(AirportApproachChangedDataContinuationRecord<'a>),
    AirportApproachProcedureDataContinuation(AirportApproachProcedureDataContinuationRecord<'a>),

    RunwayPrimary(RunwayPrimaryRecord<'a>),
    RunwayContinuation(RunwayContinuationRecord<'a>),
    RunwaySimulationContinuation(RunwaySimulationContinuationRecord<'a>),

    AirportHeliportLocalizerGlideslopePrimary(AirportHeliportLocalizerGlideslopePrimaryRecord<'a>),
    LocalizerGlideslopeContinuation(LocalizerGlideslopeContinuationRecord<'a>),
    LocalizerGlideslopeSimulationContinuation(LocalizerGlideslopeSimulationContinuationRecord<'a>),

    CompanyRoutePrimary(CompanyRoutePrimaryRecord<'a>),

    AirportHeliportLocalizerMarkerPrimary(AirportHeliportLocalizerMarkerPrimaryRecord<'a>),
    AirportHeliportLocalizerMarkerContinuation(
        AirportHeliportLocalizerMarkerContinuationRecord<'a>,
    ),

    AirportCommsPrimary(AirportCommsPrimaryRecord<'a>),
    AirportCommsContinuation(AirportCommsContinuationRecord<'a>),
    AirportCommsAdditionalContinuation(AirportCommsAdditionalContinuationRecord<'a>),

    AirwaysMarkerPrimary(AirwaysMarkerPrimaryRecord<'a>),
    AirwaysMarkerContinuation(AirwaysMarkerContinuationRecord<'a>),

    CruisingTablePrimary(CruisingTablePrimaryRecord<'a>),

    FIRUIRPrimary(FIRUIRPrimaryRecord<'a>),
    FIRUIRContinuation(FIRUIRContinuationRecord<'a>),

    RestrictiveAirspacePrimary(RestrictiveAirspacePrimaryRecord<'a>),
    RestrictiveAirspaceContinuation(RestrictiveAirspaceContinuationRecord<'a>),
    RestrictiveAirspaceFlightPlanningContinuation(
        RestrictiveAirspaceFlightPlanningContinuationRecord<'a>,
    ),

    GridMORAPrimary(GridMORAPrimaryRecord<'a>),

    AirportMSAPrimary(AirportMSAPrimaryRecord<'a>),
    AirportMSAContinuation(AirportMSAContinuationRecord<'a>),

    EnrouteAirwayRestrictionAltitudeExclusionPrimary(
        EnrouteAirwayRestrictionAltitudeExclusionPrimaryRecord<'a>,
    ),
    EnrouteAirwayRestrictionAltitudeExclusionContinuation(
        EnrouteAirwayRestrictionAltitudeExclusionContinuationRecord<'a>,
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

    EnrouteAirwayRestrictionCruisingTableReplacementPrimary(
        EnrouteAirwayRestrictionCruisingTableReplacementPrimaryRecord<'a>,
    ),
    EnrouteAirwayRestrictionCruisingTableReplacementContinuation(
        EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuationRecord<'a>,
    ),

    MLSPrimary(MLSPrimaryRecord<'a>),
    MLSContinuation(MLSContinuationRecord<'a>),

    EnrouteCommsPrimary(EnrouteCommsPrimaryRecord<'a>),
    EnrouteCommsPrimaryExtensionContinuation(EnrouteCommsPrimaryExtensionContinuationRecord<'a>),
    EnrouteCommsTimeContinuation(EnrouteCommsTimeContinuationRecord<'a>),

    PreferredSIDRoutePrimary(PreferredSIDRoutePrimaryRecord<'a>),
    PreferredSTARRoutePrimary(PreferredSTARRoutePrimaryRecord<'a>),
    PreferredAirwayRoutePrimary(PreferredAirwayRoutePrimaryRecord<'a>),
    PreferredGeneralRoutePrimary(PreferredGeneralRoutePrimaryRecord<'a>),
    PreferredRouteContinuation(PreferredRouteContinuationRecord<'a>),
    PreferredRouteTimeContinuation(PreferredRouteTimeContinuationRecord<'a>),

    ControlledAirspacePrimary(ControlledAirspacePrimaryRecord<'a>),
    ControlledAirspaceTimeContinuation(ControlledAirspaceFormattedTimeContinuationRecord<'a>),

    GeographicalReferenceTablePrimary(GeographicalReferenceTablePrimaryRecord<'a>),
    GeographicalReferenceTableContinuation(GeographicalReferenceTableContinuationRecord<'a>),

    FlightPlanningPrimary(FlightPlanningPrimaryRecord<'a>),
    FlightPlanningPrimaryExtensionContinuation(
        FlightPlanningSIDSTARPrimaryExtensionContinuationRecord<'a>,
    ),
    FlightPlanningTimeContinuation(FlightPlanningTimeContinuationRecord<'a>),
    FlightPlanningNarrativeTimeContinuation(FlightPlanningNarrativeTimeContinuationRecord<'a>),

    AirportPathPointPrimary(AirportPathPointPrimaryRecord<'a>),
    AirportPathPointContinuation(AirportPathPointContinuationRecord<'a>),
    HeliportPathPointPrimary(HeliportPathPointPrimaryRecord<'a>),
    HeliportPathPointContinuation(HeliportPathPointContinuationRecord<'a>),

    GLSPrimary(GLSPrimaryRecord<'a>),
    GLSContinuation(GLSContinuationRecord<'a>),

    AlternatePrimary(AlternatePrimaryRecord<'a>),

    AirportTAAPrimary(AirportTAAPrimaryRecord<'a>),
    AirportTAAContinuation(AirportTAAContinuationRecord<'a>),

    HeliportPrimary(HeliportPrimaryRecord<'a>),
    HeliportContinuation(HeliportContinuationRecord<'a>),
    HeliportFlightPlanningContinuation(HeliportFlightPlanningContinuationRecord<'a>),
    HeliportControlledAirspaceContinuation(HeliportControlledAirspaceContinuationRecord<'a>),

    HeliportTerminalWaypointPrimary(HeliportTerminalWaypointPrimaryRecord<'a>),
    HeliportTerminalWaypointContinuation(HeliportTerminalWaypointContinuationRecord<'a>),
    HeliportTerminalWaypointChangedDataContinuation(
        HeliportTerminalWaypointChangedDataContinuationRecord<'a>,
    ),

    HeliportSIDPrimary(HeliportSIDPrimaryRecord<'a>),
    HeliportSIDPrimaryExtensionContinuation(HeliportSIDPrimaryExtensionContinuationRecord<'a>),
    HeliportSIDFlightPlanningContinuation(HeliportSIDFlightPlanningContinuationRecord<'a>),
    HeliportSIDProcedureNameContinuation(HeliportSIDProcedureNameContinuationRecord<'a>),

    HeliportSTARPrimary(HeliportSTARPrimaryRecord<'a>),
    HeliportSTARPrimaryExtensionContinuation(HeliportSTARPrimaryExtensionContinuationRecord<'a>),
    HeliportSTARFlightPlanningContinuation(HeliportSTARFlightPlanningContinuationRecord<'a>),
    HeliportSTARChangedDataContinuation(HeliportSTARChangedDataContinuationRecord<'a>),

    HeliportApproachMSACenterFixPrimary(HeliportApproachMSACenterFixPrimaryRecord<'a>),
    HeliportApproachTAAPrimary(HeliportApproachTAAPrimaryRecord<'a>),
    HeliportApproachPrimaryExtensionContinuation(
        HeliportApproachPrimaryExtensionContinuationRecord<'a>,
    ),
    HeliportApproachFlightPlanningContinuation(
        HeliportApproachFlightPlanningContinuationRecord<'a>,
    ),
    HeliportApproachChangedDataContinuation(HeliportApproachChangedDataContinuationRecord<'a>),
    HeliportApproachProcedureDataContinuation(HeliportApproachProcedureDataContinuationRecord<'a>),

    HeliportMSAPrimary(HeliportMSAPrimaryRecord<'a>),
    HeliportMSAContinuation(HeliportMSAContinuationRecord<'a>),

    HeliportCommsPrimary(HeliportCommsPrimaryRecord<'a>),
    HeliportCommsContinuation(HeliportCommsContinuationRecord<'a>),
    HeliportCommsTimeContinuation(HeliportCommsTimeContinuationRecord<'a>),

    HeliportTAAPrimary(HeliportTAAPrimaryRecord<'a>),
    HeliportTAAContinuation(HeliportTAAContinuationRecord<'a>),
}

impl<'a> ARINCRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        let section_bytes = match input[4] {
            b'H' | b'P' => {
                if input[12] == b' ' {
                    [input[4], input[5]]
                } else {
                    [input[4], input[12]]
                }
            }
            _ => [input[4], input[5]],
        };
        match section_bytes {
            [b'A', b'S'] => GridMORARecords::parse(input),
            [b'D', BLANK] => VHFNavaidRecords::parse(input),
            [b'D', b'B'] => NDBNavaidRecords::parse(input),
            [b'D', b'T'] => TACANOnlyNavaidRecords::parse(input),
            [b'E', b'A'] => EnrouteWaypointRecords::parse(input),
            [b'E', b'M'] => AirwaysMarkerRecords::parse(input),
            [b'E', b'P'] => HoldingPatternRecords::parse(input),
            [b'E', b'R'] => EnrouteAirwayRecords::parse(input),
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
            [b'P', b'N'] => TerminalNDBNavaidRecords::parse(input),
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
            [b'U', b'C'] => ControlledAirspaceRecords::parse(input),
            [b'U', b'F'] => FIRUIRRecords::parse(input),
            [b'U', b'R'] => RestrictiveAirspaceRecords::parse(input),
            _ => Err(RecordParseError {
                message: "Invalid record type".to_string(),
            }),
        }
    }
}
