//! # ARINC 424 Definitions - Identifiers
//! This module contains the identifiers for the ARINC 424 data.
//! Identifiers are simple alphanumeric string fields. The fields may or may not be constrained to be exactly the specified length always.
//! Conversely, some fields are minimized as needed in the spec to accomodate record layouts.
//!
//! Example is 5.6 - Airport/Heliport Identifier which describes a 4-character code that identifies an airport or heliport.

use crate::parsers::arinc424::fields::FieldParseError;

fn validate_alphanumeric(bytes: &[u8]) -> Result<(), FieldParseError> {
    if !bytes
        .iter()
        .all(|&b| b.is_ascii_alphanumeric() || b == b' ')
    {
        return Err(FieldParseError {
            message: "Alphanumeric identifier is not alphanumeric".to_string(),
        });
    }
    Ok(())
}

#[test]
fn test_validate_alphanumeric() {
    assert!(validate_alphanumeric(b"ABC123").is_ok());
    assert!(validate_alphanumeric(b"ABC 123").is_ok());
    assert!(validate_alphanumeric(b"ABC\x00123").is_err());
}

#[derive(Debug, PartialEq, Eq)]
pub struct LengthLimitedIdentifier<const LEN: usize, const EXACT: bool>(Box<str>);
impl<const LEN: usize, const EXACT: bool> LengthLimitedIdentifier<LEN, EXACT> {
    pub fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        validate_alphanumeric(bytes)?;
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        if EXACT && bytes.len() != LEN {
            return Err(FieldParseError {
                message: "Identifier is not the exact length".to_string(),
            });
        } else if !EXACT && bytes.len() > LEN {
            return Err(FieldParseError {
                message: "Identifier is too long".to_string(),
            });
        }
        Ok(Some(LengthLimitedIdentifier::<LEN, EXACT>(Box::from(
            std::str::from_utf8(bytes)
                .map_err(|e| FieldParseError {
                    message: format!("Identifier is not valid UTF-8: {}", e),
                })?
                .trim_end(),
        ))))
    }
}

/// 5.6 Airport/Heliport Identifier
pub type AirportHeliportIdentifier = LengthLimitedIdentifier<4, false>;

// 5.8 Route Identifiers
/// 5.8(A) Enroute Route Identifier
pub type EnrouteRouteIdentifier = LengthLimitedIdentifier<5, false>;
/// 5.8(B) Preferred Route Identifier
pub type PreferredRouteIdentifier = LengthLimitedIdentifier<10, false>;

/// 5.9
pub type SidStarRouteIdentifier = LengthLimitedIdentifier<6, false>;

/// 5.10 Approach Route Identifier
pub type ApproachRouteIdentifier = LengthLimitedIdentifier<6, false>;

/// 5.11 Transition Identifier
pub type TransitionIdentifier = LengthLimitedIdentifier<5, false>;

/// 5.13 Fix Identifier
pub type FixIdentifier = LengthLimitedIdentifier<5, false>;

/// 5.14 ICAO Code
pub type IcaoCode = LengthLimitedIdentifier<2, false>;

// 5.15 Inbound Course Theta
pub type InboundCourseTheta = LengthLimitedIdentifier<3, true>;

// 5.16 Continuation Record Number
pub type ContinuationRecordNumber = LengthLimitedIdentifier<1, true>;

// 5.17 Waypoint Description Code
pub type WaypointDescriptionCode = LengthLimitedIdentifier<4, true>;

// 5.21 Path and Termination
pub type PathAndTermination = LengthLimitedIdentifier<2, true>;

// 5.23 Recommended NAVAID
pub type RecommendedNavaid = LengthLimitedIdentifier<4, false>;

/// 5.33 VOR/NDB Identifier
pub type VORNDBIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.38 DME Identifier
pub type DMEIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.40 Region Code
pub type RegionCode = LengthLimitedIdentifier<4, false>;
