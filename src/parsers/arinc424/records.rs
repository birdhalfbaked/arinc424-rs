//! # ARINC 424 Records
//! This module contains the record types for the ARINC 424 data.
//! Records are the top-level container for the ARINC 424 data.
//! They are used to group together related data into a single entity.
//!

use crate::parsers::arinc424::definitions::*;
use crate::parsers::arinc424::fields::{BLANK, FieldParseError, ParseableField};
#[derive(Debug)]
pub struct RecordParseError {
    pub message: String,
}
impl From<FieldParseError> for RecordParseError {
    fn from(error: FieldParseError) -> Self {
        Self {
            message: format!("Record parse error: {}", error.message),
        }
    }
}

#[derive(Debug)]
pub struct RecordField<'a, T> {
    pub raw_bytes: &'a [u8],
    pub value: Option<T>,
}
impl<'a, T: ParseableField> RecordField<'a, T> {
    pub fn from_bytes(
        input: &'a [u8],
        column: usize,
        length: usize,
    ) -> Result<Self, FieldParseError> {
        // to make it 1:1 with the spec, let's use 1-indexed columns
        let value = T::from_bytes(&input[column - 1..column - 1 + length])?;
        Ok(Self {
            raw_bytes: &input[column - 1..column - 1 + length],
            value,
        })
    }
}

/// Placeholder field is used to represent a field that is not yet implemented.
#[deprecated(note = "Replace this field with the appropriate record field type")]
#[derive(Debug)]
pub struct PlaceholderField {}
impl PlaceholderField {
    pub fn from_bytes(_bytes: &[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {})
    }
}

/// ARINC 424 Record Sum Type for all possible record types.
#[derive(Debug)]
pub enum ARINCRecord<'a> {
    VHFNavaidPrimary(VHFNavaidPrimaryRecord<'a>),
    VHFNavaidContinuation(VHFNavaidContinuationRecord<'a>),
    VHFNavaidSimulationContinuation(VHFNavaidSimulationContinuationRecord<'a>),
    VHFNavaidFlightPlanningContinuation(VHFNavaidFlightPlanningContinuationRecord<'a>),
    VHFNavaidLimitationContinuation(VHFNavaidLimitationContinuationRecord<'a>),

    NDBNavaidPrimary,
    NDBNavaidContinuation,
    NDBNavaidSimulationContinuation,
    NDBNavaidFlightPlanningContinuation,
    NDBNavaidLimitationContinuation,

    WaypointPrimary,
    WaypointContinuation,
    WaypointSimulationContinuation,

    HoldingPatternPrimary,
    HoldingPatternContinuation,
    HoldingPatternPrimaryExtensionContinuation,

    EnrouteAirwaysPrimary,
    EnrouteAirwaysContinuation,
    EnrouteAirwaysFlightPlanningContinuation,

    AirportPrimary,
    AirportContinuation,
    AirportFlightPlanningContinuation,

    AirportGatePrimary,
    AirportGateContinuation,

    AirportSIDSTARApproachPrimary,
    AirportSIDSTARApproachPrimaryExtensionContinuation,
    AirportSIDSTARApproachFlightPlanningContinuation,
    AirportProcedureDataContinuation,
    AirportSIDSTARApproachProcedureNameContinuation,

    RunwayPrimary,
    RunwayContinuation,
    RunwaySimulationContinuation,

    LocalizerGlideslopePrimary,
    LocalizerGlideslopeContinuation,
    LocalizerGlideslopeSimulationContinuation,

    CompanyRoutePrimary,

    LocalizerMarkerPrimary,
    LocalizerMarkerContinuation,

    AirportCommsPrimary,
    AirportCommsPrimaryExtensionContinuation,
    AirportCommsSectorNarrativeContinuation,
    AirportCommsFormattedTimeContinuation,
    AirportCommsNarrativeTimeContinuation,
    AirportCommsAdditionalSectorizationContinuation,

    AirwaysMarkerPrimary,
    AirwaysMarkerContinuation,

    CruisingTablePrimary,

    FIRUIRPrimary,
    FIRUIRContinuation,

    RestrictiveAirspacePrimary,
    RestrictiveAirspaceFormattedTimeContinuation,
    RestrictiveAirspaceNarrativeTimeContinuation,

    GridMORAPrimary,

    AirportMSAPrimary,
    AirportMSAPrimaryExtensionContinuation,
    AirportMSAContinuation,

    EnrouteAirwayRestrictionAltitudePrimary,
    EnrouteAirwayRestrictionAltitudeContinuation,
    EnrouteAirwayRestrictionFormattedTimeContinuation,
    EnrouteAirwayRestrictionNarrativeTimeContinuation,

    EnrouteAirwayRestrictionNoteRestrictionPrimary,
    EnrouteAirwayRestrictionNoteRestrictionContinuation,

    EnrouteAirwayRestrictionSeasonalClosurePrimary,
    EnrouteAirwayRestrictionSeasonalClosureFormattedTimeContinuation,
    EnrouteAirwayRestrictionSeasonalClosureNarrativeTimeContinuation,

    EnrouteAirwayRestrictionCruisingTableReplacementPrimary,
    EnrouteAirwayRestrictionCruisingTableReplacementFormattedTimeContinuation,
    EnrouteAirwayRestrictionCruisingTableReplacementNarrativeTimeContinuation,

    MLSPrimary,
    MLSContinuation,

    EnrouteCommsPrimary,
    EnrouteCommsPrimaryExtensionContinuation,
    EnrouteCommsFormattedTimeContinuation,
    EnrouteCommsNarrativeTimeContinuation,

    PreferredRoutePrimary,
    PreferredRouteFormattedTimeContinuation,
    PreferredRouteNotesContinuation,
    PreferredRouteNarrativeTimeContinuation,

    ControlledAirspacePrimary,
    ControlledAirspaceFormattedTimeContinuation,
    ControlledAirspacePrimaryExtension,
    ControlledAirspaceNarrativeTimeContinuation,
    ControlledAirspaceControllingAgencyContinuation,

    GeographicalReferenceTablePrimary,
    GeographicalReferenceTableContinuation,

    FlightPlanningPrimary,
    FlightPlanningPrimaryExtensionContinuation,
    FlightPlanningFormattedTimeContinuation,
    FlightPlanningNarrativeTimeContinuation,

    SBASPathPointPrimary,
    SBASPathPointContinuation,

    GLSPrimary,
    GLSContinuation,

    AlternatePrimary,

    TAAPrimary,
    TAAContinuation,

    TACANOnlyNavaidPrimary,
    TACANOnlyNavaidContinuation,
    TACANOnlyNavaidSimulationContinuation,
    TACANOnlyNavaidFlightPlanningContinuation,
    TACANOnlyNavaidLimitationContinuation,

    SpecialActivityAreaPrimary,

    CommunicationTypeTranslationPrimary,

    GBASPathPointPrimary,
    GBASPathPointContinuation,

    AirportHelipadPrimary,

    ATNDataPrimary,

    HeliportPrimary,
    HeliportContinuation,
    HeliportFlightPlanningContinuation,

    HeliportTerminalWaypointPrimary,
    HeliportTerminalWaypointContinuation,
    HeliportTerminalWaypointFlightPlanningContinuation,

    HeliportSIDSTARApproachPrimary,
    HeliportSIDSTARApproachPrimaryExtensionContinuation,
    HeliportSIDSTARApproachFlightPlanningContinuation,
    HeliportProcedureDataContinuation,
    HeliportSIDSTARApproachProcedureNameContinuation,

    HeliportMSAPrimary,
    HelportMSAPrimaryExtensionContinuation,
    HeliportMSAContinuation,

    HeliportCommsPrimary,
    HeliportCommsPrimaryExtensionContinuation,
    HeliportCommsSectorNarrativeContinuation,
    HeliportCommsFormattedTimeContinuation,
    HeliportCommsNarrativeTimeContinuation,
    HeliportCommsAdditionalSectorizationContinuation,

    HeliportTAAPrimary,
    HeliportTAAContinuation,

    HelicopterCompanyRoutePrimary,

    HelicopterSBASPathPointPrimary,
    HelicopterSBASPathPointContinuation,

    HeliportHelipadPrimary,
}

impl<'a> ARINCRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        match input[4..6] {
            [b'D', BLANK] => VHFNavaidRecords::parse(input),
            _ => Err(RecordParseError {
                message: "Invalid record type".to_string(),
            }),
        }
    }
}

// Layout dispatch helpers

fn is_primary_record(input: &[u8], continuation_column: usize) -> bool {
    matches!(input[continuation_column - 1], b'0' | b'1' | BLANK)
}

struct VHFNavaidRecords;
impl VHFNavaidRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::VHFNavaidPrimary(
                VHFNavaidPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN
                    ..Self::CONTINUATION_APPLICATION_COLUMN + 1],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => Ok(
                    ARINCRecord::VHFNavaidContinuation(VHFNavaidContinuationRecord::parse(input)?),
                ),
                Some(ContinuationRecordApplicationType::FlightPlanningApplicationContinuation) => {
                    Ok(ARINCRecord::VHFNavaidFlightPlanningContinuation(
                        VHFNavaidFlightPlanningContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::VHFNavaidTACANOnlyNavaidLimitationContinuation) => {
                    Ok(ARINCRecord::VHFNavaidLimitationContinuation(
                        VHFNavaidLimitationContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::SimulationApplicationContinuation) => {
                    Ok(ARINCRecord::VHFNavaidSimulationContinuation(
                        VHFNavaidSimulationContinuationRecord::parse(input)?,
                    ))
                }
                _ => {
                    return Err(RecordParseError {
                        message: "Invalid continuation record application type".to_string(),
                    });
                }
            }
        }
    }
}

/// 4.1.2.1 VHFNavaid Primary Record
#[derive(Debug)]
pub struct VHFNavaidPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub vor_identifier: RecordField<'a, VORNDBIdentifier>,
    pub vor_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub vor_frequency: RecordField<'a, VORNDBFrequency>,
    pub navaid_class: RecordField<'a, VHFNavaidClass>,
    pub vor_latitude: RecordField<'a, Latitude>,
    pub vor_longitude: RecordField<'a, Longitude>,
    pub dme_identifier: RecordField<'a, DMEIdentifier>,
    pub dme_latitude: RecordField<'a, Latitude>,
    pub dme_longitude: RecordField<'a, Longitude>,
    pub station_declination: RecordField<'a, DeclinationNumeric>,
    pub dme_elevation: RecordField<'a, DMEElevation>,
    pub navaid_usable_range: RecordField<'a, NavaidUsableRange>,
    pub ils_dme_bias: RecordField<'a, IlsDmeBias>,
    pub frequency_protection: RecordField<'a, FrequencyProtectionDistance>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub vor_name: RecordField<'a, NameOfFacility>,
    pub vfr_checkpoint_flag: RecordField<'a, VFRCheckpointFlag>,
    pub vor_range_power: RecordField<'a, VHFNavaidVorRangePower>,
    pub expanded_dme_service_volume: RecordField<'a, DMEExpandedServiceVolume>,
    pub route_inappropriate_dme: RecordField<'a, RouteInappropriateNavaidIndicator>,
    pub dme_operational_service_volume: RecordField<'a, DMEOperationalServiceVolume>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> VHFNavaidPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
                record_type:                    RecordField::from_bytes(input, 1, 1)?,
                customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
                section:                        RecordField::from_bytes(input, 5, 1)?,
                subsection:                     RecordField::from_bytes(input, 6, 1)?,
                airport_icao_identifier:        RecordField::from_bytes(input, 7, 4)?,
                airport_icao_code:              RecordField::from_bytes(input, 11, 2)?,
                vor_identifier:                 RecordField::from_bytes(input, 14, 4)?,
                vor_icao_code:                  RecordField::from_bytes(input, 20, 2)?,
                continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
                vor_frequency:                  RecordField::from_bytes(input, 23, 5)?,
                navaid_class:                   RecordField::from_bytes(input, 28, 5)?,
                vor_latitude:                   RecordField::from_bytes(input, 33, 9)?,
                vor_longitude:                  RecordField::from_bytes(input, 42, 10)?,
                dme_identifier:                 RecordField::from_bytes(input, 52, 4)?,
                dme_latitude:                   RecordField::from_bytes(input, 56, 9)?,
                dme_longitude:                  RecordField::from_bytes(input, 65, 10)?,
                station_declination:            RecordField::from_bytes(input, 75, 5)?,
                dme_elevation:                  RecordField::from_bytes(input, 80, 5)?,
                navaid_usable_range:            RecordField::from_bytes(input, 85, 1)?,
                ils_dme_bias:                   RecordField::from_bytes(input, 86, 2)?,
                frequency_protection:           RecordField::from_bytes(input, 88, 3)?,
                datum_code:                     RecordField::from_bytes(input, 91, 3)?,
                vor_name:                       RecordField::from_bytes(input, 94, 25)?,
                vfr_checkpoint_flag:            RecordField::from_bytes(input, 119, 1)?,
                vor_range_power:                RecordField::from_bytes(input, 120, 1)?,
                expanded_dme_service_volume:    RecordField::from_bytes(input, 121, 1)?,
                route_inappropriate_dme:        RecordField::from_bytes(input, 122, 1)?,
                dme_operational_service_volume: RecordField::from_bytes(input, 123, 1)?,
                file_record_number:             RecordField::from_bytes(input, 124, 5)?,
                cycle_date:                     RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.2.2 VHFNavaid Continuation Record
#[derive(Debug)]
pub struct VHFNavaidContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub vor_identifier: RecordField<'a, VORNDBIdentifier>,
    pub vor_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> VHFNavaidContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:         RecordField::from_bytes(input, 2, 3)?,
            section:                    RecordField::from_bytes(input, 5, 1)?,
            subsection:                 RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:    RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:          RecordField::from_bytes(input, 11, 2)?,
            vor_identifier:             RecordField::from_bytes(input, 14, 4)?,
            vor_icao_code:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number: RecordField::from_bytes(input, 22, 1)?,
            application_type:           RecordField::from_bytes(input, 23, 1)?,
            notes:                      RecordField::from_bytes(input, 24, 69)?,
            file_record_number:         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.2.3 VHFNavaid Simulation Continuation Record
#[derive(Debug)]
pub struct VHFNavaidSimulationContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub vor_identifier: RecordField<'a, VORNDBIdentifier>,
    pub vor_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub facility_characteristics: RecordField<'a, FacilityCharacteristics>,
    pub magnetic_variation: RecordField<'a, MagneticVariationNumeric>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> VHFNavaidSimulationContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:         RecordField::from_bytes(input, 2, 3)?,
            section:                    RecordField::from_bytes(input, 5, 1)?,
            subsection:                 RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:    RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:          RecordField::from_bytes(input, 11, 2)?,
            vor_identifier:             RecordField::from_bytes(input, 14, 4)?,
            vor_icao_code:              RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number: RecordField::from_bytes(input, 22, 1)?,
            application_type:           RecordField::from_bytes(input, 23, 1)?,
            facility_characteristics:   RecordField::from_bytes(input, 28, 5)?,
            magnetic_variation:         RecordField::from_bytes(input, 75, 5)?,
            facility_elevation:         RecordField::from_bytes(input, 80, 5)?,
            file_record_number:         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.2.4 VHFNavaid Flight Planning Continuation Record
#[derive(Debug)]
pub struct VHFNavaidFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub vor_identifier: RecordField<'a, VORNDBIdentifier>,
    pub vor_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub fir_identifier: RecordField<'a, FirUirIdentifier>,
    pub uir_identifier: RecordField<'a, FirUirIdentifier>,
    pub fir_fra_entry_point: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_exit_point: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_arrival_transition: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_departure_transition: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_intermediate_point: RecordField<'a, FIRFRATransitionType>,
    pub fir_fra_terminal_holding_point: RecordField<'a, FIRFRATransitionType>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> VHFNavaidFlightPlanningContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                    RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:             RecordField::from_bytes(input, 2, 3)?,
            section:                        RecordField::from_bytes(input, 5, 1)?,
            subsection:                     RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:        RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:              RecordField::from_bytes(input, 11, 2)?,
            vor_identifier:                 RecordField::from_bytes(input, 14, 4)?,
            vor_icao_code:                  RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:     RecordField::from_bytes(input, 22, 1)?,
            application_type:               RecordField::from_bytes(input, 23, 1)?,
            fir_identifier:                 RecordField::from_bytes(input, 24, 4)?,
            uir_identifier:                 RecordField::from_bytes(input, 28, 4)?,
            fir_fra_entry_point:            RecordField::from_bytes(input, 44, 1)?,
            fir_fra_exit_point:             RecordField::from_bytes(input, 45, 1)?,
            fir_fra_arrival_transition:     RecordField::from_bytes(input, 46, 1)?,
            fir_fra_departure_transition:   RecordField::from_bytes(input, 47, 1)?,
            fir_fra_intermediate_point:     RecordField::from_bytes(input, 48, 1)?,
            fir_fra_terminal_holding_point: RecordField::from_bytes(input, 49, 1)?,
            file_record_number:             RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                     RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.2.5 VHFNavaid Limitation Continuation Record
#[derive(Debug)]
pub struct VHFNavaidLimitationContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, NavaidSubsection>,
    pub airport_icao_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub vor_identifier: RecordField<'a, VORNDBIdentifier>,
    pub vor_icao_code: RecordField<'a, IcaoCode>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub navaid_limitation_code: RecordField<'a, NavaidLimitationCode>,
    pub component_affected_indicator: RecordField<'a, ComponentAffectedIndicator>,
    pub sequence_number: RecordField<'a, SequenceNumber>,
    // limitation block 1
    pub limitation_1_sector_from_sector_to: RecordField<'a, SectorFromTo>,
    pub limitation_1_distance_description: RecordField<'a, DistanceDescription>,
    pub limitation_1_distance_limit: RecordField<'a, NavaidDistanceLimitation>,
    pub limitation_1_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub limitation_1_altitude_limit: RecordField<'a, NavaidAltitudeLimitation>,
    // limitation block 2
    pub limitation_2_sector_from_sector_to: RecordField<'a, SectorFromTo>,
    pub limitation_2_distance_description: RecordField<'a, DistanceDescription>,
    pub limitation_2_distance_limit: RecordField<'a, NavaidDistanceLimitation>,
    pub limitation_2_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub limitation_2_altitude_limit: RecordField<'a, NavaidAltitudeLimitation>,
    // limitation block 3
    pub limitation_3_sector_from_sector_to: RecordField<'a, SectorFromTo>,
    pub limitation_3_distance_description: RecordField<'a, DistanceDescription>,
    pub limitation_3_distance_limit: RecordField<'a, NavaidDistanceLimitation>,
    pub limitation_3_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub limitation_3_altitude_limit: RecordField<'a, NavaidAltitudeLimitation>,
    // limitation block 4
    pub limitation_4_sector_from_sector_to: RecordField<'a, SectorFromTo>,
    pub limitation_4_distance_description: RecordField<'a, DistanceDescription>,
    pub limitation_4_distance_limit: RecordField<'a, NavaidDistanceLimitation>,
    pub limitation_4_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub limitation_4_altitude_limit: RecordField<'a, NavaidAltitudeLimitation>,
    // limitation block 5
    pub limitation_5_sector_from_sector_to: RecordField<'a, SectorFromTo>,
    pub limitation_5_distance_description: RecordField<'a, DistanceDescription>,
    pub limitation_5_distance_limit: RecordField<'a, NavaidDistanceLimitation>,
    pub limitation_5_altitude_description: RecordField<'a, CrossingAltitudeDescription>,
    pub limitation_5_altitude_limit: RecordField<'a, NavaidAltitudeLimitation>,
    pub sequence_end_indicator: RecordField<'a, SequenceEndIndicator>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> VHFNavaidLimitationContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            subsection:                         RecordField::from_bytes(input, 6, 1)?,
            airport_icao_identifier:            RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            vor_identifier:                     RecordField::from_bytes(input, 14, 4)?,
            vor_icao_code:                      RecordField::from_bytes(input, 20, 2)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            application_type:                   RecordField::from_bytes(input, 23, 1)?,
            navaid_limitation_code:             RecordField::from_bytes(input, 24, 1)?,
            component_affected_indicator:       RecordField::from_bytes(input, 25, 1)?,
            sequence_number:                    RecordField::from_bytes(input, 26, 2)?,
            limitation_1_sector_from_sector_to: RecordField::from_bytes(input, 28, 2)?,
            limitation_1_distance_description:  RecordField::from_bytes(input, 30, 1)?,
            limitation_1_distance_limit:        RecordField::from_bytes(input, 31, 6)?,
            limitation_1_altitude_description:  RecordField::from_bytes(input, 37, 1)?,
            limitation_1_altitude_limit:        RecordField::from_bytes(input, 38, 6)?,
            limitation_2_sector_from_sector_to: RecordField::from_bytes(input, 44, 2)?,
            limitation_2_distance_description:  RecordField::from_bytes(input, 46, 1)?,
            limitation_2_distance_limit:        RecordField::from_bytes(input, 47, 6)?,
            limitation_2_altitude_description:  RecordField::from_bytes(input, 53, 1)?,
            limitation_2_altitude_limit:        RecordField::from_bytes(input, 54, 6)?,
            limitation_3_sector_from_sector_to: RecordField::from_bytes(input, 60, 2)?,
            limitation_3_distance_description:  RecordField::from_bytes(input, 62, 1)?,
            limitation_3_distance_limit:        RecordField::from_bytes(input, 63, 6)?,
            limitation_3_altitude_description:  RecordField::from_bytes(input, 69, 1)?,
            limitation_3_altitude_limit:        RecordField::from_bytes(input, 70, 6)?,
            limitation_4_sector_from_sector_to: RecordField::from_bytes(input, 76, 2)?,
            limitation_4_distance_description:  RecordField::from_bytes(input, 78, 1)?,
            limitation_4_distance_limit:        RecordField::from_bytes(input, 79, 6)?,
            limitation_4_altitude_description:  RecordField::from_bytes(input, 85, 1)?,
            limitation_4_altitude_limit:        RecordField::from_bytes(input, 86, 6)?,
            limitation_5_sector_from_sector_to: RecordField::from_bytes(input, 92, 2)?,
            limitation_5_distance_description:  RecordField::from_bytes(input, 94, 1)?,
            limitation_5_distance_limit:        RecordField::from_bytes(input, 95, 6)?,
            limitation_5_altitude_description:  RecordField::from_bytes(input, 101,1)?,
            limitation_5_altitude_limit:        RecordField::from_bytes(input, 102, 6)?,
            sequence_end_indicator:             RecordField::from_bytes(input, 108, 1)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
