use crate::parsers::arinc424::definitions::*;
use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
pub(super) struct LocalizerMarkerRecords;
impl LocalizerMarkerRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;
    const RUNWAY_HELIPORT_DISCRIMINATION_START_COLUMN: usize = 28;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            // The only way to properly discriminate is if it starts with RW which always means Runway
            // note: if this becomes too cumbersome this can be combined into a more general field that captures both
            //     5.46 and 5.180
            if input[Self::RUNWAY_HELIPORT_DISCRIMINATION_START_COLUMN - 1
                ..Self::RUNWAY_HELIPORT_DISCRIMINATION_START_COLUMN + 1]
                == [b'R', b'W']
            {
                Ok(ARINCRecord::AirportLocalizerMarkerPrimary(
                    AirportLocalizerMarkerPrimaryRecord::parse(input)?,
                ))
            } else {
                Ok(ARINCRecord::HeliportLocalizerMarkerPrimary(
                    HeliportLocalizerMarkerPrimaryRecord::parse(input)?,
                ))
            }
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    if input[Self::RUNWAY_HELIPORT_DISCRIMINATION_START_COLUMN - 1
                        ..Self::RUNWAY_HELIPORT_DISCRIMINATION_START_COLUMN + 1]
                        == [b'R', b'W']
                    {
                        Ok(ARINCRecord::AirportLocalizerMarkerContinuation(
                            AirportLocalizerMarkerContinuationRecord::parse(input)?,
                        ))
                    } else {
                        Ok(ARINCRecord::HeliportLocalizerMarkerContinuation(
                            HeliportLocalizerMarkerContinuationRecord::parse(input)?,
                        ))
                    }
                }
                _ => Err(RecordParseError {
                    message: "Invalid continuation record application type".to_string(),
                }),
            }
        }
    }
}

/// 4.1.13.1(A) Airport Localizer Marker Primary Record
#[derive(Debug)]
pub struct AirportLocalizerMarkerPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub marker_type: RecordField<'a, MarkerType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub locator_frequency: RecordField<'a, VORNDBFrequency>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub marker_latitude: RecordField<'a, Latitude>,
    pub marker_longitude: RecordField<'a, Longitude>,
    pub minor_axis_bearing: RecordField<'a, MinorAxisBearing>,
    pub locator_latitude: RecordField<'a, Latitude>,
    pub locator_longitude: RecordField<'a, Longitude>,
    pub locator_class: RecordField<'a, MarkerLocatorNavaidClass>,
    pub locator_facility_characteristics: RecordField<'a, FacilityCharacteristics>,
    pub locator_identifier: RecordField<'a, VORNDBIdentifier>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> AirportLocalizerMarkerPrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                 RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:               RecordField::from_bytes(input, 14, 4)?,
            marker_type:                        RecordField::from_bytes(input, 18, 3)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            locator_frequency:                  RecordField::from_bytes(input, 23, 5)?,
            runway_identifier:                  RecordField::from_bytes(input, 28, 5)?,
            marker_latitude:                    RecordField::from_bytes(input, 33, 9)?,
            marker_longitude:                   RecordField::from_bytes(input, 42, 10)?,
            minor_axis_bearing:                 RecordField::from_bytes(input, 52, 4)?,
            locator_latitude:                   RecordField::from_bytes(input, 56, 9)?,
            locator_longitude:                  RecordField::from_bytes(input, 65, 10)?,
            locator_class:                      RecordField::from_bytes(input, 75, 5)?,
            locator_facility_characteristics:   RecordField::from_bytes(input, 80, 5)?,
            locator_identifier:                 RecordField::from_bytes(input, 85, 4)?,
            magnetic_variation:                 RecordField::from_bytes(input, 91, 5)?,
            facility_elevation:                 RecordField::from_bytes(input, 98, 5)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.13.1(B) Heliport Localizer Marker Primary Record
#[derive(Debug)]
pub struct HeliportLocalizerMarkerPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub marker_type: RecordField<'a, MarkerType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub locator_frequency: RecordField<'a, VORNDBFrequency>,
    pub pad_identifier: RecordField<'a, PadIdentifier>,
    pub marker_latitude: RecordField<'a, Latitude>,
    pub marker_longitude: RecordField<'a, Longitude>,
    pub minor_axis_bearing: RecordField<'a, MinorAxisBearing>,
    pub locator_latitude: RecordField<'a, Latitude>,
    pub locator_longitude: RecordField<'a, Longitude>,
    pub locator_class: RecordField<'a, MarkerLocatorNavaidClass>,
    pub locator_facility_characteristics: RecordField<'a, FacilityCharacteristics>,
    pub locator_identifier: RecordField<'a, VORNDBIdentifier>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub facility_elevation: RecordField<'a, FacilityElevation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HeliportLocalizerMarkerPrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                 RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:               RecordField::from_bytes(input, 14, 4)?,
            marker_type:                        RecordField::from_bytes(input, 18, 3)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            locator_frequency:                  RecordField::from_bytes(input, 23, 5)?,
            pad_identifier:                     RecordField::from_bytes(input, 28, 5)?,
            marker_latitude:                    RecordField::from_bytes(input, 33, 9)?,
            marker_longitude:                   RecordField::from_bytes(input, 42, 10)?,
            minor_axis_bearing:                 RecordField::from_bytes(input, 52, 4)?,
            locator_latitude:                   RecordField::from_bytes(input, 56, 9)?,
            locator_longitude:                  RecordField::from_bytes(input, 65, 10)?,
            locator_class:                      RecordField::from_bytes(input, 75, 5)?,
            locator_facility_characteristics:   RecordField::from_bytes(input, 80, 5)?,
            locator_identifier:                 RecordField::from_bytes(input, 85, 4)?,
            magnetic_variation:                 RecordField::from_bytes(input, 91, 5)?,
            facility_elevation:                 RecordField::from_bytes(input, 98, 5)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.13.2(A) Airport Localizer Marker Continuation Record
#[derive(Debug)]
pub struct AirportLocalizerMarkerContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub marker_type: RecordField<'a, MarkerType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> AirportLocalizerMarkerContinuationRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:                 RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:                  RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:               RecordField::from_bytes(input, 14, 4)?,
            marker_type:                        RecordField::from_bytes(input, 18, 3)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            application_type:                   RecordField::from_bytes(input, 23, 1)?,
            notes:                              RecordField::from_bytes(input, 24, 100)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.13.2(B) Heliport Localizer Marker Continuation Record
#[derive(Debug)]
pub struct HeliportLocalizerMarkerContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub heliport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub localizer_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub marker_type: RecordField<'a, MarkerType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HeliportLocalizerMarkerContinuationRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                        RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                 RecordField::from_bytes(input, 2, 3)?,
            section:                            RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:                RecordField::from_bytes(input, 7, 4)?,
            heliport_icao_code:                 RecordField::from_bytes(input, 11, 2)?,
            subsection:                         RecordField::from_bytes(input, 13, 1)?,
            localizer_identifier:               RecordField::from_bytes(input, 14, 4)?,
            marker_type:                        RecordField::from_bytes(input, 18, 3)?,
            continuation_record_number:         RecordField::from_bytes(input, 22, 1)?,
            application_type:                   RecordField::from_bytes(input, 23, 1)?,
            notes:                              RecordField::from_bytes(input, 24, 100)?,
            file_record_number:                 RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                         RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
