use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
use crate::parsers::arinc424::rev23::definitions::*;
pub(super) struct HoldingPatternRecords;
impl HoldingPatternRecords {
    const CONTINUATION_COLUMN: usize = 39;
    const CONTINUATION_APPLICATION_COLUMN: usize = 40;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::HoldingPatternPrimary(
                HoldingPatternPrimaryRecord::parse(input)?,
            ))
        } else {
            match ContinuationRecordApplicationType::from_bytes(
                &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                    ..Self::CONTINUATION_APPLICATION_COLUMN],
            )? {
                Some(ContinuationRecordApplicationType::StandardContinuation) => {
                    Ok(ARINCRecord::HoldingPatternContinuation(
                        HoldingPatternContinuationRecord::parse(input)?,
                    ))
                }
                Some(ContinuationRecordApplicationType::PrimaryRecordExtension) => {
                    Ok(ARINCRecord::HoldingPatternPrimaryExtensionContinuation(
                        HoldingPatternPrimaryExtensionContinuationRecord::parse(input)?,
                    ))
                }
                _ => Err(RecordParseError::new("Invalid continuation record application type".to_string(), Some(String::from_utf8_lossy(input).into_owned()))),
            }
        }
    }
}

/// 4.1.5.1 Holding Pattern Primary Record
#[derive(Debug)]
pub struct HoldingPatternPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub duplicate_indicator: RecordField<'a, DuplicateIndicator>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub inbound_holding_course: RecordField<'a, InboundHoldingCourse>,
    pub turn_direction: RecordField<'a, TurnDirection>,
    pub leg_length: RecordField<'a, LegLength>,
    pub leg_time: RecordField<'a, LegTime>,
    pub minumum_altitude: RecordField<'a, MinimumAltitude>,
    pub maximum_altitude: RecordField<'a, MaximumAltitude>,
    pub holding_speed: RecordField<'a, HoldingSpeed>,
    pub rnp: RecordField<'a, RequiredNavigationPerformance>,
    pub arc_radius: RecordField<'a, ArcRadius>,
    pub vertical_scale_factor: RecordField<'a, VerticalScaleFactor>,
    pub rvsm_minimum_level: RecordField<'a, RVSMMinimumLevel>,
    pub rvsm_maximum_level: RecordField<'a, RVSMMaximumLevel>,
    pub leg_inbound_outbound_indicator: RecordField<'a, HoldingPatternCourseReversalLegIndicator>,
    pub inbound_course_navaid_identifier: RecordField<'a, FixIdentifier>,
    pub inbound_course_navaid_icao_code: RecordField<'a, IcaoCode>,
    pub inbound_course_navaid_section_code: RecordField<'a, Section>,
    pub inbound_course_navaid_subsection_code: RecordField<'a, NavaidSubsection>,
    pub inbound_course_navaid_airport_identifier: RecordField<'a, FixIdentifier>,
    pub inbound_course_navaid_airport_icao_code: RecordField<'a, IcaoCode>,
    pub inbound_course_theta: RecordField<'a, InboundCourseTheta>,
    pub name: RecordField<'a, Name>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HoldingPatternPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                         RecordField::from_bytes(input, 2, 3)?,
            section:                                    RecordField::from_bytes(input, 5, 1)?,
            subsection:                                 RecordField::from_bytes(input, 6, 1)?,
            region_code:                                RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:                           RecordField::from_bytes(input, 11, 2)?,
            duplicate_indicator:                        RecordField::from_bytes(input, 28, 2)?,
            fix_identifier:                             RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                              RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                           RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                        RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:                 RecordField::from_bytes(input, 39, 1)?,
            inbound_holding_course:                     RecordField::from_bytes(input, 40, 4)?,
            turn_direction:                             RecordField::from_bytes(input, 44, 1)?,
            leg_length:                                 RecordField::from_bytes(input, 45, 3)?,
            leg_time:                                   RecordField::from_bytes(input, 48, 2)?,
            minumum_altitude:                           RecordField::from_bytes(input, 50, 5)?,
            maximum_altitude:                           RecordField::from_bytes(input, 55, 5)?,
            holding_speed:                              RecordField::from_bytes(input, 60, 3)?,
            rnp:                                        RecordField::from_bytes(input, 63, 3)?,
            arc_radius:                                 RecordField::from_bytes(input, 66, 6)?,
            vertical_scale_factor:                      RecordField::from_bytes(input, 72, 3)?,
            rvsm_minimum_level:                         RecordField::from_bytes(input, 75, 3)?,
            rvsm_maximum_level:                         RecordField::from_bytes(input, 78, 3)?,
            leg_inbound_outbound_indicator:             RecordField::from_bytes(input, 81, 1)?,
            inbound_course_navaid_identifier:           RecordField::from_bytes(input, 82, 4)?,
            inbound_course_navaid_icao_code:            RecordField::from_bytes(input, 86, 2)?,
            inbound_course_navaid_section_code:         RecordField::from_bytes(input, 88, 1)?,
            inbound_course_navaid_subsection_code:      RecordField::from_bytes(input, 89, 1)?,
            inbound_course_navaid_airport_identifier:   RecordField::from_bytes(input, 90, 4)?,
            inbound_course_navaid_airport_icao_code:    RecordField::from_bytes(input, 94, 2)?,
            inbound_course_theta:                       RecordField::from_bytes(input, 96, 3)?,
            name:                                       RecordField::from_bytes(input, 99, 25)?,
            file_record_number:                         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.5.2 Holding Pattern Continuation Record
#[derive(Debug)]
pub struct HoldingPatternContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub duplicate_indicator: RecordField<'a, DuplicateIndicator>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub notes: RecordField<'a, Notes>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HoldingPatternContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                  RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:           RecordField::from_bytes(input, 2, 3)?,
            section:                      RecordField::from_bytes(input, 5, 1)?,
            subsection:                   RecordField::from_bytes(input, 6, 1)?,
            region_code:                  RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:             RecordField::from_bytes(input, 11, 2)?,
            duplicate_indicator:          RecordField::from_bytes(input, 28, 2)?,
            fix_identifier:               RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:             RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:          RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:   RecordField::from_bytes(input, 39, 1)?,
            application_type:             RecordField::from_bytes(input, 40, 1)?,
            notes:                        RecordField::from_bytes(input, 41, 69)?,
            file_record_number:           RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                   RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.1.5.3 Holding Pattern Primary Extension Continuation Record
#[derive(Debug)]
pub struct HoldingPatternPrimaryExtensionContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, EnrouteSubsection>,
    pub region_code: RecordField<'a, RegionCode>,
    pub region_icao_code: RecordField<'a, IcaoCode>,
    pub duplicate_indicator: RecordField<'a, DuplicateIndicator>,
    pub fix_identifier: RecordField<'a, FixIdentifier>,
    pub fix_icao_code: RecordField<'a, IcaoCode>,
    pub fix_section_code: RecordField<'a, Section>,
    pub fix_subsection_code: RecordField<'a, GenericSubsection>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub holding_pattern_magnetic_variation: RecordField<'a, HoldingPatternMagneticVariation>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HoldingPatternPrimaryExtensionContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                          RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                   RecordField::from_bytes(input, 2, 3)?,
            section:                              RecordField::from_bytes(input, 5, 1)?,
            subsection:                           RecordField::from_bytes(input, 6, 1)?,
            region_code:                          RecordField::from_bytes(input, 7, 4)?,
            region_icao_code:                     RecordField::from_bytes(input, 11, 2)?,
            duplicate_indicator:                  RecordField::from_bytes(input, 28, 2)?,
            fix_identifier:                       RecordField::from_bytes(input, 30, 5)?,
            fix_icao_code:                        RecordField::from_bytes(input, 35, 2)?,
            fix_section_code:                     RecordField::from_bytes(input, 37, 1)?,
            fix_subsection_code:                  RecordField::from_bytes(input, 38, 1)?,
            continuation_record_number:           RecordField::from_bytes(input, 39, 1)?,
            application_type:                     RecordField::from_bytes(input, 40, 1)?,
            holding_pattern_magnetic_variation:   RecordField::from_bytes(input, 41, 5)?,
            file_record_number:                   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                           RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
