//! # ARINC 424 Definitions - Enums
//! This module contains the enums for the ARINC 424 data.
//! Enums are not a formally defined data type in the spec, but are obviously represented as such given
//! the character mapping tables.
//!
//! Example is 5.4 - Section Code which describes the major section of the record.
use crate::parsers::arinc424::fields::BLANK;
use crate::parsers::arinc424::fields::FieldParseError;

#[derive(Debug, PartialEq, Eq)]
pub enum RecordType {
    Standard,
    Tailored,
}

impl RecordType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"S" => RecordType::Standard,
            b"T" => RecordType::Tailored,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid record type".to_string(),
                });
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

impl Section {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid major section code".to_string(),
                });
            }
        }))
    }
}

/// 5.5(A) MORA Subsection Code
#[derive(Debug, PartialEq, Eq)]
pub enum MORASubsection {
    GridMORA,
}

impl MORASubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"S" => MORASubsection::GridMORA,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid MORA subsection code".to_string(),
                });
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

impl NavaidSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => NavaidSubsection::VHFNavaid,
            b"B" => NavaidSubsection::NDBNavaid,
            b"T" => NavaidSubsection::TACANDuplicates,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Navaid subsection code".to_string(),
                });
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

impl EnrouteSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid Enroute subsection code".to_string(),
                });
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

impl HeliportSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid Heliport subsection code".to_string(),
                });
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

impl AirportSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid Airport subsection code".to_string(),
                });
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

impl CompanyRoutesSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => CompanyRoutesSubsection::CompanyRoutes,
            b"A" => CompanyRoutesSubsection::AlternateRecords,
            b"H" => CompanyRoutesSubsection::HelicopterOperationRoutes,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Company Routes subsection code".to_string(),
                });
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

impl TablesSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => TablesSubsection::CruisingTables,
            b"G" => TablesSubsection::GeographicalReference,
            b"L" => TablesSubsection::ATNData,
            b"V" => TablesSubsection::CommunicationType,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Tables subsection code".to_string(),
                });
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

impl AirspaceSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => AirspaceSubsection::ControlledAirspace,
            b"F" => AirspaceSubsection::FIRUIR,
            b"R" => AirspaceSubsection::RestrictiveAirspace,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Airspace subsection code".to_string(),
                });
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

impl EnrouteAirwayRouteType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid enroute airway route type".to_string(),
                });
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

impl PreferredRouteRouteType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid preferred route route type".to_string(),
                });
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
    VectorRunwayTransition,
    VectorEnrouteTransition,
}

impl SIDRouteType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => SIDRouteType::EngineOut,
            b"1" => SIDRouteType::RunwayTransition,
            b"2" => SIDRouteType::CommonRoute,
            b"3" => SIDRouteType::EnrouteTransition,
            b"T" => SIDRouteType::VectorRunwayTransition,
            b"V" => SIDRouteType::VectorEnrouteTransition,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid SID route type".to_string(),
                });
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
}

impl STARRouteType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"1" => STARRouteType::EnrouteTransition,
            b"2" => STARRouteType::CommonRoute,
            b"3" => STARRouteType::RunwayTransition,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid STAR route type".to_string(),
                });
            }
        }))
    }
}

/// 5.7(E) Airport Heliport Approach Route Type
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
    LDAApproach,
    RFApproachTransition,
    MissedApproach,
}

impl AirportHeliportApproachRouteType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
            b"X" => AirportHeliportApproachRouteType::LDAApproach,
            b"Y" => AirportHeliportApproachRouteType::RFApproachTransition,
            b"Z" => AirportHeliportApproachRouteType::MissedApproach,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid airport heliport approach route type".to_string(),
                });
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

impl CustomerAreaCode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.len() != 3 {
            return Err(FieldParseError {
                message: "Customer area code must be 3 characters long".to_string(),
            });
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
            _ => CustomerAreaCode::Customer(String::from_utf8(bytes.to_vec()).map_err(|e| {
                FieldParseError {
                    message: e.to_string(),
                }
            })?),
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

impl BoundaryCode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid boundary code".to_string(),
                });
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

impl Level {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => Level::AllAltitudes,
            b"H" => Level::HighLevelAirwaysAltitudes,
            b"L" => Level::LowLevelAirwaysAltitudes,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid level".to_string(),
                });
            }
        }))
    }
}

/// 5.20 Turn Direction
#[derive(Debug, PartialEq, Eq)]
pub enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"L" => TurnDirection::Left,
            b"R" => TurnDirection::Right,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid turn direction".to_string(),
                });
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

impl TurnDirectionValid {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => TurnDirectionValid::Yes,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid turn direction valid".to_string(),
                });
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
    AtUntilInbound,
}

impl CrossingAltitudeDescription {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
            b"G" => CrossingAltitudeDescription::GlideslopeAltitude,
            b"O" => CrossingAltitudeDescription::AtUntilInbound,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid crossing altitude description".to_string(),
                });
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

impl LocalizerAzimuthPositionReference {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"@" => LocalizerAzimuthPositionReference::AheadOfApproachEnd,
            b"+" => LocalizerAzimuthPositionReference::BeyondStopEnd,
            b"-" => LocalizerAzimuthPositionReference::Aside,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid localizer azimuth position reference".to_string(),
                });
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

impl Turn {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"L" => Turn::Left,
            b"R" => Turn::Right,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid turn".to_string(),
                });
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

impl CompanyRouteVIACode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid company route VIA code".to_string(),
                });
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

impl PreferredRouteVIACode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid preferred route VIA code".to_string(),
                });
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

impl IlsMlsGlsCategory {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid ILS/MLS/GLS category".to_string(),
                });
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

impl AtcIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AtcIndicator::ATCAssignmentOptional,
            b"S" => AtcIndicator::ATCAssignmentRequired,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid ATC indicator".to_string(),
                });
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

impl WaypointUsage {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"B" => WaypointUsage::HIAndLo,
            b"H" => WaypointUsage::Hi,
            b"L" => WaypointUsage::Lo,
            [BLANK] => WaypointUsage::TerminalOnly,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid waypoint usage".to_string(),
                });
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
    FlightPlanningApplicationContinuation,
    SimulationApplicationContinuation,
    FormattedTimeOfOperationsContinuation,
    NarrativeTimeOfOperationsContinuation,
    AirportHeliportProcedureDataContinuation,
    AirportSIDSTARApproachProcedureNameContinuation,
}

impl ContinuationRecordApplicationType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
            b"P" => ContinuationRecordApplicationType::FlightPlanningApplicationContinuation,
            b"S" => ContinuationRecordApplicationType::SimulationApplicationContinuation,
            b"T" => ContinuationRecordApplicationType::FormattedTimeOfOperationsContinuation,
            b"U" => ContinuationRecordApplicationType::NarrativeTimeOfOperationsContinuation,
            b"W" => ContinuationRecordApplicationType::AirportHeliportProcedureDataContinuation,
            b"Y" => {
                ContinuationRecordApplicationType::AirportSIDSTARApproachProcedureNameContinuation
            }
            _ => {
                return Err(FieldParseError {
                    message: "Invalid continuation record application type".to_string(),
                });
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

impl GovernmentSource {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"O" => GovernmentSource::OfficialGovernment,
            b"R" => GovernmentSource::OtherSource,
            b"T" => GovernmentSource::OnlyTrue,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid government source".to_string(),
                });
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

impl ElevationType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => ElevationType::AirportHeliportElevation,
            b"L" => ElevationType::LandingThresholdElevation,
            b"R" => ElevationType::DisplacedThresholdRunwayEndElevation,
            b"T" => ElevationType::TouchdownZoneElevation,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid elevation type".to_string(),
                });
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

impl CommunicationsType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid communications type".to_string(),
                });
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

impl Radar {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(Radar::NonRadar));
        }
        Ok(Some(match bytes {
            b"R" => Radar::Radar,
            b"N" => Radar::NonRadar,
            b"U" => Radar::Unknown,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid radar".to_string(),
                });
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

impl FrequencyUnits {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid frequency units".to_string(),
                });
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

impl IfrCapability {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"I" => IfrCapability::HasApproach,
            b"N" => IfrCapability::NoApproach,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid IFR capability".to_string(),
                });
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

impl MarkerRadiationShape {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => MarkerRadiationShape::Bone,
            b"E" => MarkerRadiationShape::EllipseOrUnknown,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid marker radiation shape".to_string(),
                });
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

impl EnrouteMarkerPower {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => EnrouteMarkerPower::High,
            b"L" => EnrouteMarkerPower::Low,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid enroute marker power".to_string(),
                });
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

impl EnrouteDirectionalRestriction {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"F" => EnrouteDirectionalRestriction::ForwardOnly,
            b"B" => EnrouteDirectionalRestriction::BackwardOnly,
            [BLANK] => EnrouteDirectionalRestriction::NoRestriction,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid enroute directional restriction".to_string(),
                });
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

impl PreferredRouteDirectionalRestriction {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => PreferredRouteDirectionalRestriction::ForwardOnly,
            b"B" => PreferredRouteDirectionalRestriction::Bidirectional,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid preferred route directional restriction".to_string(),
                });
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

impl FirUirIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => FirUirIndicator::FIR,
            b"U" => FirUirIndicator::UIR,
            b"B" => FirUirIndicator::Combined,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid FIR/UIR indicator".to_string(),
                });
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

impl FirUirReportingUnitsSpeed {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => FirUirReportingUnitsSpeed::NotSpecified,
            b"1" => FirUirReportingUnitsSpeed::Knots,
            b"2" => FirUirReportingUnitsSpeed::Mach,
            b"3" => FirUirReportingUnitsSpeed::KPH,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid FIR/UIR reporting units speed".to_string(),
                });
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

impl FirUirReportingUnitsAltitude {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"0" => FirUirReportingUnitsAltitude::NotSpecified,
            b"1" => FirUirReportingUnitsAltitude::FlightLevel,
            b"2" => FirUirReportingUnitsAltitude::Meters,
            b"3" => FirUirReportingUnitsAltitude::Feet,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid FIR/UIR reporting units altitude".to_string(),
                });
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

impl FirUirEntryReport {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => FirUirEntryReport::Required,
            b"N" => FirUirEntryReport::NotRequired,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid FIR/UIR entry report".to_string(),
                });
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

impl RestrictiveAirspaceType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid restrictive airspace type".to_string(),
                });
            }
        }))
    }
}

/// 5.131(A) Primary Record Time Code
pub enum PrimaryRecordTimeCode {
    ActiveContinuouslyIncludingHolidays,
    ActiveContinuouslyExcludingHolidays,
    ActiveNonContinuously,
    ActiveDuringNOTAM,
    NotSpecified,
}

impl PrimaryRecordTimeCode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid primary record time code".to_string(),
                });
            }
        }))
    }
}

/// 5.131(B) Continuation Record Time Code
pub enum ContinuationRecordTimeCode {
    ExcludingHolidays,
    IncludingHolidays,
}

impl ContinuationRecordTimeCode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"H" => ContinuationRecordTimeCode::ExcludingHolidays,
            b"T" => ContinuationRecordTimeCode::IncludingHolidays,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid continuation record time code".to_string(),
                });
            }
        }))
    }
}

/// 5.132 NOTAM Flag
pub enum NotamFlag {
    ActiveByNotam,
    NotActiveByNotam,
}

impl NotamFlag {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"N" => NotamFlag::ActiveByNotam,
            [BLANK] => NotamFlag::NotActiveByNotam,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid NOTAM flag".to_string(),
                });
            }
        }))
    }
}

/// 5.133 Unit Indicator
pub enum AirspaceLimitUnitIndicator {
    AboveGroundLevel,
    MeanSeaLevel,
}

impl AirspaceLimitUnitIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AirspaceLimitUnitIndicator::AboveGroundLevel,
            b"M" => AirspaceLimitUnitIndicator::MeanSeaLevel,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid airspace limit unit indicator".to_string(),
                });
            }
        }))
    }
}

/// 5.134 Cruise Table Identifier
pub enum CruiseTableIdentifier {
    ICAOCruiseTable,
    ExceptionToICAOCruiseTable,
    ModifiedCruiseTable(Box<str>),
    ExceptionToModifiedCruiseTable(Box<str>),
}

impl CruiseTableIdentifier {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let string = Box::from(std::str::from_utf8(bytes).map_err(|e| FieldParseError {
            message: e.to_string(),
        })?);
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
                    return Err(FieldParseError {
                        message: "Invalid cruise table identifier".to_string(),
                    });
                }
            },
            _ => {
                return Err(FieldParseError {
                    message: "Invalid cruise table identifier".to_string(),
                });
            }
        }))
    }
}

/// 5.138 Time Indicator
pub enum TimeIndicator {
    LocalTimeWithDST,
    LocalTimeWithoutDST,
    UTCWithDST,
    UTCWithoutDST,
}

impl TimeIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"T" => TimeIndicator::LocalTimeWithDST,
            b"L" => TimeIndicator::LocalTimeWithoutDST,
            b"S" => TimeIndicator::UTCWithDST,
            b"Z" => TimeIndicator::UTCWithoutDST,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid time indicator".to_string(),
                });
            }
        }))
    }
}

/// 5.149 NavaidUsableRange
pub enum NavaidUsableRange {
    Terminal,
    LowAltitude,
    HighAltitude,
    ExtendedHighAltitude,
    NavaidNotCivil,
    NavaidOutOfService,
}

impl NavaidUsableRange {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid Navaid usable range".to_string(),
                });
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

impl BaroVnavAuthorization {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"X" => BaroVnavAuthorization::Authorized,
            [BLANK] => BaroVnavAuthorization::NotAuthorized,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid BARO-VNAV authorization flag".to_string(),
                });
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

impl VFRCheckpointFlag {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => VFRCheckpointFlag::Yes,
            [BLANK] => VFRCheckpointFlag::No,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid VFR checkpoint flag".to_string(),
                });
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

impl AtcAssignedOnly {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => AtcAssignedOnly::Yes,
            [BLANK] => AtcAssignedOnly::No,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid ATC assigned only flag".to_string(),
                });
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

impl AirwayRestrictionAltitudeUnit {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"F" => AirwayRestrictionAltitudeUnit::HundredsOfFeet,
            b"K" => AirwayRestrictionAltitudeUnit::MetricFlightLevels,
            b"L" => AirwayRestrictionAltitudeUnit::FeetFlightLevels,
            b"M" => AirwayRestrictionAltitudeUnit::TensOfMeters,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid airway restriction altitude unit".to_string(),
                });
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

impl StepClimbIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => StepClimbIndicator::StepClimbUpDown,
            b"D" => StepClimbIndicator::StepClimbDown,
            b"N" => StepClimbIndicator::NoStepClimbPermitted,
            b"U" => StepClimbIndicator::StepClimbUp,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid step climb indicator".to_string(),
                });
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

impl EnrouteAirwayRestrictionFlag {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => EnrouteAirwayRestrictionFlag::Yes,
            [BLANK] => EnrouteAirwayRestrictionFlag::No,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid enroute airway restriction flag".to_string(),
                });
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

impl MagneticTrueIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"M" => MagneticTrueIndicator::Magnetic,
            b"T" => MagneticTrueIndicator::True,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid magnetic/true indicator".to_string(),
                });
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

impl RestrictiveAirspaceLinkContinuation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => RestrictiveAirspaceLinkContinuation::Yes,
            [BLANK] => RestrictiveAirspaceLinkContinuation::No,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid restrictive airspace link continuation flag".to_string(),
                });
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

impl PublicMilitaryIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"C" => PublicMilitaryIndicator::Public,
            b"M" => PublicMilitaryIndicator::Military,
            b"J" => PublicMilitaryIndicator::Joint,
            b"P" => PublicMilitaryIndicator::Private,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid public/military indicator".to_string(),
                });
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

impl DaylightTimeObservedIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => DaylightTimeObservedIndicator::Yes,
            b"N" => DaylightTimeObservedIndicator::NoOrUnknown,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid daylight time observed indicator".to_string(),
                });
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

impl H24Indicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"Y" => H24Indicator::Yes,
            b"N" => H24Indicator::No,
            b"U" => H24Indicator::Unknown,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid H24 indicator".to_string(),
                });
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

impl DistanceDescription {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"-" => DistanceDescription::AppliedUpToDistance,
            b"+" => DistanceDescription::AppliedFromDistance,
            [BLANK] => DistanceDescription::NotAppliedOrAtSpecifiedDistance,
            b"B" => DistanceDescription::BetweenSpecifiedDistances,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid distance description".to_string(),
                });
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

impl Modulation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => Modulation::AM,
            b"F" => Modulation::FM,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid modulation".to_string(),
                });
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

impl SignalEmission {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid signal emission".to_string(),
                });
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

impl RestrictionRecordType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"AE" => RestrictionRecordType::AltitudeExclusion,
            b"TC" => RestrictionRecordType::CruisingTableReplacement,
            b"SC" => RestrictionRecordType::SeasonalRestriction,
            b"NR" => RestrictionRecordType::NoteRestriction,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid restriction record type".to_string(),
                });
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

impl AltitudeExclusionIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => AltitudeExclusionIndicator::AllAltitudesBothDirections,
            b"B" => AltitudeExclusionIndicator::AllAltitudesBackwardDirection,
            b"F" => AltitudeExclusionIndicator::AllAltitudesForwardDirection,
            [BLANK] => AltitudeExclusionIndicator::NoAltitudeExclusion,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid altitude exclusion indicator".to_string(),
                });
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

impl BlockAltitudeIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"B" => BlockAltitudeIndicator::BlockAltitude,
            b"I" => BlockAltitudeIndicator::IndividualAltitude,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid block altitude indicator".to_string(),
                });
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

impl NavaidLimitationCode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid navaid limitation code".to_string(),
                });
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

impl ComponentAffectedIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
                return Err(FieldParseError {
                    message: "Invalid component affected indicator".to_string(),
                });
            }
        }))
    }
}
