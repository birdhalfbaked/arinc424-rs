//! # ARINC 424 Definitions - Complexes
//! This module contains the complex types for the ARINC 424 data. This is not a formally defined thing, but
//! rather a term I have used to describe the encoding of multiple annotations onto one field definition in the spec
//! The first example of which is given in 5.35 - Navaid Class which describes a 5-letter code that adds annotations onto a Navaid record.
//!
//! The way we encode these complexes are as tuples. In the above example, the Navaid Class is encoded as a tuple of 5 enum fields. Thus,
//! when parsing a VHF Navaid record in particular we represent this as the combination of:
//!
//! - VHFNavaidType1
//! - VHFNavaidType2
//! - VHFRangePower
//! - VHFAdditionalInfo
//! - Collocation
//!
use crate::parsers::arinc424::fields::{BLANK, FieldParseError};

// VHF Navaid Class
#[derive(Debug, PartialEq, Eq)]
pub enum VHFNavaidType1 {
    VOR,
    Other,
}
impl VHFNavaidType1 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"V" => VHFNavaidType1::VOR,
            [BLANK] => VHFNavaidType1::Other,
            _ => {
                return Err(FieldParseError {
                    message: format!("Invalid VHF Navaid Type 1: '{}'", bytes[0] as char),
                });
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum VHFNavaidType2 {
    DME,
    TACAN,
    MILTACAN,
    ILSDMETACAN,
    MLSDMEN,
    MLSDMEP,
}
impl VHFNavaidType2 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"D" => VHFNavaidType2::DME,
            b"T" => VHFNavaidType2::TACAN,
            b"M" => VHFNavaidType2::MILTACAN,
            b"I" => VHFNavaidType2::ILSDMETACAN,
            b"N" => VHFNavaidType2::MLSDMEN,
            b"P" => VHFNavaidType2::MLSDMEP,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid VHF Navaid Type 2".to_string(),
                });
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum VHFRangePower {
    Terminal,
    LowAltitude,
    HighAltitude,
    Undefined,
    ILSTACAN,
}
impl VHFRangePower {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"T" => VHFRangePower::Terminal,
            b"L" => VHFRangePower::LowAltitude,
            b"H" => VHFRangePower::HighAltitude,
            b"U" => VHFRangePower::Undefined,
            b"C" => VHFRangePower::ILSTACAN,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Range Power".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum VHFAdditionalInfo {
    BiasedILSDMEOrILSTACAN,
    AutomaticTranscribedWeatherBroadcast,
    ScheduledWeatherBroadcast,
    NoVoiceOnFrequency,
    VoiceOnFrequency,
}

impl VHFAdditionalInfo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"D" => VHFAdditionalInfo::BiasedILSDMEOrILSTACAN,
            b"A" => VHFAdditionalInfo::AutomaticTranscribedWeatherBroadcast,
            b"B" => VHFAdditionalInfo::ScheduledWeatherBroadcast,
            b"W" => VHFAdditionalInfo::NoVoiceOnFrequency,
            [BLANK] => VHFAdditionalInfo::VoiceOnFrequency,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid VHF Additional Info".to_string(),
                });
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Collocation {
    Collocated,
    NonCollocated,
}

impl Collocation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => Collocation::Collocated,
            b"N" => Collocation::NonCollocated,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Collocation".to_string(),
                });
            }
        }))
    }
}

/// 5.35(A) NAVAID Class
#[derive(Debug, PartialEq, Eq)]
pub struct VHFNavaidClass(
    VHFNavaidType1,
    VHFNavaidType2,
    VHFRangePower,
    VHFAdditionalInfo,
    Collocation,
);
impl VHFNavaidClass {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(VHFNavaidClass(
            VHFNavaidType1::from_bytes(&bytes[0..1])?.ok_or(FieldParseError {
                message: "Invalid Navaid Type 1".to_string(),
            })?,
            VHFNavaidType2::from_bytes(&bytes[1..2])?.ok_or(FieldParseError {
                message: "Invalid Navaid Type 2".to_string(),
            })?,
            VHFRangePower::from_bytes(&bytes[2..3])?.ok_or(FieldParseError {
                message: "Invalid Range Power".to_string(),
            })?,
            VHFAdditionalInfo::from_bytes(&bytes[3..4])?.ok_or(FieldParseError {
                message: "Invalid Additional Info".to_string(),
            })?,
            Collocation::from_bytes(&bytes[4..5])?.ok_or(FieldParseError {
                message: "Invalid Collocation".to_string(),
            })?,
        )))
    }
}

// NDB Navaid Class
#[derive(Debug, PartialEq, Eq)]
pub enum NDBNavaidType1 {
    NDB,
    SABH,
    MarineBeacon,
}
impl NDBNavaidType1 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"H" => NDBNavaidType1::NDB,
            b"S" => NDBNavaidType1::SABH,
            b"M" => NDBNavaidType1::MarineBeacon,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid NDB Navaid Type 1".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NDBNavaidType2 {
    InnerMarker,
    MiddleMarker,
    OuterMarker,
    BackMarker,
}
impl NDBNavaidType2 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"I" => NDBNavaidType2::InnerMarker,
            b"M" => NDBNavaidType2::MiddleMarker,
            b"O" => NDBNavaidType2::OuterMarker,
            b"C" => NDBNavaidType2::BackMarker,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid NDB Navaid Type 2".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NDBRangePower {
    HighPoweredNDB,
    NDB,
    LowPoweredNDB,
    Locator,
}

impl NDBRangePower {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"H" => NDBRangePower::HighPoweredNDB,
            [BLANK] => NDBRangePower::NDB,
            b"M" => NDBRangePower::LowPoweredNDB,
            b"L" => NDBRangePower::Locator,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid NDB Range Power".to_string(),
                });
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum NDBAdditionalInfo {
    AutomaticTranscribedWeatherBroadcast,
    ScheduledWeatherBroadcast,
    NoVoiceOnFrequency,
    VoiceOnFrequency,
}

impl NDBAdditionalInfo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => NDBAdditionalInfo::AutomaticTranscribedWeatherBroadcast,
            b"B" => NDBAdditionalInfo::ScheduledWeatherBroadcast,
            b"W" => NDBAdditionalInfo::NoVoiceOnFrequency,
            [BLANK] => NDBAdditionalInfo::VoiceOnFrequency,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid NDB Additional Info".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NDBCollocation {
    BFOOperation,
    NonCollocated,
}

impl NDBCollocation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"B" => NDBCollocation::BFOOperation,
            b"N" => NDBCollocation::NonCollocated,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid NDB Collocation".to_string(),
                });
            }
        }))
    }
}

/// 5.35(B) NDB Navaid Class
#[derive(Debug, PartialEq, Eq)]
pub struct NDBNavaidClass(
    NDBNavaidType1,
    NDBNavaidType2,
    NDBRangePower,
    NDBAdditionalInfo,
    NDBCollocation,
);
impl NDBNavaidClass {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(NDBNavaidClass(
            NDBNavaidType1::from_bytes(&bytes[0..1])?.ok_or(FieldParseError {
                message: "Invalid Navaid Type 1".to_string(),
            })?,
            NDBNavaidType2::from_bytes(&bytes[1..2])?.ok_or(FieldParseError {
                message: "Invalid Navaid Type 2".to_string(),
            })?,
            NDBRangePower::from_bytes(&bytes[2..3])?.ok_or(FieldParseError {
                message: "Invalid Range Power".to_string(),
            })?,
            NDBAdditionalInfo::from_bytes(&bytes[3..4])?.ok_or(FieldParseError {
                message: "Invalid Additional Info".to_string(),
            })?,
            NDBCollocation::from_bytes(&bytes[4..5])?.ok_or(FieldParseError {
                message: "Invalid Collocation".to_string(),
            })?,
        )))
    }
}

// Localizer Marker/Locator Navaid Class
// we reuse all of the NDB fields except for Collocation

#[derive(Debug, PartialEq, Eq)]
pub enum MarkerCollocation {
    BFOOperation,
    LocatorMarkerCollocated,
    LocatorMarkerNotCollocated,
}
impl MarkerCollocation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"B" => MarkerCollocation::BFOOperation,
            b"A" => MarkerCollocation::LocatorMarkerCollocated,
            b"N" => MarkerCollocation::LocatorMarkerNotCollocated,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Marker Collocation".to_string(),
                });
            }
        }))
    }
}

/// 5.36(C) Localizer Marker/Locator Navaid Class
#[derive(Debug, PartialEq, Eq)]
pub struct MarkerLocatorNavaidClass(
    NDBNavaidType1,
    NDBNavaidType2,
    NDBRangePower,
    NDBAdditionalInfo,
    MarkerCollocation,
);
impl MarkerLocatorNavaidClass {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(MarkerLocatorNavaidClass(
            NDBNavaidType1::from_bytes(&bytes[0..1])?.ok_or(FieldParseError {
                message: "Invalid Navaid Type 1".to_string(),
            })?,
            NDBNavaidType2::from_bytes(&bytes[1..2])?.ok_or(FieldParseError {
                message: "Invalid Navaid Type 2".to_string(),
            })?,
            NDBRangePower::from_bytes(&bytes[2..3])?.ok_or(FieldParseError {
                message: "Invalid Range Power".to_string(),
            })?,
            NDBAdditionalInfo::from_bytes(&bytes[3..4])?.ok_or(FieldParseError {
                message: "Invalid Additional Info".to_string(),
            })?,
            MarkerCollocation::from_bytes(&bytes[4..5])?.ok_or(FieldParseError {
                message: "Invalid Collocation".to_string(),
            })?,
        )))
    }
}

// Waypoint Type

#[derive(Debug, PartialEq, Eq)]
pub enum PrimaryWaypointType {
    ArcCenterFix,
    CombinedNamedIntersectionAndOrDMEFixRNAVWaypoint,
    UnnamedChartedIntersectionAndOrDMEFix,
    MiddleInnerMarkerAsWaypoint,
    NDBAsWaypoint,
    OuterBackMarkerAsWaypoint,
    NamedIntersectionAndOrDMEFix,
    UnchartedAirwayIntersection,
    VFRWaypoint,
    RNAVWaypoint,
}
impl PrimaryWaypointType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => PrimaryWaypointType::ArcCenterFix,
            b"C" => PrimaryWaypointType::CombinedNamedIntersectionAndOrDMEFixRNAVWaypoint,
            b"I" => PrimaryWaypointType::UnnamedChartedIntersectionAndOrDMEFix,
            b"M" => PrimaryWaypointType::MiddleInnerMarkerAsWaypoint,
            b"N" => PrimaryWaypointType::NDBAsWaypoint,
            b"O" => PrimaryWaypointType::OuterBackMarkerAsWaypoint,
            b"R" => PrimaryWaypointType::NamedIntersectionAndOrDMEFix,
            b"U" => PrimaryWaypointType::UnchartedAirwayIntersection,
            b"V" => PrimaryWaypointType::VFRWaypoint,
            b"W" => PrimaryWaypointType::RNAVWaypoint,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Primary Waypoint Type".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SecondaryWaypointType {
    FinalApproachFix,
    InitialAndFinalApproachFix,
    FinalApproachCourseFix,
    IntermediateApproachFix,
    OffRouteWaypointIntersectionDMEFix,
    InitialDepartureFix,
    HelicopterOnlyAirwayFix,
    InitialApproachFix,
    RequiredOffRouteWaypoint,
    InitialAndFinalApproachCourseFix,
    IntermediateAndFinalApproachCourseFix,
    MissedApproachFix,
    InitialAndMissedApproachFix,
    OceanicGatewayFix,
    UnnamedStepdownFix,
    RFLegFixNotAtProcedureFix,
    NamedStepdownFix,
    FIRUIRControlledAirspaceFix,
    FullDegreeLatLongFix,
    HalfDegreeLatLongFix,
}
impl SecondaryWaypointType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"A" => SecondaryWaypointType::FinalApproachFix,
            b"B" => SecondaryWaypointType::InitialAndFinalApproachFix,
            b"C" => SecondaryWaypointType::FinalApproachCourseFix,
            b"D" => SecondaryWaypointType::IntermediateApproachFix,
            b"F" => SecondaryWaypointType::OffRouteWaypointIntersectionDMEFix,
            b"G" => SecondaryWaypointType::InitialDepartureFix,
            b"H" => SecondaryWaypointType::HelicopterOnlyAirwayFix,
            b"I" => SecondaryWaypointType::InitialApproachFix,
            b"J" => SecondaryWaypointType::RequiredOffRouteWaypoint,
            b"K" => SecondaryWaypointType::InitialAndFinalApproachCourseFix,
            b"L" => SecondaryWaypointType::IntermediateAndFinalApproachCourseFix,
            b"M" => SecondaryWaypointType::MissedApproachFix,
            b"N" => SecondaryWaypointType::InitialAndMissedApproachFix,
            b"O" => SecondaryWaypointType::OceanicGatewayFix,
            b"P" => SecondaryWaypointType::UnnamedStepdownFix,
            b"R" => SecondaryWaypointType::RFLegFixNotAtProcedureFix,
            b"S" => SecondaryWaypointType::NamedStepdownFix,
            b"U" => SecondaryWaypointType::FIRUIRControlledAirspaceFix,
            b"V" => SecondaryWaypointType::FullDegreeLatLongFix,
            b"W" => SecondaryWaypointType::HalfDegreeLatLongFix,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Secondary Waypoint Type".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PublishedUse {
    PublishedForUseInSID,
    PublishedForUseInSTAR,
    PublishedForUseInApproachProcedure,
    PublishedForUseInMultipleTerminalProcedures,
}

impl PublishedUse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"D" => PublishedUse::PublishedForUseInSID,
            b"E" => PublishedUse::PublishedForUseInSTAR,
            b"F" => PublishedUse::PublishedForUseInApproachProcedure,
            b"Z" => PublishedUse::PublishedForUseInMultipleTerminalProcedures,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Published Use".to_string(),
                });
            }
        }))
    }
}

/// 5.42 Waypoint Type (TYPE)
#[derive(Debug, PartialEq, Eq)]
pub struct WaypointType(PrimaryWaypointType, SecondaryWaypointType, PublishedUse);
impl WaypointType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(WaypointType(
            PrimaryWaypointType::from_bytes(&bytes[0..1])?.ok_or(FieldParseError {
                message: "Invalid Primary Waypoint Type".to_string(),
            })?,
            SecondaryWaypointType::from_bytes(&bytes[1..2])?.ok_or(FieldParseError {
                message: "Invalid Secondary Waypoint Type".to_string(),
            })?,
            PublishedUse::from_bytes(&bytes[2..3])?.ok_or(FieldParseError {
                message: "Invalid Published Use".to_string(),
            })?,
        )))
    }
}

// 5.93 Facility Characteristics (FAC CHAR)
// Note: This will be a best guess at layout since the table is unclear in its capture of the logic required

// VHF Navaid/TACAN Only record class

// NDB Navaid Class

// Localizer Marker / Localizer Glideslope Class

// MLS Class
