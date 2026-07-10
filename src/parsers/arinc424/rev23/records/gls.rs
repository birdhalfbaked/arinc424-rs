use crate::parsers::arinc424::definitions::*;
use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
pub(super) struct GLSRecords;
impl GLSRecords {
    const CONTINUATION_COLUMN: usize = 22;
    const CONTINUATION_APPLICATION_COLUMN: usize = 23;
    const RUNWAY_HELIPORT_DISCRIMINATION_START_COLUMN: usize = 28;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if input[Self::RUNWAY_HELIPORT_DISCRIMINATION_START_COLUMN - 1
            ..Self::RUNWAY_HELIPORT_DISCRIMINATION_START_COLUMN + 1]
            == [b'R', b'W']
        {
            if is_primary_record(input, Self::CONTINUATION_COLUMN) {
                Ok(ARINCRecord::AirportGLSPrimary(
                    AirportGLSPrimaryRecord::parse(input)?,
                ))
            } else {
                match ContinuationRecordApplicationType::from_bytes(
                    &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                        ..Self::CONTINUATION_APPLICATION_COLUMN],
                )? {
                    Some(ContinuationRecordApplicationType::StandardContinuation) => {
                        Ok(ARINCRecord::AirportGLSContinuation(
                            AirportGLSContinuationRecord::parse(input)?,
                        ))
                    }
                    _ => Err(RecordParseError {
                        message: "Invalid continuation record application type".to_string(),
                    }),
                }
            }
        } else {
            if is_primary_record(input, Self::CONTINUATION_COLUMN) {
                Ok(ARINCRecord::HeliportGLSPrimary(
                    HeliportGLSPrimaryRecord::parse(input)?,
                ))
            } else {
                match ContinuationRecordApplicationType::from_bytes(
                    &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                        ..Self::CONTINUATION_APPLICATION_COLUMN],
                )? {
                    Some(ContinuationRecordApplicationType::StandardContinuation) => {
                        Ok(ARINCRecord::HeliportGLSContinuation(
                            HeliportGLSContinuationRecord::parse(input)?,
                        ))
                    }
                    _ => Err(RecordParseError {
                        message: "Invalid continuation record application type".to_string(),
                    }),
                }
            }
        }
    }
}

/// 4.1.29.1(A) Airport GLS Primary Record
#[derive(Debug)]
pub struct AirportGLSPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub gls_ref_path_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub gls_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub gbas_sbas_channel: RecordField<'a, SBASGBASChannel>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub gls_approach_bearing: RecordField<'a, LocalizerBearing>,
    pub station_latitude: RecordField<'a, Latitude>,
    pub station_longitude: RecordField<'a, Longitude>,
    pub station_identifier: RecordField<'a, ComponentElevation>,
    pub service_volume_radius: RecordField<'a, GlsServiceVolumeRadius>,
    pub tdma_slots: RecordField<'a, GlsTdmaSlots>,
    pub gls_approach_slope: RecordField<'a, GlideslopeAngle>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub station_elevation: RecordField<'a, ComponentElevation>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub station_type: RecordField<'a, GLSStationType>,
    pub station_ellipsoid_height: RecordField<'a, GLSWgs84StationElevation>,
    pub glide_path_tch: RecordField<'a, ThresholdCrossingHeight>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> AirportGLSPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:           RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            gls_ref_path_identifier:      RecordField::from_bytes(input, 14, 4)?,
            gls_category:                 RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            gbas_sbas_channel:            RecordField::from_bytes(input, 23, 5)?,
            runway_identifier:            RecordField::from_bytes(input, 28, 5)?,
            gls_approach_bearing:         RecordField::from_bytes(input, 52, 4)?,
            station_latitude:             RecordField::from_bytes(input, 56, 9)?,
            station_longitude:            RecordField::from_bytes(input, 65, 10)?,
            station_identifier:           RecordField::from_bytes(input, 75, 4)?,
            service_volume_radius:        RecordField::from_bytes(input, 84, 2)?,
            tdma_slots:                   RecordField::from_bytes(input, 86, 2)?,
            gls_approach_slope:           RecordField::from_bytes(input, 88, 2)?,
            magnetic_variation:           RecordField::from_bytes(input, 91, 5)?,
            station_elevation:            RecordField::from_bytes(input, 98, 5)?,
            datum_code:                   RecordField::from_bytes(input, 103, 3)?,
            station_type:                 RecordField::from_bytes(input, 106, 3)?,
            station_ellipsoid_height:     RecordField::from_bytes(input, 111, 5)?,
            glide_path_tch:               RecordField::from_bytes(input, 116, 3)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.29.1(B) Heliport GLS Primary Record
#[derive(Debug)]
pub struct HeliportGLSPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub gls_ref_path_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub gls_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub gbas_sbas_channel: RecordField<'a, SBASGBASChannel>,
    pub helipad_identifier: RecordField<'a, PadIdentifier>,
    pub gls_approach_bearing: RecordField<'a, LocalizerBearing>,
    pub station_latitude: RecordField<'a, Latitude>,
    pub station_longitude: RecordField<'a, Longitude>,
    pub station_identifier: RecordField<'a, ComponentElevation>,
    pub service_volume_radius: RecordField<'a, GlsServiceVolumeRadius>,
    pub tdma_slots: RecordField<'a, GlsTdmaSlots>,
    pub gls_approach_slope: RecordField<'a, GlideslopeAngle>,
    pub magnetic_variation: RecordField<'a, MagneticVariation>,
    pub station_elevation: RecordField<'a, ComponentElevation>,
    pub datum_code: RecordField<'a, DatumCode>,
    pub station_type: RecordField<'a, GLSStationType>,
    pub station_ellipsoid_height: RecordField<'a, GLSWgs84StationElevation>,
    pub glide_path_tch: RecordField<'a, ThresholdCrossingHeight>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HeliportGLSPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:          RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            gls_ref_path_identifier:      RecordField::from_bytes(input, 14, 4)?,
            gls_category:                 RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            gbas_sbas_channel:            RecordField::from_bytes(input, 23, 5)?,
            helipad_identifier:           RecordField::from_bytes(input, 28, 5)?,
            gls_approach_bearing:         RecordField::from_bytes(input, 52, 4)?,
            station_latitude:             RecordField::from_bytes(input, 56, 9)?,
            station_longitude:            RecordField::from_bytes(input, 65, 10)?,
            station_identifier:           RecordField::from_bytes(input, 75, 4)?,
            service_volume_radius:        RecordField::from_bytes(input, 84, 2)?,
            tdma_slots:                   RecordField::from_bytes(input, 86, 2)?,
            gls_approach_slope:           RecordField::from_bytes(input, 88, 2)?,
            magnetic_variation:           RecordField::from_bytes(input, 91, 5)?,
            station_elevation:            RecordField::from_bytes(input, 98, 5)?,
            datum_code:                   RecordField::from_bytes(input, 103, 3)?,
            station_type:                 RecordField::from_bytes(input, 106, 3)?,
            station_ellipsoid_height:     RecordField::from_bytes(input, 111, 5)?,
            glide_path_tch:               RecordField::from_bytes(input, 116, 3)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.29.2(A) Airport GLS Continuation Record
#[derive(Debug)]
pub struct AirportGLSContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub airport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub gls_ref_path_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub gls_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> AirportGLSContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            airport_identifier:           RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            gls_ref_path_identifier:      RecordField::from_bytes(input, 14, 4)?,
            gls_category:                 RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            application_type:             RecordField::from_bytes(input, 23, 1)?,
            notes:                        RecordField::from_bytes(input, 24, 100)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.29.2(B) Heliport GLS Continuation Record
#[derive(Debug)]
pub struct HeliportGLSContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub heliport_identifier: RecordField<'a, AirportHeliportIdentifier>,
    pub airport_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub gls_ref_path_identifier: RecordField<'a, LocalizerMlsGlsIdentifier>,
    pub gls_category: RecordField<'a, IlsMlsGlsCategory>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HeliportGLSContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            heliport_identifier:           RecordField::from_bytes(input, 7, 4)?,
            airport_icao_code:            RecordField::from_bytes(input, 11, 2)?,
            subsection:                   RecordField::from_bytes(input, 13, 1)?,
            gls_ref_path_identifier:      RecordField::from_bytes(input, 14, 4)?,
            gls_category:                 RecordField::from_bytes(input, 18, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 22, 1)?,
            application_type:             RecordField::from_bytes(input, 23, 1)?,
            notes:                        RecordField::from_bytes(input, 24, 100)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
