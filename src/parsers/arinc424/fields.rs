//! # ARINC 424 Field Parsers
//! This module contains the parsers for the ARINC 424 field types.
//!
//! ## Field Types
//! - Alpha
//! - Alphanumeric
//! - Numeric
//!
//! ## Field Parsers
//! - FieldRaw
//! - FieldParseError
//!
//! ## Raw Fields
//! We define raw field types that do a minimum data validation for first pass data loading.
//! Since Generative LLMs were used to help generate the field names, there may be errors in what the field is supposed to be.
//! To ensure quality, human-verified raw fields are denoted with ✅
//!
//! ### Variants
//!
//! Variants are denoted with (A), (B), (C), (D), etc.
//! They are used ONLY when the length between records is necessarily different.
//! If there are conditionally Numeric AND Alpha fields, it is preferred to use the Alphanumeric field type.
//! and validate on record level later on.
#![allow(non_camel_case_types)]

pub const BLANK: u8 = b' ';
#[derive(Debug, PartialEq, Eq)]
pub struct FieldParseError {
    pub message: String,
}

pub type DType = u8;

const DTYPE_ALPHA: DType = 0;
const DTYPE_ALPHANUMERIC: DType = 1;
const DTYPE_NUMERIC: DType = 2;

#[derive(Debug, PartialEq, Eq)]
pub struct FieldRaw<
    'a,
    const DTYPE: DType,
    const MAX_LEN: usize,
    const STARTCOL: usize,
    const LEN: usize,
> {
    pub bytes: &'a [u8],
}
impl<'a, const DTYPE: DType, const MAX_LEN: usize, const STARTCOL: usize, const LEN: usize>
    FieldRaw<'a, DTYPE, MAX_LEN, STARTCOL, LEN>
{
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            bytes: &input[STARTCOL - 1..STARTCOL + LEN - 1],
        }
    }
}
impl<'a, const MAX_LEN: usize, const STARTCOL: usize, const LEN: usize>
    FieldRaw<'a, DTYPE_ALPHA, MAX_LEN, STARTCOL, LEN>
{
    pub fn as_value(&self) -> Result<&str, FieldParseError> {
        if LEN > MAX_LEN {
            return Err(FieldParseError {
                message: "LEN must be <= MAX_LEN".to_string(),
            });
        }
        if STARTCOL < 1 {
            return Err(FieldParseError {
                message: "STARTCOL must be >= 1".to_string(),
            });
        }
        if self.bytes.len() < STARTCOL + LEN - 1 {
            return Err(FieldParseError {
                message: format!(
                    "Input too short: expected {} bytes, got {} bytes",
                    STARTCOL + LEN - 1,
                    self.bytes.len()
                ),
            });
        }
        if let Ok(s) = std::str::from_utf8(&self.bytes.trim_ascii_end()) {
            if s.chars().any(|c| {
                !(c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace())
                    || c.is_ascii_digit()
            }) {
                return Err(FieldParseError {
                    message: format!("Invalid alpha data: {s}"),
                });
            }
            Ok(s)
        } else {
            Err(FieldParseError {
                message: "Unexpected character encountered".to_string(),
            })
        }
    }
}
impl<'a, const MAX_LEN: usize, const STARTCOL: usize, const LEN: usize>
    FieldRaw<'a, DTYPE_ALPHANUMERIC, MAX_LEN, STARTCOL, LEN>
{
    pub fn as_value(&self) -> Result<&str, FieldParseError> {
        if LEN > MAX_LEN {
            return Err(FieldParseError {
                message: "LEN must be <= MAX_LEN".to_string(),
            });
        }
        if STARTCOL < 1 {
            return Err(FieldParseError {
                message: "STARTCOL must be >= 1".to_string(),
            });
        }
        if self.bytes.len() < STARTCOL + LEN - 1 {
            return Err(FieldParseError {
                message: format!(
                    "Input too short: expected {} bytes, got {} bytes",
                    STARTCOL + LEN - 1,
                    self.bytes.len()
                ),
            });
        }
        if let Ok(s) = std::str::from_utf8(&self.bytes.trim_ascii_end()) {
            if s.chars().any(|c| {
                !(c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace())
            }) {
                return Err(FieldParseError {
                    message: format!("Invalid alphanumeric data: {s}"),
                });
            }
            Ok(s)
        } else {
            Err(FieldParseError {
                message: "Unexpected character encountered".to_string(),
            })
        }
    }
}
impl<'a, const MAX_LEN: usize, const STARTCOL: usize, const LEN: usize>
    FieldRaw<'a, DTYPE_NUMERIC, MAX_LEN, STARTCOL, LEN>
{
    pub fn as_value(&self) -> Result<u64, FieldParseError> {
        if LEN > MAX_LEN {
            return Err(FieldParseError {
                message: "LEN must be <= MAX_LEN".to_string(),
            });
        }
        if STARTCOL < 1 {
            return Err(FieldParseError {
                message: "STARTCOL must be >= 1".to_string(),
            });
        }
        if self.bytes.len() < STARTCOL + LEN - 1 {
            return Err(FieldParseError {
                message: format!(
                    "Input too short: expected {} bytes, got {} bytes",
                    STARTCOL + LEN - 1,
                    self.bytes.len()
                ),
            });
        }
        // needs to handle left padding with zeros
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            if s.chars().any(|c| !c.is_ascii_digit()) {
                return Err(FieldParseError {
                    message: format!("Invalid numeric data: {s}"),
                });
            }
            s.parse::<u64>().map_err(|e| FieldParseError {
                message: format!("Invalid numeric data: {s}: {e}"),
            })
        } else {
            Err(FieldParseError {
                message: "Unexpected character encountered".to_string(),
            })
        }
    }
}

#[test]
pub fn test_as_alpha() {
    // should trim trailing blanks
    let r: FieldRaw<DTYPE_ALPHA, 3, 1, 3> = FieldRaw::new(&[b'D', b'-', b' ']);
    assert_eq!(r.as_value(), Ok("D-"));
    // should keep leading blanks
    let r: FieldRaw<DTYPE_ALPHA, 3, 1, 3> = FieldRaw::new(&[b' ', b'@', b'D']);
    assert_eq!(r.as_value(), Ok(" @D"));
    // should error on non-alpha characters
    let r: FieldRaw<DTYPE_ALPHA, 3, 1, 3> = FieldRaw::new(&[b'0', b'0', b'S']);
    assert_eq!(
        r.as_value(),
        Err(FieldParseError {
            message: "Invalid alpha data: 00S".to_string(),
        })
    );
}

#[test]
pub fn test_as_alphanumeric() {
    // similar behavior to as_alpha
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 3, 1, 3> = FieldRaw::new(&[b'D', b'-', b' ']);
    assert_eq!(r.as_value(), Ok("D-"));
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 3, 1, 3> = FieldRaw::new(&[b' ', b'@', b'D']);
    assert_eq!(r.as_value(), Ok(" @D"));
    // except now this should be ok
    let r: FieldRaw<DTYPE_ALPHANUMERIC, 3, 1, 3> = FieldRaw::new(&[b'0', b'0', b'S']);
    assert_eq!(r.as_value(), Ok("00S"));
}

#[test]
pub fn test_as_numeric() {
    let r: FieldRaw<DTYPE_NUMERIC, 3, 1, 3> = FieldRaw::new(&[b'0', b'0', b'1']);
    assert_eq!(r.as_value(), Ok(1));
    let r: FieldRaw<DTYPE_NUMERIC, 3, 1, 3> = FieldRaw::new(&[b'0', b'0', b' ']);
    assert!(
        r.as_value()
            .unwrap_err()
            .message
            .contains("Invalid numeric data: 00")
    );
}

#[test]
#[should_panic(expected = "STARTCOL must be >= 1")]
pub fn test_field_raw_startcol_must_be_greater_than_zero() {
    let _: FieldRaw<DTYPE_NUMERIC, 3, 0, 3> = FieldRaw::new(&[b'0', b'0', b'1']);
}

#[test]
#[should_panic(expected = "LEN must be <= MAX_LEN")]
pub fn test_field_raw_len_must_be_less_than_max_len() {
    let _: FieldRaw<DTYPE_NUMERIC, 3, 1, 4> = FieldRaw::new(&[b'0', b'0', b'1']);
}

/// Helper field for spacing to keep record definitions easier to maintain
pub type BlankSpacingRaw<'a, const STARTCOL: usize, const LENGTH: usize> =
    FieldRaw<'a, DTYPE_ALPHA, STARTCOL, LENGTH, LENGTH>;

/// Helper field for when there is a fixed length, and not just max length
pub type FixedLengthFieldRaw<'a, const DTYPE: DType, const STARTCOL: usize, const LEN: usize> =
    FieldRaw<'a, DTYPE, LEN, STARTCOL, LEN>;

// --- ARINC 424 Chapter 5 navigation field raw types (Section 5.0 field definitions) ---

/// 5.2 – Record Type (S/T) ✅
pub type _5_2_RecordTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.3 – Customer/Area Code (CUST/AREA), Area ✅
pub type _5_3_CustomerAreaCodeRaw<'a, const STARTCOL: usize, const LEN: usize = 3> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 3, STARTCOL, LEN>;
/// 5.4 – Section Code (SEC CODE) ✅
pub type _5_4_SectionCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.5 – Subsection Code (SUB CODE) ✅
pub type _5_5_SubsectionCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.6 – Airport/Heliport Identifier (ARPT/HELI IDENT) ✅
pub type _5_6_AirportHeliportIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 4> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 4, STARTCOL, LEN>;
/// 5.7 – Route Type (RT TYPE), Enroute Airway ✅
pub type _5_7_RouteTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.8(A) – Route Identifier (ROUTE IDENT), Enroute Airway ✅
pub type _5_8_A_EnrouteRouteIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHA, 5, STARTCOL, LEN>;
/// 5.8(B) – Route Identifier (ROUTE IDENT), Preferred Route ✅
pub type _5_8_B_PreferredRouteIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 10> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 10, STARTCOL, LEN>;
/// 5.9 – SID/STAR Route Identifier (SID/STAR IDENT) ✅
pub type _5_9_SidStarRouteIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 6> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 6, STARTCOL, LEN>;
/// 5.10 – Approach Route Identifier (APPROACH IDENT) ✅
pub type _5_10_ApproachRouteIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 6> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 6, STARTCOL, LEN>;
/// 5.11 – Transition Identifier (TRANS IDENT) ✅
pub type _5_11_TransitionIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.12(A) – Sequence Number (SEQ NR), 4 characters ✅
pub type _5_12_A_SequenceNumber4CharacterRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.12(B) – Sequence Number (SEQ NR), 3 characters ✅
pub type _5_12_B_SequenceNumber3CharacterRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.12(C) – Sequence Number (SEQ NR), 2 characters ✅
pub type _5_12_C_SequenceNumber2CharacterRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.12(D) – Sequence Number (SEQ NR), 1 character ✅
pub type _5_12_D_SequenceNumber1CharacterRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.13 – Fix Identifier (FIX IDENT) ✅
pub type _5_13_FixIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.14 – ICAO Code (ICAO CODE) ✅
pub type _5_14_IcaoCodeRaw<'a, const STARTCOL: usize, const LEN: usize = 2> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 2, STARTCOL, LEN>;
/// 5.15 – Inbound Course Theta (holding pattern) ✅
pub type _5_15_InboundCourseThetaRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.16 – Continuation Record Number (CONT NR) ✅
pub type _5_16_ContinuationRecordNumberRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.17 – Waypoint Description Code (DESC CODE) ✅
pub type _5_17_WaypointDescriptionCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 4>;
/// 5.18 – Boundary Code (BDY CODE) ✅
pub type _5_18_BoundaryCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.19 – Level (LEVEL) ✅
pub type _5_19_LevelRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.20 – Turn Direction (TURN DIR) ✅
pub type _5_20_TurnDirectionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.21 – Path and Termination (PATH TERM) ✅
pub type _5_21_PathAndTerminationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 2>;
/// 5.22 – Turn Direction Valid (TDV) ✅
pub type _5_22_TurnDirectionValidRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.23 – Recommended NAVAID (RECD NAV) ✅
pub type _5_23_RecommendedNavaidRaw<'a, const STARTCOL: usize, const LEN: usize = 4> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 4, STARTCOL, LEN>;
/// 5.24 – Theta (THETA) ✅
pub type _5_24_ThetaRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.25 – Rho (RHO) ✅
pub type _5_25_RhoRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.26 – Outbound Course (OB CRS) ✅
pub type _5_26_OutboundCourseRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.27 – Route Distance From, Holding Distance/Time ✅
pub type _5_27_RouteDistanceFromHoldingDistanceTimeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.28 – Inbound Course (IB CRS) ✅
pub type _5_28_InboundCourseRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.29 – Altitude Description (ALT DESC) ✅
pub type _5_29_AltitudeDescriptionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.30 – Altitude / Minimum Altitude ✅
pub type _5_30_AltitudeMinimumAltitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.31 – File Record Number (FRN) ✅
pub type _5_31_FileRecordNumberRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.32 – Cycle Date (CYCLE) ✅
pub type _5_32_CycleDateRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.33 – VOR/NDB Identifier (VOR IDENT/NDB IDENT) ✅
pub type _5_33_VorNdbIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 4> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 4, STARTCOL, LEN>;
/// 5.34 – VOR/NDB Frequency (VOR/NDB FREQ) ✅
pub type _5_34_VorNdbFrequencyRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.35 – NAVAID Class (CLASS) ✅
pub type _5_35_NavaidClassRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 5>;
/// 5.36 – Latitude (LATITUDE) ✅
pub type _5_36_LatitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 9>;
/// 5.37 – Longitude (LONGITUDE) ✅
pub type _5_37_LongitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.38 – DME Identifier (DME IDENT) ✅
pub type _5_38_DMEIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 4> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 4, STARTCOL, LEN>;
/// 5.39 – Magnetic Variation (MAG VAR, D MAG VAR) ✅
pub type _5_39_MagneticVariationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.40 – DME Elevation (DME ELEV) ✅
pub type _5_40_DmeElevationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.41 – Region Code (REGN CODE) ✅
pub type _5_41_RegionCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.42 – Waypoint Type (TYPE) ✅
pub type _5_42_WaypointTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 3>;
/// 5.43 – Waypoint Name/Description (NAME/DESC) ✅
pub type _5_43_WaypointNameDescriptionRaw<'a, const STARTCOL: usize, const LEN: usize = 25> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 25, STARTCOL, LEN>;
/// 5.44 – Localizer/MLS/GLS Identifier (LOC, MLS, GLS IDENT) ✅
pub type _5_44_LocalizerMlsGlsIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 4> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 4, STARTCOL, LEN>;
/// 5.45 – Localizer Frequency (FREQ) ✅
pub type _5_45_LocalizerFrequencyRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.46 – Runway Identifier (RUNWAY ID) ✅
pub type _5_46_RunwayIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.47 – Localizer Bearing (LOC BRG) ✅
pub type _5_47_LocalizerBearingRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.48 – Localizer/Azimuth Position (LOC FR RW END / AZ/BAZ FR RW END) ✅
pub type _5_48_LocalizerAzimuthPositionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.49 – Localizer/Azimuth Position Reference (@, +, -) ✅
pub type _5_49_LocalizerAzimuthPositionReferenceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.50 – Glideslope/Elevation Position (GS FR RW THRES / EL FR RW THRES) ✅
pub type _5_50_GlideslopeElevationPositionRaw<'a, const STARTCOL: usize, const LEN: usize = 4> =
    FieldRaw<'a, DTYPE_NUMERIC, 4, STARTCOL, LEN>;
/// 5.51 – Localizer Width (LOC WIDTH) ✅
pub type _5_51_LocalizerWidthRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.52 – Glideslope Angle / Minimum Elevation Angle (GS ANGLE / MIN ELEV ANGLE) ✅
pub type _5_52_GlideslopeAngleRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.53 – Transition Altitude/Level (TRANS ALTITUDE/LEVEL) ✅
pub type _5_53_TransitionAltitudeLevelRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.54 – Longest Runway (LONGEST RWY) ✅
pub type _5_54_LongestRunwayRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.55 – Airport/Heliport Elevation (ELEV) ✅
pub type _5_55_AirportHeliportElevationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.56 – Gate Identifier (GATE IDENT) ✅
pub type _5_56_GateIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.57 – Runway Length (RUNWAY LENGTH) ✅
pub type _5_57_RunwayLengthRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.58 – Runway Bearing (RWY BRG) ✅
pub type _5_58_RunwayBearingRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.59 – Runway Description (RUNWAY DESCRIPTION) ✅
pub type _5_59_RunwayDescriptionRaw<'a, const STARTCOL: usize, const LEN: usize = 22> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 22, STARTCOL, LEN>;
/// 5.60 – Name (NAME), Gate and Holding Pattern records ✅
pub type _5_60_GateHoldingPatternNameRaw<'a, const STARTCOL: usize, const LEN: usize = 25> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 25, STARTCOL, LEN>;
/// 5.61 – Notes, continuation records (NOTES) ✅
pub type _5_61_ContinuationNotesRaw<'a, const STARTCOL: usize, const LEN: usize = 102> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 102, STARTCOL, LEN>;
/// 5.62 – Inbound Holding Course (IB HOLD CRS) ✅
pub type _5_62_InboundHoldingCourseRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.63 – Turn (TURN), Holding Pattern records ✅
pub type _5_63_HoldingPatternTurnRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.64 – Leg Length (LEG LENGTH) ✅
pub type _5_64_HoldingLegLengthRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.65 – Leg Time (LEG TIME) ✅
pub type _5_65_HoldingLegTimeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.66 – Station Declination (STN DEC) ✅
pub type _5_66_StationDeclinationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.67 – Threshold Crossing Height (TCH) ✅
pub type _5_67_ThresholdCrossingHeightRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.68 – Landing Threshold Elevation (LANDING THRES ELEV) ✅
pub type _5_68_LandingThresholdElevationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.69 – Threshold Displacement Distance (DSPLCD THR) ✅
pub type _5_69_ThresholdDisplacementDistanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.70 – Vertical Angle (VERT ANGLE) ✅
pub type _5_70_VerticalAngleRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.71 – Name Field, Navaid/Airport/Heliport/Enroute Marker records ✅
pub type _5_71_FacilityNameFieldRaw<'a, const STARTCOL: usize, const LEN: usize = 30> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 30, STARTCOL, LEN>;
/// 5.72 – Speed Limit (SPEED LIMIT) ✅
pub type _5_72_SpeedLimitRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.73 – Speed Limit Altitude ✅
pub type _5_73_SpeedLimitAltitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.74 – Component Elevation (GS ELEV, EL ELEV, AZ ELEV, BAZ ELEV, GLS ELEV) ✅
pub type _5_74_ComponentElevationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.75 – From/To Airport/Heliport/Fix ✅
pub type _5_75_FromToAirportHeliportFixRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.76 – Company Route Ident ✅
pub type _5_76_CompanyRouteIdentRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.77 – VIA Code ✅
pub type _5_77_ViaCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.78 – SID/STAR/App/AWY (S/S/A/AWY), SID/STAR/AWY (S/S/AWY) ✅
pub type _5_78_SidStarApproachAirwayRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.79 – Stopway ✅
pub type _5_79_StopwayRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.80 – ILS/MLS/GLS Category (CAT) ✅
pub type _5_80_IlsMlsGlsCategoryRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.81 – ATC Indicator (ATC) ✅
pub type _5_81_AtcIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.82 – Waypoint Usage ✅
pub type _5_82_WaypointUsageRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.83 – To Fix, Company Route / Helicopter Operations Company Route (6 characters max) ✅
pub type _5_83_CompanyRouteToFixRaw<'a, const STARTCOL: usize, const LEN: usize = 6> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 6, STARTCOL, LEN>;
/// 5.83 – To Fix, Preferred Route (5 characters max) ✅
pub type _5_83_PreferredRouteToFixRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.84 – Runway Transition (RUNWAY TRANS) ✅
pub type _5_84_RunwayTransitionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.85 – Enroute Transition (ENRT TRANS) ✅
pub type _5_85_EnrouteTransitionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.86 – Cruise Altitude ✅
pub type _5_86_CruiseAltitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.87 – Terminal/Alternate Airport (TERM/ALT ARPT) ✅
pub type _5_87_TerminalAlternateAirportRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.88 – Alternate Distance (ALT DIST) ✅
pub type _5_88_AlternateDistanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.89 – Cost Index ✅
pub type _5_89_CostIndexRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.90 – ILS/DME Bias ✅
pub type _5_90_IlsDmeBiasRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.91 – Continuation Record Application Type (APPL) ✅
pub type _5_91_ContinuationRecordApplicationTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.92 – Facility Elevation (FAC ELEV) ✅
pub type _5_92_FacilityElevationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.93 – Facility Characteristics (FAC CHAR) ✅
pub type _5_93_FacilityCharacteristicsRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.94 – True Bearing (TRUE BRG) ✅
pub type _5_94_TrueBearingRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.95 – Government Source (SOURCE) ✅
pub type _5_95_GovernmentSourceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.96 – Glideslope Beam Width (GS BEAM WIDTH) ✅
pub type _5_96_GlideslopeBeamWidthRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.97 – Touchdown Zone Elevation (TDZE) ✅
pub type _5_97_TouchdownZoneElevationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.98 – Elevation Type ✅
pub type _5_98_ElevationTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.99 – Marker Type (MKR TYPE) ✅
pub type _5_99_MarkerTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 3>;
/// 5.100 – Minor Axis True Bearing (MINOR AXIS TRUE BRG) ✅
pub type _5_100_MinorAxisTrueBearingRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.101 – Communications Type (COMM TYPE) ✅
pub type _5_101_CommunicationsTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 3>;
/// 5.102 – Radar ✅
pub type _5_102_RadarRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.103 – Communications Frequency ✅
pub type _5_103_CommunicationsFrequencyRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 7>;
/// 5.104 – Frequency Units ✅
pub type _5_104_FrequencyUnitsRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.105 – Call Sign ✅
pub type _5_105_CallSignRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.106 – Service Indicator (SERV IND) ✅
pub type _5_106_ServiceIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 3>;
/// 5.107 – ATA/IATA Designator ✅
pub type _5_107_AtaIataDesignatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 3>;
/// 5.108 – IFR Capability ✅
pub type _5_108_IfrCapabilityRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.109 – Runway Width ✅
pub type _5_109_RunwayWidthRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.110 – Marker Identifier (Enroute Marker) (IDENT) ✅
pub type _5_110_EnrouteMarkerIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.111 – Marker Code (Morse); spec lists Alpha, Morse encoding often uses dot/dash ✅
pub type _5_111_EnrouteMarkerMorseCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.112 – Marker Shape ✅
pub type _5_112_MarkerShapeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.113 – High/Low (Enroute Marker) ✅
pub type _5_113_EnrouteMarkerHighLowRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.114 – Duplicate Indicator ✅
pub type _5_114_DuplicateIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.115 – Direction Restriction ✅
pub type _5_115_DirectionRestrictionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.116 – FIR/UIR Identifier; spec lists Alpha, examples include digits ✅
pub type _5_116_FirUirIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.117 – FIR/UIR Indicator (IND) ✅
pub type _5_117_FirUirIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.118 – Boundary Via ✅
pub type _5_118_BoundaryViaRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 2>;
/// 5.119 – Arc Distance ✅
pub type _5_119_ArcDistanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.120 – Arc Bearing ✅
pub type _5_120_ArcBearingRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.121 – Lower/Upper Limit ✅
pub type _5_121_FirUirLowerUpperLimitRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.122 – FIR/UIR Reporting Units Speed ✅
pub type _5_122_FirUirRUSRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.123 – FIR/UIR Reporting Units Altitude ✅
pub type _5_123_FirUirRUARaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.124 – FIR/UIR Entry Report (ENTRY) ✅
pub type _5_124_FirUirEntryReportRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.125 – FIR/UIR Name ✅
pub type _5_125_FirUirNameRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.126 – Restrictive Airspace Name ✅
pub type _5_126_RestrictiveAirspaceNameRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 30>;
/// 5.127 – Maximum Altitude ✅
pub type _5_127_MaximumAltitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.128 – Restrictive Airspace Type ✅
pub type _5_128_RestrictiveAirspaceTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.129 – Restrictive Airspace Designation ✅
pub type _5_129_RestrictiveAirspaceDesignationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.130 – Multiple Code (MULTI CD) ✅
pub type _5_130_MultipleCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.131 – Time Code (TIME CD) ✅
pub type _5_131_TimeCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.132 – NOTAM ✅
pub type _5_132_NotamRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.133 – Unit Indicator (UNIT IND) ✅
pub type _5_133_UnitIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.134 – Cruise Table Identifier (CRSE TBL IDENT) ✅
pub type _5_134_CruiseTableIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.135 – Course From/To (Cruise Table) ✅
pub type _5_135_CruiseTableCourseFromToRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.136 – Cruise Level From/To ✅
pub type _5_136_CruiseLevelFromToRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.137 – Vertical Separation ✅
pub type _5_137_VerticalSeparationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.138 – Time Indicator (TIME IND) ✅
pub type _5_138_TimeIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.139 – Procedure Name ✅
pub type _5_139_ProcedureNameRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 78>;
/// 5.140 – Controlling Agency ✅
pub type _5_140_ControllingAgencyRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.141 – Starting Latitude (Grid MORA) ✅
pub type _5_141_GridMoraStartingLatitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.142 – Starting Longitude (Grid MORA) ✅
pub type _5_142_GridMoraStartingLongitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.143 – Grid MORA ✅
pub type _5_143_GridMoraRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.144 – Center Fix (CENTER FIX) (5 characters max) ✅
pub type _5_144_CenterFixRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.145 – Radius Limit (MSA) ✅
pub type _5_145_MsaRadiusLimitRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.146 – Sector Bearing (SEC BRG) ✅
pub type _5_146_SectorBearingRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 6>;
/// 5.147 – Sector Altitude (SEC ALT) ✅
pub type _5_147_SectorAltitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.148 – Enroute Alternate Airport/Heliport (EAA) ✅
pub type _5_148_EnrouteAlternateAirportHeliportRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.149 – Navaid Usable Range (Figure of Merit) ✅
pub type _5_149_NavaidUsableRangeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.150 – Frequency Protection Distance (FREQ PRD) ✅
pub type _5_150_FrequencyProtectionDistanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.151 – FIR/UIR Address (ADDRESS) ✅
pub type _5_151_FirUirAddressRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 4>;
/// 5.154 – Restriction Identifier (REST IDENT) ✅
pub type _5_154_AirwayRestrictionIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.155 – BARO-VNAV Not Authorized ✅
pub type _5_155_BaroVnavNotAuthorizedRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.157 – Airway Restriction StartCOL/End Date (STARTCOL/END DATE) ✅
pub type _5_157_AirwayRestrictionStartCOLEndDateRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 7>;
/// 5.158 – VFR Checkpoint Flag ✅ (Possible error in specification)
pub type _5_158_VfrCheckpointFlagRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.159 – ATC Assigned Only ✅ (Possible error in specification)
pub type _5_159_AtcAssignedOnlyRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.160 – Units of Altitude for airway restriction (UNIT IND); distinct from 5.133 ✅
pub type _5_160_AirwayRestrictionAltitudeUnitRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.161 – Restriction Altitude (RSTR ALT) ✅
pub type _5_161_AirwayRestrictionAltitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.162 – Step Climb Indicator (STEP) ✅
pub type _5_162_StepClimbIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.163 – Restriction Notes ✅
pub type _5_163_AirwayRestrictionNotesRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 104>;
/// 5.164 – EU Indicator (EU IND) ✅
pub type _5_164_EuIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.165 – Magnetic/True Indicator (M/T IND) ✅
pub type _5_165_MagneticTrueIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.166 – Channel (MLS) ✅
pub type _5_166_MLSChannelRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.167 – MLS Azimuth Bearing (MLS AZ BRG) ✅
pub type _5_167_MLSAzimuthBearingRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.168 – MLS Azimuth / Back Azimuth proportional angle (AZ PRO / BAZ PRO RIGHT/LEFT) ✅
pub type _5_168_MLSAzimuthProportionalAngleRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.169 – Elevation Angle Span (EL ANGLE SPAN) ✅
pub type _5_169_MLSElevationAngleSpanRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.172 – Azimuth / Back Azimuth coverage sector (AZ COV / BAZ COV RIGHT/LEFT) ✅
pub type _5_172_MLSAzimuthCoverageSectorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.173 – Nominal Elevation Angle (NOM ELEV ANGLE) ✅
pub type _5_173_MLSNominalElevationAngleRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.174 – Restrictive Airspace Link Continuation (LC) ✅
pub type _5_174_RestrictiveAirspaceLinkContinuationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.175 – Holding Speed (HOLD SPEED) ✅
pub type _5_175_HoldingSpeedRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.176 – Pad Dimensions ✅ (possible error in specification)
pub type _5_176_PadDimensionsRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 8>;
/// 5.177 – Public/Military Indicator (PUB/MIL) ✅
pub type _5_177_PublicMilitaryIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.178 – Time Zone ✅
pub type _5_178_TimeZoneRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.179 – Daylight Time Indicator (DAY TIME) ✅
pub type _5_179_DaylightTimeIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.180 – Pad Identifier (PAD IDENT) (5 characters max) ✅
pub type _5_180_PadIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.181 – H24 Indicator (H24) ✅
pub type _5_181_H24IndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.183 – Sectorization (SECTOR)
pub type _5_183_CommunicationsSectorizationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.184 – Communications Altitude (COMM ALTITUDE); Altitude 1 or 2 column ✅
pub type _5_184_CommunicationsAltitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.185 – Sector Facility (SEC FAC) ✅
pub type _5_185_CommunicationsSectorFacilityRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.186 – Sectorization Narrative ✅
pub type _5_186_CommunicationsSectorizationNarrativeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 60>;
/// 5.187 – Distance Description (DIST DESC) ✅
pub type _5_187_CommunicationsDistanceDescriptionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.188 – Communications Distance (COMM DIST)✅
pub type _5_188_CommunicationsDistanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.189 – Position Narrative ✅
pub type _5_189_CommunicationsPositionNarrativeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 25>;
/// 5.190 – FIR/RDO Identifier (FIR/RDO) ✅
pub type _5_190_FirRdoIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.194 – Initial / Terminus Fix or Airport  ✅
pub type _5_194_InitialTerminusFixOrAirportRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.195 – Time of Operation ✅
pub type _5_195_TimeOfOperationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.196 – Name Format Indicator (NAME IND) ✅
pub type _5_196_WaypointNameFormatIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 3>;
/// 5.197 – Modulation (MODULN) ✅
pub type _5_197_CommunicationsModulationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.198 – Datum Code (DATUM) ✅
pub type _5_198_DatumCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 3>;
/// 5.199 – Signal Emission (SIG EM) ✅
pub type _5_199_SignalEmissionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.200 – Remote Facility (REM FAC) ✅
pub type _5_200_CommunicationsRemoteFacilityRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.201 – Restriction Record Type (REST TYPE) ✅
pub type _5_201_AirwayRestrictionRecordTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 2>;
/// 5.202 – Exclusion Indicator (EXC IND) ✅
pub type _5_202_AirwayRestrictionExclusionIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.203 – Block Indicator (BLOCK IND) ✅
pub type _5_203_AirwayRestrictionBlockIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.204 – ARC Radius (ARC RAD) ✅
pub type _5_204_ArcRadiusRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 6>;
/// 5.205 – Navaid Limitation Code (NLC) ✅
pub type _5_205_NavaidLimitationCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.206 – Component Affected Indicator (COMP AFFTD IND) ✅
pub type _5_206_NavaidLimitationComponentAffectedRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.207 – Sector From / Sector To (SECTR) ✅
pub type _5_207_NavaidLimitationSectorFromToRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 2>;
/// 5.208 – Distance Limitation (DIST LIMIT) ✅
pub type _5_208_NavaidLimitationDistanceLimitRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.209 – Altitude Limitation (ALT LIMIT) ✅
pub type _5_209_NavaidLimitationAltitudeLimitRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.210 – Sequence End Indicator (SEQ END) ✅
pub type _5_210_NavaidLimitationSequenceEndRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.211 – Required Navigation Performance (RNP) ✅
pub type _5_211_RequiredNavigationPerformanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.212 – Runway Gradient (RWY GRAD) ✅
pub type _5_212_RunwayGradientRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.213 – Controlled Airspace Type (ARSP TYPE) ✅
pub type _5_213_ControlledAirspaceTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.214 – Controlled Airspace Center (ARSP CNTR) ✅
pub type _5_214_ControlledAirspaceCenterRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.215 – Controlled Airspace Classification (ARSP CLASS) ✅
pub type _5_215_ControlledAirspaceClassificationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.216 – Controlled Airspace Name (ARSP NAME) ✅
pub type _5_216_ControlledAirspaceNameRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 30>;
/// 5.217 – Controlled Airspace Indicator (CTLD ARSP IND) ✅
pub type _5_217_ControlledAirspaceIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.218 – Geographical Reference Table Identifier (GEO REF TBL ID) ✅
pub type _5_218_GeographicalReferenceTableIdRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.219 – Geographical Entity (GEO ENT) ✅
pub type _5_219_GeographicalEntityRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 29>;
/// 5.220 – Preferred Route Use Indicator (ET IND) ✅
pub type _5_220_PreferredRouteUseIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 2>;
/// 5.221 – Aircraft Use Group (ACFT USE GP) ✅
pub type _5_221_AircraftUseGroupRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 2>;
/// 5.222 – GNSS/FMS Indicator (GNSS/FMS IND) ✅
pub type _5_222_GnssFmsIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.223 – Operation Type (OPS TYPE) ✅
pub type _5_223_OperationTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.224 – Route Indicator (RTE IND) ✅
pub type _5_224_FinalApproachRouteIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.225 – Ellipsoidal Height ✅
pub type _5_225_EllipsoidalHeightRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.226 – Glide Path Angle (GPA) ✅
pub type _5_226_GlidePathAngleRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.227 – Orthometric Height (ORTH HGT) ✅
pub type _5_227_OrthometricHeightRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.228 – Course Width At Threshold (CRS WDTH) ✅
pub type _5_228_CourseWidthAtThresholdRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.229 – Final Approach Segment Data CRC Remainder (FAS CRC) ✅
pub type _5_229_FinalApproachSegmentDataCrcRemainderRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 8>;
/// 5.230 – Procedure Type (PROC TYPE) ✅
pub type _5_230_FlightPlanningProcedureTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.231 – Along Track Distance (ATD) ✅
pub type _5_231_FlightPlanningAlongTrackDistanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.232 – Number of Engines Restriction (NOE) ✅
pub type _5_232_FlightPlanningEnginesRestrictionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 4>;
/// 5.233 – Turboprop/Jet Indicator (TURBO) ✅
pub type _5_233_FlightPlanningTurbopropJetIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.234 – RNAV Flag (RNAV) ✅
pub type _5_234_FlightPlanningRnavFlagRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.235 – ATC Weight Category (ATC WC) ✅
pub type _5_235_FlightPlanningAtcWeightCategoryRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.236 – ATC Identifier (ATC ID) ✅
pub type _5_236_FlightPlanningAtcIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 7>;
/// 5.237 – Procedure Description (PROC DESC) ✅
pub type _5_237_FlightPlanningProcedureDescriptionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 15>;
/// 5.238 – Leg Type Code (LTC) ✅
pub type _5_238_FlightPlanningLegTypeCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 2>;
/// 5.239 – Reporting Code (RPT) ✅
pub type _5_239_FlightPlanningReportingCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.240 – Altitude (ALT) ✅
pub type _5_240_FlightPlanningAltitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.241 – Fix Related Transition Code (FRT Code) ✅
pub type _5_241_FlightPlanningFixRelatedTransitionCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.242 – Procedure Category (PROC CAT) ✅
pub type _5_242_SidStarApproachProcedureCategoryRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 4>;
/// 5.243 – GLS Station Identifier (4 characters max) ✅
pub type _5_243_GlsStationIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 4> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 4, STARTCOL, LEN>;
/// 5.244 – SBAS/GBAS Channel ✅
pub type _5_244_SbasGbasChannelRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.245 – Service Volume Radius ✅
pub type _5_245_GlsServiceVolumeRadiusRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.246 – TDMA Slots ✅
pub type _5_246_GlsTdmaSlotsRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.247 – Station Type ✅
pub type _5_247_GlsStationTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.248 – Station Elevation WGS84 ✅
pub type _5_248_GlsStationElevationWgs84Raw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.249 – Surface Code (SC) ✅
pub type _5_249_SurfaceCodeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.250 – Alternate Record Type (ART) ✅
pub type _5_250_AlternateRecordTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 2>;
/// 5.251 – Distance To Alternate (DTA) (3 characters max) ✅
pub type _5_251_DistanceToAlternateRaw<'a, const STARTCOL: usize, const LEN: usize = 3> =
    FieldRaw<'a, DTYPE_NUMERIC, 3, STARTCOL, LEN>;
/// 5.252 – Alternate Type (ALT TYPE) ✅
pub type _5_252_AlternateTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.253 – Primary and Additional Alternate Identifier (ALT IDENT) (10 characters max) ✅
pub type _5_253_AlternateIdentifierRaw<'a, const STARTCOL: usize, const LEN: usize = 10> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 10, STARTCOL, LEN>;
/// 5.254 – Fixed Radius Transition Indicator (FIXED RAD IND) ✅
pub type _5_254_FixedRadiusTransitionIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.255 – SBAS Service Provider Identifier (SBAS ID) ✅
pub type _5_255_SbasServiceProviderIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.256 – Reference Path Data Selector (REF PDS) ✅
pub type _5_256_ReferencePathDataSelectorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.257 – Reference Path Identifier (REF ID) ✅
pub type _5_257_ReferencePathIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.258 – Approach Performance Designator (APD) ✅
pub type _5_258_ApproachPerformanceDesignatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 1>;
/// 5.259 – Length Offset (OFFSET) ✅
pub type _5_259_PathPointLengthOffsetRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.260 – Terminal Procedure Flight Planning Leg Distance (LEG DIST) ✅
pub type _5_260_TerminalProcedureFlightPlanningLegDistanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.261 – Speed Limit Description (SLD) ✅
pub type _5_261_SpeedLimitDescriptionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.262 – Approach Type Identifier (ATI) ✅
pub type _5_262_ApproachTypeIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.263 – Horizontal Alert Limit (HAL) / Lateral Alert Limit (LAL) ✅
pub type _5_263_HorizontalOrLateralAlertLimitRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.264 – Vertical Alert Limit (VAL) ✅
pub type _5_264_VerticalAlertLimitRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.265 – Path Point TCH ✅
pub type _5_265_PathPointTchRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 6>;
/// 5.266 – TCH Units Indicator ✅
pub type _5_266_TchUnitsIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.267 – High Precision Latitude (HPLAT) ✅
pub type _5_267_HighPrecisionLatitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 11>;
/// 5.268 – High Precision Longitude (HPLONG) ✅
pub type _5_268_HighPrecisionLongitudeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 12>;
/// 5.269 – Helicopter Procedure Course (HPC) ✅
pub type _5_269_HelicopterProcedureCourseRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.270 – TCH Value Indicator (TCHVI) ✅
pub type _5_270_TchValueIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.271 – Procedure Turn (PROC TURN) ✅
pub type _5_271_TaaProcedureTurnRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.272 – TAA Sector Identifier ✅
pub type _5_272_TaaSectorIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.273 – TAA Waypoint (5-character max) ✅
pub type _5_273_TaaWaypointRaw<'a, const STARTCOL: usize, const LEN: usize = 5> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.274 – TAA Sector Radius ✅
pub type _5_274_TaaSectorRadiusRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.275 – Level of Service Name (LSN) ✅
pub type _5_275_LevelOfServiceNameRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 10>;
/// 5.276 – Level of Service Authorized ✅
pub type _5_276_LevelOfServiceAuthorizedRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.277 – DME Operational Service Volume (D-OSV) ✅
pub type _5_277_DmeOperationalServiceVolumeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.278 – Activity Type ✅
pub type _5_278_SpecialActivityAreaTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.279 – Activity Identifier ✅
pub type _5_279_SpecialActivityAreaIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.280 – Special Activity Area Size ✅
pub type _5_280_SpecialActivityAreaSizeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.281 – Special Activity Area Volume ✅
pub type _5_281_SpecialActivityAreaVolumeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.282 – Special Activity Area Operating Times ✅
pub type _5_282_SpecialActivityAreaOperatingTimesRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 3>;
/// 5.283 – Communications Class (Comm Class) ✅
pub type _5_283_CommunicationsClassRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 4>;
/// 5.284 – Assigned Sector Name (ASN) (25 characters max) ✅
pub type _5_284_AssignedSectorNameRaw<'a, const STARTCOL: usize, const LEN: usize = 25> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 25, STARTCOL, LEN>;
/// 5.285 – Time Narrative (100 characters max per record) ✅
pub type _5_285_CommunicationsTimeNarrativeRaw<'a, const STARTCOL: usize, const LEN: usize = 100> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 100, STARTCOL, LEN>;
/// 5.286 – Multi-Sector Indicator (MSEC IND) ✅
pub type _5_286_MultiSectorIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.287 – Type Recognized By (TRB) ✅
pub type _5_287_CommunicationsTypeRecognizedByRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.288 – Translation (80 characters max) ✅
pub type _5_288_CommunicationsTypeTranslationRaw<'a, const STARTCOL: usize, const LEN: usize = 80> =
    FieldRaw<'a, DTYPE_ALPHANUMERIC, 80, STARTCOL, LEN>;
/// 5.289 – Used On ✅
pub type _5_289_CommunicationsTypeTranslationTableUsedOnRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.290 – Procedure Design Mag Var (PDMV) ✅
pub type _5_290_ProcedureDesignMagneticVariationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.291 – Procedure Design Mag Var Indicator (PDMVI) ✅
pub type _5_291_ProcedureDesignMagneticVariationIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.292 – Category Distance ✅
pub type _5_292_CirclingCategoryDistanceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.293 – Vertical Scale Factor (VSF) ✅
pub type _5_293_VerticalScaleFactorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.294 – RVSM Minimum Level ✅
pub type _5_294_RvsmMinimumLevelRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.295 – RVSM Maximum Level ✅
pub type _5_295_RvsmMaximumLevelRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.296 – RNP Level of Service (LSN) ✅
pub type _5_296_RnpApproachLevelOfServiceNameRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.297 – Route Inappropriate Navaid Indicator ✅
pub type _5_297_RouteInappropriateNavaidIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.298 – Holding Pattern/Race Track Course Reversal Leg Inbound/Outbound Indicator ✅
pub type _5_298_HoldingLegInboundOutboundIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.299 – Procedure Referenced Fix Identifier ✅
pub type _5_299_ProcedureReferencedFixIdentifierRaw<
    'a,
    const STARTCOL: usize,
    const LEN: usize = 5,
> = FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.300 – Final Approach Course as Runway ✅
pub type _5_300_FinalApproachCourseAsRunwayRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.301 – Procedure Design Aircraft Category or Type ✅
pub type _5_301_ProcedureDesignAircraftCategoryOrTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.302 – Surface Type ✅
pub type _5_302_RunwaySurfaceTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 4>;
/// 5.303 – Helipad Shape ✅
pub type _5_303_HelipadShapeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.304 – Sector Bearing Reference Waypoint (5 characters max) ✅
pub type _5_304_TaaSectorBearingReferenceWaypointRaw<
    'a,
    const STARTCOL: usize,
    const LEN: usize = 5,
> = FieldRaw<'a, DTYPE_ALPHANUMERIC, 5, STARTCOL, LEN>;
/// 5.305 – Heliport Type ✅
pub type _5_305_HeliportTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.306 – Preferred Multiple Approach Indicator ✅
pub type _5_306_PreferredMultipleApproachIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.307 – Special Indicator ✅
pub type _5_307_TerminalProcedureSpecialIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.308 – Remote Altimeter Flag ✅
pub type _5_308_RemoteAltimeterFlagRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.309 – Maximum Allowable Helicopter Weight ✅
pub type _5_309_MaximumAllowableHelicopterWeightRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 3>;
/// 5.310 – Helicopter Performance Requirement (M/S/U; length 1 per code table) ✅
pub type _5_310_HelipadPerformanceRequirementRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.311 – FIR/FRA Transition ✅
pub type _5_311_FirFraTransitionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.312 – StartCOLer Extension ✅
pub type _5_312_RunwayStartCOLerExtensionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.313 – TORA ✅
pub type _5_313_RunwayToraRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.314 – TODA ✅
pub type _5_314_RunwayTodaRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.315 – ASDA ✅
pub type _5_315_RunwayAsdaRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.316 – LDA ✅
pub type _5_316_RunwayLdaRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 5>;
/// 5.317 – Runway Usage Indicator ✅
pub type _5_317_RunwayUsageIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.318 – Runway Accuracy Compliance Flag ✅
pub type _5_318_RunwayAccuracyComplianceFlagRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.319 – Landing Threshold Elevation Accuracy Compliance Flag ✅
pub type _5_319_LandingThresholdElevationAccuracyComplianceFlagRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.320 – SBAS Final Approach Course ✅
pub type _5_320_SbasFinalApproachCourseRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.321 – Helipad Maximum Rotor Diameter ✅
pub type _5_321_HelipadMaximumRotorDiameterRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 3>;
/// 5.322 – Helipad Type ✅
pub type _5_322_HelipadTypeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.323 – Helipad Orientation ✅
pub type _5_323_HelipadOrientationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.324 – Helipad Identifier Orientation ✅
pub type _5_324_HelipadIdentifierOrientationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
/// 5.325 – Preferred Approach Bearing ✅
pub type _5_325_PreferredApproachBearingRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.326 – Ground Facility Identifier ✅
pub type _5_326_AtnGroundFacilityIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 8>;
/// 5.327 – Authority Format Identifier (AFI) ✅
pub type _5_327_AtnAuthorityFormatIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 2>;
/// 5.328 – Initial Domain Identifier ✅
pub type _5_328_AtnInitialDomainIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_NUMERIC, STARTCOL, 4>;
/// 5.329 – Version (VER) ✅
pub type _5_329_AtnVersionRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.330 – Administration (ADM) ✅
pub type _5_330_AtnAdministrationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.331 – Routing Domain Format (RDF) ✅
pub type _5_331_AtnRoutingDomainFormatRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.332 – Administrative Region Selector (ARS) ✅
pub type _5_332_AtnAdministrativeRegionSelectorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 6>;
/// 5.333 – Location (LOC) (NSAP routing location subfield) ✅
pub type _5_333_AtnRoutingLocationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.334 – System Identifier (SYS) ✅
pub type _5_334_AtnSystemIdentifierRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 12>;
/// 5.335 – Network Service Access Point Selector (NSEL) ✅
pub type _5_335_AtnNetworkServiceAccessPointSelectorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 2>;
/// 5.336 – Context Management Transport Selector (CM TSEL) ✅
pub type _5_336_AtnContextManagementTransportSelectorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 4>;
/// 5.337 – Use Indicator (ATN ATSU ground facility) ✅
pub type _5_337_AtnGroundFacilityUseIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 1>;
/// 5.338 – VOR Range/Power (VORPWR) ✅
pub type _5_338_VhfNavaidVorRangePowerRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.339 – DME Expanded Service Volume (DESV) ✅
pub type _5_339_VhfNavaidDmeExpandedServiceVolumeRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.340 – Unmanned Aerial Vehicle (UAV) Only ✅ (possibly error in specification)
pub type _5_340_UnmannedAerialVehicleOperationsOnlyRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.341 – Military Indicator (terminal SID/STAR/APP) ✅
pub type _5_341_TerminalProcedureMilitaryIndicatorRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.342 – Source of LAL/VAL ✅
pub type _5_342_SbasApproachMinimaLalValSourceRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHA, STARTCOL, 1>;
/// 5.343 – Holding Pattern Magnetic Variation (HPMV) ✅
pub type _5_343_HoldingPatternMagneticVariationRaw<'a, const STARTCOL: usize> =
    FixedLengthFieldRaw<'a, DTYPE_ALPHANUMERIC, STARTCOL, 5>;
