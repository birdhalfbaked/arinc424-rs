//! # ARINC 424 Definitions - Enums
//! This module contains the enums for the ARINC 424 data.
//! Enums are not a formally defined data type in the spec, but are obviously represented as such given
//! the character mapping tables.
//!
//! Example is 5.4 - Section Code which describes the major section of the record.
use crate::parsers::arinc424::types::fields::{BLANK, FieldParseError, ParseableField};

#[derive(Debug, PartialEq, Eq)]
pub enum RecordType {
    Standard,
    Tailored,
}

impl ParseableField for RecordType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"S" => RecordType::Standard,
            b"T" => RecordType::Tailored,
            _ => {
                return Err(FieldParseError::new("Invalid record type".to_string()));
            }
        }))
    }
}

/// 5.4 Section Code
#[derive(Debug, PartialEq, Eq)]
pub enum Section {
    MORA,
    Navaid,
    Enroute,
    Heliport,
    Airport,
    CompanyRoutes,
    Tables,
    Airspace,
}

impl ParseableField for Section {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => Section::MORA,
            b"D" => Section::Navaid,
            b"E" => Section::Enroute,
            b"H" => Section::Heliport,
            b"P" => Section::Airport,
            b"R" => Section::CompanyRoutes,
            b"T" => Section::Tables,
            b"U" => Section::Airspace,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid major section code".to_string(),
                ));
            }
        }))
    }
}

/// 5.5(A) MORA Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum MORASubsection {
    GridMORA,
}

impl ParseableField for MORASubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"S" => MORASubsection::GridMORA,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid MORA subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.5(B) Navaid Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum NavaidSubsection {
    VHFNavaid,
    NDBNavaid,
    TACANDuplicates,
}

impl ParseableField for NavaidSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => NavaidSubsection::VHFNavaid,
            b"B" => NavaidSubsection::NDBNavaid,
            b"T" => NavaidSubsection::TACANDuplicates,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Navaid subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(A) Enroute Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteSubsection {
    Waypoints,
    AirwayMarkers,
    HoldingPatterns,
    AirwaysAndRoutes,
    SpecialActivityAreas,
    PreferredRoutes,
    AirwayRestrictions,
    Communications,
}

impl ParseableField for EnrouteSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => EnrouteSubsection::Waypoints,
            b"M" => EnrouteSubsection::AirwayMarkers,
            b"P" => EnrouteSubsection::HoldingPatterns,
            b"R" => EnrouteSubsection::AirwaysAndRoutes,
            b"S" => EnrouteSubsection::SpecialActivityAreas,
            b"T" => EnrouteSubsection::PreferredRoutes,
            b"U" => EnrouteSubsection::AirwayRestrictions,
            b"V" => EnrouteSubsection::Communications,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Enroute subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(B) Heliport Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum HeliportSubsection {
    ReferencePoints,
    TerminalWaypoints,
    SIDS,
    STARS,
    ApproachProcedures,
    Helipads,
    TAA,
    MSA,
    SBASPathPoint,
    Communications,
}

impl ParseableField for HeliportSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => HeliportSubsection::ReferencePoints,
            b"C" => HeliportSubsection::TerminalWaypoints,
            b"D" => HeliportSubsection::SIDS,
            b"E" => HeliportSubsection::STARS,
            b"F" => HeliportSubsection::ApproachProcedures,
            b"H" => HeliportSubsection::Helipads,
            b"K" => HeliportSubsection::TAA,
            b"S" => HeliportSubsection::MSA,
            b"P" => HeliportSubsection::SBASPathPoint,
            b"V" => HeliportSubsection::Communications,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Heliport subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(C) Airport Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum AirportSubsection {
    ReferencePoints,
    Gates,
    TerminalWaypoints,
    SIDS,
    STARS,
    ApproachProcedures,
    Runways,
    Helipads,
    LocalizerGlideslope,
    TAA,
    MLS,
    LocalizerMarker,
    TerminalNDB,
    SBASPathPoint,
    GBASPathPoint,
    FlightPlanningARRDEP,
    MSA,
    GLSStation,
    Communications,
}

impl ParseableField for AirportSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AirportSubsection::ReferencePoints,
            b"B" => AirportSubsection::Gates,
            b"C" => AirportSubsection::TerminalWaypoints,
            b"D" => AirportSubsection::SIDS,
            b"E" => AirportSubsection::STARS,
            b"F" => AirportSubsection::ApproachProcedures,
            b"G" => AirportSubsection::Runways,
            b"H" => AirportSubsection::Helipads,
            b"I" => AirportSubsection::LocalizerGlideslope,
            b"K" => AirportSubsection::TAA,
            b"L" => AirportSubsection::MLS,
            b"M" => AirportSubsection::LocalizerMarker,
            b"N" => AirportSubsection::TerminalNDB,
            b"P" => AirportSubsection::SBASPathPoint,
            b"Q" => AirportSubsection::GBASPathPoint,
            b"R" => AirportSubsection::FlightPlanningARRDEP,
            b"S" => AirportSubsection::MSA,
            b"T" => AirportSubsection::GLSStation,
            b"V" => AirportSubsection::Communications,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(D) Company Routes Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum CompanyRoutesSubsection {
    CompanyRoutes,
    AlternateRecords,
    HelicopterOperationRoutes,
}

impl ParseableField for CompanyRoutesSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => CompanyRoutesSubsection::CompanyRoutes,
            b"A" => CompanyRoutesSubsection::AlternateRecords,
            b"H" => CompanyRoutesSubsection::HelicopterOperationRoutes,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Company Routes subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(E) Tables Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum TablesSubsection {
    CruisingTables,
    GeographicalReference,
    ATNData,
    CommunicationType,
}

impl ParseableField for TablesSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => TablesSubsection::CruisingTables,
            b"G" => TablesSubsection::GeographicalReference,
            b"L" => TablesSubsection::ATNData,
            b"V" => TablesSubsection::CommunicationType,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Tables subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.6(F) Airspace Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum AirspaceSubsection {
    ControlledAirspace,
    FIRUIR,
    RestrictiveAirspace,
}

impl ParseableField for AirspaceSubsection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => AirspaceSubsection::ControlledAirspace,
            b"F" => AirspaceSubsection::FIRUIR,
            b"R" => AirspaceSubsection::RestrictiveAirspace,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airspace subsection code".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(A) Enroute Airway Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteAirwayRouteType {
    AirlineAirway,
    Control,
    DirectRoute,
    HelicopterAirway,
    DesignatedAirway,
    RNAVRNPAirway,
    UndesignatedATSRoute,
    TACANAirway,
}

impl ParseableField for EnrouteAirwayRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => EnrouteAirwayRouteType::AirlineAirway,
            b"C" => EnrouteAirwayRouteType::Control,
            b"D" => EnrouteAirwayRouteType::DirectRoute,
            b"H" => EnrouteAirwayRouteType::HelicopterAirway,
            b"O" => EnrouteAirwayRouteType::DesignatedAirway,
            b"R" => EnrouteAirwayRouteType::RNAVRNPAirway,
            b"S" => EnrouteAirwayRouteType::UndesignatedATSRoute,
            b"T" => EnrouteAirwayRouteType::TACANAirway,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid enroute airway route type".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(A.1) Enroute Airway Route Type Qualifier 1
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteAirwayRouteTypeQualifier1 {
    GNSSRequired,
    GNSSOrDMEDMEIRURequired,
    GNSSOrDMEDMEIRUOrDMEDMERequired,
    EquipmentRequirementsUnspecified,
}
impl ParseableField for EnrouteAirwayRouteTypeQualifier1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"G" => EnrouteAirwayRouteTypeQualifier1::GNSSRequired,
            b"F" => EnrouteAirwayRouteTypeQualifier1::GNSSOrDMEDMEIRURequired,
            b"A" => EnrouteAirwayRouteTypeQualifier1::GNSSOrDMEDMEIRUOrDMEDMERequired,
            b"U" => EnrouteAirwayRouteTypeQualifier1::EquipmentRequirementsUnspecified,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Enroute Airway Route Type Qualifier 1".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(A.2) Enroute Airway Route Type Qualifier 2
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteAirwayRouteTypeQualifier2 {
    FRTRequired,
    ParallelOffsetRequired,
    TOACRequired,
}
impl ParseableField for EnrouteAirwayRouteTypeQualifier2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"R" => EnrouteAirwayRouteTypeQualifier2::FRTRequired,
            b"P" => EnrouteAirwayRouteTypeQualifier2::ParallelOffsetRequired,
            b"T" => EnrouteAirwayRouteTypeQualifier2::TOACRequired,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Enroute Airway Route Type Capability Requirement".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(A.3) Enroute Airway Route Type Qualifier 3
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteAirwayRouteTypeQualifier3 {
    RNAV10,
    RNAV5,
    RNAV2,
    RNAV1,
    BRNAV,
    PRNAV,
    RNP4,
    RNP2,
    RNP1,
    ARNP,
    RNP0_3,
    Unspecified,
    VORDMERNAV,
    NonRNAVRNPSegment,
}

impl ParseableField for EnrouteAirwayRouteTypeQualifier3 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"W" => EnrouteAirwayRouteTypeQualifier3::RNAV10,
            b"Z" => EnrouteAirwayRouteTypeQualifier3::RNAV5,
            b"Y" => EnrouteAirwayRouteTypeQualifier3::RNAV2,
            b"X" => EnrouteAirwayRouteTypeQualifier3::RNAV1,
            b"B" => EnrouteAirwayRouteTypeQualifier3::BRNAV,
            b"P" => EnrouteAirwayRouteTypeQualifier3::PRNAV,
            b"C" => EnrouteAirwayRouteTypeQualifier3::RNP4,
            b"D" => EnrouteAirwayRouteTypeQualifier3::RNP2,
            b"E" => EnrouteAirwayRouteTypeQualifier3::RNP1,
            b"A" => EnrouteAirwayRouteTypeQualifier3::ARNP,
            b"G" => EnrouteAirwayRouteTypeQualifier3::RNP0_3,
            b"U" => EnrouteAirwayRouteTypeQualifier3::Unspecified,
            b"V" => EnrouteAirwayRouteTypeQualifier3::VORDMERNAV,
            b"N" => EnrouteAirwayRouteTypeQualifier3::NonRNAVRNPSegment,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Enroute Airway Route Type PBN".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(B) Preferred Route Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum PreferredRouteRouteType {
    NACommonRoute,
    PreferentialRoute,
    PACOTSRoute,
    TACANAustraliaRoute,
    NANonCommonRoute,
    PreferredOverflightRoute,
    PreferredRoute,
    TOSRoute,
    TECRoute,
}

impl ParseableField for PreferredRouteRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => PreferredRouteRouteType::NACommonRoute,
            b"D" => PreferredRouteRouteType::PreferentialRoute,
            b"J" => PreferredRouteRouteType::PACOTSRoute,
            b"M" => PreferredRouteRouteType::TACANAustraliaRoute,
            b"N" => PreferredRouteRouteType::NANonCommonRoute,
            b"O" => PreferredRouteRouteType::PreferredOverflightRoute,
            b"P" => PreferredRouteRouteType::PreferredRoute,
            b"S" => PreferredRouteRouteType::TOSRoute,
            b"T" => PreferredRouteRouteType::TECRoute,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred route route type".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(C) SID Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum SIDRouteType {
    EngineOut,
    RunwayTransition,
    CommonRoute,
    EnrouteTransition,
    RNAVRunwayTransition,
    RNAVCommonRoute,
    RNAVEnrouteTransition,
    FMSRunwayTransition,
    FMSCommonRoute,
    FMSEnrouteTransition,
    VectorRunwayTransition,
    VectorEnrouteTransition,
}

impl ParseableField for SIDRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => SIDRouteType::EngineOut,
            b"1" => SIDRouteType::RunwayTransition,
            b"2" => SIDRouteType::CommonRoute,
            b"3" => SIDRouteType::EnrouteTransition,
            b"4" => SIDRouteType::RNAVEnrouteTransition,
            b"5" => SIDRouteType::RNAVRunwayTransition,
            b"6" => SIDRouteType::RNAVEnrouteTransition,
            b"F" => SIDRouteType::FMSEnrouteTransition,
            b"M" => SIDRouteType::FMSEnrouteTransition,
            b"S" => SIDRouteType::FMSEnrouteTransition,
            b"T" => SIDRouteType::VectorRunwayTransition,
            b"V" => SIDRouteType::VectorEnrouteTransition,
            _ => {
                return Err(FieldParseError::new("Invalid SID route type".to_string()));
            }
        }))
    }
}

/// 5.7(C.1) Airport/Heliport SID Route Type Qualifier 1
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportSIDRouteTypeQualifier1 {
    DMERequired,
    GNSSRequired,
    RadarRequired,
    HelicopterSIDFromRunway,
    PointInSpaceSID,
}
impl ParseableField for AirportHeliportSIDRouteTypeQualifier1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"D" => AirportHeliportSIDRouteTypeQualifier1::DMERequired,
            b"G" => AirportHeliportSIDRouteTypeQualifier1::GNSSRequired,
            b"R" => AirportHeliportSIDRouteTypeQualifier1::RadarRequired,
            b"H" => AirportHeliportSIDRouteTypeQualifier1::HelicopterSIDFromRunway,
            b"P" => AirportHeliportSIDRouteTypeQualifier1::PointInSpaceSID,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport SID Route Type Qualifier 1".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(C.2) Airport/Heliport SID Route Type Qualifier 2
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportSIDRouteTypeQualifier2 {
    RNAV,
    RNP,
    FMSRequired,
    ConventionalDepartures,
    PointInSpaceDepartureProceedVisually,
    PointInSpaceDepartureProceedVFR,
}

impl ParseableField for AirportHeliportSIDRouteTypeQualifier2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"D" => AirportHeliportSIDRouteTypeQualifier2::RNAV,
            b"E" => AirportHeliportSIDRouteTypeQualifier2::RNP,
            b"F" => AirportHeliportSIDRouteTypeQualifier2::FMSRequired,
            b"G" => AirportHeliportSIDRouteTypeQualifier2::ConventionalDepartures,
            b"W" => AirportHeliportSIDRouteTypeQualifier2::PointInSpaceDepartureProceedVisually,
            b"X" => AirportHeliportSIDRouteTypeQualifier2::PointInSpaceDepartureProceedVFR,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport SID Route Type Qualifier 2".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(C.3) Airport/Heliport SID Route Type Qualifier 3
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportSIDRouteTypeQualifier3 {
    RNAV5,
    RNAV2,
    RNAV1,
    BRNAV,
    PRNAV,
    RNP2,
    RNP1,
    RNPAR,
    ARNP,
    RNP0_3,
    RNP1OrRNAV1,
    Unspecified,
    VORDMERNAV,
}
impl ParseableField for AirportHeliportSIDRouteTypeQualifier3 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Z" => AirportHeliportSIDRouteTypeQualifier3::RNAV5,
            b"Y" => AirportHeliportSIDRouteTypeQualifier3::RNAV2,
            b"X" => AirportHeliportSIDRouteTypeQualifier3::RNAV1,
            b"B" => AirportHeliportSIDRouteTypeQualifier3::BRNAV,
            b"P" => AirportHeliportSIDRouteTypeQualifier3::PRNAV,
            b"D" => AirportHeliportSIDRouteTypeQualifier3::RNP2,
            b"E" => AirportHeliportSIDRouteTypeQualifier3::RNP1,
            b"F" => AirportHeliportSIDRouteTypeQualifier3::RNPAR,
            b"A" => AirportHeliportSIDRouteTypeQualifier3::ARNP,
            b"G" => AirportHeliportSIDRouteTypeQualifier3::RNP0_3,
            b"M" => AirportHeliportSIDRouteTypeQualifier3::RNP1OrRNAV1,
            b"U" => AirportHeliportSIDRouteTypeQualifier3::Unspecified,
            b"V" => AirportHeliportSIDRouteTypeQualifier3::VORDMERNAV,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport SID Route Type Qualifier 3".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(D) STAR Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum STARRouteType {
    EnrouteTransition,
    CommonRoute,
    RunwayTransition,
    RNAVEnrouteTransition,
    RNAVCommonRoute,
    RNAVRunwayTransition,
    ProfileDescentEnrouteTransition,
    ProfileDescentCommonRoute,
    ProfileDescentRunwayTransition,
    FMSEnrouteTransition,
    FMSCommonRoute,
    FMSRunwayTransition,
}

impl ParseableField for STARRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"1" => STARRouteType::EnrouteTransition,
            b"2" => STARRouteType::CommonRoute,
            b"3" => STARRouteType::RunwayTransition,
            b"4" => STARRouteType::RNAVEnrouteTransition,
            b"5" => STARRouteType::RNAVCommonRoute,
            b"6" => STARRouteType::RNAVRunwayTransition,
            b"7" => STARRouteType::ProfileDescentEnrouteTransition,
            b"8" => STARRouteType::ProfileDescentCommonRoute,
            b"9" => STARRouteType::ProfileDescentRunwayTransition,
            b"F" => STARRouteType::FMSEnrouteTransition,
            b"M" => STARRouteType::FMSCommonRoute,
            b"S" => STARRouteType::FMSRunwayTransition,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid STAR route type: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.7(D.1) STAR Route Type Qualifier 1
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportSTARRouteTypeQualifier1 {
    DMERequired,
    RadarRequired,
    GNSSRequired,
    HelicopterSTARToRunway,
    ContinuousDescentSTAR,
}

impl ParseableField for AirportHeliportSTARRouteTypeQualifier1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"D" => AirportHeliportSTARRouteTypeQualifier1::DMERequired,
            b"R" => AirportHeliportSTARRouteTypeQualifier1::RadarRequired,
            b"G" => AirportHeliportSTARRouteTypeQualifier1::GNSSRequired,
            b"H" => AirportHeliportSTARRouteTypeQualifier1::HelicopterSTARToRunway,
            b"P" => AirportHeliportSTARRouteTypeQualifier1::ContinuousDescentSTAR,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport STAR Route Type Qualifier 1".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(D.2) STAR Route Type Qualifier 2
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportSTARRouteTypeQualifier2 {
    RNAV,
    RNP,
    FMSRequired,
    ConventionalArrivals,
    NotApplicable,
}
impl ParseableField for AirportHeliportSTARRouteTypeQualifier2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"D" => AirportHeliportSTARRouteTypeQualifier2::RNAV,
            b"E" => AirportHeliportSTARRouteTypeQualifier2::RNP,
            b"F" => AirportHeliportSTARRouteTypeQualifier2::FMSRequired,
            b"G" => AirportHeliportSTARRouteTypeQualifier2::ConventionalArrivals,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport STAR Route Type Qualifier 2".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(D.3) STAR Route Type Qualifier 3
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportSTARRouteTypeQualifier3 {
    RNAV5,
    RNAV2,
    RNAV1,
    BRNAV,
    PRNAV,
    RNP2,
    RNP1,
    RNPAR,
    ARNP,
    RNP0_3,
    RNP1OrRNAV1,
    Unspecified,
    VORDMERNAV,
    NotApplicable,
}
impl ParseableField for AirportHeliportSTARRouteTypeQualifier3 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(Some(AirportHeliportSTARRouteTypeQualifier3::NotApplicable));
        }
        Ok(Some(match bytes {
            b"Z" => AirportHeliportSTARRouteTypeQualifier3::RNAV5,
            b"Y" => AirportHeliportSTARRouteTypeQualifier3::RNAV2,
            b"X" => AirportHeliportSTARRouteTypeQualifier3::RNAV1,
            b"B" => AirportHeliportSTARRouteTypeQualifier3::BRNAV,
            b"P" => AirportHeliportSTARRouteTypeQualifier3::PRNAV,
            b"D" => AirportHeliportSTARRouteTypeQualifier3::RNP2,
            b"E" => AirportHeliportSTARRouteTypeQualifier3::RNP1,
            b"F" => AirportHeliportSTARRouteTypeQualifier3::RNPAR,
            b"A" => AirportHeliportSTARRouteTypeQualifier3::ARNP,
            b"G" => AirportHeliportSTARRouteTypeQualifier3::RNP0_3,
            b"M" => AirportHeliportSTARRouteTypeQualifier3::RNP1OrRNAV1,
            b"U" => AirportHeliportSTARRouteTypeQualifier3::Unspecified,
            b"V" => AirportHeliportSTARRouteTypeQualifier3::VORDMERNAV,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Airport Heliport STAR Route Type Qualifier 3".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(E) Airport Heliport Approach Route Type
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportApproachRouteType {
    ApproachTransition,
    LocalizerBackcourseApproach,
    VORDMEApproach,
    FMSApproach,
    IGSApproach,
    RNAVRNPApproach,
    ILSApproach,
    GLSApproach,
    LOCApproach,
    MLSApproach,
    NDBApproach,
    GPSApproach,
    NDBDMEApproach,
    RNAVApproach,
    VORTACApproach,
    TACANApproach,
    SDFApproach,
    VORApproach,
    MLSTypeAApproach,
    LDAApproach,
    RFApproachTransition,
    MissedApproach,
}

impl ParseableField for AirportHeliportApproachRouteType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AirportHeliportApproachRouteType::ApproachTransition,
            b"B" => AirportHeliportApproachRouteType::LocalizerBackcourseApproach,
            b"D" => AirportHeliportApproachRouteType::VORDMEApproach,
            b"F" => AirportHeliportApproachRouteType::FMSApproach,
            b"G" => AirportHeliportApproachRouteType::IGSApproach,
            b"H" => AirportHeliportApproachRouteType::RNAVRNPApproach,
            b"I" => AirportHeliportApproachRouteType::ILSApproach,
            b"J" => AirportHeliportApproachRouteType::GLSApproach,
            b"L" => AirportHeliportApproachRouteType::LOCApproach,
            b"M" => AirportHeliportApproachRouteType::MLSApproach,
            b"N" => AirportHeliportApproachRouteType::NDBApproach,
            b"P" => AirportHeliportApproachRouteType::GPSApproach,
            b"Q" => AirportHeliportApproachRouteType::NDBDMEApproach,
            b"R" => AirportHeliportApproachRouteType::RNAVApproach,
            b"S" => AirportHeliportApproachRouteType::VORTACApproach,
            b"T" => AirportHeliportApproachRouteType::TACANApproach,
            b"U" => AirportHeliportApproachRouteType::SDFApproach,
            b"V" => AirportHeliportApproachRouteType::VORApproach,
            b"W" => AirportHeliportApproachRouteType::MLSTypeAApproach,
            b"X" => AirportHeliportApproachRouteType::LDAApproach,
            b"Y" => AirportHeliportApproachRouteType::RFApproachTransition,
            b"Z" => AirportHeliportApproachRouteType::MissedApproach,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid airport heliport approach route type".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(E.1) Approach Route Type Qualifier 1
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportApproachRouteTypeQualifier1 {
    DMERequired,
    GNSSRequiredDMEDMENotAuthorized,
    GBASProcedure,
    DMENotRequired,
    GNSSRequired,
    GNSSOrDMEDMERequired,
    DMEDMERequired,
    RNAVOrRNP,
    VORDMERNAV,
    SBASFASRequired,
    UnknownAllowedValue,
}
impl ParseableField for AirportHeliportApproachRouteTypeQualifier1 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"D" => AirportHeliportApproachRouteTypeQualifier1::DMERequired,
            b"F" => AirportHeliportApproachRouteTypeQualifier1::UnknownAllowedValue,
            b"J" => AirportHeliportApproachRouteTypeQualifier1::GNSSRequiredDMEDMENotAuthorized,
            b"L" => AirportHeliportApproachRouteTypeQualifier1::GBASProcedure,
            b"N" => AirportHeliportApproachRouteTypeQualifier1::DMENotRequired,
            b"P" => AirportHeliportApproachRouteTypeQualifier1::GNSSRequired,
            b"R" => AirportHeliportApproachRouteTypeQualifier1::GNSSOrDMEDMERequired,
            b"T" => AirportHeliportApproachRouteTypeQualifier1::DMEDMERequired,
            b"U" => AirportHeliportApproachRouteTypeQualifier1::RNAVOrRNP,
            b"V" => AirportHeliportApproachRouteTypeQualifier1::VORDMERNAV,
            b"W" => AirportHeliportApproachRouteTypeQualifier1::SBASFASRequired,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Approach Route Type Qualifier 1".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(E.2) Approach Route Type Qualifier 2
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportApproachRouteTypeQualifier2 {
    PrimaryMissedApproach,
    SecondaryMissedApproach,
    EngineOutMissedApproach,
    CircleToLandMinimums,
    HelicopterStraightInMinimums,
    HelicopterCircleToLandMinimums,
    HelicopterLandingMinimums,
    StraightInMinimums,
    VMCMinimums,
    PointInSpaceProceedVisually,
    PointInSpaceProceedVFR,
}

impl ParseableField for AirportHeliportApproachRouteTypeQualifier2 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AirportHeliportApproachRouteTypeQualifier2::PrimaryMissedApproach,
            b"B" => AirportHeliportApproachRouteTypeQualifier2::SecondaryMissedApproach,
            b"E" => AirportHeliportApproachRouteTypeQualifier2::EngineOutMissedApproach,
            b"C" => AirportHeliportApproachRouteTypeQualifier2::CircleToLandMinimums,
            b"H" => AirportHeliportApproachRouteTypeQualifier2::HelicopterStraightInMinimums,
            b"I" => AirportHeliportApproachRouteTypeQualifier2::HelicopterCircleToLandMinimums,
            b"L" => AirportHeliportApproachRouteTypeQualifier2::HelicopterLandingMinimums,
            b"S" => AirportHeliportApproachRouteTypeQualifier2::StraightInMinimums,
            b"V" => AirportHeliportApproachRouteTypeQualifier2::VMCMinimums,
            b"W" => AirportHeliportApproachRouteTypeQualifier2::PointInSpaceProceedVisually,
            b"X" => AirportHeliportApproachRouteTypeQualifier2::PointInSpaceProceedVFR,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Approach Route Type Qualifier 2".to_string(),
                ));
            }
        }))
    }
}

/// 5.7(E.3) Approach Route Type Qualifier 3
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportApproachRouteTypeQualifier3 {
    RNAV1,
    RNP1,
    RNPApproach,
    RNP0_3,
    ARNP,
    RNPAR,
    RNAVVisual,
}

impl ParseableField for AirportHeliportApproachRouteTypeQualifier3 {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"X" => AirportHeliportApproachRouteTypeQualifier3::RNAV1,
            b"E" => AirportHeliportApproachRouteTypeQualifier3::RNP1,
            b"H" => AirportHeliportApproachRouteTypeQualifier3::RNPApproach,
            b"G" => AirportHeliportApproachRouteTypeQualifier3::RNP0_3,
            b"A" => AirportHeliportApproachRouteTypeQualifier3::ARNP,
            b"F" => AirportHeliportApproachRouteTypeQualifier3::RNPAR,
            b"B" => AirportHeliportApproachRouteTypeQualifier3::RNAVVisual,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Approach Route Type Qualifier 3".to_string(),
                ));
            }
        }))
    }
}

/// 5.8 Customer/Area Code
#[derive(Debug, PartialEq, Eq)]
pub enum CustomerAreaCode {
    Africa,
    Canada,
    EasternEurope,
    Europe,
    LatinAmerica,
    MiddleEast,
    Pacific,
    SouthAmerica,
    SouthPacific,
    USA,
    Customer(String),
}

impl ParseableField for CustomerAreaCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.len() != 3 {
            return Err(FieldParseError::new(
                "Customer area code must be 3 characters long".to_string(),
            ));
        }
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"EEU" => CustomerAreaCode::EasternEurope,
            b"EUR" => CustomerAreaCode::Europe,
            b"USA" => CustomerAreaCode::USA,
            b"CAN" => CustomerAreaCode::Canada,
            b"PAC" => CustomerAreaCode::Pacific,
            b"AFR" => CustomerAreaCode::Africa,
            b"LAM" => CustomerAreaCode::LatinAmerica,
            b"MES" => CustomerAreaCode::MiddleEast,
            b"SAM" => CustomerAreaCode::SouthAmerica,
            b"SPA" => CustomerAreaCode::SouthPacific,
            _ => CustomerAreaCode::Customer(
                String::from_utf8(bytes.to_vec())
                    .map_err(|e| FieldParseError::new(e.to_string()))?,
            ),
        }))
    }
}

/// 5.18 Boundary Code
#[derive(Debug, PartialEq, Eq)]
pub enum BoundaryCode {
    USA,
    CanadaAlaska,
    Pacific,
    LatinAmerica,
    SouthAmerica,
    SouthPacific,
    Europe,
    EasternEurope,
    MiddleEastSouthAsia,
    Africa,
}

impl ParseableField for BoundaryCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"U" => BoundaryCode::USA,
            b"C" => BoundaryCode::CanadaAlaska,
            b"P" => BoundaryCode::Pacific,
            b"L" => BoundaryCode::LatinAmerica,
            b"S" => BoundaryCode::SouthAmerica,
            b"1" => BoundaryCode::SouthPacific,
            b"E" => BoundaryCode::Europe,
            b"2" => BoundaryCode::EasternEurope,
            b"M" => BoundaryCode::MiddleEastSouthAsia,
            b"A" => BoundaryCode::Africa,
            _ => {
                return Err(FieldParseError::new("Invalid boundary code".to_string()));
            }
        }))
    }
}

/// 5.19 Level
#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    AllAltitudes,
    HighLevelAirwaysAltitudes,
    LowLevelAirwaysAltitudes,
}

impl ParseableField for Level {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => Level::AllAltitudes,
            b"H" => Level::HighLevelAirwaysAltitudes,
            b"L" => Level::LowLevelAirwaysAltitudes,
            _ => {
                return Err(FieldParseError::new("Invalid level".to_string()));
            }
        }))
    }
}

/// 5.20 Turn Direction
#[derive(Debug, PartialEq, Eq)]
pub enum TurnDirection {
    Left,
    Right,
    Either,
}

impl ParseableField for TurnDirection {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"L" => TurnDirection::Left,
            b"R" => TurnDirection::Right,
            b"E" => TurnDirection::Either,
            _ => {
                return Err(FieldParseError::new(
                    format!("Invalid turn direction: {}", bytes[0] as char).to_string(),
                ));
            }
        }))
    }
}

/// 5.22 Turn Direction Valid
#[derive(Debug, PartialEq, Eq)]
pub enum TurnDirectionValid {
    Yes,
    No,
}

impl ParseableField for TurnDirectionValid {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => TurnDirectionValid::Yes,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid turn direction valid".to_string(),
                ));
            }
        }))
    }
}

/// 5.29 Crossing Altitude Description
#[derive(Debug, PartialEq, Eq)]
pub enum CrossingAltitudeDescription {
    AtOrAbove,
    AtOrBelow,
    At,
    Between,
    ConditionalAtOrAboveEarliest,
    ConditionalAtOrAboveLatest,
    GlideslopeAltitude,
    GlideslopeInterceptAltitude,
    AtUntilInbound,
    AtVerticalAngle,
}

impl ParseableField for CrossingAltitudeDescription {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"+" => CrossingAltitudeDescription::AtOrAbove,
            b"-" => CrossingAltitudeDescription::AtOrBelow,
            b"@" => CrossingAltitudeDescription::At,
            b"B" => CrossingAltitudeDescription::Between,
            b"C" => CrossingAltitudeDescription::ConditionalAtOrAboveEarliest,
            b"D" => CrossingAltitudeDescription::ConditionalAtOrAboveLatest,
            b"G" | b"H" => CrossingAltitudeDescription::GlideslopeAltitude,
            b"I" | b"J" => CrossingAltitudeDescription::GlideslopeInterceptAltitude,
            b"O" => CrossingAltitudeDescription::AtUntilInbound,
            b"V" | b"X" | b"Y" => CrossingAltitudeDescription::AtVerticalAngle,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid crossing altitude description: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.49 Localizer Azimuth Position Reference (@, +, -)
#[derive(Debug, PartialEq, Eq)]
pub enum LocalizerAzimuthPositionReference {
    AheadOfApproachEnd,
    BeyondStopEnd,
    Aside,
}

impl ParseableField for LocalizerAzimuthPositionReference {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"@" => LocalizerAzimuthPositionReference::AheadOfApproachEnd,
            b"+" => LocalizerAzimuthPositionReference::BeyondStopEnd,
            b"-" => LocalizerAzimuthPositionReference::Aside,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid localizer azimuth position reference".to_string(),
                ));
            }
        }))
    }
}

/// 5.63 Turn (TURN) for Holding Pattern records (TODO: investigate deduplication with 5.20 Turn Direction)
#[derive(Debug, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
}

impl ParseableField for Turn {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"L" => Turn::Left,
            b"R" => Turn::Right,
            _ => {
                return Err(FieldParseError::new("Invalid turn".to_string()));
            }
        }))
    }
}

/// 5.77(A) Company Route VIA Code
#[derive(Debug, PartialEq, Eq)]
pub enum CompanyRouteVIACode {
    AlternateAirport,
    ApproachRoute,
    ApproachTransition,
    DesignatedAirway,
    DirectToFix,
    InitialFix,
    PreferredRoute,
    SID,
    SIDEnrouteTransition,
    SIDRunwayTransition,
    STARProfileDescent,
    STARProfileDescentEnrouteTransition,
    STARProfileDescentRunwayTransition,
}

impl ParseableField for CompanyRouteVIACode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"ALT" => CompanyRouteVIACode::AlternateAirport,
            b"APP" => CompanyRouteVIACode::ApproachRoute,
            b"APT" => CompanyRouteVIACode::ApproachTransition,
            b"AWY" => CompanyRouteVIACode::DesignatedAirway,
            b"DIR" => CompanyRouteVIACode::DirectToFix,
            b"INT" => CompanyRouteVIACode::InitialFix,
            b"PRE" => CompanyRouteVIACode::PreferredRoute,
            b"SID" => CompanyRouteVIACode::SID,
            b"SDE" => CompanyRouteVIACode::SIDEnrouteTransition,
            b"SDY" => CompanyRouteVIACode::SIDRunwayTransition,
            b"STR" => CompanyRouteVIACode::STARProfileDescent,
            b"STE" => CompanyRouteVIACode::STARProfileDescentEnrouteTransition,
            b"STY" => CompanyRouteVIACode::STARProfileDescentRunwayTransition,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid company route VIA code".to_string(),
                ));
            }
        }))
    }
}

/// 5.77(B) Preferred Route VIA Code
pub enum PreferredRouteVIACode {
    DesignatedAirway,
    DirectToFix,
    InitialFix,
    RouteViaFix,
    RouteViaFixNotPermitted,
    SID,
    STARProfileDescent,
}

impl ParseableField for PreferredRouteVIACode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"AWY" => PreferredRouteVIACode::DesignatedAirway,
            b"DIR" => PreferredRouteVIACode::DirectToFix,
            b"INT" => PreferredRouteVIACode::InitialFix,
            b"RVF" => PreferredRouteVIACode::RouteViaFix,
            b"RNF" => PreferredRouteVIACode::RouteViaFixNotPermitted,
            b"SID" => PreferredRouteVIACode::SID,
            b"STR" => PreferredRouteVIACode::STARProfileDescent,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred route VIA code".to_string(),
                ));
            }
        }))
    }
}

/// 5.80 ILS/MLS/GLS Category (CAT)
#[derive(Debug, PartialEq, Eq)]
pub enum IlsMlsGlsCategory {
    LocalizerOnlyNoGS,
    ILSMLSGLSCatI,
    ILSMLSGLSCatII,
    ILSMLSGLSCatIII,
    IGSFacility,
    LDAWithGS,
    LDANoGS,
    SDFWithGS,
    SDFNoGS,
}

impl ParseableField for IlsMlsGlsCategory {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => IlsMlsGlsCategory::LocalizerOnlyNoGS,
            b"1" => IlsMlsGlsCategory::ILSMLSGLSCatI,
            b"2" => IlsMlsGlsCategory::ILSMLSGLSCatII,
            b"3" => IlsMlsGlsCategory::ILSMLSGLSCatIII,
            b"I" => IlsMlsGlsCategory::IGSFacility,
            b"L" => IlsMlsGlsCategory::LDAWithGS,
            b"A" => IlsMlsGlsCategory::LDANoGS,
            b"S" => IlsMlsGlsCategory::SDFWithGS,
            b"F" => IlsMlsGlsCategory::SDFNoGS,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid ILS/MLS/GLS category".to_string(),
                ));
            }
        }))
    }
}

/// 5.81 ATC Indicator (ATC)
///
/// Note: This is confusing as written in the spec, but I have encoded it such that it is more properly understood.
#[derive(Debug, PartialEq, Eq)]
pub enum AtcIndicator {
    ATCAssignmentOptional,
    ATCAssignmentRequired,
}

impl ParseableField for AtcIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AtcIndicator::ATCAssignmentOptional,
            b"S" => AtcIndicator::ATCAssignmentRequired,
            _ => {
                return Err(FieldParseError::new("Invalid ATC indicator".to_string()));
            }
        }))
    }
}

/// 5.82 Waypoint Usage
#[derive(Debug, PartialEq, Eq)]
pub enum WaypointUsage {
    HIAndLo,
    Hi,
    Lo,
    TerminalOnly,
}

impl ParseableField for WaypointUsage {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"B" => WaypointUsage::HIAndLo,
            b"H" => WaypointUsage::Hi,
            b"L" => WaypointUsage::Lo,
            [BLANK] => WaypointUsage::TerminalOnly,
            _ => {
                return Err(FieldParseError::new("Invalid waypoint usage".to_string()));
            }
        }))
    }
}

/// 5.91 Continuation Record Application Type (APPL)
#[derive(Debug, PartialEq, Eq)]
pub enum ContinuationRecordApplicationType {
    StandardContinuation,
    ControllingAgencyContinuation,
    PrimaryRecordExtension,
    AdditionalSectorizationContinuation,
    VHFNavaidTACANOnlyNavaidLimitationContinuation,
    SectorNarrativeContinuation,
    FlightPlanningContinuation,
    SimulationContinuation,
    FormattedTimeOfOperationsContinuation,
    NarrativeTimeOfOperationsContinuation,
    AirportHeliportProcedureDataContinuation,
    AirportSIDSTARApproachProcedureNameContinuation,
}

impl ParseableField for ContinuationRecordApplicationType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ContinuationRecordApplicationType::StandardContinuation,
            b"C" => ContinuationRecordApplicationType::ControllingAgencyContinuation,
            b"E" => ContinuationRecordApplicationType::PrimaryRecordExtension,
            b"F" => ContinuationRecordApplicationType::AdditionalSectorizationContinuation,
            b"L" => {
                ContinuationRecordApplicationType::VHFNavaidTACANOnlyNavaidLimitationContinuation
            }
            b"N" => ContinuationRecordApplicationType::SectorNarrativeContinuation,
            b"P" => ContinuationRecordApplicationType::FlightPlanningContinuation,
            b"S" => ContinuationRecordApplicationType::SimulationContinuation,
            b"T" => ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation,
            b"U" => ContinuationRecordApplicationType::NarrativeTimeOfOperationsContinuation,
            b"W" => ContinuationRecordApplicationType::AirportHeliportProcedureDataContinuation,
            b"Y" => {
                ContinuationRecordApplicationType::AirportSIDSTARApproachProcedureNameContinuation
            }
            _ => {
                return Err(FieldParseError::new(
                    "Invalid continuation record application type".to_string(),
                ));
            }
        }))
    }
}

/// 5.95 Government Source
#[derive(Debug, PartialEq, Eq)]
pub enum GovernmentSource {
    OfficialGovernment,
    OtherSource,
    OnlyTrue,
}

impl ParseableField for GovernmentSource {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"O" => GovernmentSource::OfficialGovernment,
            b"R" => GovernmentSource::OtherSource,
            b"T" => GovernmentSource::OnlyTrue,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid government source".to_string(),
                ));
            }
        }))
    }
}

/// 5.98 Elevation Type
#[derive(Debug, PartialEq, Eq)]
pub enum ElevationType {
    AirportHeliportElevation,
    LandingThresholdElevation,
    DisplacedThresholdRunwayEndElevation,
    TouchdownZoneElevation,
}

impl ParseableField for ElevationType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ElevationType::AirportHeliportElevation,
            b"L" => ElevationType::LandingThresholdElevation,
            b"R" => ElevationType::DisplacedThresholdRunwayEndElevation,
            b"T" => ElevationType::TouchdownZoneElevation,
            _ => {
                return Err(FieldParseError::new("Invalid elevation type".to_string()));
            }
        }))
    }
}

/// 5.101 Communications Type
#[derive(Debug, PartialEq, Eq)]
pub enum CommunicationsType {
    AreaControlCenter,
    AirliftCommandPost,
    AirToAir,
    ApproachControl,
    ArrivalControl,
    ASOS,
    ATIS,
    AWIB,
    AWOS,
    AWIS,
    ClassBAirspace,
    ClassCAirspace,
    ClearanceDelivery,
    ClearancePreTaxi,
    TerminalControlArea,
    CTAF,
    Control,
    DepartureControl,
    ApproachControlRadarDirector,
    EFAS,
    Emergency,
    FSS,
    GroundCommOutlet,
    GroundControl,
    GateControl,
    HelicopterFrequency,
    Information,
    MandatoryBroadcastZone,
    MilitaryFrequency,
    Multicom,
    Operations,
    PilotActivatedLighting,
    Radio,
    Radar,
    RFSS,
    RampControl,
    ARSA,
    TCA,
    TMA,
    Terminal,
    TRSA,
    TWEB,
    TowerControl,
    UpperAreaControl,
    Unicom,
    Volmet,
}

impl ParseableField for CommunicationsType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"ACC" => CommunicationsType::AreaControlCenter,
            b"ACP" => CommunicationsType::AirliftCommandPost,
            b"AIR" => CommunicationsType::AirToAir,
            b"APP" => CommunicationsType::ApproachControl,
            b"ARR" => CommunicationsType::ArrivalControl,
            b"ASO" => CommunicationsType::ASOS,
            b"ATI" => CommunicationsType::ATIS,
            b"AWB" => CommunicationsType::AWIB,
            b"AWO" => CommunicationsType::AWOS,
            b"AWS" => CommunicationsType::AWIS,
            b"CBA" => CommunicationsType::ClassBAirspace,
            b"CCA" => CommunicationsType::ClassCAirspace,
            b"CLD" => CommunicationsType::ClearanceDelivery,
            b"CPT" => CommunicationsType::ClearancePreTaxi,
            b"CTA" => CommunicationsType::TerminalControlArea,
            b"CTF" => CommunicationsType::CTAF,
            b"CTL" => CommunicationsType::Control,
            b"DEP" => CommunicationsType::DepartureControl,
            b"DIR" => CommunicationsType::ApproachControlRadarDirector,
            b"EFS" => CommunicationsType::EFAS,
            b"EMR" => CommunicationsType::Emergency,
            b"FSS" => CommunicationsType::FSS,
            b"GCO" => CommunicationsType::GroundCommOutlet,
            b"GND" => CommunicationsType::GroundControl,
            b"GTE" => CommunicationsType::GateControl,
            b"HEL" => CommunicationsType::HelicopterFrequency,
            b"INF" => CommunicationsType::Information,
            b"MBZ" => CommunicationsType::MandatoryBroadcastZone,
            b"MIL" => CommunicationsType::MilitaryFrequency,
            b"MUL" => CommunicationsType::Multicom,
            b"OPS" => CommunicationsType::Operations,
            b"PAL" => CommunicationsType::PilotActivatedLighting,
            b"RDO" => CommunicationsType::Radio,
            b"RDR" => CommunicationsType::Radar,
            b"RFS" => CommunicationsType::RFSS,
            b"RMP" => CommunicationsType::RampControl,
            b"RSA" => CommunicationsType::ARSA,
            b"TCA" => CommunicationsType::TCA,
            b"TMA" => CommunicationsType::TMA,
            b"TML" => CommunicationsType::Terminal,
            b"TRS" => CommunicationsType::TRSA,
            b"TWE" => CommunicationsType::TWEB,
            b"TWR" => CommunicationsType::TowerControl,
            b"UAC" => CommunicationsType::UpperAreaControl,
            b"UNI" => CommunicationsType::Unicom,
            b"VOL" => CommunicationsType::Volmet,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid communications type".to_string(),
                ));
            }
        }))
    }
}

/// 5.102 Radar
#[derive(Debug, PartialEq, Eq)]
pub enum Radar {
    Radar,
    NonRadar,
    Unknown,
}

impl ParseableField for Radar {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(Radar::NonRadar));
        }
        Ok(Some(match bytes {
            b"R" => Radar::Radar,
            b"N" => Radar::NonRadar,
            b"U" => Radar::Unknown,
            _ => {
                return Err(FieldParseError::new("Invalid radar".to_string()));
            }
        }))
    }
}

/// 5.104 Frequency Units
#[derive(Debug, PartialEq, Eq)]
pub enum FrequencyUnits {
    DigitalService,
    LF,
    MF,
    HF,
    VHF100KHzSpacing,
    VHF50KHzSpacing,
    VHF25KHzSpacing,
    VHF8_33KHzSpacing,
    VHFNonStandardSpacing,
    UHF,
}

impl ParseableField for FrequencyUnits {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"D" => FrequencyUnits::DigitalService,
            b"L" => FrequencyUnits::LF,
            b"M" => FrequencyUnits::MF,
            b"H" => FrequencyUnits::HF,
            b"K" => FrequencyUnits::VHF100KHzSpacing,
            b"F" => FrequencyUnits::VHF50KHzSpacing,
            b"T" => FrequencyUnits::VHF25KHzSpacing,
            b"C" => FrequencyUnits::VHF8_33KHzSpacing,
            b"V" => FrequencyUnits::VHFNonStandardSpacing,
            b"U" => FrequencyUnits::UHF,
            _ => {
                return Err(FieldParseError::new("Invalid frequency units".to_string()));
            }
        }))
    }
}

/// 5.108 IFR Capability
#[derive(Debug, PartialEq, Eq)]
pub enum IfrCapability {
    HasApproach,
    NoApproach,
}

impl ParseableField for IfrCapability {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => IfrCapability::HasApproach,
            b"N" => IfrCapability::NoApproach,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid IFR capability: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.112 Marker Radiation Shape
#[derive(Debug, PartialEq, Eq)]
pub enum MarkerRadiationShape {
    Bone,
    EllipseOrUnknown,
}

impl ParseableField for MarkerRadiationShape {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => MarkerRadiationShape::Bone,
            b"E" => MarkerRadiationShape::EllipseOrUnknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid marker radiation shape".to_string(),
                ));
            }
        }))
    }
}

/// 5.113 High/Low (Enroute Marker)
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteMarkerPower {
    High,
    Low,
}

impl ParseableField for EnrouteMarkerPower {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => EnrouteMarkerPower::High,
            b"L" => EnrouteMarkerPower::Low,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid enroute marker power".to_string(),
                ));
            }
        }))
    }
}

/// 5.115(A) Enroute Directional Restriction
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteDirectionalRestriction {
    ForwardOnly,
    BackwardOnly,
    NoRestriction,
}

impl ParseableField for EnrouteDirectionalRestriction {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"F" => EnrouteDirectionalRestriction::ForwardOnly,
            b"B" => EnrouteDirectionalRestriction::BackwardOnly,
            [BLANK] => EnrouteDirectionalRestriction::NoRestriction,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid enroute directional restriction".to_string(),
                ));
            }
        }))
    }
}

/// 5.115(B) Preferred Route Directional Restriction
#[derive(Debug, PartialEq, Eq)]
pub enum PreferredRouteDirectionalRestriction {
    ForwardOnly,
    Bidirectional,
}

impl ParseableField for PreferredRouteDirectionalRestriction {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => PreferredRouteDirectionalRestriction::ForwardOnly,
            b"B" => PreferredRouteDirectionalRestriction::Bidirectional,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred route directional restriction".to_string(),
                ));
            }
        }))
    }
}

/// 5.117 FIR/UIR Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum FirUirIndicator {
    FIR,
    UIR,
    Combined,
}

impl ParseableField for FirUirIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => FirUirIndicator::FIR,
            b"U" => FirUirIndicator::UIR,
            b"B" => FirUirIndicator::Combined,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/UIR indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.122 FIR/UIR Reporting Units Speed
#[derive(Debug, PartialEq, Eq)]
pub enum FirUirReportingUnitsSpeed {
    NotSpecified,
    Knots,
    Mach,
    KPH,
}

impl ParseableField for FirUirReportingUnitsSpeed {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => FirUirReportingUnitsSpeed::NotSpecified,
            b"1" => FirUirReportingUnitsSpeed::Knots,
            b"2" => FirUirReportingUnitsSpeed::Mach,
            b"3" => FirUirReportingUnitsSpeed::KPH,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/UIR reporting units speed".to_string(),
                ));
            }
        }))
    }
}

/// 5.123 FIR/UIR Reporting Units Altitude
#[derive(Debug, PartialEq, Eq)]
pub enum FirUirReportingUnitsAltitude {
    NotSpecified,
    FlightLevel,
    Meters,
    Feet,
}

impl ParseableField for FirUirReportingUnitsAltitude {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => FirUirReportingUnitsAltitude::NotSpecified,
            b"1" => FirUirReportingUnitsAltitude::FlightLevel,
            b"2" => FirUirReportingUnitsAltitude::Meters,
            b"3" => FirUirReportingUnitsAltitude::Feet,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/UIR reporting units altitude".to_string(),
                ));
            }
        }))
    }
}

/// 5.124 FIR/UIR Entry Report
#[derive(Debug, PartialEq, Eq)]
pub enum FirUirEntryReport {
    Required,
    NotRequired,
}

impl ParseableField for FirUirEntryReport {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => FirUirEntryReport::Required,
            b"N" => FirUirEntryReport::NotRequired,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/UIR entry report".to_string(),
                ));
            }
        }))
    }
}

/// 5.128 Restrictive Airspace Type
#[derive(Debug, PartialEq, Eq)]
pub enum RestrictiveAirspaceType {
    Alert,
    Caution,
    Danger,
    LongTermTFR,
    MilitaryOperationsArea,
    NationalSecurityArea,
    Prohibited,
    Restricted,
    Training,
    Warning,
    UnspecifiedOrUnknown,
}

impl ParseableField for RestrictiveAirspaceType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => RestrictiveAirspaceType::Alert,
            b"C" => RestrictiveAirspaceType::Caution,
            b"D" => RestrictiveAirspaceType::Danger,
            b"L" => RestrictiveAirspaceType::LongTermTFR,
            b"M" => RestrictiveAirspaceType::MilitaryOperationsArea,
            b"N" => RestrictiveAirspaceType::NationalSecurityArea,
            b"P" => RestrictiveAirspaceType::Prohibited,
            b"R" => RestrictiveAirspaceType::Restricted,
            b"T" => RestrictiveAirspaceType::Training,
            b"W" => RestrictiveAirspaceType::Warning,
            b"U" => RestrictiveAirspaceType::UnspecifiedOrUnknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid restrictive airspace type".to_string(),
                ));
            }
        }))
    }
}

/// 5.131(A) Primary Record Time Code
#[derive(Debug, PartialEq, Eq)]
pub enum PrimaryRecordTimeCode {
    ActiveContinuouslyIncludingHolidays,
    ActiveContinuouslyExcludingHolidays,
    ActiveNonContinuously,
    ActiveDuringNOTAM,
    NotSpecified,
}

impl ParseableField for PrimaryRecordTimeCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => PrimaryRecordTimeCode::ActiveContinuouslyIncludingHolidays,
            b"H" => PrimaryRecordTimeCode::ActiveContinuouslyExcludingHolidays,
            b"N" => PrimaryRecordTimeCode::ActiveNonContinuously,
            b"P" => PrimaryRecordTimeCode::ActiveDuringNOTAM,
            b"U" => PrimaryRecordTimeCode::NotSpecified,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid primary record time code".to_string(),
                ));
            }
        }))
    }
}

/// 5.131(B) Continuation Record Time Code
#[derive(Debug, PartialEq, Eq)]
pub enum ContinuationRecordTimeCode {
    ExcludingHolidays,
    IncludingHolidays,
}

impl ParseableField for ContinuationRecordTimeCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => ContinuationRecordTimeCode::ExcludingHolidays,
            b"T" => ContinuationRecordTimeCode::IncludingHolidays,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid continuation record time code".to_string(),
                ));
            }
        }))
    }
}

/// 5.132 NOTAM Flag
#[derive(Debug, PartialEq, Eq)]
pub enum NotamFlag {
    ActiveByNotam,
    NotActiveByNotam,
}

impl ParseableField for NotamFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"N" => NotamFlag::ActiveByNotam,
            [BLANK] => NotamFlag::NotActiveByNotam,
            _ => {
                return Err(FieldParseError::new("Invalid NOTAM flag".to_string()));
            }
        }))
    }
}

/// 5.133 Unit Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum AirspaceLimitUnitIndicator {
    AboveGroundLevel,
    MeanSeaLevel,
}

impl ParseableField for AirspaceLimitUnitIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AirspaceLimitUnitIndicator::AboveGroundLevel,
            b"M" => AirspaceLimitUnitIndicator::MeanSeaLevel,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid airspace limit unit indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.134 Cruise Table Identifier
#[derive(Debug, PartialEq, Eq)]
pub enum CruiseTableIdentifier {
    ICAOCruiseTable,
    ExceptionToICAOCruiseTable,
    ModifiedCruiseTable(Box<str>),
    ExceptionToModifiedCruiseTable(Box<str>),
}

impl ParseableField for CruiseTableIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let string =
            Box::from(std::str::from_utf8(bytes).map_err(|e| FieldParseError::new(e.to_string()))?);
        Ok(Some(match bytes {
            b"AA" => CruiseTableIdentifier::ICAOCruiseTable,
            b"AO" => CruiseTableIdentifier::ExceptionToICAOCruiseTable,
            [l1, l2] => match (*l1, *l2) {
                (l1, l2) if l1 == l2 && matches!(l1, b'B'..=b'Z') => {
                    CruiseTableIdentifier::ModifiedCruiseTable(string)
                }
                (l, b'O') if matches!(l, b'B'..=b'Z') => {
                    CruiseTableIdentifier::ExceptionToModifiedCruiseTable(string)
                }
                _ => {
                    return Err(FieldParseError::new(
                        "Invalid cruise table identifier".to_string(),
                    ));
                }
            },
            _ => {
                return Err(FieldParseError::new(
                    "Invalid cruise table identifier".to_string(),
                ));
            }
        }))
    }
}

/// 5.138 Time Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TimeIndicator {
    LocalTimeWithDST,
    LocalTimeWithoutDST,
    UTCWithDST,
    UTCWithoutDST,
}

impl ParseableField for TimeIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"T" => TimeIndicator::LocalTimeWithDST,
            b"L" => TimeIndicator::LocalTimeWithoutDST,
            b"S" => TimeIndicator::UTCWithDST,
            b"Z" => TimeIndicator::UTCWithoutDST,
            _ => {
                return Err(FieldParseError::new("Invalid time indicator".to_string()));
            }
        }))
    }
}

/// 5.149 NavaidUsableRange
#[derive(Debug, PartialEq, Eq)]
pub enum NavaidUsableRange {
    Terminal,
    LowAltitude,
    HighAltitude,
    ExtendedHighAltitude,
    NavaidNotCivil,
    NavaidOutOfService,
}

impl ParseableField for NavaidUsableRange {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => NavaidUsableRange::Terminal,
            b"1" => NavaidUsableRange::LowAltitude,
            b"2" => NavaidUsableRange::HighAltitude,
            b"3" => NavaidUsableRange::ExtendedHighAltitude,
            b"7" => NavaidUsableRange::NavaidNotCivil,
            b"9" => NavaidUsableRange::NavaidOutOfService,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid Navaid usable range".to_string(),
                ));
            }
        }))
    }
}

/// 5.155 BARO-VNAV Authorization
#[derive(Debug, PartialEq, Eq)]
pub enum BaroVnavAuthorization {
    Authorized,
    NotAuthorized,
}

impl ParseableField for BaroVnavAuthorization {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"X" => BaroVnavAuthorization::Authorized,
            [BLANK] => BaroVnavAuthorization::NotAuthorized,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid BARO-VNAV authorization flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.158 VFR Checkpoint Flag
#[derive(Debug, PartialEq, Eq)]
pub enum VFRCheckpointFlag {
    Yes,
    No,
}

impl ParseableField for VFRCheckpointFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => VFRCheckpointFlag::Yes,
            [BLANK] => VFRCheckpointFlag::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid VFR checkpoint flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.159 ATC Assigned Only
#[derive(Debug, PartialEq, Eq)]
pub enum AtcAssignedOnly {
    Yes,
    No,
}

impl ParseableField for AtcAssignedOnly {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => AtcAssignedOnly::Yes,
            [BLANK] => AtcAssignedOnly::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid ATC assigned only flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.160 Units of Altitude for airway restriction
#[derive(Debug, PartialEq, Eq)]
pub enum AirwayRestrictionAltitudeUnit {
    HundredsOfFeet,
    MetricFlightLevels,
    FeetFlightLevels,
    TensOfMeters,
}

impl ParseableField for AirwayRestrictionAltitudeUnit {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => AirwayRestrictionAltitudeUnit::HundredsOfFeet,
            b"K" => AirwayRestrictionAltitudeUnit::MetricFlightLevels,
            b"L" => AirwayRestrictionAltitudeUnit::FeetFlightLevels,
            b"M" => AirwayRestrictionAltitudeUnit::TensOfMeters,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid airway restriction altitude unit".to_string(),
                ));
            }
        }))
    }
}

/// 5.162 Step Climb Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum StepClimbIndicator {
    StepClimbUpDown,
    StepClimbDown,
    NoStepClimbPermitted,
    StepClimbUp,
}

impl ParseableField for StepClimbIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => StepClimbIndicator::StepClimbUpDown,
            b"D" => StepClimbIndicator::StepClimbDown,
            b"N" => StepClimbIndicator::NoStepClimbPermitted,
            b"U" => StepClimbIndicator::StepClimbUp,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid step climb indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.164 Enroute Airway Restriction Flag
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteAirwayRestrictionFlag {
    Yes,
    No,
}

impl ParseableField for EnrouteAirwayRestrictionFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => EnrouteAirwayRestrictionFlag::Yes,
            [BLANK] => EnrouteAirwayRestrictionFlag::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid enroute airway restriction flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.165 Magnetic/True Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum MagneticTrueIndicator {
    Magnetic,
    True,
}

impl ParseableField for MagneticTrueIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"M" => MagneticTrueIndicator::Magnetic,
            b"T" => MagneticTrueIndicator::True,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid magnetic/true indicator: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.174 Restrictive Airspace Link Continuation
#[derive(Debug, PartialEq, Eq)]
pub enum RestrictiveAirspaceLinkContinuation {
    Yes,
    No,
}

impl ParseableField for RestrictiveAirspaceLinkContinuation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => RestrictiveAirspaceLinkContinuation::Yes,
            [BLANK] => RestrictiveAirspaceLinkContinuation::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid restrictive airspace link continuation flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.177 Public/Military Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum PublicMilitaryIndicator {
    Public,
    Military,
    Private,
    Joint,
}

impl ParseableField for PublicMilitaryIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => PublicMilitaryIndicator::Public,
            b"M" => PublicMilitaryIndicator::Military,
            b"J" => PublicMilitaryIndicator::Joint,
            b"P" => PublicMilitaryIndicator::Private,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid public/military indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.179 Daylight Time Observed Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum DaylightTimeObservedIndicator {
    Yes,
    NoOrUnknown,
}

impl ParseableField for DaylightTimeObservedIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => DaylightTimeObservedIndicator::Yes,
            b"N" => DaylightTimeObservedIndicator::NoOrUnknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid daylight time observed indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.181 H24 Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum H24Indicator {
    Yes,
    No,
    Unknown,
}

impl ParseableField for H24Indicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => H24Indicator::Yes,
            b"N" => H24Indicator::No,
            b"U" => H24Indicator::Unknown,
            _ => {
                return Err(FieldParseError::new("Invalid H24 indicator".to_string()));
            }
        }))
    }
}

/// 5.187 Distance Description
#[derive(Debug, PartialEq, Eq)]
pub enum DistanceDescription {
    AppliedUpToDistance,
    AppliedFromDistance,
    NotAppliedOrAtSpecifiedDistance,
    BetweenSpecifiedDistances,
}

impl ParseableField for DistanceDescription {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"-" => DistanceDescription::AppliedUpToDistance,
            b"+" => DistanceDescription::AppliedFromDistance,
            [BLANK] => DistanceDescription::NotAppliedOrAtSpecifiedDistance,
            b"B" => DistanceDescription::BetweenSpecifiedDistances,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid distance description".to_string(),
                ));
            }
        }))
    }
}

/// 5.197 Modulation
#[derive(Debug, PartialEq, Eq)]
pub enum Modulation {
    AM,
    FM,
}

impl ParseableField for Modulation {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => Modulation::AM,
            b"F" => Modulation::FM,
            _ => {
                return Err(FieldParseError::new("Invalid modulation".to_string()));
            }
        }))
    }
}

/// 5.199 Signal Emission
#[derive(Debug, PartialEq, Eq)]
pub enum SignalEmission {
    DoubleSideband,
    SingleSidebandReducedCarrier,
    DualIndependentSidebands,
    SingleSidebandFullCarrier,
    SingleSidebandSuppressedCarrier,
    LowerSidebandUnknownCarrier,
    UpperSidebandUnknownCarrier,
}

impl ParseableField for SignalEmission {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"3" => SignalEmission::DoubleSideband,
            b"A" => SignalEmission::SingleSidebandReducedCarrier,
            b"B" => SignalEmission::DualIndependentSidebands,
            b"H" => SignalEmission::SingleSidebandFullCarrier,
            b"J" => SignalEmission::SingleSidebandSuppressedCarrier,
            b"L" => SignalEmission::LowerSidebandUnknownCarrier,
            b"U" => SignalEmission::UpperSidebandUnknownCarrier,
            _ => {
                return Err(FieldParseError::new("Invalid signal emission".to_string()));
            }
        }))
    }
}

/// 5.201 Restriction Record Type
#[derive(Debug, PartialEq, Eq)]
pub enum RestrictionRecordType {
    AltitudeExclusion,
    CruisingTableReplacement,
    SeasonalRestriction,
    NoteRestriction,
}

impl ParseableField for RestrictionRecordType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"AE" => RestrictionRecordType::AltitudeExclusion,
            b"TC" => RestrictionRecordType::CruisingTableReplacement,
            b"SC" => RestrictionRecordType::SeasonalRestriction,
            b"NR" => RestrictionRecordType::NoteRestriction,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid restriction record type".to_string(),
                ));
            }
        }))
    }
}

/// 5.202 Exclusion Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum AltitudeExclusionIndicator {
    AllAltitudesBothDirections,
    AllAltitudesBackwardDirection,
    AllAltitudesForwardDirection,
    NoAltitudeExclusion,
}

impl ParseableField for AltitudeExclusionIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => AltitudeExclusionIndicator::AllAltitudesBothDirections,
            b"B" => AltitudeExclusionIndicator::AllAltitudesBackwardDirection,
            b"F" => AltitudeExclusionIndicator::AllAltitudesForwardDirection,
            [BLANK] => AltitudeExclusionIndicator::NoAltitudeExclusion,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid altitude exclusion indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.203 Block Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum BlockAltitudeIndicator {
    BlockAltitude,
    IndividualAltitude,
}

impl ParseableField for BlockAltitudeIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => BlockAltitudeIndicator::BlockAltitude,
            b"I" => BlockAltitudeIndicator::IndividualAltitude,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid block altitude indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.205 Navaid Limitation
#[derive(Debug, PartialEq, Eq)]
pub enum NavaidLimitationCode {
    Coverage,
    Fluctuations,
    SignalRoughness,
    UnreliableAtLimitation,
    RestrictedAtLimitation,
    UnusableAtLimitation,
    OutOfToleranceAtLimitation,
}

impl ParseableField for NavaidLimitationCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => NavaidLimitationCode::Coverage,
            b"F" => NavaidLimitationCode::Fluctuations,
            b"G" => NavaidLimitationCode::SignalRoughness,
            b"N" => NavaidLimitationCode::UnreliableAtLimitation,
            b"R" => NavaidLimitationCode::RestrictedAtLimitation,
            b"T" => NavaidLimitationCode::UnusableAtLimitation,
            b"U" => NavaidLimitationCode::OutOfToleranceAtLimitation,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid navaid limitation code".to_string(),
                ));
            }
        }))
    }
}

/// 5.206 Component Affected Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum ComponentAffectedIndicator {
    TACANAzimuthOnly,
    VORDMEAzimuthAndDistance,
    VORDMEDistanceOnly,
    TACANAzimuthAndDistance,
    TACANDistanceOnly,
    VORAzimuthOnly,
    VORDMETACANAZimuthAndDistance,
}

impl ParseableField for ComponentAffectedIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ComponentAffectedIndicator::TACANAzimuthOnly,
            b"B" => ComponentAffectedIndicator::VORDMEAzimuthAndDistance,
            b"D" => ComponentAffectedIndicator::VORDMEDistanceOnly,
            b"M" => ComponentAffectedIndicator::TACANAzimuthAndDistance,
            b"T" => ComponentAffectedIndicator::TACANDistanceOnly,
            b"V" => ComponentAffectedIndicator::VORAzimuthOnly,
            b"Z" => ComponentAffectedIndicator::VORDMETACANAZimuthAndDistance,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid component affected indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.210 Sequence End Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum SequenceEndIndicator {
    EndOfSequence,
    NotEndOfSequence,
}

impl ParseableField for SequenceEndIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"E" => SequenceEndIndicator::EndOfSequence,
            [BLANK] => SequenceEndIndicator::NotEndOfSequence,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid sequence end indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.213 Controlled Airspace Type
#[derive(Debug, PartialEq, Eq)]
pub enum ControlledAirspaceType {
    ClassC,
    ControlArea,
    TerminalControlArea,
    RadarArea,
    ClassB,
    RadioMandatoryZone,
    TransponderMandatoryZone,
    ClassD,
}

impl ParseableField for ControlledAirspaceType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ControlledAirspaceType::ClassC,
            b"C" => ControlledAirspaceType::ClassC,
            b"M" => ControlledAirspaceType::TerminalControlArea,
            b"R" => ControlledAirspaceType::RadarArea,
            b"T" => ControlledAirspaceType::ClassB,
            b"U" => ControlledAirspaceType::RadioMandatoryZone,
            b"V" => ControlledAirspaceType::TransponderMandatoryZone,
            b"Z" => ControlledAirspaceType::ClassD,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid controlled airspace type".to_string(),
                ));
            }
        }))
    }
}

/// 5.217 Controlled Airspace Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum ControlledAirspaceIndicator {
    WithinClassC,
    WithinControlArea,
    WithinTerminalControlArea,
    WithinRadarArea,
    WithinClassB,
}

impl ParseableField for ControlledAirspaceIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => ControlledAirspaceIndicator::WithinClassC,
            b"C" => ControlledAirspaceIndicator::WithinControlArea,
            b"M" => ControlledAirspaceIndicator::WithinTerminalControlArea,
            b"R" => ControlledAirspaceIndicator::WithinRadarArea,
            b"T" => ControlledAirspaceIndicator::WithinClassB,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid controlled airspace indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.222 GNSS/FMS Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum GNSSFMSIndicator {
    NotAuthorizedForGNSSOrFMSOverlay,
    GNSSOverlayWithNavaidMonitoring,
    GNSSOverlayWithNavaid,
    GNSSOverlay,
    FMSOverlay,
    SBASWithVNAV,
    RNPRNAVVisualNoSBASVNAV,
    RNPSBASVNAVNotPublished,
    RNPSBASNoSBASVNAV,
    StandaloneGNSS,
    RNPApproachAsGPS,
    ILSLocalizerOnly,
    OverlayAuthorizationNotPublished,
}

impl ParseableField for GNSSFMSIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => GNSSFMSIndicator::NotAuthorizedForGNSSOrFMSOverlay,
            b"1" => GNSSFMSIndicator::GNSSOverlayWithNavaidMonitoring,
            b"2" => GNSSFMSIndicator::GNSSOverlayWithNavaid,
            b"3" => GNSSFMSIndicator::GNSSOverlay,
            b"4" => GNSSFMSIndicator::FMSOverlay,
            b"A" => GNSSFMSIndicator::SBASWithVNAV,
            b"B" => GNSSFMSIndicator::RNPRNAVVisualNoSBASVNAV,
            b"C" => GNSSFMSIndicator::RNPSBASVNAVNotPublished,
            b"D" => GNSSFMSIndicator::RNPSBASNoSBASVNAV,
            b"P" => GNSSFMSIndicator::StandaloneGNSS,
            b"G" => GNSSFMSIndicator::RNPApproachAsGPS,
            b"L" => GNSSFMSIndicator::ILSLocalizerOnly,
            b"U" => GNSSFMSIndicator::OverlayAuthorizationNotPublished,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid GNSS/FMS indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.223(A) SBAS Operation Type
#[derive(Debug, PartialEq, Eq)]
pub enum SBASOperationType {
    StraightInOrPointInSpaceApproach,
    Reserved,
    Spare,
}

impl ParseableField for SBASOperationType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }

        let numeric_value = u8::from_str_radix(
            std::str::from_utf8(bytes)
                .map_err(|e| FieldParseError::new(format!("Numeric is not valid UTF-8: {}", e)))?,
            10,
        )
        .map_err(|e| FieldParseError::new(format!("Numeric is not a valid u8: {}", e)))?;

        Ok(Some(match numeric_value {
            0 => SBASOperationType::StraightInOrPointInSpaceApproach,
            1..=2 => SBASOperationType::Reserved,
            3..=15 => SBASOperationType::Spare,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid SBAS operation type".to_string(),
                ));
            }
        }))
    }
}

/// 5.223(B) GBAS Operation Type
#[derive(Debug, PartialEq, Eq)]
pub enum GBASOperationType {
    StraightInApproachPath,
    TerminalAreaPath,
    MissedApproach,
    Spare,
}

impl ParseableField for GBASOperationType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }

        let numeric_value = u8::from_str_radix(
            std::str::from_utf8(bytes)
                .map_err(|e| FieldParseError::new(format!("Numeric is not valid UTF-8: {}", e)))?,
            10,
        )
        .map_err(|e| FieldParseError::new(format!("Numeric is not a valid u8: {}", e)))?;

        Ok(Some(match numeric_value {
            0 => GBASOperationType::StraightInApproachPath,
            1 => GBASOperationType::TerminalAreaPath,
            2 => GBASOperationType::MissedApproach,
            3..=15 => GBASOperationType::Spare,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid GBAS operation type".to_string(),
                ));
            }
        }))
    }
}

/// 5.230 Procedure Type
#[derive(Debug, PartialEq, Eq)]
pub enum ProcedureType {
    ArrivalProcedureInDatabase,
    ArrivalProcedureNotInDatabase,
    DepartureProcedureInDatabase,
    DepartureProcedureNotInDatabase,
    STARInDatabase,
    STARNotInDatabase,
    SIDInDatabase,
    SIDNotInDatabase,
    VectorSIDInDatabase,
    VectorSIDNotInDatabase,
    ApproachProcedureInDatabase,
    ApproachProcedureNotInDatabase,
}

impl ParseableField for ProcedureType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ProcedureType::ArrivalProcedureInDatabase,
            b"B" => ProcedureType::ArrivalProcedureNotInDatabase,
            b"C" => ProcedureType::DepartureProcedureInDatabase,
            b"D" => ProcedureType::DepartureProcedureNotInDatabase,
            b"E" => ProcedureType::STARInDatabase,
            b"F" => ProcedureType::STARNotInDatabase,
            b"G" => ProcedureType::SIDInDatabase,
            b"H" => ProcedureType::SIDNotInDatabase,
            b"I" => ProcedureType::VectorSIDInDatabase,
            b"J" => ProcedureType::VectorSIDNotInDatabase,
            b"K" => ProcedureType::ApproachProcedureInDatabase,
            b"L" => ProcedureType::ApproachProcedureNotInDatabase,
            _ => {
                return Err(FieldParseError::new("Invalid procedure type".to_string()));
            }
        }))
    }
}

/// 5.233 Turboprop/Jet Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TurbopropJetIndicator {
    AllAircraft,
    JetsAndTurboprops,
    AllAircraftLessThan250Kts,
    NonJetAndTurboprop,
    MultiEngineProps,
    Jets,
    NonJetNonTurboprop,
    Turboprops,
}

impl ParseableField for TurbopropJetIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => TurbopropJetIndicator::AllAircraft,
            b"B" => TurbopropJetIndicator::JetsAndTurboprops,
            b"C" => TurbopropJetIndicator::AllAircraftLessThan250Kts,
            b"D" => TurbopropJetIndicator::NonJetAndTurboprop,
            b"E" => TurbopropJetIndicator::MultiEngineProps,
            b"J" => TurbopropJetIndicator::Jets,
            b"N" => TurbopropJetIndicator::NonJetNonTurboprop,
            b"P" => TurbopropJetIndicator::Turboprops,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid turboprop/jet indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.234 RNAV Flag
#[derive(Debug, PartialEq, Eq)]
pub enum RNAVFlag {
    RNAV,
    NotRNAV,
}

impl ParseableField for RNAVFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => RNAVFlag::RNAV,
            b"N" => RNAVFlag::NotRNAV,
            _ => {
                return Err(FieldParseError::new("Invalid RNAV flag".to_string()));
            }
        }))
    }
}

/// 5.235 ATC Weight Category
#[derive(Debug, PartialEq, Eq)]
pub enum ATCWeightCategory {
    Heavy,
    Medium,
    Light,
}

impl ParseableField for ATCWeightCategory {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => ATCWeightCategory::Heavy,
            b"M" => ATCWeightCategory::Medium,
            b"L" => ATCWeightCategory::Light,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid ATC weight category".to_string(),
                ));
            }
        }))
    }
}

/// 5.239 Reporting Code
#[derive(Debug, PartialEq, Eq)]
pub enum ReportingCode {
    ReportingRequired,
    ReportingNotRequired,
}

impl ParseableField for ReportingCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => ReportingCode::ReportingRequired,
            b"X" => ReportingCode::ReportingNotRequired,
            _ => {
                return Err(FieldParseError::new("Invalid reporting code".to_string()));
            }
        }))
    }
}

/// 5.241 Fix Related Transition Code
#[derive(Debug, PartialEq, Eq)]
pub enum FixRelatedTransitionCode {
    SIDRunwayTransition,
    SIDCommonPortion,
    SIDEnrouteTransition,
    STAREnrouteTransition,
    STARCommonPortion,
    STARRunwayTransition,
}

impl ParseableField for FixRelatedTransitionCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"1" => FixRelatedTransitionCode::SIDRunwayTransition,
            b"2" => FixRelatedTransitionCode::SIDCommonPortion,
            b"3" => FixRelatedTransitionCode::SIDEnrouteTransition,
            b"4" => FixRelatedTransitionCode::STAREnrouteTransition,
            b"5" => FixRelatedTransitionCode::STARCommonPortion,
            b"6" => FixRelatedTransitionCode::STARRunwayTransition,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid fix related transition code".to_string(),
                ));
            }
        }))
    }
}

/// 5.242 Procedure Category
#[derive(Debug, PartialEq, Eq)]
pub enum ProcedureCategory {
    LAAS,
    WAAS,
    FMS,
    GPS,
    VORDMEVORTAC,
    CircleToLand,
}

impl ParseableField for ProcedureCategory {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"LAAS" => ProcedureCategory::LAAS,
            b"WAAS" => ProcedureCategory::WAAS,
            b"FMS " => ProcedureCategory::FMS,
            b"GPS " => ProcedureCategory::GPS,
            b"VDME" => ProcedureCategory::VORDMEVORTAC,
            b"CIRC" => ProcedureCategory::CircleToLand,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid procedure category".to_string(),
                ));
            }
        }))
    }
}

/// 5.249 Runway Surface Code
#[derive(Debug, PartialEq, Eq)]
pub enum RunwaySurfaceCode {
    HardSurface,
    SoftSurface,
    WaterRunway,
    Undefined,
}

impl ParseableField for RunwaySurfaceCode {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => RunwaySurfaceCode::HardSurface,
            b"S" => RunwaySurfaceCode::SoftSurface,
            b"W" => RunwaySurfaceCode::WaterRunway,
            b"U" => RunwaySurfaceCode::Undefined,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid runway surface code".to_string(),
                ));
            }
        }))
    }
}

/// 5.250 Alternate Record Type
#[derive(Debug, PartialEq, Eq)]
pub enum AlternateRecordType {
    ArrivalAirport,
    DepartureAirport,
    EndFix,
}

impl ParseableField for AlternateRecordType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"AA" => AlternateRecordType::ArrivalAirport,
            b"DA" => AlternateRecordType::DepartureAirport,
            b"EA" => AlternateRecordType::EndFix,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid alternate record type".to_string(),
                ));
            }
        }))
    }
}

/// 5.252 Alternate Type
#[derive(Debug, PartialEq, Eq)]
pub enum AlternateType {
    Airport,
    CompanyRoute,
}

impl ParseableField for AlternateType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AlternateType::Airport,
            b"C" => AlternateType::CompanyRoute,
            _ => {
                return Err(FieldParseError::new("Invalid alternate type".to_string()));
            }
        }))
    }
}

/// 5.255 SBAS Service Provider Identifier
#[derive(Debug, PartialEq, Eq)]
pub enum SbasServiceProviderIdentifier {
    WAAS,
    EGNOS,
    MSAS,
    GAGAN,
    SDCM,
    Spare,
    CRCForGBAS,
    AnyServiceProvider,
}

impl ParseableField for SbasServiceProviderIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let numeric_value: u8 = u8::from_str_radix(
            std::str::from_utf8(bytes).map_err(|e| {
                FieldParseError::new(format!("Invalid SBAS service provider identifier: {}", e))
            })?,
            10,
        )
        .map_err(|e| {
            FieldParseError::new(format!("Invalid SBAS service provider identifier: {}", e))
        })?;
        Ok(Some(match numeric_value {
            0 => SbasServiceProviderIdentifier::WAAS,
            1 => SbasServiceProviderIdentifier::EGNOS,
            2 => SbasServiceProviderIdentifier::MSAS,
            3 => SbasServiceProviderIdentifier::GAGAN,
            4 => SbasServiceProviderIdentifier::SDCM,
            5..=13 => SbasServiceProviderIdentifier::Spare,
            14 => SbasServiceProviderIdentifier::CRCForGBAS,
            15 => SbasServiceProviderIdentifier::AnyServiceProvider,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid SBAS service provider identifier: {}",
                    numeric_value
                )));
            }
        }))
    }
}

/// 5.258 GBAS Approach Performance Designator
#[derive(Debug, PartialEq, Eq)]
pub enum GBASApproachPerformanceDesignator {
    GASTAOrGASTB,
    GASTC,
    GASTCOrGASTD,
    Spare,
}

impl ParseableField for GBASApproachPerformanceDesignator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let numeric_value = u8::from_str_radix(
            std::str::from_utf8(bytes).map_err(|e| {
                FieldParseError::new(format!(
                    "Invalid GBAS approach performance designator: {}",
                    e
                ))
            })?,
            10,
        )
        .map_err(|e| {
            FieldParseError::new(format!(
                "Invalid GBAS approach performance designator: {}",
                e
            ))
        })?;
        Ok(Some(match numeric_value {
            0 => GBASApproachPerformanceDesignator::GASTAOrGASTB,
            1 => GBASApproachPerformanceDesignator::GASTC,
            2 => GBASApproachPerformanceDesignator::GASTCOrGASTD,
            3..=7 => GBASApproachPerformanceDesignator::Spare,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid GBAS approach performance designator".to_string(),
                ));
            }
        }))
    }
}

/// 5.261 Speed Limit Description
#[derive(Debug, PartialEq, Eq)]
pub enum SpeedLimitDescription {
    AtSpeed,
    AtOrAboveSpeed,
    AtOrBelowSpeed,
}

impl ParseableField for SpeedLimitDescription {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"@" => SpeedLimitDescription::AtSpeed,
            b"+" => SpeedLimitDescription::AtOrAboveSpeed,
            b"-" => SpeedLimitDescription::AtOrBelowSpeed,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid speed limit description".to_string(),
                ));
            }
        }))
    }
}

/// 5.266 TCH Units Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TCHUnitsIndicator {
    Feet,
    Meters,
}

impl ParseableField for TCHUnitsIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => TCHUnitsIndicator::Feet,
            b"M" => TCHUnitsIndicator::Meters,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid TCH units indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.270 TCH Value Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TCHValueIndicator {
    ElectronicGlideslope,
    RNAVProcedureToRunway,
    VisualGlideslope,
    DefaultValue,
}

impl ParseableField for TCHValueIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"I" => TCHValueIndicator::ElectronicGlideslope,
            b"R" => TCHValueIndicator::RNAVProcedureToRunway,
            b"V" => TCHValueIndicator::VisualGlideslope,
            b"D" => TCHValueIndicator::DefaultValue,
            _ => {
                return Err(FieldParseError::new(format!(
                    "Invalid TCH value indicator: {}",
                    bytes[0] as char
                )));
            }
        }))
    }
}

/// 5.271 Procedure Turn
#[derive(Debug, PartialEq, Eq)]
pub enum ProcedureTurn {
    Required,
    NoProcedureTurn,
}

impl ParseableField for ProcedureTurn {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => ProcedureTurn::Required,
            b"N" => ProcedureTurn::NoProcedureTurn,
            _ => {
                return Err(FieldParseError::new("Invalid procedure turn".to_string()));
            }
        }))
    }
}

/// 5.272 TAA Sector Identifier
#[derive(Debug, PartialEq, Eq)]
pub enum TaaSectorIdentifier {
    StraightInOrCenterFix,
    LeftBaseArea,
    RightBaseArea,
}

impl ParseableField for TaaSectorIdentifier {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => TaaSectorIdentifier::StraightInOrCenterFix,
            b"L" => TaaSectorIdentifier::LeftBaseArea,
            b"T" => TaaSectorIdentifier::RightBaseArea,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid TAA sector identifier".to_string(),
                ));
            }
        }))
    }
}

/// 5.276 Level of Service Authorized
#[derive(Debug, PartialEq, Eq)]
pub enum LevelOfServiceAuthorized {
    Authorized,
    NotAuthorized,
}

impl ParseableField for LevelOfServiceAuthorized {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => LevelOfServiceAuthorized::Authorized,
            [BLANK] | b"N" => LevelOfServiceAuthorized::NotAuthorized,
            _ => {
                return Err(FieldParseError::new(
                    format!("Invalid level of service authorized: {}", bytes[0] as char)
                        .to_string(),
                ));
            }
        }))
    }
}

/// 5.277 DME Operational Service Volume
#[derive(Debug, PartialEq, Eq)]
pub enum DMEOperationalServiceVolume {
    LessThan40NM,
    LessThan70NM,
    LessThan130NM,
    GreaterThan130NM,
    Unspecified,
}

impl ParseableField for DMEOperationalServiceVolume {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => DMEOperationalServiceVolume::LessThan40NM,
            b"B" => DMEOperationalServiceVolume::LessThan70NM,
            b"C" => DMEOperationalServiceVolume::LessThan130NM,
            b"D" => DMEOperationalServiceVolume::GreaterThan130NM,
            b"U" => DMEOperationalServiceVolume::Unspecified,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid DME operational service volume".to_string(),
                ));
            }
        }))
    }
}

/// 5.278 Special Activity Type
#[derive(Debug, PartialEq, Eq)]
pub enum SpecialActivityType {
    ParachuteJumping,
    Glider,
    HangGlider,
    Ultralight,
}

impl ParseableField for SpecialActivityType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"P" => SpecialActivityType::ParachuteJumping,
            b"G" => SpecialActivityType::Glider,
            b"H" => SpecialActivityType::HangGlider,
            b"U" => SpecialActivityType::Ultralight,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid special activity type".to_string(),
                ));
            }
        }))
    }
}

/// 5.283 Communications Class
#[derive(Debug, PartialEq, Eq)]
pub enum CommunicationsClass {
    LinkedToFIRUIRForControl,
    LinkedToFIRUIRForInformation,
    UsedWithinFIRUIRForOtherPurposes,
    UsedWithinFIRUIRForBroadcastServices,
    UsedWithinTerminalAreaForControl,
    UsedwithinTerminalAreaForOtherPurposes,
    UsedWithinTerminalAreaForBroadcastServices,
}

impl ParseableField for CommunicationsClass {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"LIRC" => CommunicationsClass::LinkedToFIRUIRForControl,
            b"LIRI" => CommunicationsClass::LinkedToFIRUIRForInformation,
            b"USVC" => CommunicationsClass::UsedWithinFIRUIRForOtherPurposes,
            b"ASVC" => CommunicationsClass::UsedWithinFIRUIRForBroadcastServices,
            b"ATCF" => CommunicationsClass::UsedWithinTerminalAreaForControl,
            b"AOTF" => CommunicationsClass::UsedwithinTerminalAreaForOtherPurposes,
            b"AFAC" => CommunicationsClass::UsedWithinTerminalAreaForBroadcastServices,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid communications class".to_string(),
                ));
            }
        }))
    }
}

/// 5.286 Multi-Sector Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum MultiSectorIndicator {
    MultiSector,
    SingleSector,
}

impl ParseableField for MultiSectorIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => MultiSectorIndicator::MultiSector,
            b"N" => MultiSectorIndicator::SingleSector,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid multi-sector indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.287 Communications Type Recognized By
#[derive(Debug, PartialEq, Eq)]
pub enum CommunicationsTypeRecognizedBy {
    ICAO,
    FAA,
    ICAOAndFAA,
    CountryAuthority,
    DataProvider,
}

impl ParseableField for CommunicationsTypeRecognizedBy {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"I" => CommunicationsTypeRecognizedBy::ICAO,
            b"F" => CommunicationsTypeRecognizedBy::FAA,
            b"B" => CommunicationsTypeRecognizedBy::ICAOAndFAA,
            b"C" => CommunicationsTypeRecognizedBy::CountryAuthority,
            b"S" => CommunicationsTypeRecognizedBy::DataProvider,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid communications type recognized by".to_string(),
                ));
            }
        }))
    }
}

/// 5.289 Communications Used On
#[derive(Debug, PartialEq, Eq)]
pub enum CommunicationsUsedOn {
    AirportCommunicationsRecordsOnly,
    EnrouteCommunicationsRecordsOnly,
    HeliportCommunicationsRecordsOnly,
    AllApplicableCommunicationsRecords,
    AirportAndHeliportCommunicationsRecords,
}

impl ParseableField for CommunicationsUsedOn {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => CommunicationsUsedOn::AirportCommunicationsRecordsOnly,
            b"E" => CommunicationsUsedOn::EnrouteCommunicationsRecordsOnly,
            b"H" => CommunicationsUsedOn::HeliportCommunicationsRecordsOnly,
            b"B" => CommunicationsUsedOn::AllApplicableCommunicationsRecords,
            b"C" => CommunicationsUsedOn::AirportAndHeliportCommunicationsRecords,
            _ => {
                return Err(FieldParseError::new("Invalid used on".to_string()));
            }
        }))
    }
}

/// 5.291 Procedure Design Magnetic Variation Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum ProcedureDesignMagneticVariationIndicator {
    EntireProcedure,
    AssociatedLeg,
}

impl ParseableField for ProcedureDesignMagneticVariationIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"P" => ProcedureDesignMagneticVariationIndicator::EntireProcedure,
            b"L" => ProcedureDesignMagneticVariationIndicator::AssociatedLeg,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid procedure design magnetic variation indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.297 Route Inappropriate Navaid Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum RouteInappropriateNavaidIndicator {
    Appropriate,
    Inappropriate,
}

impl ParseableField for RouteInappropriateNavaidIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => RouteInappropriateNavaidIndicator::Inappropriate,
            b"N" => RouteInappropriateNavaidIndicator::Appropriate,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid route inappropriate navaid indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.298 Holding Pattern/Race Track Course Reversal Leg Inbound/Outbound Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum HoldingPatternCourseReversalLegIndicator {
    Inbound,
    Outbound,
}

impl ParseableField for HoldingPatternCourseReversalLegIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"I" => HoldingPatternCourseReversalLegIndicator::Inbound,
            b"O" => HoldingPatternCourseReversalLegIndicator::Outbound,
            _ => {
                return Err(FieldParseError::new("Invalid holding pattern/race track course reversal leg inbound/outbound indicator".to_string()));
            }
        }))
    }
}

/// 5.301 Procedure Design Aircraft Category or Type
#[derive(Debug, PartialEq, Eq)]
pub enum ProcedureDesignAircraftCategoryOrType {
    CategoryA,
    CategoryB,
    CategoryC,
    CategoryD,
    CategoryE,
    CategoriesAB,
    CategoriesCD,
    CategoriesABC,
    CategoriesABCD,
    CategoriesABCDE,
    CategoriesDE,
    Helicopters,
    CategoriesBC,
    CategoriesCDE,
    CategoriesBCDE,
    Jets,
    NonJets,
    Pistons,
    NotLimited,
    TurbojetAndTurboprop,
    Turbojet,
    Turboprop,
    Prop,
    TurbopropAndProp,
    NonTurbojets,
    NotProvided,
}

impl ParseableField for ProcedureDesignAircraftCategoryOrType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => ProcedureDesignAircraftCategoryOrType::CategoryA,
            b"B" => ProcedureDesignAircraftCategoryOrType::CategoryB,
            b"C" => ProcedureDesignAircraftCategoryOrType::CategoryC,
            b"D" => ProcedureDesignAircraftCategoryOrType::CategoryD,
            b"E" => ProcedureDesignAircraftCategoryOrType::CategoryE,
            b"F" => ProcedureDesignAircraftCategoryOrType::CategoriesAB,
            b"G" => ProcedureDesignAircraftCategoryOrType::CategoriesCD,
            b"I" => ProcedureDesignAircraftCategoryOrType::CategoriesABC,
            b"J" => ProcedureDesignAircraftCategoryOrType::CategoriesABCD,
            b"K" => ProcedureDesignAircraftCategoryOrType::CategoriesABCDE,
            b"L" => ProcedureDesignAircraftCategoryOrType::CategoriesDE,
            b"H" => ProcedureDesignAircraftCategoryOrType::Helicopters,
            b"M" => ProcedureDesignAircraftCategoryOrType::CategoriesBC,
            b"N" => ProcedureDesignAircraftCategoryOrType::CategoriesCDE,
            b"O" => ProcedureDesignAircraftCategoryOrType::CategoriesBCDE,
            b"W" => ProcedureDesignAircraftCategoryOrType::Jets,
            b"X" => ProcedureDesignAircraftCategoryOrType::NonJets,
            b"Y" => ProcedureDesignAircraftCategoryOrType::Pistons,
            b"P" => ProcedureDesignAircraftCategoryOrType::NotLimited,
            b"Q" => ProcedureDesignAircraftCategoryOrType::TurbojetAndTurboprop,
            b"R" => ProcedureDesignAircraftCategoryOrType::Turbojet,
            b"S" => ProcedureDesignAircraftCategoryOrType::Turboprop,
            b"T" => ProcedureDesignAircraftCategoryOrType::Prop,
            b"U" => ProcedureDesignAircraftCategoryOrType::TurbopropAndProp,
            b"V" => ProcedureDesignAircraftCategoryOrType::NonTurbojets,
            [BLANK] => ProcedureDesignAircraftCategoryOrType::NotProvided,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid procedure design aircraft category or type".to_string(),
                ));
            }
        }))
    }
}

/// 5.299 Surface Type
#[derive(Debug, PartialEq, Eq)]
pub enum SurfaceType {
    Asphalt,
    ApshaltAndGrass,
    BituminousSurface,
    Brick,
    Clay,
    Concrete,
    ConcreteAndAsphalt,
    ConcreteAndGrass,
    Coral,
    Dirt,
    Grass,
    Gravel,
    Ice,
    Laterite,
    Macadam,
    LandingMat,
    Laminate,
    Metal,
    NonBituminousMix,
    Other,
    Paved,
    PiercedSteelPlanking,
    Sand,
    Sealed,
    Silt,
    Snow,
    Soil,
    Stone,
    Tarmac,
    Treated,
    Turf,
    Unknown,
    Unpaved,
    Water,
}

impl ParseableField for SurfaceType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"ASPH" => SurfaceType::Asphalt,
            b"ASGR" => SurfaceType::ApshaltAndGrass,
            b"BITU" => SurfaceType::BituminousSurface,
            b"BRCK" => SurfaceType::Brick,
            b"CLAY" => SurfaceType::Clay,
            b"CONC" => SurfaceType::Concrete,
            b"COAS" => SurfaceType::ConcreteAndAsphalt,
            b"COGS" => SurfaceType::ConcreteAndGrass,
            b"CORL" => SurfaceType::Coral,
            b"DIRT" => SurfaceType::Dirt,
            b"GRAS" => SurfaceType::Grass,
            b"GRVL" => SurfaceType::Gravel,
            b"ICE " => SurfaceType::Ice,
            b"LATE" => SurfaceType::Laterite,
            b"MACA" => SurfaceType::Macadam,
            b"MATS" => SurfaceType::LandingMat,
            b"MEMB" => SurfaceType::Laminate,
            b"META" => SurfaceType::Metal,
            b"MIX " => SurfaceType::NonBituminousMix,
            b"OTHR" => SurfaceType::Other,
            b"PAVD" => SurfaceType::Paved,
            b"PSP " => SurfaceType::PiercedSteelPlanking,
            b"SAND" => SurfaceType::Sand,
            b"SELD" => SurfaceType::Sealed,
            b"SILT" => SurfaceType::Silt,
            b"SNOW" => SurfaceType::Snow,
            b"SOIL" => SurfaceType::Soil,
            b"STON" => SurfaceType::Stone,
            b"TARM" => SurfaceType::Tarmac,
            b"TRTD" => SurfaceType::Treated,
            b"TURF" => SurfaceType::Turf,
            b"UNKN" => SurfaceType::Unknown,
            b"UNPV" => SurfaceType::Unpaved,
            b"WATE" => SurfaceType::Water,
            _ => {
                return Err(FieldParseError::new("Invalid surface type".to_string()));
            }
        }))
    }
}

/// 5.303 Helipad Shape
#[derive(Debug, PartialEq, Eq)]
pub enum HelipadShape {
    Circle,
    Rectangular,
    Runway,
    Undefined,
}

impl ParseableField for HelipadShape {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => HelipadShape::Circle,
            b"S" => HelipadShape::Rectangular,
            b"R" => HelipadShape::Runway,
            b"U" => HelipadShape::Undefined,
            _ => {
                return Err(FieldParseError::new("Invalid helipad shape".to_string()));
            }
        }))
    }
}

/// 5.305 Heliport Type
#[derive(Debug, PartialEq, Eq)]
pub enum HeliportType {
    Hospital,
    OilRig,
    Other,
    NotProvided,
}

impl ParseableField for HeliportType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => HeliportType::Hospital,
            b"O" => HeliportType::OilRig,
            [BLANK] => HeliportType::Other,
            b"N" => HeliportType::NotProvided,
            _ => {
                return Err(FieldParseError::new("Invalid heliport type".to_string()));
            }
        }))
    }
}

/// 5.306 Preferred Multiple Approach Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum PreferredMultipleApproachIndicator {
    Preferred,
    NotPreferred,
}

impl ParseableField for PreferredMultipleApproachIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"P" => PreferredMultipleApproachIndicator::Preferred,
            [BLANK] => PreferredMultipleApproachIndicator::NotPreferred,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid preferred multiple approach indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.307 Terminal Procedure Special Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum SpecialProcedureIndicator {
    SpecialProcedure,
    NotASpecialProcedure,
}

impl ParseableField for SpecialProcedureIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => SpecialProcedureIndicator::SpecialProcedure,
            [BLANK] => SpecialProcedureIndicator::NotASpecialProcedure,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid terminal procedure special indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.308 Remote Altimeter Flag
#[derive(Debug, PartialEq, Eq)]
pub enum RemoteAltimeterFlag {
    LNAVVNAVRestricted,
    NotRestricted,
}

impl ParseableField for RemoteAltimeterFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => RemoteAltimeterFlag::LNAVVNAVRestricted,
            [BLANK] => RemoteAltimeterFlag::NotRestricted,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid remote altimeter flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.310 Helicopter Performance Requirement
#[derive(Debug, PartialEq, Eq)]
pub enum HelicopterPerformanceRequirement {
    MultiEngineRequired,
    SingleEngine,
    Unknown,
}

impl ParseableField for HelicopterPerformanceRequirement {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"M" => HelicopterPerformanceRequirement::MultiEngineRequired,
            b"S" => HelicopterPerformanceRequirement::SingleEngine,
            b"U" => HelicopterPerformanceRequirement::Unknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid helicopter performance requirement".to_string(),
                ));
            }
        }))
    }
}

/// 5.311 FIR/FRA Transition Type
#[derive(Debug, PartialEq, Eq)]
pub enum FIRFRATransitionType {
    EntryPoint,
    ExitPoint,
    ArrivalTransitionPoint,
    DepartureTransitinoPoint,
    IntermediatePoint,
    Unknown,
}

impl ParseableField for FIRFRATransitionType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"E" => FIRFRATransitionType::EntryPoint,
            b"X" => FIRFRATransitionType::ExitPoint,
            b"A" => FIRFRATransitionType::ArrivalTransitionPoint,
            b"D" => FIRFRATransitionType::DepartureTransitinoPoint,
            b"I" => FIRFRATransitionType::IntermediatePoint,
            b"H" => FIRFRATransitionType::Unknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid FIR/FRA transition type".to_string(),
                ));
            }
        }))
    }
}

/// 5.317 Runway Usage Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum RunwayUsageIndicator {
    LandingOnly,
    TakeoffOnly,
    TakeoffAndLanding,
}

impl ParseableField for RunwayUsageIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"L" => RunwayUsageIndicator::LandingOnly,
            b"T" => RunwayUsageIndicator::TakeoffOnly,
            b"B" => RunwayUsageIndicator::TakeoffAndLanding,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid runway usage indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.318 Runway Accuracy Compliance Flag
#[derive(Debug, PartialEq, Eq)]
pub enum RunwayAccuracyComplianceFlag {
    Compliant,
    NonCompliant,
    NotEvaluated,
}

impl ParseableField for RunwayAccuracyComplianceFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => RunwayAccuracyComplianceFlag::Compliant,
            b"N" => RunwayAccuracyComplianceFlag::NonCompliant,
            [BLANK] => RunwayAccuracyComplianceFlag::NotEvaluated,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid runway accuracy compliance flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.319 Landing Threshold Elevation Accuracy Compliance Flag
#[derive(Debug, PartialEq, Eq)]
pub enum LandingThresholdElevationAccuracyComplianceFlag {
    Compliant,
    NonCompliant,
    NotEvaluated,
}

impl ParseableField for LandingThresholdElevationAccuracyComplianceFlag {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => LandingThresholdElevationAccuracyComplianceFlag::Compliant,
            b"N" => LandingThresholdElevationAccuracyComplianceFlag::NonCompliant,
            [BLANK] => LandingThresholdElevationAccuracyComplianceFlag::NotEvaluated,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid runway accuracy compliance flag".to_string(),
                ));
            }
        }))
    }
}

/// 5.322 Helipad Type
#[derive(Debug, PartialEq, Eq)]
pub enum HelipadType {
    Elevated,
    OtherOrUnknown,
}

impl ParseableField for HelipadType {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"E" => HelipadType::Elevated,
            [BLANK] => HelipadType::OtherOrUnknown,
            _ => {
                return Err(FieldParseError::new("Invalid helipad type".to_string()));
            }
        }))
    }
}

/// 5.337 ATN ATSU Ground Facility Use Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum ATNATSUGroundFacilityUseIndicator {
    Implemented,
    Future,
    TestFacility,
    Unknown,
}

impl ParseableField for ATNATSUGroundFacilityUseIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => ATNATSUGroundFacilityUseIndicator::Implemented,
            b"N" => ATNATSUGroundFacilityUseIndicator::Future,
            b"T" => ATNATSUGroundFacilityUseIndicator::TestFacility,
            [BLANK] => ATNATSUGroundFacilityUseIndicator::Unknown,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid ATN ATSU ground facility use indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.338 VHF Navaid VOR Range/Power
#[derive(Debug, PartialEq, Eq)]
pub enum VHFNavaidVorRangePower {
    Within25NMTo12000Feet,
    Within40NMTo18000Feet,
    Within130NMTo60000Feet,
    NotProvided,
    Within70NMTo18000FeetExpanded,
    Within130NMTo60000FeetExpanded,
}

impl ParseableField for VHFNavaidVorRangePower {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"T" => VHFNavaidVorRangePower::Within25NMTo12000Feet,
            b"L" => VHFNavaidVorRangePower::Within40NMTo18000Feet,
            b"H" => VHFNavaidVorRangePower::Within130NMTo60000Feet,
            b"U" => VHFNavaidVorRangePower::NotProvided,
            b"M" => VHFNavaidVorRangePower::Within70NMTo18000FeetExpanded,
            b"N" => VHFNavaidVorRangePower::Within130NMTo60000FeetExpanded,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid VHF Navaid VOR Range/Power".to_string(),
                ));
            }
        }))
    }
}

/// 5.339 DME Expanded Service Volume
#[derive(Debug, PartialEq, Eq)]
pub enum DMEExpandedServiceVolume {
    Within130NMTo18000FeetExpanded,
    Within130NMTo60000FeetExpanded,
    NotProvided,
}

impl ParseableField for DMEExpandedServiceVolume {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => DMEExpandedServiceVolume::Within130NMTo18000FeetExpanded,
            b"G" => DMEExpandedServiceVolume::Within130NMTo60000FeetExpanded,
            b"U" => DMEExpandedServiceVolume::NotProvided,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid DME expanded service volume".to_string(),
                ));
            }
        }))
    }
}

/// 5.340 Unmanned Aerial Vehicle (UAV) Only
#[derive(Debug, PartialEq, Eq)]
pub enum UnmannedAerialVehicleOnly {
    Yes,
    No,
}

impl ParseableField for UnmannedAerialVehicleOnly {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => UnmannedAerialVehicleOnly::Yes,
            [BLANK] => UnmannedAerialVehicleOnly::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid unmanned aerial vehicle only".to_string(),
                ));
            }
        }))
    }
}

/// 5.341 Terminal Procedure For Military Indicator
#[derive(Debug, PartialEq, Eq)]
pub enum TerminalProcedureForMilitaryIndicator {
    Yes,
    No,
}

impl ParseableField for TerminalProcedureForMilitaryIndicator {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => TerminalProcedureForMilitaryIndicator::Yes,
            [BLANK] => TerminalProcedureForMilitaryIndicator::No,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid terminal procedure for military indicator".to_string(),
                ));
            }
        }))
    }
}

/// 5.342 Source of LAL/VAL
#[derive(Debug, PartialEq, Eq)]
pub enum SourceOfLALVAL {
    OfficialSource,
    DerivedFromLinesOfMinima,
    BasicDefaults,
}

impl ParseableField for SourceOfLALVAL {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => SourceOfLALVAL::OfficialSource,
            b"M" => SourceOfLALVAL::DerivedFromLinesOfMinima,
            b"N" => SourceOfLALVAL::BasicDefaults,
            _ => {
                return Err(FieldParseError::new(
                    "Invalid source of LAL/VAL".to_string(),
                ));
            }
        }))
    }
}
