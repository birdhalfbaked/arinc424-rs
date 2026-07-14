//! # ARINC 424 Records
//! This module contains the record types for the ARINC 424 data.
//! Records are the top-level container for the ARINC 424 data.
//! They are used to group together related data into a single entity.
//!

use crate::parsers::arinc424::rev23::records::*;

use crate::parsers::arinc424::types::fields::BLANK;
use crate::parsers::arinc424::types::records::RecordParseError;

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

    TerminalNDBNavaidPrimary(TerminalNDBNavaidPrimaryRecord<'a>),
    TerminalNDBNavaidContinuation(TerminalNDBNavaidContinuationRecord<'a>),
    TerminalNDBNavaidSimulationContinuation(TerminalNDBNavaidSimulationContinuationRecord<'a>),
    TerminalNDBNavaidFlightPlanningContinuation(
        TerminalNDBNavaidFlightPlanningContinuationRecord<'a>,
    ),

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
            [b'T', b'L'] => ATNRecords::parse(input),
            [b'T', b'V'] => CommunicationTypeTranslationRecords::parse(input),
            [b'U', b'C'] => ControlledAirspaceRecords::parse(input),
            [b'U', b'F'] => FIRUIRRecords::parse(input),
            [b'U', b'R'] => RestrictiveAirspaceRecords::parse(input),
            _ => Err(RecordParseError::new(
                "Invalid record type".to_string(),
                Some(String::from_utf8_lossy(input).into_owned()),
            )),
        }
    }
}
