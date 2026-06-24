//! # ARINC 424 Definitions - Complexes
//! This module contains the complex types for the ARINC 424 data. This is not a formally defined thing, but
//! rather a term I have used to describe the encoding of multiple annotations onto one field definition in the spec
//! The first example of which is given in 5.35 - Navaid Class which describes a 5-letter code that adds annotations onto a Navaid record.
//!
//! The way we encode these complexes are as tuples or nested enums. In the above example, the Navaid Class is encoded as a tuple of 5 enum fields. Thus,
//! when parsing a VHF Navaid record in particular we represent this as the combination of:
//!
//! - VHFNavaidType1
//! - VHFNavaidType2
//! - VHFRangePower
//! - VHFAdditionalInfo
//! - Collocation
//!
use super::numerics::{AltitudeNumeric, UintNumeric};
use crate::parsers::arinc424::{
    definitions::MultiUnitAltitudeNumeric,
    fields::{BLANK, FieldParseError},
};

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

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsSynchronicity {
    Synchronous,
    Asynchronous,
    Unknown,
}
impl FacilityCharacteristicsSynchronicity {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"S" => FacilityCharacteristicsSynchronicity::Synchronous,
            b"A" => FacilityCharacteristicsSynchronicity::Asynchronous,
            b"U" => FacilityCharacteristicsSynchronicity::Unknown,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid VHFNavaid Facility Characteristics 1".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsVoiceIdent {
    VoiceIdent,
    NoVoiceIdent,
    Undefined,
    NotApplicable,
}

impl FacilityCharacteristicsVoiceIdent {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => FacilityCharacteristicsVoiceIdent::VoiceIdent,
            b"N" => FacilityCharacteristicsVoiceIdent::NoVoiceIdent,
            b"U" => FacilityCharacteristicsVoiceIdent::Undefined,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid VHFNavaid Facility Characteristics 2".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsEmissionType {
    UnmodulatedCarrier,
    CarrierKeyed,
    ToneKeyed,
    NotApplicable,
}

impl FacilityCharacteristicsEmissionType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(FacilityCharacteristicsEmissionType::NotApplicable));
        }
        Ok(Some(match bytes {
            b"0" => FacilityCharacteristicsEmissionType::UnmodulatedCarrier,
            b"1" => FacilityCharacteristicsEmissionType::CarrierKeyed,
            b"2" => FacilityCharacteristicsEmissionType::ToneKeyed,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Facility Characteristics Emission Type".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsFreqAndBackcourse {
    Freq400Hz,
    Freq1020Hz,
    BackcourseUsable,
    BackcourseUnusable,
    BackcourseRestricted,
    BackcourseUndefined,
    MLSHighRateAzimuthGuidance,
    NotApplicable,
}
impl FacilityCharacteristicsFreqAndBackcourse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(
                FacilityCharacteristicsFreqAndBackcourse::NotApplicable,
            ));
        }
        Ok(Some(match bytes {
            b"4" => FacilityCharacteristicsFreqAndBackcourse::Freq400Hz,
            b"1" => FacilityCharacteristicsFreqAndBackcourse::Freq1020Hz,
            b"Y" => FacilityCharacteristicsFreqAndBackcourse::BackcourseUsable,
            b"N" => FacilityCharacteristicsFreqAndBackcourse::BackcourseUnusable,
            b"R" => FacilityCharacteristicsFreqAndBackcourse::BackcourseRestricted,
            b"U" => FacilityCharacteristicsFreqAndBackcourse::BackcourseUndefined,
            b"H" => FacilityCharacteristicsFreqAndBackcourse::MLSHighRateAzimuthGuidance,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Facility Characteristics Freq And Backcourse".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FacilityCharacteristicsRepetitionAndCollocation {
    CollocatedWithLocalizer,
    CollocatedWithGlideslope,
    CollocatedWithAzimuth,
    CollocatedWithElevation,
    NotCollocatedWithAzOrElev,
    NotApplicable,
    KnownRepetition(u8),
}
impl FacilityCharacteristicsRepetitionAndCollocation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(
                FacilityCharacteristicsRepetitionAndCollocation::NotApplicable,
            ));
        }
        Ok(Some(match bytes {
            b"0" => FacilityCharacteristicsRepetitionAndCollocation::CollocatedWithLocalizer,
            b"1" => FacilityCharacteristicsRepetitionAndCollocation::CollocatedWithGlideslope,
            b"2" => FacilityCharacteristicsRepetitionAndCollocation::CollocatedWithAzimuth,
            b"3" => FacilityCharacteristicsRepetitionAndCollocation::CollocatedWithElevation,
            b"4" => FacilityCharacteristicsRepetitionAndCollocation::NotCollocatedWithAzOrElev,
            _ => {
                if bytes[0].is_ascii_digit() {
                    return Ok(Some(
                        FacilityCharacteristicsRepetitionAndCollocation::KnownRepetition(
                            bytes[0] - b'0',
                        ),
                    ));
                } else {
                    return Err(FieldParseError {
                        message: "Invalid Facility Characteristics Repetition And Collocation"
                            .to_string(),
                    });
                }
            }
        }))
    }
}

/// 5.93 Facility Characteristics (FAC CHAR)
#[derive(Debug, PartialEq, Eq)]
pub struct FacilityCharacteristics(
    FacilityCharacteristicsSynchronicity,
    FacilityCharacteristicsVoiceIdent,
    FacilityCharacteristicsEmissionType,
    FacilityCharacteristicsFreqAndBackcourse,
    FacilityCharacteristicsRepetitionAndCollocation,
);
impl FacilityCharacteristics {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(FacilityCharacteristics(
            FacilityCharacteristicsSynchronicity::from_bytes(&bytes[0..1])?.ok_or(
                FieldParseError {
                    message: "Invalid Facility Characteristics Synchronicity".to_string(),
                },
            )?,
            FacilityCharacteristicsVoiceIdent::from_bytes(&bytes[1..2])?.ok_or(
                FieldParseError {
                    message: "Invalid Facility Characteristics Voice Ident".to_string(),
                },
            )?,
            FacilityCharacteristicsEmissionType::from_bytes(&bytes[2..3])?.ok_or(
                FieldParseError {
                    message: "Invalid Facility Characteristics Emission Type".to_string(),
                },
            )?,
            FacilityCharacteristicsFreqAndBackcourse::from_bytes(&bytes[3..4])?.ok_or(
                FieldParseError {
                    message: "Invalid Facility Characteristics Freq And Backcourse".to_string(),
                },
            )?,
            FacilityCharacteristicsRepetitionAndCollocation::from_bytes(&bytes[4..5])?.ok_or(
                FieldParseError {
                    message: "Invalid Facility Characteristics Repetition And Collocation"
                        .to_string(),
                },
            )?,
        )))
    }
}

#[test]
fn test_facility_characteristics() {
    let bytes = b"SY049";
    let facility_characteristics = FacilityCharacteristics::from_bytes(bytes).unwrap().unwrap();
    assert_eq!(
        facility_characteristics.0,
        FacilityCharacteristicsSynchronicity::Synchronous
    );
    assert_eq!(
        facility_characteristics.1,
        FacilityCharacteristicsVoiceIdent::VoiceIdent
    );
    assert_eq!(
        facility_characteristics.2,
        FacilityCharacteristicsEmissionType::UnmodulatedCarrier
    );
    assert_eq!(
        facility_characteristics.3,
        FacilityCharacteristicsFreqAndBackcourse::Freq400Hz
    );
    assert_eq!(
        facility_characteristics.4,
        FacilityCharacteristicsRepetitionAndCollocation::KnownRepetition(9)
    );
}

// Marker Type

#[derive(Debug, PartialEq, Eq)]
pub enum MarkerTypeLocator {
    Locator,
    NotApplicable,
}
impl MarkerTypeLocator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(MarkerTypeLocator::NotApplicable));
        }
        Ok(Some(match bytes {
            b"L" => MarkerTypeLocator::Locator,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Marker Type Locator".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarkerTypeLocation {
    Inner,
    Middle,
    Outer,
    Back,
    NotApplicable,
}
impl MarkerTypeLocation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(MarkerTypeLocation::NotApplicable));
        }
        Ok(Some(match bytes {
            b"I" => MarkerTypeLocation::Inner,
            b"M" => MarkerTypeLocation::Middle,
            b"O" => MarkerTypeLocation::Outer,
            b"B" => MarkerTypeLocation::Back,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Marker Type Location".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MarkerTypeMarker {
    Marker,
    NotApplicable,
}
impl MarkerTypeMarker {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes[0] == BLANK {
            return Ok(Some(MarkerTypeMarker::NotApplicable));
        }
        Ok(Some(match bytes {
            b"M" => MarkerTypeMarker::Marker,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Marker Type Marker".to_string(),
                });
            }
        }))
    }
}

/// 5.99 Marker Type
#[derive(Debug, PartialEq, Eq)]
pub struct MarkerType(MarkerTypeLocator, MarkerTypeLocation, MarkerTypeMarker);
impl MarkerType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(MarkerType(
            MarkerTypeLocator::from_bytes(&bytes[0..1])?.ok_or(FieldParseError {
                message: "Invalid Marker Type Locator".to_string(),
            })?,
            MarkerTypeLocation::from_bytes(&bytes[1..2])?.ok_or(FieldParseError {
                message: "Invalid Marker Type Location".to_string(),
            })?,
            MarkerTypeMarker::from_bytes(&bytes[2..3])?.ok_or(FieldParseError {
                message: "Invalid Marker Type Marker".to_string(),
            })?,
        )))
    }
}

// Service Indicator

// Airport Heliport Communications Records
#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportCommunicationsServiceIndicator1 {
    AirportAdvisoryService,
    CommunityAerodromeRadioStation,
    DepartureServiceOtherThanDepartureControl,
    FlightInformationService,
    InitialContact,
    ArrivalServiceOtherThanArrivalControl,
    AerodromeFlightInformationService,
    TerminalAreaControlOtherThanDedicatedTerminalControl,
    NotApplicable,
}

impl AirportHeliportCommunicationsServiceIndicator1 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => AirportHeliportCommunicationsServiceIndicator1::NotApplicable,
            b"A" => AirportHeliportCommunicationsServiceIndicator1::AirportAdvisoryService,
            b"C" => AirportHeliportCommunicationsServiceIndicator1::CommunityAerodromeRadioStation,
            b"D" => AirportHeliportCommunicationsServiceIndicator1::DepartureServiceOtherThanDepartureControl,
            b"F" => AirportHeliportCommunicationsServiceIndicator1::FlightInformationService,
            b"I" => AirportHeliportCommunicationsServiceIndicator1::InitialContact,
            b"L" => AirportHeliportCommunicationsServiceIndicator1::ArrivalServiceOtherThanArrivalControl,
            b"S" => AirportHeliportCommunicationsServiceIndicator1::AerodromeFlightInformationService,
            b"T" => AirportHeliportCommunicationsServiceIndicator1::TerminalAreaControlOtherThanDedicatedTerminalControl,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Airport Heliport Communications Service Indicator 1".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportCommunicationsServiceIndicator2 {
    AerodromeTrafficFrequency,
    CommonTrafficAdvisoryFrequency,
    MandatoryFrequency,
    SecondaryFrequency,
    NotApplicable,
}
impl AirportHeliportCommunicationsServiceIndicator2 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => AirportHeliportCommunicationsServiceIndicator2::NotApplicable,
            b"A" => AirportHeliportCommunicationsServiceIndicator2::AerodromeTrafficFrequency,
            b"C" => AirportHeliportCommunicationsServiceIndicator2::CommonTrafficAdvisoryFrequency,
            b"M" => AirportHeliportCommunicationsServiceIndicator2::MandatoryFrequency,
            b"S" => AirportHeliportCommunicationsServiceIndicator2::SecondaryFrequency,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Airport Heliport Communications Service Indicator 2"
                        .to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AirportHeliportCommunicationsServiceIndicator3 {
    VHFDirectionFindingService,
    LanguageOtherThanEnglish,
    MilitaryUseFrequency,
    PilotControlledLight,
    NotApplicable,
}
impl AirportHeliportCommunicationsServiceIndicator3 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => AirportHeliportCommunicationsServiceIndicator3::NotApplicable,
            b"D" => AirportHeliportCommunicationsServiceIndicator3::VHFDirectionFindingService,
            b"L" => AirportHeliportCommunicationsServiceIndicator3::LanguageOtherThanEnglish,
            b"M" => AirportHeliportCommunicationsServiceIndicator3::MilitaryUseFrequency,
            b"P" => AirportHeliportCommunicationsServiceIndicator3::PilotControlledLight,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Airport Heliport Communications Service Indicator 3"
                        .to_string(),
                });
            }
        }))
    }
}

/// 5.106(A) Airport Heliport Communications Service Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct AirportHeliportCommunicationsServiceIndicator(
    AirportHeliportCommunicationsServiceIndicator1,
    AirportHeliportCommunicationsServiceIndicator2,
    AirportHeliportCommunicationsServiceIndicator3,
);
impl AirportHeliportCommunicationsServiceIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(AirportHeliportCommunicationsServiceIndicator(
            AirportHeliportCommunicationsServiceIndicator1::from_bytes(&bytes[0..1])?.ok_or(
                FieldParseError {
                    message: "Invalid Airport Heliport Communications Service Indicator 1"
                        .to_string(),
                },
            )?,
            AirportHeliportCommunicationsServiceIndicator2::from_bytes(&bytes[1..2])?.ok_or(
                FieldParseError {
                    message: "Invalid Airport Heliport Communications Service Indicator 2"
                        .to_string(),
                },
            )?,
            AirportHeliportCommunicationsServiceIndicator3::from_bytes(&bytes[2..3])?.ok_or(
                FieldParseError {
                    message: "Invalid Airport Heliport Communications Service Indicator 3"
                        .to_string(),
                },
            )?,
        )))
    }
}

// Enroute Communications Records
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteCommunicationsServiceIndicator1 {
    AeronauticalEnrouteInformationService,
    FlightInformationService,
    NotApplicable,
}
impl EnrouteCommunicationsServiceIndicator1 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => EnrouteCommunicationsServiceIndicator1::NotApplicable,
            b"A" => EnrouteCommunicationsServiceIndicator1::AeronauticalEnrouteInformationService,
            b"F" => EnrouteCommunicationsServiceIndicator1::FlightInformationService,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Enroute Communications Service Indicator 1".to_string(),
                });
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteCommunicationsServiceIndicator2 {
    AirGround,
    DiscreteFrequency,
    MandatoryFrequency,
    SecondaryFrequency,
    NotApplicable,
}
impl EnrouteCommunicationsServiceIndicator2 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => EnrouteCommunicationsServiceIndicator2::NotApplicable,
            b"A" => EnrouteCommunicationsServiceIndicator2::AirGround,
            b"D" => EnrouteCommunicationsServiceIndicator2::DiscreteFrequency,
            b"M" => EnrouteCommunicationsServiceIndicator2::MandatoryFrequency,
            b"S" => EnrouteCommunicationsServiceIndicator2::SecondaryFrequency,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Enroute Communications Service Indicator 2".to_string(),
                });
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum EnrouteCommunicationsServiceIndicator3 {
    VHFDirectionFindingService,
    LanguageOtherThanEnglish,
    MilitaryUseFrequency,
    NotApplicable,
}
impl EnrouteCommunicationsServiceIndicator3 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            [BLANK] => EnrouteCommunicationsServiceIndicator3::NotApplicable,
            b"D" => EnrouteCommunicationsServiceIndicator3::VHFDirectionFindingService,
            b"L" => EnrouteCommunicationsServiceIndicator3::LanguageOtherThanEnglish,
            b"M" => EnrouteCommunicationsServiceIndicator3::MilitaryUseFrequency,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid Enroute Communications Service Indicator 3".to_string(),
                });
            }
        }))
    }
}

/// 5.106(B) Enroute Communications Service Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct EnrouteCommunicationsServiceIndicator(
    EnrouteCommunicationsServiceIndicator1,
    EnrouteCommunicationsServiceIndicator2,
    EnrouteCommunicationsServiceIndicator3,
);
impl EnrouteCommunicationsServiceIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(EnrouteCommunicationsServiceIndicator(
            EnrouteCommunicationsServiceIndicator1::from_bytes(&bytes[0..1])?.ok_or(
                FieldParseError {
                    message: "Invalid Enroute Communications Service Indicator 1".to_string(),
                },
            )?,
            EnrouteCommunicationsServiceIndicator2::from_bytes(&bytes[2..3])?.ok_or(
                FieldParseError {
                    message: "Invalid Enroute Communications Service Indicator 2".to_string(),
                },
            )?,
            EnrouteCommunicationsServiceIndicator3::from_bytes(&bytes[3..4])?.ok_or(
                FieldParseError {
                    message: "Invalid Enroute Communications Service Indicator 3".to_string(),
                },
            )?,
        )))
    }
}

// Duplicate Indicator

#[derive(Debug, PartialEq, Eq)]
pub enum DuplicateIndicatorAirspace {
    Undefined,
    HighAltitude,
    LowAltitude,
    SID,
    STAR,
    Approach,
    MissedApproach,
    UndefinedWithOther,
    AllAltitude,
}

impl DuplicateIndicatorAirspace {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"0" => DuplicateIndicatorAirspace::Undefined,
            b"1" => DuplicateIndicatorAirspace::HighAltitude,
            b"2" => DuplicateIndicatorAirspace::LowAltitude,
            b"3" => DuplicateIndicatorAirspace::SID,
            b"4" => DuplicateIndicatorAirspace::STAR,
            b"5" => DuplicateIndicatorAirspace::Approach,
            b"6" => DuplicateIndicatorAirspace::MissedApproach,
            b"7" => DuplicateIndicatorAirspace::UndefinedWithOther,
            b"8" => DuplicateIndicatorAirspace::AllAltitude,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid duplicate indicator airspace".to_string(),
                });
            }
        }))
    }
}

pub type DuplicateIndicatorDuplicates = u8;

/// 5.114 Duplicate Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct DuplicateIndicator(DuplicateIndicatorAirspace, DuplicateIndicatorDuplicates);
impl DuplicateIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(DuplicateIndicator(
            DuplicateIndicatorAirspace::from_bytes(&bytes[0..1])?.ok_or(FieldParseError {
                message: "Invalid duplicate indicator airspace".to_string(),
            })?,
            bytes[1] - b'0' as u8,
        )))
    }
}

// Boundary Via

#[derive(Debug, PartialEq, Eq)]
pub enum BoundaryViaPathType {
    Circle,
    GreatCircle,
    RhumbLine,
    CounterclockwiseArc,
    ClockwiseArc,
}
impl BoundaryViaPathType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"C" => BoundaryViaPathType::Circle,
            b"G" => BoundaryViaPathType::GreatCircle,
            b"H" => BoundaryViaPathType::RhumbLine,
            b"L" => BoundaryViaPathType::CounterclockwiseArc,
            b"R" => BoundaryViaPathType::ClockwiseArc,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid boundary via path type".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoundaryViaEndPoint {
    EndOfDescription,
    Other,
}
impl BoundaryViaEndPoint {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"E" => BoundaryViaEndPoint::EndOfDescription,
            [BLANK] => BoundaryViaEndPoint::Other,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid boundary via end point".to_string(),
                });
            }
        }))
    }
}
/// 5.118 Boundary Via
#[derive(Debug, PartialEq, Eq)]
pub struct BoundaryVia(BoundaryViaPathType, BoundaryViaEndPoint);
impl BoundaryVia {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(BoundaryVia(
            BoundaryViaPathType::from_bytes(&bytes[0..1])?.ok_or(FieldParseError {
                message: "Invalid boundary via path type".to_string(),
            })?,
            BoundaryViaEndPoint::from_bytes(&bytes[1..2])?.ok_or(FieldParseError {
                message: "Invalid boundary via end point value".to_string(),
            })?,
        )))
    }
}

// Lower/Upper Limit

#[derive(Debug, PartialEq, Eq)]
pub enum DescriptiveAltitudeLimits {
    NotSpecified,
    Unlimited,
    Ground,
    MeanSeaLevel,
    SeeNOTAM,
}

impl DescriptiveAltitudeLimits {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"NOTSP" => DescriptiveAltitudeLimits::NotSpecified,
            b"UNLTD" => DescriptiveAltitudeLimits::Unlimited,
            b"GND  " => DescriptiveAltitudeLimits::Ground,
            b"MSL  " => DescriptiveAltitudeLimits::MeanSeaLevel,
            b"NOTAM" => DescriptiveAltitudeLimits::SeeNOTAM,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid descriptive altitude limits".to_string(),
                });
            }
        }))
    }
}

/// 5.121 Lower/Upper Limit
#[derive(Debug, PartialEq, Eq)]
pub enum LowerUpperLimit {
    Descriptive(DescriptiveAltitudeLimits),
    Numeric(AltitudeNumeric),
}
impl LowerUpperLimit {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        // try to parse as descriptive altitude limit first, on error handle as numeric altitude
        let descriptive = DescriptiveAltitudeLimits::from_bytes(bytes);
        if let Ok(Some(descriptive)) = descriptive {
            return Ok(Some(LowerUpperLimit::Descriptive(descriptive)));
        }
        match AltitudeNumeric::from_bytes(bytes) {
            Ok(Some(numeric)) => Ok(Some(LowerUpperLimit::Numeric(numeric))),
            Ok(None) => Ok(None),
            _ => {
                return Err(FieldParseError {
                    message: "Invalid lower/upper limit".to_string(),
                });
            }
        }
    }
}

#[test]
pub fn test_lower_upper_limit() {
    let r = LowerUpperLimit::from_bytes(b"10000");
    if let Ok(Some(LowerUpperLimit::Numeric(numeric))) = r {
        let val: i32 = numeric.into();
        assert_eq!(val, 10000);
    } else {
        panic!("Failed to parse lower/upper limit");
    }
    let r = LowerUpperLimit::from_bytes(b"NOTSP");
    if let Ok(Some(LowerUpperLimit::Descriptive(descriptive))) = r {
        assert_eq!(descriptive, DescriptiveAltitudeLimits::NotSpecified);
    } else {
        panic!("Failed to parse lower/upper limit");
    }
    let r = LowerUpperLimit::from_bytes(b"FAILS");
    if let Ok(_) = r {
        panic!("Should have failed to parse lower/upper limit");
    }
    let r = LowerUpperLimit::from_bytes(b"     ");
    if let Ok(optional) = r {
        if let Some(_) = optional {
            panic!("Should have parsed None for blank lower/upper limit");
        }
    }
}

/// 5.127 Maximum Altitude
#[derive(Debug, PartialEq, Eq)]
pub enum MaximumAltitude {
    Descriptive(DescriptiveAltitudeLimits),
    Numeric(AltitudeNumeric),
}
impl MaximumAltitude {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let descriptive = DescriptiveAltitudeLimits::from_bytes(bytes);
        if let Ok(Some(descriptive)) = descriptive {
            // we only allow unlimited as descriptive altitude limit
            match descriptive {
                DescriptiveAltitudeLimits::Unlimited => {
                    return Ok(Some(MaximumAltitude::Descriptive(descriptive)));
                }
                _ => {
                    return Err(FieldParseError {
                        message: "Invalid maximum altitude".to_string(),
                    });
                }
            }
        }
        match AltitudeNumeric::from_bytes(bytes) {
            Ok(Some(numeric)) => Ok(Some(MaximumAltitude::Numeric(numeric))),
            Ok(None) => Ok(None),
            _ => {
                return Err(FieldParseError {
                    message: "Invalid lower/upper limit".to_string(),
                });
            }
        }
    }
}

/// 5.136 Cruise Level From/To
#[derive(Debug, PartialEq, Eq)]
pub enum CruiseLevelFromTo {
    Descriptive(DescriptiveAltitudeLimits),
    Numeric(MultiUnitAltitudeNumeric),
}
impl CruiseLevelFromTo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let descriptive = DescriptiveAltitudeLimits::from_bytes(bytes);
        if let Ok(Some(descriptive)) = descriptive {
            // we only allow unlimited as descriptive altitude limit
            match descriptive {
                DescriptiveAltitudeLimits::Unlimited => {
                    return Ok(Some(CruiseLevelFromTo::Descriptive(descriptive)));
                }
                _ => {
                    return Err(FieldParseError {
                        message: "Invalid maximum altitude".to_string(),
                    });
                }
            }
        }
        match MultiUnitAltitudeNumeric::from_bytes(bytes) {
            Ok(Some(numeric)) => Ok(Some(CruiseLevelFromTo::Numeric(numeric))),
            Ok(None) => Ok(None),
            _ => {
                return Err(FieldParseError {
                    message: "Invalid cruise level from to".to_string(),
                });
            }
        }
    }
}

#[test]
pub fn test_cruise_level_from_to() {
    let r = CruiseLevelFromTo::from_bytes(b"M1000");
    if let Ok(Some(CruiseLevelFromTo::Numeric(MultiUnitAltitudeNumeric::Meters(altitude)))) = r {
        let val: i32 = altitude.into();
        assert_eq!(val, 1000);
    } else {
        panic!("Failed to parse cruise level from to");
    }
    let r = CruiseLevelFromTo::from_bytes(b"10000");
    if let Ok(Some(CruiseLevelFromTo::Numeric(MultiUnitAltitudeNumeric::Feet(altitude)))) = r {
        let val: i32 = altitude.into();
        assert_eq!(val, 10000);
    } else {
        panic!("Failed to parse cruise level from to");
    }
}

/// 5.146 MSA Sector Bearing
#[derive(Debug, PartialEq, Eq)]
pub struct SectorBearing {
    pub start_bearing: UintNumeric,
    pub end_bearing: UintNumeric,
}
impl SectorBearing {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let start_bearing = UintNumeric::from_bytes(&bytes[0..3])?.ok_or(FieldParseError {
            message: "Invalid sector bearing".to_string(),
        })?;
        let end_bearing = UintNumeric::from_bytes(&bytes[3..6])?.ok_or(FieldParseError {
            message: "Invalid sector bearing".to_string(),
        })?;
        Ok(Some(SectorBearing {
            start_bearing,
            end_bearing,
        }))
    }
}

// Pad Dimensions
#[derive(Debug, PartialEq, Eq)]
pub struct RectangularPadDimensions {
    pub width: UintNumeric,
    pub length: UintNumeric,
}
impl RectangularPadDimensions {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let width = UintNumeric::from_bytes(&bytes[0..5])?.ok_or(FieldParseError {
            message: "Invalid rectangular pad dimensions".to_string(),
        })?;
        let length = UintNumeric::from_bytes(&bytes[5..8])?.ok_or(FieldParseError {
            message: "Invalid rectangular pad dimensions".to_string(),
        })?;
        Ok(Some(RectangularPadDimensions { width, length }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CircularPadDimensions {
    pub diameter: UintNumeric,
}
impl CircularPadDimensions {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let diameter = UintNumeric::from_bytes(&bytes[0..5])?.ok_or(FieldParseError {
            message: "Invalid circular pad dimensions".to_string(),
        })?;
        Ok(Some(CircularPadDimensions { diameter }))
    }
}

/// 5.176 Pad Dimensions
///
/// Note: This is a complex type that can be either rectangular or circular. Construction is manual
/// based on the indicator that tells us which it is
#[derive(Debug, PartialEq, Eq)]
pub enum PadDimensions {
    Rectangular(RectangularPadDimensions),
    Circular(CircularPadDimensions),
}

// Timezone (ugh)

#[derive(Debug, PartialEq, Eq)]
pub enum TimezoneZone {
    UTCMinus1,
    UTCMinus2,
    UTCMinus3,
    UTCMinus4,
    UTCMinus5,
    UTCMinus6,
    UTCMinus7,
    UTCMinus8,
    UTCMinus9,
    UTCMinus10,
    UTCMinus11,
    UTCMinus12,
    UTCMinus13,
    UTCMinus14,
    UTC,
    UTCPlus1,
    UTCPlus2,
    UTCPlus3,
    UTCPlus4,
    UTCPlus5,
    UTCPlus6,
    UTCPlus7,
    UTCPlus8,
    UTCPlus9,
    UTCPlus10,
    UTCPlus11,
    UTCPlus12,
}

impl TimezoneZone {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => TimezoneZone::UTCMinus1,
            b"B" => TimezoneZone::UTCMinus2,
            b"C" => TimezoneZone::UTCMinus3,
            b"D" => TimezoneZone::UTCMinus4,
            b"E" => TimezoneZone::UTCMinus5,
            b"F" => TimezoneZone::UTCMinus6,
            b"G" => TimezoneZone::UTCMinus7,
            b"H" => TimezoneZone::UTCMinus8,
            b"I" => TimezoneZone::UTCMinus9,
            b"K" => TimezoneZone::UTCMinus10,
            b"L" => TimezoneZone::UTCMinus11,
            b"M" => TimezoneZone::UTCMinus12,
            b"1" => TimezoneZone::UTCMinus13,
            b"2" => TimezoneZone::UTCMinus14,
            b"Z" => TimezoneZone::UTC,
            b"N" => TimezoneZone::UTCPlus1,
            b"O" => TimezoneZone::UTCPlus2,
            b"P" => TimezoneZone::UTCPlus3,
            b"Q" => TimezoneZone::UTCPlus4,
            b"R" => TimezoneZone::UTCPlus5,
            b"S" => TimezoneZone::UTCPlus6,
            b"T" => TimezoneZone::UTCPlus7,
            b"U" => TimezoneZone::UTCPlus8,
            b"V" => TimezoneZone::UTCPlus9,
            b"W" => TimezoneZone::UTCPlus10,
            b"X" => TimezoneZone::UTCPlus11,
            b"Y" => TimezoneZone::UTCPlus12,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid timezone zone".to_string(),
                });
            }
        }))
    }
}

/// 5.178 Timezone
#[derive(Debug, PartialEq, Eq)]
pub struct Timezone {
    pub zone: TimezoneZone,
    pub minutes_offset: Option<UintNumeric>,
}
impl Timezone {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let zone = TimezoneZone::from_bytes(&bytes[0..1])?;
        let minutes_offset = UintNumeric::from_bytes(&bytes[1..3])?;
        if let Some(zone) = zone {
            Ok(Some(Timezone {
                zone: zone,
                minutes_offset: minutes_offset,
            }))
        } else {
            Ok(None)
        }
    }
}

#[test]
pub fn test_timezone() {
    let r = Timezone::from_bytes(b"A30");
    if let Ok(Some(Timezone {
        zone,
        minutes_offset: Some(minutes_offset),
    })) = r
    {
        let minutes_offset: u64 = minutes_offset.into();
        assert_eq!(zone, TimezoneZone::UTCMinus1);
        assert_eq!(minutes_offset, 30);
    } else {
        panic!("Failed to parse timezone");
    }
    let r = Timezone::from_bytes(b"A  ");
    if let Ok(Some(Timezone {
        zone,
        minutes_offset,
    })) = r
    {
        assert_eq!(zone, TimezoneZone::UTCMinus1);
        assert!(minutes_offset.is_none());
    } else {
        panic!("Failed to parse timezone");
    }
}

/// 5.183 Comms Sectorization
pub type CommunicationsSectorization = SectorBearing;

// Name Format Indicator

#[derive(Debug, PartialEq, Eq)]
pub enum NameFormatType1 {
    AbeamFix,
    BearingDistanceFix,
    AirportNameAsFix,
    FIRFix,
    PhoneticLetterNameFix,
    AirportIdentAsFix,
    LatLongFix,
    MultipleWordNameFix,
    NavadIdentAsFix,
    PublishedFiveLetterNameFix,
    PublishedLessThanFiveLetterNameFix,
    PublishedMoreThanFiveLetterNameFix,
    AirportRunwayRelatedFix,
    UIRFix,
    VFRCheckpointReportingPointAsFix,
}

impl NameFormatType1 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => NameFormatType1::AbeamFix,
            b"B" => NameFormatType1::BearingDistanceFix,
            b"D" => NameFormatType1::AirportNameAsFix,
            b"F" => NameFormatType1::FIRFix,
            b"H" => NameFormatType1::PhoneticLetterNameFix,
            b"I" => NameFormatType1::AirportIdentAsFix,
            b"L" => NameFormatType1::LatLongFix,
            b"M" => NameFormatType1::MultipleWordNameFix,
            b"N" => NameFormatType1::NavadIdentAsFix,
            b"P" => NameFormatType1::PublishedFiveLetterNameFix,
            b"Q" => NameFormatType1::PublishedLessThanFiveLetterNameFix,
            b"R" => NameFormatType1::PublishedMoreThanFiveLetterNameFix,
            b"T" => NameFormatType1::AirportRunwayRelatedFix,
            b"U" => NameFormatType1::UIRFix,
            b"V" => NameFormatType1::VFRCheckpointReportingPointAsFix,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid name format indicator".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NameFormatType2 {
    LocalizerMarkerWithPublishedIdentifier,
    LocalizerMarkerWithoutPublishedIdentifier,
    NotApplicable,
}
impl NameFormatType2 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(Some(NameFormatType2::NotApplicable));
        }
        Ok(Some(match bytes {
            b"O" => NameFormatType2::LocalizerMarkerWithPublishedIdentifier,
            b"M" => NameFormatType2::LocalizerMarkerWithoutPublishedIdentifier,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid name format indicator".to_string(),
                });
            }
        }))
    }
}

/// 5.196 Name Format Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct NameFormat(NameFormatType1, NameFormatType2);
impl NameFormat {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let type1 = NameFormatType1::from_bytes(&bytes[0..1])?.ok_or(FieldParseError {
            message: "Invalid name format indicator".to_string(),
        })?;
        let type2 = NameFormatType2::from_bytes(&bytes[1..2])?.ok_or(FieldParseError {
            message: "Invalid name format indicator".to_string(),
        })?;
        Ok(Some(NameFormat(type1, type2)))
    }
}

/// 5.207 Sector From / Sector To
#[derive(Debug, PartialEq, Eq)]
pub struct SectorFromTo {
    pub from: Box<str>,
    pub to: Box<str>,
}
impl SectorFromTo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        if !(matches!(bytes[0], b'A'..=b'X') && matches!(bytes[1], b'A'..=b'X')) {
            return Err(FieldParseError {
                message: "Invalid sector in 'from' or 'to' position".to_string(),
            });
        }
        let from_str =
            Box::from(
                std::str::from_utf8(&bytes[0..1]).map_err(|e| FieldParseError {
                    message: format!("Could not convert 'from' sector to string: {}", e),
                })?,
            );

        let to_str = Box::from(
            std::str::from_utf8(&bytes[1..2]).map_err(|e| FieldParseError {
                message: format!("Could not convert 'to' sector to string: {}", e),
            })?,
        );
        Ok(Some(SectorFromTo {
            from: from_str,
            to: to_str,
        }))
    }
}

/// 5.208 Navaid Distance Limitation
#[derive(Debug, PartialEq, Eq)]
pub struct NavaidDistanceLimitation {
    pub first_limit: UintNumeric,
    pub second_limit: UintNumeric,
}

impl NavaidDistanceLimitation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let first_limit = UintNumeric::from_bytes(&bytes[0..3])?;
        let second_limit = UintNumeric::from_bytes(&bytes[3..6])?;
        if let Some(first_limit) = first_limit
            && let Some(second_limit) = second_limit
        {
            Ok(Some(NavaidDistanceLimitation {
                first_limit: first_limit,
                second_limit: second_limit,
            }))
        } else {
            return Err(FieldParseError {
                message: "Invalid first or second limit".to_string(),
            });
        }
    }
}

/// 5.209 Navaid Altitude Limitation
#[derive(Debug, PartialEq, Eq)]
pub struct NavaidAltitudeLimitation {
    pub first_limit: UintNumeric,
    pub second_limit: UintNumeric,
}

impl NavaidAltitudeLimitation {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let first_limit = UintNumeric::from_bytes(&bytes[0..3])?;
        let second_limit = UintNumeric::from_bytes(&bytes[3..6])?;
        if let Some(first_limit) = first_limit
            && let Some(second_limit) = second_limit
        {
            Ok(Some(NavaidAltitudeLimitation {
                first_limit: first_limit,
                second_limit: second_limit,
            }))
        } else {
            return Err(FieldParseError {
                message: "Invalid first or second limit".to_string(),
            });
        }
    }
}

// Preferred Route Use
#[derive(Debug, PartialEq, Eq)]
pub enum PreferredRouteType {
    PointToPoint,
    AreaToArea,
}
impl PreferredRouteType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"P" => PreferredRouteType::PointToPoint,
            b"A" => PreferredRouteType::AreaToArea,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid preferred route type".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PreferredRouteRNAVRequirement {
    RNAVRequired,
    RNAVNotRequired,
}

impl PreferredRouteRNAVRequirement {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"R" => PreferredRouteRNAVRequirement::RNAVRequired,
            b"N" => PreferredRouteRNAVRequirement::RNAVNotRequired,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid preferred route RNAV requirement".to_string(),
                });
            }
        }))
    }
}

/// 5.220 Preferred Route Use Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct PreferredRouteUseIndicator(PreferredRouteType, PreferredRouteRNAVRequirement);
impl PreferredRouteUseIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let route_type = PreferredRouteType::from_bytes(&bytes[0..1])?;
        let rnav_requirement = PreferredRouteRNAVRequirement::from_bytes(&bytes[1..2])?;
        if let Some(route_type) = route_type
            && let Some(rnav_requirement) = rnav_requirement
        {
            Ok(Some(PreferredRouteUseIndicator(
                route_type,
                rnav_requirement,
            )))
        } else {
            return Err(FieldParseError {
                message: "Invalid preferred route type or RNAV requirement".to_string(),
            });
        }
    }
}

// Aircraft Use Group
#[derive(Debug, PartialEq, Eq)]
pub enum AircraftUseGroup {
    AllAircraft,
    AllAircraftLessThan250Kts,
    NonJetAndTurboprop,
    MultiEnginePropsOnly,
    JetsAndTurboPropsGreaterThan190Kts,
    HelicopterOnly,
    JetPower,
    TurbopropGreaterThan190Kts,
    NonJetNonTurboprop,
    NonJetGreaterThan190Kts,
    NonJetLessThan190Kts,
    AsDefinedInNotes,
    SingleEngine,
    TwinEngine,
}

impl AircraftUseGroup {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"A" => AircraftUseGroup::AllAircraft,
            b"C" => AircraftUseGroup::AllAircraftLessThan250Kts,
            b"D" => AircraftUseGroup::NonJetAndTurboprop,
            b"E" => AircraftUseGroup::MultiEnginePropsOnly,
            b"F" => AircraftUseGroup::JetsAndTurboPropsGreaterThan190Kts,
            b"H" => AircraftUseGroup::HelicopterOnly,
            b"J" => AircraftUseGroup::JetPower,
            b"M" => AircraftUseGroup::TurbopropGreaterThan190Kts,
            b"N" => AircraftUseGroup::NonJetNonTurboprop,
            b"P" => AircraftUseGroup::NonJetGreaterThan190Kts,
            b"Q" => AircraftUseGroup::NonJetLessThan190Kts,
            b"R" => AircraftUseGroup::AsDefinedInNotes,
            b"S" => AircraftUseGroup::SingleEngine,
            b"T" => AircraftUseGroup::TwinEngine,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid aircraft use group".to_string(),
                });
            }
        }))
    }
}

/// 5.221 Aircraft Use Group Indicator
#[derive(Debug, PartialEq, Eq)]
pub struct AircraftUseGroupIndicator(AircraftUseGroup, Option<AircraftUseGroup>);
impl AircraftUseGroupIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let aircraft_use_primary_group = AircraftUseGroup::from_bytes(&bytes[0..1])?;
        let aircraft_use_alternate_group = AircraftUseGroup::from_bytes(&bytes[1..2])?;
        if let Some(aircraft_use_primary_group) = aircraft_use_primary_group {
            // second group is optional
            Ok(Some(AircraftUseGroupIndicator(
                aircraft_use_primary_group,
                aircraft_use_alternate_group,
            )))
        } else {
            return Err(FieldParseError {
                message: "Invalid aircraft use group indicator".to_string(),
            });
        }
    }
}

// Number of Engines Restriction
// Side note, this is the least efficient field use I have found yet.. I would like to know more
#[derive(Debug, PartialEq, Eq)]
pub enum NumberOfEnginesRestrictionIndicator {
    Restricted,
    NotRestricted,
}
impl NumberOfEnginesRestrictionIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"Y" => NumberOfEnginesRestrictionIndicator::Restricted,
            b"N" => NumberOfEnginesRestrictionIndicator::NotRestricted,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid number of engines restriction indicator".to_string(),
                });
            }
        }))
    }
}

/// 5.232 Number of Engines Restriction
#[derive(Debug, PartialEq, Eq)]
pub struct NumberOfEnginesRestriction(
    NumberOfEnginesRestrictionIndicator,
    NumberOfEnginesRestrictionIndicator,
    NumberOfEnginesRestrictionIndicator,
    NumberOfEnginesRestrictionIndicator,
);
impl NumberOfEnginesRestriction {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        // all must have values
        let indicator1 = NumberOfEnginesRestrictionIndicator::from_bytes(&bytes[0..1])?;
        let indicator2 = NumberOfEnginesRestrictionIndicator::from_bytes(&bytes[1..2])?;
        let indicator3 = NumberOfEnginesRestrictionIndicator::from_bytes(&bytes[2..3])?;
        let indicator4 = NumberOfEnginesRestrictionIndicator::from_bytes(&bytes[3..4])?;
        if let Some(indicator1) = indicator1
            && let Some(indicator2) = indicator2
            && let Some(indicator3) = indicator3
            && let Some(indicator4) = indicator4
        {
            Ok(Some(NumberOfEnginesRestriction(
                indicator1, indicator2, indicator3, indicator4,
            )))
        } else {
            return Err(FieldParseError {
                message: "Invalid number of engines restriction".to_string(),
            });
        }
    }
}

// Leg Type Code
#[derive(Debug, PartialEq, Eq)]
pub enum LegTypePath {
    PointToPoint,
    CurvedLine,
}

impl LegTypePath {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"S" => LegTypePath::PointToPoint,
            b"C" => LegTypePath::CurvedLine,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid leg type path".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LegTypeTurnIndication {
    Left,
    Right,
}

impl LegTypeTurnIndication {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"L" => LegTypeTurnIndication::Left,
            b"R" => LegTypeTurnIndication::Right,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid leg type turn indication".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LegTypeCode(LegTypePath, Option<LegTypeTurnIndication>);
impl LegTypeCode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let leg_type_path = LegTypePath::from_bytes(&bytes[0..1])?;
        let leg_type_turn_indication = LegTypeTurnIndication::from_bytes(&bytes[1..2])?;
        // only the path is required
        if let Some(leg_type_path) = leg_type_path {
            Ok(Some(LegTypeCode(leg_type_path, leg_type_turn_indication)))
        } else {
            return Err(FieldParseError {
                message: "Invalid leg type code".to_string(),
            });
        }
    }
}

// GLS Station Type
#[derive(Debug, PartialEq, Eq)]
pub enum GLSStationType1 {
    LAASOrGLSGroundStation,
    SCAT1Station,
}

impl GLSStationType1 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"LAAS" => GLSStationType1::LAASOrGLSGroundStation,
            b"SCAT1" => GLSStationType1::SCAT1Station,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid GLS station type".to_string(),
                });
            }
        }))
    }
}

// reserved for future use
#[derive(Debug, PartialEq, Eq)]
pub enum GLSStationType2 {}
// reserved for future use
#[derive(Debug, PartialEq, Eq)]
pub enum GLSStationType3 {}

/// 5.247 GLS Station Type
#[derive(Debug, PartialEq, Eq)]
pub struct GLSStationType(
    GLSStationType1,
    Option<GLSStationType2>,
    Option<GLSStationType3>,
);
impl GLSStationType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let gls_station_type1 = GLSStationType1::from_bytes(&bytes[0..1])?;
        if let Some(gls_station_type1) = gls_station_type1 {
            Ok(Some(GLSStationType(gls_station_type1, None, None)))
        } else {
            return Err(FieldParseError {
                message: "Invalid GLS station type".to_string(),
            });
        }
    }
}

/// 5.274 TAA Sector Radius
pub struct TaaSectorRadius {
    pub start_radius: UintNumeric,
    pub end_radius: UintNumeric,
}
impl TaaSectorRadius {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let start_radius = UintNumeric::from_bytes(&bytes[0..2])?.ok_or(FieldParseError {
            message: "Invalid TAA sector radius".to_string(),
        })?;
        let end_radius = UintNumeric::from_bytes(&bytes[2..4])?.ok_or(FieldParseError {
            message: "Invalid TAA sector radius".to_string(),
        })?;
        Ok(Some(TaaSectorRadius {
            start_radius,
            end_radius,
        }))
    }
}

// Special Activity Area Operating Times
#[derive(Debug, PartialEq, Eq)]
pub enum SpecialActivityTimesDayIndicator {
    WeekdaysAndWeekends,
    WeekdaysOnly,
    WeekendsOnly,
    Other,
    Unknown,
}

impl SpecialActivityTimesDayIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"C" => SpecialActivityTimesDayIndicator::WeekdaysAndWeekends,
            b"D" => SpecialActivityTimesDayIndicator::WeekdaysOnly,
            b"E" => SpecialActivityTimesDayIndicator::WeekendsOnly,
            b"O" => SpecialActivityTimesDayIndicator::Other,
            b"U" => SpecialActivityTimesDayIndicator::Unknown,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid special activity times day indicator".to_string(),
                });
            }
        }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpecialActivityTimesHolidayIndicator {
    IncludingHolidays,
    ExcludingHolidays,
    Unknown,
}

impl SpecialActivityTimesHolidayIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"H" => SpecialActivityTimesHolidayIndicator::IncludingHolidays,
            b"X" => SpecialActivityTimesHolidayIndicator::ExcludingHolidays,
            b"U" => SpecialActivityTimesHolidayIndicator::Unknown,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid special activity times holiday indicator".to_string(),
                });
            }
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum SpecialActivityTimesTimeIndicator {
    SunriseOrSunset,
    NightUse,
    Continuous,
    ActiveByNotam,
}

impl SpecialActivityTimesTimeIndicator {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        Ok(Some(match bytes {
            b"D" => SpecialActivityTimesTimeIndicator::SunriseOrSunset,
            b"N" => SpecialActivityTimesTimeIndicator::NightUse,
            b"C" => SpecialActivityTimesTimeIndicator::Continuous,
            b"A" => SpecialActivityTimesTimeIndicator::ActiveByNotam,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid special activity times time indicator".to_string(),
                });
            }
        }))
    }
}

/// 5.282 Special Activity Area Operating Times
#[derive(Debug, PartialEq, Eq)]
pub struct SpecialActivityTimes(
    SpecialActivityTimesDayIndicator,
    SpecialActivityTimesHolidayIndicator,
    SpecialActivityTimesTimeIndicator,
);
impl SpecialActivityTimes {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        // all fields must be Some type
        let day_indicator = SpecialActivityTimesDayIndicator::from_bytes(&bytes[0..1])?;
        let holiday_indicator = SpecialActivityTimesHolidayIndicator::from_bytes(&bytes[1..2])?;
        let time_indicator = SpecialActivityTimesTimeIndicator::from_bytes(&bytes[2..3])?;
        if day_indicator.is_some() && holiday_indicator.is_some() && time_indicator.is_some() {
            Ok(Some(SpecialActivityTimes(
                day_indicator.unwrap(),
                holiday_indicator.unwrap(),
                time_indicator.unwrap(),
            )))
        } else {
            Err(FieldParseError {
                message: "Invalid special activity times".to_string(),
            })
        }
    }
}
