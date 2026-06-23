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
use crate::parsers::arinc424::fields::FieldParseError;
#[cfg(test)]
use crate::test_util::assert_within_epsilon;
use std::convert::Into;

#[derive(Debug, PartialEq, Eq)]
pub struct UintNumeric(u64);
impl UintNumeric {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let value = u64::from_str_radix(
            std::str::from_utf8(bytes).map_err(|e| FieldParseError {
                message: format!("Numeric is not valid UTF-8: {}", e),
            })?,
            10,
        )
        .map_err(|e| FieldParseError {
            message: format!("Numeric is not a valid u64: {}", e),
        })?;
        Ok(Some(UintNumeric(value)))
    }
}

impl Into<u64> for UintNumeric {
    fn into(self: UintNumeric) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntNumeric(i64);
impl IntNumeric {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let value = i64::from_str_radix(
            std::str::from_utf8(bytes).map_err(|e| FieldParseError {
                message: format!("Numeric is not valid UTF-8: {}", e),
            })?,
            10,
        )
        .map_err(|e| FieldParseError {
            message: format!("Numeric is not a valid i64: {}", e),
        })?;
        Ok(Some(IntNumeric(value)))
    }
}

#[test]
pub fn test_int_numeric() {
    let r = IntNumeric::from_bytes(&[b'0', b'0', b'1']);
    if let Ok(Some(IntNumeric(value))) = r {
        assert_eq!(value, 1);
    } else {
        panic!("Failed to parse int numeric");
    }
}

#[derive(Debug, PartialEq)]
pub struct FloatNumeric<const RADIX_SHIFT: i32 = 0>(f64);
impl<const RADIX_SHIFT: i32> FloatNumeric<RADIX_SHIFT> {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let int_val = u64::from_str_radix(
            std::str::from_utf8(bytes.trim_ascii_end()).map_err(|e| FieldParseError {
                message: format!("Numeric is not valid UTF-8: {}", e),
            })?,
            10,
        )
        .map_err(|e| FieldParseError {
            message: format!("Numeric is not a valid u64: {}", e),
        })?;
        Ok(Some(FloatNumeric(
            int_val as f64 * 10_f64.powi(RADIX_SHIFT),
        )))
    }
}

impl<const RADIX_SHIFT: i32> Into<f64> for FloatNumeric<RADIX_SHIFT> {
    fn into(self: FloatNumeric<RADIX_SHIFT>) -> f64 {
        self.0
    }
}

#[test]
pub fn test_float_numeric() {
    let r = FloatNumeric::<0>::from_bytes(&[b'0', b'0', b'1']);
    if let Ok(Some(f)) = r {
        let val: f64 = f.into();
        assert_within_epsilon(val, 1.0);
    } else {
        panic!("Failed to parse float numeric");
    }

    let r = FloatNumeric::<-1>::from_bytes(&[b'2', b'7', b'6', b'1']);
    if let Ok(Some(f)) = r {
        let val: f64 = f.into();
        assert_within_epsilon(val, 276.1);
    } else {
        panic!("Failed to parse float numeric");
    }
}

// This is special since it has two values in one field because of the way
// some fields are handled.
#[derive(Debug, PartialEq)]
pub enum TimeDistanceNumeric<const RADIX_SHIFT: i32 = 0> {
    Time(FloatNumeric<RADIX_SHIFT>),
    Distance(FloatNumeric<RADIX_SHIFT>),
}

impl<const RADIX_SHIFT: i32> TimeDistanceNumeric<RADIX_SHIFT> {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        // if starts with T, it is a time
        if bytes.starts_with(b"T") {
            let time = FloatNumeric::<RADIX_SHIFT>::from_bytes(&bytes[1..])?;
            if let Some(time) = time {
                return Ok(Some(TimeDistanceNumeric::Time(time)));
            }
            return Ok(None);
        }
        // otherwise it is a distance
        let distance = FloatNumeric::<RADIX_SHIFT>::from_bytes(&bytes)?;
        if let Some(distance) = distance {
            return Ok(Some(TimeDistanceNumeric::Distance(distance)));
        }
        return Ok(None);
    }
}

impl<const RADIX_SHIFT: i32> Into<f64> for TimeDistanceNumeric<RADIX_SHIFT> {
    fn into(self: TimeDistanceNumeric<RADIX_SHIFT>) -> f64 {
        match self {
            TimeDistanceNumeric::Time(t) => t.into(),
            TimeDistanceNumeric::Distance(d) => d.into(),
        }
    }
}
#[test]

pub fn test_time_distance_numeric() {
    let r = TimeDistanceNumeric::<-1>::from_bytes(&[b'T', b'0', b'0', b'1']);
    if let Ok(Some(TimeDistanceNumeric::Time(t))) = r {
        let val: f64 = t.into();
        assert_within_epsilon(val, 0.1);
    } else {
        panic!("Failed to parse time distance numeric");
    }
    let r = TimeDistanceNumeric::<-1>::from_bytes(&[b'1', b'0', b'0', b'1']);
    if let Ok(Some(TimeDistanceNumeric::Distance(d))) = r {
        let val: f64 = d.into();
        assert_within_epsilon(val, 100.1);
    } else {
        panic!("Failed to parse time distance numeric");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AltitudeNumeric(i32);
impl AltitudeNumeric {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        // if starts with FL, it is a flight level and we convert to altitude
        if bytes.starts_with(b"FL") {
            let value = i32::from_str_radix(
                std::str::from_utf8(&bytes[2..]).map_err(|e| FieldParseError {
                    message: format!("Numeric is not valid UTF-8: {}", e),
                })?,
                10,
            )
            .map_err(|e| FieldParseError {
                message: format!("Numeric is not a valid i32: {}", e),
            })?;
            return Ok(Some(AltitudeNumeric(value * 100)));
        }
        // otherwise it is an altitude
        let value = i32::from_str_radix(
            std::str::from_utf8(bytes).map_err(|e| FieldParseError {
                message: format!("Numeric is not valid UTF-8: {}", e),
            })?,
            10,
        )
        .map_err(|e| FieldParseError {
            message: format!("Numeric is not a valid u64: {}", e),
        })?;
        Ok(Some(AltitudeNumeric(value as i32)))
    }
}

impl Into<i32> for AltitudeNumeric {
    fn into(self: AltitudeNumeric) -> i32 {
        self.0
    }
}

#[test]
pub fn test_altitude() {
    let r = AltitudeNumeric::from_bytes(&[b'1', b'0', b'0', b'0']);
    if let Ok(Some(AltitudeNumeric(altitude))) = r {
        assert_eq!(altitude, 1000);
    } else {
        panic!("Failed to parse altitude");
    }
    let r = AltitudeNumeric::from_bytes(&[b'F', b'L', b'1', b'0', b'0']);
    if let Ok(Some(AltitudeNumeric(altitude))) = r {
        assert_eq!(altitude, 10000);
    } else {
        panic!("Failed to parse altitude");
    }
    let r = AltitudeNumeric::from_bytes(&[b'-', b'1', b'1', b'0', b'0']);
    if let Ok(Some(AltitudeNumeric(altitude))) = r {
        assert_eq!(altitude, -1100);
    } else {
        panic!("Failed to parse altitude");
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MultiUnitAltitudeNumeric {
    Meters(AltitudeNumeric),
    Feet(AltitudeNumeric),
}

impl MultiUnitAltitudeNumeric {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        match bytes {
            [b'M', rest @ ..] => {
                let value = AltitudeNumeric::from_bytes(rest)?;
                if let Some(value) = value {
                    Ok(Some(MultiUnitAltitudeNumeric::Meters(value)))
                } else {
                    Ok(None)
                }
            }
            _ => {
                let value = AltitudeNumeric::from_bytes(bytes)?;
                if let Some(value) = value {
                    Ok(Some(MultiUnitAltitudeNumeric::Feet(value)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl Into<i32> for MultiUnitAltitudeNumeric {
    fn into(self: MultiUnitAltitudeNumeric) -> i32 {
        match self {
            MultiUnitAltitudeNumeric::Meters(m) => m.into(),
            MultiUnitAltitudeNumeric::Feet(f) => f.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LatitudeNumeric(f64);
impl LatitudeNumeric {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let sign = if bytes[0] == b'S' { -1.0 } else { 1.0 };
        let [degrees, minutes, seconds, hundredths]: [u32; 4] = bytes[1..9]
            .chunks_exact(2)
            .map(|b| u32::from_str_radix(std::str::from_utf8(b).unwrap(), 10).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|e| FieldParseError {
                message: format!(
                    "Failed to convert bytes to [u32; 4]: {:?}, got bytes: {:?}",
                    e,
                    &bytes[1..9]
                ),
            })?;
        let value = sign
            * (degrees as f64
                + minutes as f64 / 60.0
                + seconds as f64 / 3600.0
                + hundredths as f64 / 360000.0);
        Ok(Some(LatitudeNumeric(value)))
    }
}

impl Into<f64> for LatitudeNumeric {
    fn into(self: LatitudeNumeric) -> f64 {
        self.0
    }
}

#[test]
pub fn test_latitude() {
    let r = LatitudeNumeric::from_bytes(b"N39513881");
    if let Ok(Some(LatitudeNumeric(latitude))) = r {
        assert_within_epsilon(latitude, 39.860780556);
    }
}

#[derive(Debug, PartialEq)]
pub struct LongitudeNumeric(f64);
impl LongitudeNumeric {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let sign = if bytes[0] == b'W' { -1.0 } else { 1.0 };
        let (degree_bytes, rest) = &bytes[1..].split_at(3);
        let degrees =
            u32::from_str_radix(std::str::from_utf8(degree_bytes).unwrap(), 10).map_err(|e| {
                FieldParseError {
                    message: format!(
                        "Failed to convert bytes to u32: {:?}, got bytes: {:?}",
                        e, degree_bytes
                    ),
                }
            })?;
        let [minutes, seconds, hundredths]: [u32; 3] = rest
            .chunks_exact(2)
            .map(|b| u32::from_str_radix(std::str::from_utf8(b).unwrap(), 10).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|e| FieldParseError {
                message: format!(
                    "Failed to convert bytes to [u32; 4]: {:?}, got bytes: {:?}",
                    e, rest
                ),
            })?;
        let value = sign
            * (degrees as f64
                + minutes as f64 / 60.0
                + seconds as f64 / 3600.0
                + hundredths as f64 / 360000.0);
        Ok(Some(LongitudeNumeric(value)))
    }
}

impl Into<f64> for LongitudeNumeric {
    fn into(self: LongitudeNumeric) -> f64 {
        self.0
    }
}

#[test]
pub fn test_longitude() {
    let r = LongitudeNumeric::from_bytes(b"W039513881");
    if let Ok(Some(LongitudeNumeric(longitude))) = r {
        assert_within_epsilon(longitude, -39.860780556);
    }
}

#[derive(Debug, PartialEq)]
pub struct MagneticVariationNumeric(f64);
impl MagneticVariationNumeric {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let sign = match bytes[0] {
            b'E' => 1.0,
            b'W' => -1.0,
            b'T' => 0.0,
            b'G' => 0.0,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid magnetic variation".to_string(),
                });
            }
        };
        let value = 0.1
            * sign
            * u32::from_str_radix(
                std::str::from_utf8(&bytes[1..]).map_err(|e| FieldParseError {
                    message: format!("Failed to convert bytes to u32: {:?}", e),
                })?,
                10,
            )
            .map_err(|e| FieldParseError {
                message: format!("Failed to convert bytes to u32: {:?}", e),
            })? as f64;
        Ok(Some(MagneticVariationNumeric(value)))
    }
}
impl Into<f64> for MagneticVariationNumeric {
    fn into(self: MagneticVariationNumeric) -> f64 {
        self.0
    }
}

#[test]
pub fn test_magnetic_variation() {
    let r = MagneticVariationNumeric::from_bytes(b"W0010");
    if let Ok(Some(MagneticVariationNumeric(magnetic_variation))) = r {
        assert_within_epsilon(magnetic_variation, -1.0);
    }
}

/// Declination Numeric at first glance seems similar to Magnetic Variation, but is actually used in
/// much more different contexts that need different semantic handling of values.
pub enum DeclinationNumeric {
    StandardDeclination(f64),
    TrueNorth(f64),
    GridNorth(f64),
}

impl DeclinationNumeric {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        let sign = match bytes[0] {
            b'E' => 1.0,
            b'W' => -1.0,
            b'T' => 0.0,
            b'G' => 0.0,
            _ => {
                return Err(FieldParseError {
                    message: "Invalid declination".to_string(),
                });
            }
        };
        let value = sign
            * 0.1
            * u32::from_str_radix(
                std::str::from_utf8(&bytes[1..]).map_err(|e| FieldParseError {
                    message: format!("Failed to convert bytes to u32: {:?}", e),
                })?,
                10,
            )
            .map_err(|e| FieldParseError {
                message: format!("Failed to convert bytes to u32: {:?}", e),
            })? as f64;
        Ok(Some(match bytes[0] {
            b'E' => DeclinationNumeric::StandardDeclination(value),
            b'W' => DeclinationNumeric::StandardDeclination(value),
            b'T' => DeclinationNumeric::TrueNorth(value),
            b'G' => DeclinationNumeric::GridNorth(value),
            _ => {
                return Err(FieldParseError {
                    message: "Invalid declination".to_string(),
                });
            }
        }))
    }
}

#[test]
pub fn test_declination() {
    let r = DeclinationNumeric::from_bytes(b"E0010");
    if let Ok(Some(DeclinationNumeric::StandardDeclination(declination))) = r {
        assert_within_epsilon(declination, 1.0);
    }
    let r = DeclinationNumeric::from_bytes(b"W0010");
    if let Ok(Some(DeclinationNumeric::StandardDeclination(declination))) = r {
        assert_within_epsilon(declination, -1.0);
    }
    let r = DeclinationNumeric::from_bytes(b"T0010");
    if let Ok(Some(DeclinationNumeric::TrueNorth(declination))) = r {
        assert_within_epsilon(declination, 0.0);
    }
    let r = DeclinationNumeric::from_bytes(b"G0010");
    if let Ok(Some(DeclinationNumeric::GridNorth(declination))) = r {
        assert_within_epsilon(declination, 0.0);
    }
}

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
pub enum AltitudeMinimumAltitude {
    Established(AltitudeNumeric),
    Unknown,
    NotEstablished,
}

impl AltitudeMinimumAltitude {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        match bytes {
            b"UNKNN" => Ok(Some(AltitudeMinimumAltitude::Unknown)),
            b"NESTB" => Ok(Some(AltitudeMinimumAltitude::NotEstablished)),
            _ => {
                let altitude = AltitudeNumeric::from_bytes(&bytes)?;
                if let Some(altitude) = altitude {
                    Ok(Some(AltitudeMinimumAltitude::Established(altitude)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[test]
pub fn test_altitude_minimum_altitude() {
    let r = AltitudeMinimumAltitude::from_bytes(&[b'F', b'L', b'1', b'0', b'0']);
    if let Ok(Some(AltitudeMinimumAltitude::Established(altitude))) = r {
        assert_eq!(altitude, AltitudeNumeric(10000));
    } else {
        panic!("Failed to parse altitude minimum altitude");
    }
    let r = AltitudeMinimumAltitude::from_bytes(&[b'-', b'1', b'1', b'0', b'0']);
    if let Ok(Some(AltitudeMinimumAltitude::Established(altitude))) = r {
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

/// Glideslope angle (GS ANGLE) Minimum Elevation Angle (MIN ELEV ANGLE)
pub type GlideslopeAngle = FloatNumeric<-2>;

/// 5.52 Transition Altitude/Level (TRANS ALTITUDE/LEVEL)
pub type TransitionAltitudeLevel = IntNumeric;

/// 5.53 Longest Runway (LONGEST RWY)
pub type LongestRunway = UintNumeric;

/// 5.54 Airport/Heliport Elevation (ELEV)
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

/// 5.104 Communications Frequency
///
/// This enum is needed since this is one of the few fields we need to perform a
/// lookup on the record as we parse the value out to know how to handle the value properly.
/// As such this does not have a from_bytes method, but rather the value using this is manually constructed
#[derive(Debug, PartialEq)]
pub enum CommunicationsFrequency {
    HighFrequency(FloatNumeric<-2>),
    VeryHighFrequency(FloatNumeric<-3>),
    UltraHighFrequency(FloatNumeric<-2>),
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

impl SectorAltitude {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
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
