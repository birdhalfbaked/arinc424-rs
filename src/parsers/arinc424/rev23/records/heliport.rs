use crate::parsers::arinc424::rev23::records::record::ARINCRecord;

use crate::parsers::arinc424::rev23::definitions::*;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{
    Arinc424RecordSpec, RecordField, RecordParseError, RecordValidationError, is_primary_record,
};
pub(super) struct HeliportRecords;
impl HeliportRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HeliportPrimary(HeliportPrimaryRecord::parse(
                input,
            )?))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => Ok(
                    ARINCRecord::HeliportContinuation(HeliportContinuationRecord::parse(input)?),
                ),
                Some(ContinuationRecordApplicationType::FlightPlanningContinuation) => {
                    Ok(ARINCRecord::HeliportFlightPlanningContinuation(
                        HeliportFlightPlanningContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError::new(
                    "Invalid continuation record application type".to_string(),
                    Some(String::from_utf8_lossy(input).into_owned()),
                )),
            }
        }
    }
}

/// 4.2.1.1
#[derive(Debug)]
pub struct HeliportPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub iata_code: RecordField<'a, AtaIataDesignator>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub speed_limit_altitude: RecordField<'a, SpeedLimitAltitude>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub ifr_capability: RecordField<'a, IfrCapability>,
    pub heliport_type: RecordField<'a, HeliportType>,
    pub heliport_reference_point_latitude: RecordField<'a, Latitude>,
    pub heliport_reference_point_longitude: RecordField<'a, Longitude>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub heliport_elevation: RecordField<'a, AirportHeliportElevation>,
    pub speed_limit: RecordField<'a, SpeedLimit>,
    pub recommended_navaid: RecordField<'a, RecommendedNavaid>,
    pub recommended_navaid_icao_code: RecordField<'a, IcaoCode>,
    pub transition_altitude: RecordField<'a, TransitionAltitudeLevel>,
    pub transition_level: RecordField<'a, TransitionAltitudeLevel>,
    pub public_military_indicator: RecordField<'a, PublicMilitaryIndicator>,
    pub time_zone: RecordField<'a, Timezone>,
    pub daylight_indicator: RecordField<'a, DaylightTimeObservedIndicator>,
    pub magnetic_true_indicator: RecordField<'a, MagneticTrueIndicator>,
    pub heliport_name: RecordField<'a, NameOfFacility>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportPrimaryRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportPrimaryRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                   RecordField::from_bytes(input, 2, 3)?,
            section:                              RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                  RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                   RecordField::from_bytes(input, 11, 2)?,
            subsection:                           RecordField::from_bytes(input, 13, 1)?,
            iata_code:                            RecordField::from_bytes(input, 14, 3)?,
            continuation_record_number:           RecordField::from_bytes(input, 22, 1)?,
            speed_limit_altitude:                 RecordField::from_bytes(input, 23, 5)?,
            datum_code:                           RecordField::from_bytes(input, 28, 3)?,
            ifr_capability:                       RecordField::from_bytes(input, 31, 1)?,
            heliport_type:                        RecordField::from_bytes(input, 32, 1)?,
            heliport_reference_point_latitude:    RecordField::from_bytes(input, 33, 9)?,
            heliport_reference_point_longitude:   RecordField::from_bytes(input, 42, 10)?,
            magnetic_variation:                   RecordField::from_bytes(input, 52, 5)?,
            heliport_elevation:                   RecordField::from_bytes(input, 57, 5)?,
            speed_limit:                          RecordField::from_bytes(input, 62, 3)?,
            recommended_navaid:                   RecordField::from_bytes(input, 65, 4)?,
            recommended_navaid_icao_code:         RecordField::from_bytes(input, 69, 2)?,
            transition_altitude:                  RecordField::from_bytes(input, 71, 5)?,
            transition_level:                     RecordField::from_bytes(input, 76, 5)?,
            public_military_indicator:            RecordField::from_bytes(input, 81, 1)?,
            time_zone:                            RecordField::from_bytes(input, 82, 3)?,
            daylight_indicator:                   RecordField::from_bytes(input, 85, 1)?,
            magnetic_true_indicator:              RecordField::from_bytes(input, 92, 1)?,
            heliport_name:                        RecordField::from_bytes(input, 94, 30)?,
            file_record_number:                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                           RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.2.1.2 Heliport Continuation Record
#[derive(Debug)]
pub struct HeliportContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub iata_code: RecordField<'a, AtaIataDesignator>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                         RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                  RecordField::from_bytes(input, 2, 3)?,
            section:                             RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                 RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            subsection:                          RecordField::from_bytes(input, 13, 1)?,
            iata_code:                           RecordField::from_bytes(input, 14, 3)?,
            continuation_record_number:          RecordField::from_bytes(input, 22, 1)?,
            application_type:                    RecordField::from_bytes(input, 23, 1)?,
            notes:                               RecordField::from_bytes(input, 24, 69)?,
            file_record_number:                  RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                          RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}

/// 4.2.1.3 Heliport Flight Planning Continuation Record
#[derive(Debug)]
pub struct HeliportFlightPlanningContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub iata_code: RecordField<'a, AtaIataDesignator>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub fir_identifier: RecordField<'a, FirUirIdentifier>,
    pub uir_identifier: RecordField<'a, FirUirIdentifier>,
    pub controlled_airspace_indicator: RecordField<'a, ControlledAirspaceIndicator>,
    pub controlled_airspace_airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub controlled_airspace_airport_icao_code: RecordField<'a, IcaoCode>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> Arinc424RecordSpec<'a> for HeliportFlightPlanningContinuationRecord<'a> {
    fn record_name() -> &'static str {
        "HeliportFlightPlanningContinuationRecord"
    }

    fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                              RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                       RecordField::from_bytes(input, 2, 3)?,
            section:                                  RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                      RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                       RecordField::from_bytes(input, 11, 2)?,
            subsection:                               RecordField::from_bytes(input, 13, 1)?,
            iata_code:                                RecordField::from_bytes(input, 14, 3)?,
            continuation_record_number:               RecordField::from_bytes(input, 22, 1)?,
            application_type:                         RecordField::from_bytes(input, 23, 1)?,
            fir_identifier:                           RecordField::from_bytes(input, 24, 4)?,
            uir_identifier:                           RecordField::from_bytes(input, 28, 4)?,
            controlled_airspace_indicator:            RecordField::from_bytes(input, 67, 1)?,
            controlled_airspace_airport_identifier:   RecordField::from_bytes(input, 68, 4)?,
            controlled_airspace_airport_icao_code:    RecordField::from_bytes(input, 72, 2)?,
            file_record_number:                       RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                               RecordField::from_bytes(input, 129, 4)?,
        })
    }

    fn validate(&self) -> Result<(), RecordValidationError> {
        Ok(())
    }
}
