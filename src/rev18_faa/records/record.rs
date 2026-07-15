//! # ARINC 424 Records
//! This module contains the record types for the ARINC 424 data.
//! Records are the top-level container for the ARINC 424 data.
//! They are used to group together related data into a single entity.
//!

use crate::rev18_faa::records::*;

use crate::types::fields::BLANK;
use crate::types::records::{Arinc424RecordSpec, RecordParseError, RecordValidationError};

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
    AirportSIDFlightPlanningContinuation(AirportSIDFlightPlanningContinuationRecord<'a>),
    AirportSIDChangedDataContinuation(AirportSIDChangedDataContinuationRecord<'a>),

    AirportSTARPrimary(AirportSTARPrimaryRecord<'a>),
    AirportSTARFlightPlanningContinuation(AirportSTARFlightPlanningContinuationRecord<'a>),
    AirportSTARChangedDataContinuation(AirportSTARChangedDataContinuationRecord<'a>),

    AirportApproachMSACenterFixPrimary(AirportApproachMSACenterFixPrimaryRecord<'a>),
    AirportApproachTAAPrimary(AirportApproachTAAPrimaryRecord<'a>),
    AirportApproachPrimaryExtensionContinuation(
        AirportApproachPrimaryExtensionContinuationRecord<'a>,
    ),
    AirportApproachFlightPlanningContinuation(AirportApproachFlightPlanningContinuationRecord<'a>),
    AirportApproachMSACenterFixChangedDataContinuation(
        AirportApproachMSACenterFixChangedDataContinuationRecord<'a>,
    ),
    AirportApproachTAAProcedureDataContinuation(
        AirportApproachTAAProcedureDataContinuationRecord<'a>,
    ),
    AirportApproachProcedureDataContinuation(AirportApproachProcedureDataContinuationRecord<'a>),

    RunwayPrimary(RunwayPrimaryRecord<'a>),
    RunwayContinuation(RunwayContinuationRecord<'a>),
    RunwaySimulationContinuation(RunwaySimulationContinuationRecord<'a>),

    LocalizerGlideslopePrimary(LocalizerGlideslopePrimaryRecord<'a>),
    LocalizerGlideslopeContinuation(LocalizerGlideslopeContinuationRecord<'a>),
    LocalizerGlideslopeSimulationContinuation(LocalizerGlideslopeSimulationContinuationRecord<'a>),

    CompanyRoutePrimary(CompanyRoutePrimaryRecord<'a>),

    LocalizerMarkerPrimary(LocalizerMarkerPrimaryRecord<'a>),
    LocalizerMarkerContinuation(LocalizerMarkerContinuationRecord<'a>),

    AirportCommsPrimary(AirportCommsPrimaryRecord<'a>),
    AirportCommsSectorNarrativeContinuation(AirportCommsSectorNarrativeContinuationRecord<'a>),
    AirportCommsTimeContinuation(AirportCommsTimeContinuationRecord<'a>),

    AirwaysMarkerPrimary(AirwaysMarkerPrimaryRecord<'a>),
    AirwaysMarkerContinuation(AirwaysMarkerContinuationRecord<'a>),

    CruisingTablePrimary(CruisingTablePrimaryRecord<'a>),

    FIRUIRPrimary(FIRUIRPrimaryRecord<'a>),
    FIRUIRContinuation(FIRUIRContinuationRecord<'a>),

    RestrictiveAirspacePrimary(RestrictiveAirspacePrimaryRecord<'a>),
    RestrictiveAirspaceTimeControllingAgencyContinuation(
        RestrictiveAirspaceTimeControllingAgencyContinuationRecord<'a>,
    ),
    RestrictiveAirspaceFlightPlanningContinuation(
        RestrictiveAirspaceFlightPlanningContinuationRecord<'a>,
    ),
    RestrictiveAirspaceControllingAgencyContinuation(
        RestrictiveAirspaceControllingAgencyContinuationRecord<'a>,
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
    EnrouteCommsCallsignAndTimeContinuation(EnrouteCommsCallsignAndTimeContinuationRecord<'a>),
    EnrouteCommsTimeContinuation(EnrouteCommsTimeContinuationRecord<'a>),

    PreferredRoutePrimary(PreferredRoutePrimaryRecord<'a>),
    PreferredRouteContinuation(PreferredRouteContinuationRecord<'a>),
    PreferredRouteTimeContinuation(PreferredRouteTimeContinuationRecord<'a>),

    ControlledAirspacePrimary(ControlledAirspacePrimaryRecord<'a>),
    ControlledAirspaceControllingAgencyAndTimeContinuation(
        ControlledAirspaceControllingAgencyAndTimeContinuationRecord<'a>,
    ),

    GeographicalReferenceTablePrimary(GeographicalReferenceTablePrimaryRecord<'a>),
    GeographicalReferenceTableContinuation(GeographicalReferenceTableContinuationRecord<'a>),

    FlightPlanningSIDSTARPrimary(FlightPlanningSIDSTARPrimaryRecord<'a>),
    FlightPlanningSIDSTARContinuation(FlightPlanningSIDSTARContinuationRecord<'a>),
    FlightPlanningSIDSTARTimeContinuation(FlightPlanningSIDSTARTimeContinuationRecord<'a>),
    FlightPlanningApproachPrimary(FlightPlanningApproachPrimaryRecord<'a>),
    FlightPlanningApproachContinuation(FlightPlanningApproachContinuationRecord<'a>),
    FlightPlanningApproachTimeContinuation(FlightPlanningApproachTimeContinuationRecord<'a>),

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
    HeliportControllingAgencyAndTimeContinuation(
        HeliportControllingAgencyAndTimeContinuationRecord<'a>,
    ),
    HeliportNarrativeTimeContinuation(HeliportNarrativeTimeContinuationRecord<'a>),

    HeliportTerminalWaypointPrimary(HeliportTerminalWaypointPrimaryRecord<'a>),
    HeliportTerminalWaypointContinuation(HeliportTerminalWaypointContinuationRecord<'a>),
    HeliportTerminalWaypointFlightPlanningContinuation(
        HeliportTerminalWaypointFlightPlanningContinuationRecord<'a>,
    ),
    HeliportTerminalWaypointChangedDataContinuation(
        HeliportTerminalWaypointChangedDataContinuationRecord<'a>,
    ),

    HeliportSIDPrimary(HeliportSIDPrimaryRecord<'a>),
    HeliportSIDFlightPlanningContinuation(HeliportSIDFlightPlanningContinuationRecord<'a>),
    HeliportSIDChangedDataContinuation(HeliportSIDChangedDataContinuationRecord<'a>),

    HeliportSTARPrimary(HeliportSTARPrimaryRecord<'a>),
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
    HeliportApproachMSACenterFixChangedDataContinuation(
        HeliportApproachMSACenterFixChangedDataContinuationRecord<'a>,
    ),
    HeliportApproachTAAProcedureDataContinuation(
        HeliportApproachTAAProcedureDataContinuationRecord<'a>,
    ),
    HeliportApproachProcedureDataContinuation(HeliportApproachProcedureDataContinuationRecord<'a>),

    HeliportMSAPrimary(HeliportMSAPrimaryRecord<'a>),
    HeliportMSAContinuation(HeliportMSAContinuationRecord<'a>),

    HeliportCommsPrimary(HeliportCommsPrimaryRecord<'a>),
    HeliportCommsSectorNarrativeContinuation(HeliportCommsSectorNarrativeContinuationRecord<'a>),
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
            [b'H', b'K'] => HeliportTAARecords::parse(input),
            [b'H', b'S'] => HeliportMSARecords::parse(input),
            [b'H', b'V'] => HeliportCommsRecords::parse(input),
            [b'P', b'A'] => AirportRecords::parse(input),
            [b'P', b'B'] => AirportGateRecords::parse(input),
            [b'P', b'C'] => TerminalWaypointRecords::parse(input),
            [b'P', b'D'] => AirportSIDRecords::parse(input),
            [b'P', b'E'] => AirportSTARRecords::parse(input),
            [b'P', b'F'] => AirportApproachRecords::parse(input),
            [b'P', b'G'] => RunwayRecords::parse(input),
            [b'P', b'I'] => LocalizerGlideslopeRecords::parse(input),
            [b'P', b'K'] => AirportTAARecords::parse(input),
            [b'P', b'L'] => MLSRecords::parse(input),
            [b'P', b'M'] => LocalizerMarkerRecords::parse(input),
            [b'P', b'N'] => TerminalNDBNavaidRecords::parse(input),
            [b'P', b'P'] => PathPointRecords::parse(input),
            [b'P', b'R'] => FlightPlanningDataRecords::parse(input),
            [b'P', b'S'] => AirportMSARecords::parse(input),
            [b'P', b'T'] => GLSRecords::parse(input),
            [b'P', b'V'] => AirportCommsRecords::parse(input),
            [b'R', BLANK] => CompanyRouteRecords::parse(input),
            [b'R', b'A'] => AlternateRecords::parse(input),
            [b'T', b'C'] => CruisingTableRecords::parse(input),
            [b'T', b'G'] => GeographicalReferenceTableRecords::parse(input),
            [b'U', b'C'] => ControlledAirspaceRecords::parse(input),
            [b'U', b'F'] => FIRUIRRecords::parse(input),
            [b'U', b'R'] => RestrictiveAirspaceRecords::parse(input),
            _ => Err(RecordParseError::new(
                "Invalid record type".to_string(),
                Some(String::from_utf8_lossy(input).into_owned()),
            )),
        }
    }
    pub fn validate(&self) -> Result<(), RecordValidationError> {
        match self {
            Self::VHFNavaidPrimary(record) => record.validate(),
            Self::VHFNavaidContinuation(record) => record.validate(),
            Self::VHFNavaidSimulationContinuation(record) => record.validate(),
            Self::VHFNavaidFlightPlanningContinuation(record) => record.validate(),
            Self::VHFNavaidChangedDataContinuation(record) => record.validate(),
            Self::VHFNavaidLimitationContinuation(record) => record.validate(),
            Self::NDBNavaidPrimary(record) => record.validate(),
            Self::NDBNavaidContinuation(record) => record.validate(),
            Self::NDBNavaidSimulationContinuation(record) => record.validate(),
            Self::NDBNavaidFlightPlanningContinuation(record) => record.validate(),
            Self::NDBNavaidChangedDataContinuation(record) => record.validate(),
            Self::TerminalNDBNavaidPrimary(record) => record.validate(),
            Self::TerminalNDBNavaidContinuation(record) => record.validate(),
            Self::TerminalNDBNavaidSimulationContinuation(record) => record.validate(),
            Self::TerminalNDBNavaidFlightPlanningContinuation(record) => record.validate(),
            Self::TerminalNDBNavaidChangedDataContinuation(record) => record.validate(),
            Self::EnrouteWaypointPrimary(record) => record.validate(),
            Self::EnrouteWaypointContinuation(record) => record.validate(),
            Self::EnrouteWaypointFlightPlanningContinuation(record) => record.validate(),
            Self::EnrouteWaypointChangedDataContinuation(record) => record.validate(),
            Self::TerminalWaypointPrimary(record) => record.validate(),
            Self::TerminalWaypointContinuation(record) => record.validate(),
            Self::TerminalWaypointFlightPlanningContinuation(record) => record.validate(),
            Self::TerminalWaypointChangedDataContinuation(record) => record.validate(),
            Self::HoldingPatternPrimary(record) => record.validate(),
            Self::HoldingPatternContinuation(record) => record.validate(),
            Self::EnrouteAirwayPrimary(record) => record.validate(),
            Self::EnrouteAirwayContinuation(record) => record.validate(),
            Self::EnrouteAirwayFlightPlanningContinuation(record) => record.validate(),
            Self::EnrouteAirwayChangedDataContinuation(record) => record.validate(),
            Self::AirportPrimary(record) => record.validate(),
            Self::AirportContinuation(record) => record.validate(),
            Self::AirportFlightPlanningContinuation(record) => record.validate(),
            Self::AirportChangedDataContinuation(record) => record.validate(),
            Self::AirportGatePrimary(record) => record.validate(),
            Self::AirportGateContinuation(record) => record.validate(),
            Self::AirportSIDPrimary(record) => record.validate(),
            Self::AirportSIDFlightPlanningContinuation(record) => record.validate(),
            Self::AirportSIDChangedDataContinuation(record) => record.validate(),
            Self::AirportSTARPrimary(record) => record.validate(),
            Self::AirportSTARFlightPlanningContinuation(record) => record.validate(),
            Self::AirportSTARChangedDataContinuation(record) => record.validate(),
            Self::AirportApproachMSACenterFixPrimary(record) => record.validate(),
            Self::AirportApproachTAAPrimary(record) => record.validate(),
            Self::AirportApproachPrimaryExtensionContinuation(record) => record.validate(),
            Self::AirportApproachFlightPlanningContinuation(record) => record.validate(),
            Self::AirportApproachMSACenterFixChangedDataContinuation(record) => record.validate(),
            Self::AirportApproachTAAProcedureDataContinuation(record) => record.validate(),
            Self::AirportApproachProcedureDataContinuation(record) => record.validate(),
            Self::RunwayPrimary(record) => record.validate(),
            Self::RunwayContinuation(record) => record.validate(),
            Self::RunwaySimulationContinuation(record) => record.validate(),
            Self::LocalizerGlideslopePrimary(record) => record.validate(),
            Self::LocalizerGlideslopeContinuation(record) => record.validate(),
            Self::LocalizerGlideslopeSimulationContinuation(record) => record.validate(),
            Self::CompanyRoutePrimary(record) => record.validate(),
            Self::LocalizerMarkerPrimary(record) => record.validate(),
            Self::LocalizerMarkerContinuation(record) => record.validate(),
            Self::AirportCommsPrimary(record) => record.validate(),
            Self::AirportCommsSectorNarrativeContinuation(record) => record.validate(),
            Self::AirportCommsTimeContinuation(record) => record.validate(),
            Self::AirwaysMarkerPrimary(record) => record.validate(),
            Self::AirwaysMarkerContinuation(record) => record.validate(),
            Self::CruisingTablePrimary(record) => record.validate(),
            Self::FIRUIRPrimary(record) => record.validate(),
            Self::FIRUIRContinuation(record) => record.validate(),
            Self::RestrictiveAirspacePrimary(record) => record.validate(),
            Self::RestrictiveAirspaceTimeControllingAgencyContinuation(record) => record.validate(),
            Self::RestrictiveAirspaceFlightPlanningContinuation(record) => record.validate(),
            Self::RestrictiveAirspaceControllingAgencyContinuation(record) => record.validate(),
            Self::GridMORAPrimary(record) => record.validate(),
            Self::AirportMSAPrimary(record) => record.validate(),
            Self::AirportMSAContinuation(record) => record.validate(),
            Self::EnrouteAirwayRestrictionAltitudeExclusionPrimary(record) => record.validate(),
            Self::EnrouteAirwayRestrictionAltitudeExclusionContinuation(record) => {
                record.validate()
            }
            Self::EnrouteAirwayRestrictionNoteRestrictionPrimary(record) => record.validate(),
            Self::EnrouteAirwayRestrictionNoteRestrictionContinuation(record) => record.validate(),
            Self::EnrouteAirwayRestrictionSeasonalClosurePrimary(record) => record.validate(),
            Self::EnrouteAirwayRestrictionCruisingTableReplacementPrimary(record) => {
                record.validate()
            }
            Self::EnrouteAirwayRestrictionCruisingTableReplacementContinuation(record) => {
                record.validate()
            }
            Self::MLSPrimary(record) => record.validate(),
            Self::MLSContinuation(record) => record.validate(),
            Self::EnrouteCommsPrimary(record) => record.validate(),
            Self::EnrouteCommsCallsignAndTimeContinuation(record) => record.validate(),
            Self::EnrouteCommsTimeContinuation(record) => record.validate(),
            Self::PreferredRoutePrimary(record) => record.validate(),
            Self::PreferredRouteContinuation(record) => record.validate(),
            Self::PreferredRouteTimeContinuation(record) => record.validate(),
            Self::ControlledAirspacePrimary(record) => record.validate(),
            Self::ControlledAirspaceControllingAgencyAndTimeContinuation(record) => {
                record.validate()
            }
            Self::GeographicalReferenceTablePrimary(record) => record.validate(),
            Self::GeographicalReferenceTableContinuation(record) => record.validate(),
            Self::FlightPlanningSIDSTARPrimary(record) => record.validate(),
            Self::FlightPlanningSIDSTARContinuation(record) => record.validate(),
            Self::FlightPlanningSIDSTARTimeContinuation(record) => record.validate(),
            Self::FlightPlanningApproachPrimary(record) => record.validate(),
            Self::FlightPlanningApproachContinuation(record) => record.validate(),
            Self::FlightPlanningApproachTimeContinuation(record) => record.validate(),
            Self::AirportPathPointPrimary(record) => record.validate(),
            Self::AirportPathPointContinuation(record) => record.validate(),
            Self::HeliportPathPointPrimary(record) => record.validate(),
            Self::HeliportPathPointContinuation(record) => record.validate(),
            Self::GLSPrimary(record) => record.validate(),
            Self::GLSContinuation(record) => record.validate(),
            Self::AlternatePrimary(record) => record.validate(),
            Self::AirportTAAPrimary(record) => record.validate(),
            Self::AirportTAAContinuation(record) => record.validate(),
            Self::HeliportPrimary(record) => record.validate(),
            Self::HeliportContinuation(record) => record.validate(),
            Self::HeliportControllingAgencyAndTimeContinuation(record) => record.validate(),
            Self::HeliportNarrativeTimeContinuation(record) => record.validate(),
            Self::HeliportTerminalWaypointPrimary(record) => record.validate(),
            Self::HeliportTerminalWaypointContinuation(record) => record.validate(),
            Self::HeliportTerminalWaypointFlightPlanningContinuation(record) => record.validate(),
            Self::HeliportTerminalWaypointChangedDataContinuation(record) => record.validate(),
            Self::HeliportSIDPrimary(record) => record.validate(),
            Self::HeliportSIDFlightPlanningContinuation(record) => record.validate(),
            Self::HeliportSIDChangedDataContinuation(record) => record.validate(),
            Self::HeliportSTARPrimary(record) => record.validate(),
            Self::HeliportSTARFlightPlanningContinuation(record) => record.validate(),
            Self::HeliportSTARChangedDataContinuation(record) => record.validate(),
            Self::HeliportApproachMSACenterFixPrimary(record) => record.validate(),
            Self::HeliportApproachTAAPrimary(record) => record.validate(),
            Self::HeliportApproachPrimaryExtensionContinuation(record) => record.validate(),
            Self::HeliportApproachFlightPlanningContinuation(record) => record.validate(),
            Self::HeliportApproachMSACenterFixChangedDataContinuation(record) => record.validate(),
            Self::HeliportApproachTAAProcedureDataContinuation(record) => record.validate(),
            Self::HeliportApproachProcedureDataContinuation(record) => record.validate(),
            Self::HeliportMSAPrimary(record) => record.validate(),
            Self::HeliportMSAContinuation(record) => record.validate(),
            Self::HeliportCommsPrimary(record) => record.validate(),
            Self::HeliportCommsSectorNarrativeContinuation(record) => record.validate(),
            Self::HeliportCommsTimeContinuation(record) => record.validate(),
            Self::HeliportTAAPrimary(record) => record.validate(),
            Self::HeliportTAAContinuation(record) => record.validate(),
        }
    }
}
