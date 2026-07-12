//! # ARINC 424 Definitions - Identifiers
//! This module contains the identifiers for the ARINC 424 data.
//! Identifiers are simple alphanumeric string fields. The fields may or may not be constrained to be exactly the specified length always.
//! Conversely, some fields are minimized as needed in the spec to accomodate record layouts.
//!
//! Example is 5.6 - Airport/Heliport Identifier which describes a 4-character code that identifies an airport or heliport.

use crate::parsers::arinc424::types::fields::*;

/// 5.5 Generic subsection code
///
/// Note: This is necessary because some records need to validate across several sets of valid subsection codes.
/// For most cases you should validate against the specific subsection code enum for the appropriate record section instead.
pub type GenericSubsection = LengthLimitedIdentifier<1, true>;

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

/// 5.16 Continuation Record Number
pub type ContinuationRecordNumber = LengthLimitedIdentifier<1, true>;

/// 5.17 Waypoint Description Code
/// Note: This should be a complex type, but the rules are a bit difficult to parse out atm
/// TODO: come back to this
pub type WaypointDescriptionCode = LengthLimitedIdentifier<4, true>;

/// 5.21 Path and Termination
pub type PathAndTermination = LengthLimitedIdentifier<2, true>;

/// 5.23 Recommended NAVAID
pub type RecommendedNavaid = LengthLimitedIdentifier<4, false>;

/// 5.33 VOR/NDB Identifier
pub type VORNDBIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.38 DME Identifier
pub type DMEIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.41 Region Code
pub type RegionCode = LengthLimitedIdentifier<4, false>;

/// 5.43 Waypoint Name/Description (NAME/DESC)
pub type WaypointNameDescription = LengthLimitedIdentifier<25, false>;

/// 5.44 Localizer/MLS/GLS Identifier (LOC, MLS, GLS IDENT)
pub type LocalizerMlsGlsIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.46 Runway Identifier (RUNWAY ID)
pub type RunwayIdentifier = LengthLimitedIdentifier<5, false>;

/// 5.56 Gate Identifier (GATE IDENT)
pub type GateIdentifier = LengthLimitedIdentifier<5, false>;

/// 5.59 Runway Description (RUNWAY DESCRIPTION)
pub type RunwayDescription = LengthLimitedIdentifier<22, false>;

/// 5.60 Name (NAME), Gate and Holding Pattern records
pub type Name = LengthLimitedIdentifier<25, false>;

/// 5.61 Notes (Continuation Records) (NOTES)
pub type Notes = LengthLimitedIdentifier<70, false>;

/// 5.71 Name Field, Navaid/Airport/Heliport/Enroute Marker records
pub type NameOfFacility = LengthLimitedIdentifier<30, false>;

/// 5.75 From/To Airport/Heliport/Fix (FROM/TO AIRPORT/HELIPORT/FIX)
///
/// Note: This warrants some change from the spec, because what this is actually referring to is the origin/end point that
/// the route builds between. Suggestion: TerminusAirportHeliportFix
pub type FromToAirportHeliportFix = LengthLimitedIdentifier<5, false>;

/// 5.76 Company Route Ident
pub type CompanyRouteIdent = LengthLimitedIdentifier<10, false>;

/// 5.78 SID/STAR/App/AWY (S/S/A/AWY), SID/STAR/AWY (S/S/AWY)
pub type SidStarApproachAirwayIdentifier = LengthLimitedIdentifier<6, false>;

/// 5.83(A) To Fix for Company Routes
///
/// Note: This warrants some change from the spec, because what this is actually referring to is the fix that is the next
/// immediate point within the route that the specific record refers to. Suggestion: NextFix
pub type CompanyRouteToFix = LengthLimitedIdentifier<6, false>;

/// 5.83(B) To Fix for Preferred Routes
///
/// Note: This warrants some change from the spec, because what this is actually referring to is the fix that is the next
/// immediate point within the route that the specific record refers to. Suggestion: NextFix
pub type PreferredRouteToFix = LengthLimitedIdentifier<5, false>;

/// 5.84 Runway Transition
pub type RunwayTransition = LengthLimitedIdentifier<5, false>;

/// 5.85 Enrt Transition
pub type EnrouteTransition = LengthLimitedIdentifier<5, false>;

/// 5.87 Terminal/Alternate Airport
pub type TerminalAlternateAirport = LengthLimitedIdentifier<4, false>;

/// 5.105 Callsign
pub type Callsign = LengthLimitedIdentifier<25, false>;

/// 5.107 ATA/IATA Designator
pub type AtaIataDesignator = LengthLimitedIdentifier<3, true>;

/// 5.110 Marker Identifier (Enroute Marker) (IDENT)
pub type MarkerIdentifier = LengthLimitedIdentifier<4, true>;

/// 5.111 Marker Code (Morse)
pub type MarkerCode = LengthLimitedIdentifier<4, true>;

/// 5.116 FIR/UIR Identifier
pub type FirUirIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.125 FIR/UIR Name
pub type FirUirName = LengthLimitedIdentifier<25, false>;

/// 5.126 Restrictive Airspace Name
pub type RestrictiveAirspaceName = LengthLimitedIdentifier<30, false>;

/// 5.129 Restrictive Airspace Designation
pub type RestrictiveAirspaceDesignation = LengthLimitedIdentifier<10, false>;

/// 5.130 Multiple Code
pub type MultipleCode = LengthLimitedIdentifier<1, true>;

/// 5.140 Controlling Agency
pub type ControllingAgency = LengthLimitedIdentifier<25, false>;

/// 5.141 MORA Starting Latitude
pub type MoraStartingLatitude = LengthLimitedIdentifier<3, false>;

/// 5.142 MORA Starting Longitude
pub type MoraStartingLongitude = LengthLimitedIdentifier<4, false>;

/// 5.143 Grid MORA
#[derive(Debug, PartialEq, Eq)]
pub enum GridMora {
    Mapped(LengthLimitedIdentifier<3, false>),
    UnknownOrUnmapped,
}

impl ParseableField for GridMora {
    fn from_bytes(bytes: &[u8]) -> Result<Option<Self>, FieldParseError> {
        if bytes.trim_ascii_end().is_empty() {
            return Ok(None);
        }
        Ok(Some(match bytes {
            b"UNK" => GridMora::UnknownOrUnmapped,
            _ => {
                if let Ok(Some(value)) = LengthLimitedIdentifier::<3, false>::from_bytes(bytes) {
                    GridMora::Mapped(value)
                } else {
                    return Err(FieldParseError::new("Invalid grid MORA".to_string()));
                }
            }
        }))
    }
}

/// 5.144 Center Fix
pub type CenterFix = LengthLimitedIdentifier<5, false>;

/// 5.148 Enroute Alternate Airport/Heliport
pub type EnrouteAlternateAirportHeliport = LengthLimitedIdentifier<4, false>;

/// 5.151 FIR/UIR Address
pub type FirUirAddress = LengthLimitedIdentifier<4, true>;

/// 5.154 Airway Restriction Identifier
///
/// Note: This is numeric only, but is an identifier nonetheless.
pub type AirwayRestrictionIdentifier = LengthLimitedIdentifier<3, true>;

/// 5.157 Airway Restriction Start/End Date
pub type AirwayRestrictionStartEndDate = LengthLimitedIdentifier<7, false>;

/// 5.163 Airway Restriction Notes
pub type AirwayRestrictionNotes = LengthLimitedIdentifier<104, false>;

/// 5.180 Pad Identifier
pub type PadIdentifier = LengthLimitedIdentifier<5, false>;

/// 5.185 Sector Facility
pub type SectorFacility = LengthLimitedIdentifier<4, false>;

/// 5.186 Sectorization Narrative
pub type SectorizationNarrative = LengthLimitedIdentifier<60, false>;

/// 5.189 Remote Site Name
pub type RemoteSiteName = LengthLimitedIdentifier<25, false>;

/// 5.190 FIR/RDO Identifier
pub type FirRdoIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.194 Initial/Terminus Fix or Airport
pub type InitialTerminusFixOrAirport = LengthLimitedIdentifier<5, false>;

/// 5.195 Time of Operation
///
/// This should be a complex type but man I do not want to touch this right now.
/// The mix of sunrise/sunset logic with the times is hellish.
pub type TimeOfOperation = LengthLimitedIdentifier<10, false>;

/// 5.197 Datum Code
///
/// Validation left to user for now. Refer to Attachment 2 in ARINC 424
pub type DatumCode = LengthLimitedIdentifier<3, true>;

/// 5.200 Remote Facility
pub type RemoteFacility = LengthLimitedIdentifier<4, false>;

/// 5.214 Controlled Airspace Center
pub type ControlledAirspaceCenter = LengthLimitedIdentifier<5, false>;

/// 5.215 Controlled Airspace Classification
///
/// Note: This is differentiated from the ControlledAirspaceType enum as it is a value local to specific
/// governments independent of this standard.
pub type ControlledAirspaceClassification = LengthLimitedIdentifier<1, true>;

/// 5.216 Controlled Airspace Name
pub type ControlledAirspaceName = LengthLimitedIdentifier<30, false>;

/// 5.218 Geographical Reference Table Identifier
pub type GeographicalReferenceTableIdentifier = LengthLimitedIdentifier<2, false>;

/// 5.219 Geographical Entity
pub type GeographicalEntity = LengthLimitedIdentifier<29, false>;

/// 5.224 Route Indicator
pub type RouteIndicator = LengthLimitedIdentifier<1, true>;

/// 5.229 Final Approach Segment Data CRC Remainder
pub type FinalApproachSegmentDataCrcRemainder = LengthLimitedIdentifier<8, true>;

/// 5.236 ATC Identifier
pub type AtcIdentifier = LengthLimitedIdentifier<7, false>;

/// 5.237 Procedure Description
pub type ProcedureDescription = LengthLimitedIdentifier<15, false>;

/// 5.243 GLS Station Identifier
pub type GlsStationIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.246 GLS TDMA Slots
pub type GlsTdmaSlots = LengthLimitedIdentifier<2, false>;

/// 5.253 Primary and Additional Alternate Identifier
pub type PrimaryAndAdditionalAlternateIdentifier = LengthLimitedIdentifier<10, false>;

/// 5.257 Reference Path Identifier
pub type ReferencePathIdentifier = LengthLimitedIdentifier<4, false>;

/// 5.262 Approach Type Identifier
pub type ApproachTypeIdentifier = LengthLimitedIdentifier<10, false>;

/// 5.273 TAA Waypoint
pub type TaaWaypoint = LengthLimitedIdentifier<5, false>;

/// 5.275 Level of Service Name
pub type LevelOfServiceName = LengthLimitedIdentifier<10, false>;
