//! # ARINC 424 Definitions - Numerics
//! This module contains the numeric types for the ARINC 424 data.
//! Numerics are simple numeric fields. There are several cases we need to handle in the spec, but are more nice to work
//! with when converted appropriately. Latitudes and Longitudes are such cases where we convert to a more readable format.
//!
//! ## Combined Numerics
//!
//! Some fields encode multiple diverging types of numeric values.
//! For example, 5.27 - Route Distance From encodes both a time and a distance.
//!
//! To handle this, there are specific fields that capture the union of the two types appropriately.
#[cfg(test)]
use std::convert::Into;

use crate::parsers::arinc424::types::fields::*;

/// 5.12 Sequence Number
pub type SequenceNumber = UintNumeric;

/// 5.24 Theta
pub type Theta = FloatNumeric<-1>;

/// 5.25 Rho
pub type Rho = FloatNumeric<-1>;

/// 5.26 Outbound Course
pub type OutboundCourse = FloatNumeric<-1>;

/// 5.27 Route Distance From
pub type RouteDistanceFrom = TimeDistanceNumeric<-1>;

/// 5.28 Inbound Course
pub type InboundCourse = FloatNumeric<-1>;

/// 5.30 Altitude / Minimum Altitude
#[derive(Debug, PartialEq, Eq)]
pub enum MinimumAltitude {
    Established(AltitudeNumeric),
    Unknown,
    NotEstablished,
}

impl ParseableField for MinimumAltitude {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        let trimmed_bytes = bytes.trim_ascii();
        if trimmed_bytes.is_empty() {
            return Ok(None);
        }
        match trimmed_bytes {
            b"UNKNN" => Ok(Some(MinimumAltitude::Unknown)),
            b"NESTB" => Ok(Some(MinimumAltitude::NotEstablished)),
            _ => {
                let altitude = AltitudeNumeric::from_bytes(&bytes)?;
                if let Some(altitude) = altitude {
                    Ok(Some(MinimumAltitude::Established(altitude)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[test]
pub fn test_altitude_minimum_altitude() {
    let r = MinimumAltitude::from_bytes(&[b'F', b'L', b'1', b'0', b'0']);
    if let Ok(Some(MinimumAltitude::Established(altitude))) = r {
        assert_eq!(altitude, AltitudeNumeric(10000));
    } else {
        panic!("Failed to parse altitude minimum altitude");
    }
    let r = MinimumAltitude::from_bytes(&[b'-', b'1', b'1', b'0', b'0']);
    if let Ok(Some(MinimumAltitude::Established(altitude))) = r {
        assert_eq!(altitude, AltitudeNumeric(-1100));
    } else {
        panic!("Failed to parse altitude minimum altitude");
    }
}

/// 5.31 File Record Number
pub type FileRecordNumber = UintNumeric;

/// 5.32 Cycle Date
pub type CycleDate = UintNumeric;

/// 5.34 VOR/NDB Frequency
pub type VORNDBFrequency = FloatNumeric<-2>;

/// 5.36 Latitude
pub type Latitude = LatitudeNumeric;

/// 5.37 Longitude
pub type Longitude = LongitudeNumeric;

/// 5.39 Magnetic Variation
pub type MagneticVariation = MagneticVariationNumeric;

/// 5.40 DME Elevation
pub type DMEElevation = IntNumeric;

/// 5.45 Localizer Frequency (FREQ)
pub type LocalizerFrequency = FloatNumeric<-2>;

/// 5.47 Localizer Bearing (LOC BRG)
pub type LocalizerBearing = FloatNumeric<-1>;

/// 5.48 Localizer Position (LOC FR RW END / AZ/BAZ FR RW END) Azimuth/Back Azimuth Position (AZ/BAZ FR RW END)
pub type LocalizerPosition = IntNumeric;

/// 5.50 Glideslope Position (GS FR RW THRES) Elevation Position (EL FR RW THRES)
pub type GlideslopePosition = IntNumeric;

/// 5.51 Localizer Width (LOC WIDTH)
pub type LocalizerWidth = FloatNumeric<-1>;

/// 5.52 Glideslope angle (GS ANGLE) Minimum Elevation Angle (MIN ELEV ANGLE)
pub type GlideslopeAngle = FloatNumeric<-2>;

/// 5.53 Transition Altitude/Level (TRANS ALTITUDE/LEVEL)
pub type TransitionAltitudeLevel = IntNumeric;

/// 5.54 Longest Runway (LONGEST RWY)
pub type LongestRunway = UintNumeric;

/// 5.55 Airport/Heliport Elevation (ELEV)
pub type AirportHeliportElevation = IntNumeric;

/// 5.57 Runway Length (RUNWAY LENGTH)
pub type RunwayLength = UintNumeric;

/// 5.58 Runway Bearing (RWY BRG)
pub type RunwayBearing = FloatNumeric<-1>;

/// 5.62 Inbound Holding Course (IB HOLD CRS)
pub type InboundHoldingCourse = FloatNumeric<-1>;

/// 5.64 Leg Length (LEG LENGTH)
pub type LegLength = FloatNumeric<-1>;

/// 5.65 Leg Time (LEG TIME)
pub type LegTime = FloatNumeric<-1>;

/// 5.66 Station Declination (STN DEC)
pub type StationDeclination = DeclinationNumeric;

/// 5.67 Threshold Crossing Height (TCH)
pub type ThresholdCrossingHeight = UintNumeric;

/// 5.68 Landing Threshold Elevation (LANDING THRES ELEV)
pub type LandingThresholdElevation = IntNumeric;

/// 5.69 Threshold Displacement Distance (DSPLCD THR)
pub type ThresholdDisplacementDistance = UintNumeric;

/// 5.70 Vertical Angle (VERT ANGLE)
pub type VerticalAngle = FloatNumeric<-2>;

/// 5.72 Speed Limit (SPEED LIMIT)
pub type SpeedLimit = UintNumeric;

/// 5.73 Speed Limit Altitude (SPEED LIMIT ALTITUDE)
pub type SpeedLimitAltitude = AltitudeNumeric;

/// 5.74 Component Elevation (GS ELEV, EL ELEV, AZ ELEV, BAZ ELEV, GLS ELEV)
pub type ComponentElevation = IntNumeric;

/// 5.79 Stopway (STOPWAY)
pub type Stopway = UintNumeric;

/// 5.86 Cruise Altitude (CRUISE ALTITUDE)
pub type CruiseAltitude = AltitudeNumeric;

/// 5.88 Alternate Distance
pub type AlternateDistance = UintNumeric;

/// 5.89 Cost Index
pub type CostIndex = UintNumeric;

/// 5.90 ILS/DME Bias
pub type IlsDmeBias = FloatNumeric<-1>;

/// 5.92 Facility Elevation (FAC ELEV)
pub type FacilityElevation = IntNumeric;

/// 5.94 True Bearing
pub type TrueBearing = FloatNumeric<-2>;

/// 5.96 Glideslope Beam Width
pub type GlideslopeBeamWidth = FloatNumeric<-2>;

/// 5.97 Touchdown Zone Elevation
pub type TouchdownZoneElevation = IntNumeric;

/// 5.100 Minor Axis Bearing
pub type MinorAxisBearing = FloatNumeric<-1>;

// Communications Frequency

pub type HighFrequencyCommunicationsFrequency = FloatNumeric<-2>;
pub type VeryHighFrequencyCommunicationsFrequency = FloatNumeric<-3>;
pub type UltraHighFrequencyCommunicationsFrequency = FloatNumeric<-2>;

/// 5.103 Communications Frequency
///
/// This enum is needed since this is one of the few fields we need to perform a
/// lookup on the record as we parse the value out to know how to handle the value properly.
/// As such this does not have a from_bytes method, but rather the value using this is manually constructed
#[derive(Debug, PartialEq)]
pub enum CommunicationsFrequency {
    HighFrequency(HighFrequencyCommunicationsFrequency),
    VeryHighFrequency(VeryHighFrequencyCommunicationsFrequency),
    UltraHighFrequency(UltraHighFrequencyCommunicationsFrequency),
}

/// 5.109 Runway Width
pub type RunwayWidth = UintNumeric;

/// 5.119 Arc Distance
pub type ArcDistance = FloatNumeric<-1>;

/// 5.120 Arc Bearing
pub type ArcBearing = FloatNumeric<-1>;

/// 5.135 Course From/To (Cruise Table)
pub type CruiseTableCourseFromTo = FloatNumeric<-1>;

/// 5.137 Vertical Separation
pub type VerticalSeparation = MultiUnitAltitudeNumeric;

/// 5.145 MSA Radius Limit
pub type MsaRadiusLimit = UintNumeric;

/// 5.147 Sector Altitude
#[derive(Debug, PartialEq, Eq)]
pub enum SectorAltitude {
    NoSectorAltitude,
    SectorAltitude(UintNumeric),
}

impl ParseableField for SectorAltitude {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        match bytes {
            b"999" => Ok(Some(SectorAltitude::NoSectorAltitude)),
            _ => {
                let altitude = UintNumeric::from_bytes(bytes)?;
                if let Some(altitude) = altitude {
                    Ok(Some(SectorAltitude::SectorAltitude(altitude)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

/// 5.150 Frequency Protection Distance
pub type FrequencyProtectionDistance = UintNumeric;

/// 5.161 Airway Restriction Altitude
pub type AirwayRestrictionAltitude = UintNumeric;

/// 5.166 MLS Channel
pub type MLSChannel = UintNumeric;

/// 5.167 MLS Azimuth/Back Azimuth Bearing
pub type MLSAzimuthBearing = FloatNumeric<-1>;

/// 5.168 MLS Azimuth/Back Azimuth Proportional Angle
pub type MLSAzimuthProportionalAngle = UintNumeric;

/// 5.169 MLS Elevation Angle Span
pub type MLSElevationAngleSpan = FloatNumeric<-1>;

/// 5.172 MLS Azimuth/Back Azimuth Coverage Sector
pub type MLSAzimuthCoverageSector = UintNumeric;

/// 5.173 MLS Nominal Elevation Angle
pub type MLSNominalElevationAngle = FloatNumeric<-2>;

/// 5.175 Holding Speed
pub type HoldingSpeed = UintNumeric;

/// 5.184 Communication Altitude
pub type CommunicationsAltitude = UintNumeric;

/// 5.188 Communications Distance
pub type CommunicationsDistance = UintNumeric;

/// 5.204 ARC Radius
pub type ArcRadius = FloatNumeric<-3>;

/// 5.211 Required Navigation Performance
pub type RequiredNavigationPerformance = VariableFloatNumeric<-1>;

/// 5.212 Runway Gradient
pub type RunwayGradient = FloatNumeric<-3>;

/// 5.225 WGS-84 ellipsoid height
pub type WGS84EllipsoidHeight = FloatNumeric<-1>;

/// 5.226 SBAS/GBAS Glide Path Angle
pub type SBASGBASGlidePathAngle = FloatNumeric<-2>;

/// 5.227 Orthometric height
///
/// Note: All values are in reference to MSL
pub type OrthometricHeight = FloatNumeric<-1>;

/// 5.228 Course width at threshold
pub type CourseWidthAtThreshold = FloatNumeric<-2>;

/// 5.231 Along Track Distance
pub type AlongTrackDistance = UintNumeric;

/// 5.240 Flight Planning Altitude
pub type FlightPlanningAltitude = UintNumeric;

/// 5.244 SBAS/GBAS Channel
pub type SBASGBASChannel = UintNumeric;

/// 5.245 GLS Service Volume Radius
pub type GlsServiceVolumeRadius = UintNumeric;

/// 5.248 GLS WGS84 Station Elevation
///
/// Note: Be aware that this value is in whole feet, not meters with tenths like 5.225
pub type GLSWgs84StationElevation = IntNumeric;

/// 5.251 Distance To Alternate
pub type DistanceToAlternate = UintNumeric;

/// 5.254 Fixed Radius Transition Indicator
pub type FixedRadiusTransitionIndicator = FloatNumeric<-1>;

/// 5.256 Reference Path Data Selector
pub type ReferencePathDataSelector = UintNumeric;

/// 5.259 SBAS/GBAS Length Offset
pub type SBASGBASLengthOffset = UintNumeric;

/// 5.260 Terminal Procedure Flight Planning Leg Distance
pub type TerminalProcedureFlightPlanningLegDistance = FloatNumeric<-1>;

/// 5.263 Horizontal Alert Limit
pub type HorizontalAlertLimit = FloatNumeric<-1>;

/// 5.264 Vertical Alert Limit
pub type VerticalAlertLimit = FloatNumeric<-1>;

// Path Point TCH
pub type PathPointTCHFeet = FloatNumeric<-1>;
pub type PathPointTCHMeters = FloatNumeric<-2>;

/// 5.265 Path Point TCH
///
/// Note: This must be manually constructed as its value handling depends on another field's value.
#[derive(Debug, PartialEq)]
pub enum PathPointTCH {
    Feet(PathPointTCHFeet),
    Meters(PathPointTCHMeters),
}

/// 5.267 High Precision Latitude
pub type HighPrecisionLatitude = LatitudeNumeric;

/// 5.268 High Precision Longitude
pub type HighPrecisionLongitude = LongitudeNumeric;

/// 5.269 Helicopter Procedure Course
pub type HelicopterProcedureCourse = UintNumeric;

/// 5.280 Special Activity Area Size
pub type SpecialActivityAreaSize = FloatNumeric<-1>;

/// 5.290 Procedure Design Magnetic Variation
pub type ProcedureDesignMagneticVariation = MagneticVariationNumeric;

/// 5.292 Circling Category Distance
pub type CirclingCategoryDistance = FloatNumeric<-1>;

/// 5.293 Vertical Scale Factor
pub type VerticalScaleFactor = UintNumeric;

/// 5.294 RVSM Minimum Level
pub type RVSMMinimumLevel = UintNumeric;

/// 5.295 RVSM Maximum Level
pub type RVSMMaximumLevel = UintNumeric;

/// 5.296 RNP Level of Service
pub type RNPLevelOfService = VariableFloatNumeric<-1>;

/// 5.299 Final Approach Course as Runway
///
/// Note: This is a very odd field, but one that seems to benefit specifically Path Point approaches
#[derive(Debug)]
pub struct FinalApproachCourseAsRunway(u64);
impl ParseableField for FinalApproachCourseAsRunway {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let value = coalesce_into_number::<u64>(&bytes[0..2])? * 10;
        Ok(Some(FinalApproachCourseAsRunway(value)))
    }
}

impl Into<u64> for FinalApproachCourseAsRunway {
    fn into(self: FinalApproachCourseAsRunway) -> u64 {
        self.0
    }
}

/// 5.309 Maximum Allowable Helicopter Weight
pub type MaximumAllowableHelicopterWeight = UintNumeric;

/// 5.312 Runway Starter Extension
pub type RunwayStarterExtension = UintNumeric;

/// 5.313 TORA
pub type TORA = UintNumeric;

/// 5.314 TODA
pub type TODA = UintNumeric;

/// 5.315 ASDA
pub type ASDA = UintNumeric;

/// 5.316 LDA
pub type LDA = UintNumeric;

/// 5.320 SBAS Final Approach Course
pub type SBASFinalApproachCourse = RunwayBearing;

/// 5.321 Helipad Maximum Rotor Diameter
pub type HelipadMaximumRotorDiameter = UintNumeric;

/// 5.323 Heliport Orientation
pub type HeliportOrientation = FloatNumeric<-2>;

/// 5.324 Heliport Identifier Orientation
pub type HeliportIdentifierOrientation = FloatNumeric<-2>;

/// 5.325 Preferred Approach Bearing
pub type PreferredApproachBearing = RunwayBearing;

/// 5.343 Holding Pattern Magnetic Variation
pub type HoldingPatternMagneticVariation = MagneticVariationNumeric;
