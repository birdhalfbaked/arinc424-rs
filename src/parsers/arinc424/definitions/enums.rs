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

#[derive(Debug, PartialEq, Eq)]
pub enum MORASubsection {
    GridMORA,
}

impl MORASubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
#[derive(Debug, PartialEq, Eq)]
pub enum TablesSubsection {
    CruisingTables,
    GeographicalReference,
    ATNData,
    CommunicationType,
}

impl TablesSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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

#[derive(Debug, PartialEq, Eq)]
pub enum AirspaceSubsection {
    ControlledAirspace,
    FIRUIR,
    RestrictiveAirspace,
}

impl AirspaceSubsection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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

#[derive(Debug, PartialEq, Eq)]
pub enum STARRouteType {
    EnrouteTransition,
    CommonRoute,
    RunwayTransition,
}

impl STARRouteType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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

// 5.18 Boundary Code

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

// 5.19 Level

#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    AllAltitudes,
    HighLevelAirwaysAltitudes,
    LowLevelAirwaysAltitudes,
}

impl Level {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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

// 5.20 Turn Direction
#[derive(Debug, PartialEq, Eq)]
pub enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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

// 5.22 Turn Direction Valid

#[derive(Debug, PartialEq, Eq)]
pub enum TurnDirectionValid {
    Yes,
    No,
}

impl TurnDirectionValid {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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

// 5.29 Crossing Altitude Description
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
