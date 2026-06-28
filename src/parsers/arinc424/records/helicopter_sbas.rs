use crate::parsers::arinc424::definitions::*;
use crate::parsers::arinc424::fields::ParseableField;
use crate::parsers::arinc424::records::record::{
    ARINCRecord, RecordField, RecordParseError, is_primary_record,
};
pub(super) struct HelicopterSBASRecords;
impl HelicopterSBASRecords {
    const CONTINUATION_COLUMN: usize = 27;
    const CONTINUATION_APPLICATION_COLUMN: usize = 28;
    const DISCRIMINATOR_COLUMN: usize = 20;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            if input[Self::DISCRIMINATOR_COLUMN - 1..Self::DISCRIMINATOR_COLUMN + 1] == [b'R', b'W']
            {
                Ok(ARINCRecord::HelicopterRunwaySBASPathPointPrimary(
                    HelicopterRunwaySBASPathPointPrimaryRecord::parse(input)?,
                ))
            } else {
                Ok(
                    ARINCRecord::HelicopterFinalApproachCourseAsRunwaySBASPathPointPrimary(
                        HelicopterFinalApproachCourseAsRunwaySBASPathPointPrimaryRecord::parse(
                            input,
                        )?,
                    ),
                )
            }
        } else {
            if input[Self::DISCRIMINATOR_COLUMN - 1..Self::DISCRIMINATOR_COLUMN + 1] == [b'R', b'W']
            {
                match ContinuationRecordApplicationType::from_bytes(
                    &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                        ..Self::CONTINUATION_APPLICATION_COLUMN],
                )? {
                    Some(ContinuationRecordApplicationType::StandardContinuation) => {
                        Ok(ARINCRecord::HelicopterRunwaySBASPathPointContinuation(
                            HelicopterRunwaySBASPathPointContinuationRecord::parse(input)?,
                        ))
                    }
                    _ => Err(RecordParseError {
                        message: "Invalid continuation record application type".to_string(),
                    }),
                }
            } else {
                match ContinuationRecordApplicationType::from_bytes(
                    &input[Self::CONTINUATION_APPLICATION_COLUMN - 1
                        ..Self::CONTINUATION_APPLICATION_COLUMN],
                )? {
                    Some(ContinuationRecordApplicationType::StandardContinuation) => {
                        Ok(ARINCRecord::HelicopterFinalApproachCourseAsRunwaySBASPathPointContinuation(
                            HelicopterFinalApproachCourseAsRunwaySBASPathPointContinuationRecord::parse(input)?,
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

fn parse_path_point_tch<'a>(
    input: &'a [u8],
) -> Result<RecordField<'a, PathPointTCH>, RecordParseError> {
    let tch_units_indicator = TCHUnitsIndicator::from_bytes(&input[108..109])?;
    let tch_value_bytes = &input[103..109];
    let tch_value = RecordField {
        value: match tch_units_indicator {
            Some(TCHUnitsIndicator::Feet) => {
                if let Some(tch_inner_value) = PathPointTCHFeet::from_bytes(tch_value_bytes)? {
                    Some(PathPointTCH::Feet(tch_inner_value))
                } else {
                    None
                }
            }
            Some(TCHUnitsIndicator::Meters) => {
                if let Some(tch_inner_value) = PathPointTCHMeters::from_bytes(tch_value_bytes)? {
                    Some(PathPointTCH::Meters(tch_inner_value))
                } else {
                    None
                }
            }
            None => {
                return Err(RecordParseError {
                    message: "Invalid TCH units indicator".to_string(),
                })?;
            }
        },
        raw_bytes: tch_value_bytes,
    };
    Ok(tch_value)
}

/// 4.2.8.1(A) Helicopter Runway SBAS Path Point Primary Record
#[derive(Debug)]
pub struct HelicopterRunwaySBASPathPointPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub helipad_identifier: RecordField<'a, PadIdentifier>,
    pub helipad_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub operation_type: RecordField<'a, SBASOperationType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub route_indicator: RecordField<'a, SBASGBASRouteIndicator>,
    pub sbas_service_provider_identifier: RecordField<'a, SbasServiceProviderIdentifier>,
    pub reference_path_data_selector: RecordField<'a, ReferencePathDataSelector>,
    pub reference_path_identifier: RecordField<'a, ReferencePathIdentifier>,
    pub approach_performance_designator: RecordField<'a, GBASApproachPerformanceDesignator>,
    pub landing_threshold_point_latitude: RecordField<'a, Latitude>,
    pub landing_threshold_point_longitude: RecordField<'a, Longitude>,
    pub landing_threshold_point_ellipsoid_height: RecordField<'a, WGS84EllipsoidHeight>,
    pub glide_path_angle: RecordField<'a, SBASGBASGlidePathAngle>,
    pub flight_path_alignment_point_latitude: RecordField<'a, Latitude>,
    pub flight_path_alignment_point_longitude: RecordField<'a, Longitude>,
    pub course_width_at_threshold: RecordField<'a, CourseWidthAtThreshold>,
    pub length_offset: RecordField<'a, SBASGBASLengthOffset>,
    pub path_point_tch: RecordField<'a, PathPointTCH>,
    pub tch_units_indicator: RecordField<'a, TCHUnitsIndicator>,
    pub horizontal_alert_limit: RecordField<'a, HorizontalAlertLimit>,
    pub vertical_alert_limit: RecordField<'a, VerticalAlertLimit>,
    pub fas_data_crc_remainder: RecordField<'a, FinalApproachSegmentDataCrcRemainder>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterRunwaySBASPathPointPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        let tch_value = parse_path_point_tch(input)?;
        Ok(Self {
            record_type:                                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                         RecordField::from_bytes(input, 2, 3)?,
            section:                                    RecordField::from_bytes(input, 5, 1)?,
            helipad_identifier:                         RecordField::from_bytes(input, 7, 4)?,
            helipad_icao_code:                          RecordField::from_bytes(input, 11, 2)?,
            subsection:                                 RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                        RecordField::from_bytes(input, 14, 6)?,
            runway_identifier:                          RecordField::from_bytes(input, 20, 5)?,
            operation_type:                             RecordField::from_bytes(input, 25, 2)?,
            continuation_record_number:                 RecordField::from_bytes(input, 27, 1)?,
            route_indicator:                            RecordField::from_bytes(input, 28, 1)?,
            sbas_service_provider_identifier:           RecordField::from_bytes(input, 29, 2)?,
            reference_path_data_selector:               RecordField::from_bytes(input, 31, 2)?,
            reference_path_identifier:                  RecordField::from_bytes(input, 33, 4)?,
            approach_performance_designator:            RecordField::from_bytes(input, 37, 1)?,
            landing_threshold_point_latitude:           RecordField::from_bytes(input, 38, 11)?,
            landing_threshold_point_longitude:          RecordField::from_bytes(input, 49, 12)?,
            landing_threshold_point_ellipsoid_height:   RecordField::from_bytes(input, 61, 6)?,
            glide_path_angle:                           RecordField::from_bytes(input, 67, 4)?,
            flight_path_alignment_point_latitude:       RecordField::from_bytes(input, 71, 11)?,
            flight_path_alignment_point_longitude:      RecordField::from_bytes(input, 82, 12)?,
            course_width_at_threshold:                  RecordField::from_bytes(input, 94, 5)?,
            length_offset:                              RecordField::from_bytes(input, 99, 4)?,
            path_point_tch:                             tch_value,
            tch_units_indicator:                        RecordField::from_bytes(input, 109, 1)?,
            horizontal_alert_limit:                     RecordField::from_bytes(input, 110, 3)?,
            vertical_alert_limit:                       RecordField::from_bytes(input, 113, 3)?,
            fas_data_crc_remainder:                     RecordField::from_bytes(input, 116, 8)?,
            file_record_number:                         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.2.8.1(B) Helicopter Final Approach Course as Runway SBAS Path Point Primary Record
#[derive(Debug)]
pub struct HelicopterFinalApproachCourseAsRunwaySBASPathPointPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub helipad_identifier: RecordField<'a, PadIdentifier>,
    pub helipad_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub final_approach_course_as_runway: RecordField<'a, FinalApproachCourseAsRunway>,
    pub operation_type: RecordField<'a, SBASOperationType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub route_indicator: RecordField<'a, SBASGBASRouteIndicator>,
    pub sbas_service_provider_identifier: RecordField<'a, SbasServiceProviderIdentifier>,
    pub reference_path_data_selector: RecordField<'a, ReferencePathDataSelector>,
    pub reference_path_identifier: RecordField<'a, ReferencePathIdentifier>,
    pub approach_performance_designator: RecordField<'a, GBASApproachPerformanceDesignator>,
    pub landing_threshold_point_latitude: RecordField<'a, Latitude>,
    pub landing_threshold_point_longitude: RecordField<'a, Longitude>,
    pub landing_threshold_point_ellipsoid_height: RecordField<'a, WGS84EllipsoidHeight>,
    pub glide_path_angle: RecordField<'a, SBASGBASGlidePathAngle>,
    pub flight_path_alignment_point_latitude: RecordField<'a, Latitude>,
    pub flight_path_alignment_point_longitude: RecordField<'a, Longitude>,
    pub course_width_at_threshold: RecordField<'a, CourseWidthAtThreshold>,
    pub length_offset: RecordField<'a, SBASGBASLengthOffset>,
    pub path_point_tch: RecordField<'a, PathPointTCH>,
    pub tch_units_indicator: RecordField<'a, TCHUnitsIndicator>,
    pub horizontal_alert_limit: RecordField<'a, HorizontalAlertLimit>,
    pub vertical_alert_limit: RecordField<'a, VerticalAlertLimit>,
    pub fas_data_crc_remainder: RecordField<'a, FinalApproachSegmentDataCrcRemainder>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterFinalApproachCourseAsRunwaySBASPathPointPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        let tch_value = parse_path_point_tch(input)?;
        Ok(Self {
            record_type:                                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                         RecordField::from_bytes(input, 2, 3)?,
            section:                                    RecordField::from_bytes(input, 5, 1)?,
            helipad_identifier:                         RecordField::from_bytes(input, 7, 4)?,
            helipad_icao_code:                          RecordField::from_bytes(input, 11, 2)?,
            subsection:                                 RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                        RecordField::from_bytes(input, 14, 6)?,
            final_approach_course_as_runway:            RecordField::from_bytes(input, 20, 2)?,
            operation_type:                             RecordField::from_bytes(input, 25, 2)?,
            continuation_record_number:                 RecordField::from_bytes(input, 27, 1)?,
            route_indicator:                            RecordField::from_bytes(input, 28, 1)?,
            sbas_service_provider_identifier:           RecordField::from_bytes(input, 29, 2)?,
            reference_path_data_selector:               RecordField::from_bytes(input, 31, 2)?,
            reference_path_identifier:                  RecordField::from_bytes(input, 33, 4)?,
            approach_performance_designator:            RecordField::from_bytes(input, 37, 1)?,
            landing_threshold_point_latitude:           RecordField::from_bytes(input, 38, 11)?,
            landing_threshold_point_longitude:          RecordField::from_bytes(input, 49, 12)?,
            landing_threshold_point_ellipsoid_height:   RecordField::from_bytes(input, 61, 6)?,
            glide_path_angle:                           RecordField::from_bytes(input, 67, 4)?,
            flight_path_alignment_point_latitude:       RecordField::from_bytes(input, 71, 11)?,
            flight_path_alignment_point_longitude:      RecordField::from_bytes(input, 82, 12)?,
            course_width_at_threshold:                  RecordField::from_bytes(input, 94, 5)?,
            length_offset:                              RecordField::from_bytes(input, 99, 4)?,
            path_point_tch:                             tch_value,
            tch_units_indicator:                        RecordField::from_bytes(input, 109, 1)?,
            horizontal_alert_limit:                     RecordField::from_bytes(input, 110, 3)?,
            vertical_alert_limit:                       RecordField::from_bytes(input, 113, 3)?,
            fas_data_crc_remainder:                     RecordField::from_bytes(input, 116, 8)?,
            file_record_number:                         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.2.8.2(A) Helicopter Runway SBAS Path Point Continuation Record
#[derive(Debug)]
pub struct HelicopterRunwaySBASPathPointContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub helipad_identifier: RecordField<'a, PadIdentifier>,
    pub helipad_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub runway_identifier: RecordField<'a, RunwayIdentifier>,
    pub operation_type: RecordField<'a, SBASOperationType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub fpap_ellipsoid_height: RecordField<'a, WGS84EllipsoidHeight>,
    pub fpap_orthometric_height: RecordField<'a, OrthometricHeight>,
    pub ltp_orthometric_height: RecordField<'a, OrthometricHeight>,
    pub approach_type_identifier: RecordField<'a, ApproachTypeIdentifier>,
    pub gbas_sbas_channel_number: RecordField<'a, SBASGBASChannel>,
    pub sbas_final_approach_course: RecordField<'a, SBASFinalApproachCourse>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterRunwaySBASPathPointContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                         RecordField::from_bytes(input, 2, 3)?,
            section:                                    RecordField::from_bytes(input, 5, 1)?,
            helipad_identifier:                         RecordField::from_bytes(input, 7, 4)?,
            helipad_icao_code:                          RecordField::from_bytes(input, 11, 2)?,
            subsection:                                 RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                        RecordField::from_bytes(input, 14, 6)?,
            runway_identifier:                          RecordField::from_bytes(input, 20, 5)?,
            operation_type:                             RecordField::from_bytes(input, 25, 2)?,
            continuation_record_number:                 RecordField::from_bytes(input, 27, 1)?,
            application_type:                           RecordField::from_bytes(input, 28, 1)?,
            fpap_ellipsoid_height:                      RecordField::from_bytes(input, 29, 6)?,
            fpap_orthometric_height:                    RecordField::from_bytes(input, 35, 6)?,
            ltp_orthometric_height:                     RecordField::from_bytes(input, 41, 6)?,
            approach_type_identifier:                   RecordField::from_bytes(input, 47, 10)?,
            gbas_sbas_channel_number:                   RecordField::from_bytes(input, 57, 5)?,
            sbas_final_approach_course:                 RecordField::from_bytes(input, 62, 4)?,
            file_record_number:                         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}

/// 4.2.8.2(B) Helicopter Final Approach Course as Runway SBAS Path Point Continuation Record
#[derive(Debug)]
pub struct HelicopterFinalApproachCourseAsRunwaySBASPathPointContinuationRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub customer_area_code: RecordField<'a, CustomerAreaCode>,
    pub section: RecordField<'a, Section>,
    pub helipad_identifier: RecordField<'a, PadIdentifier>,
    pub helipad_icao_code: RecordField<'a, IcaoCode>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub approach_identifier: RecordField<'a, ApproachRouteIdentifier>,
    pub final_approach_course_as_runway: RecordField<'a, FinalApproachCourseAsRunway>,
    pub operation_type: RecordField<'a, SBASOperationType>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub application_type: RecordField<'a, ContinuationRecordApplicationType>,
    pub fpap_ellipsoid_height: RecordField<'a, WGS84EllipsoidHeight>,
    pub fpap_orthometric_height: RecordField<'a, OrthometricHeight>,
    pub ltp_orthometric_height: RecordField<'a, OrthometricHeight>,
    pub approach_type_identifier: RecordField<'a, ApproachTypeIdentifier>,
    pub gbas_sbas_channel_number: RecordField<'a, SBASGBASChannel>,
    pub sbas_final_approach_course: RecordField<'a, SBASFinalApproachCourse>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> HelicopterFinalApproachCourseAsRunwaySBASPathPointContinuationRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:                                RecordField::from_bytes(input, 1, 1)?,
            customer_area_code:                         RecordField::from_bytes(input, 2, 3)?,
            section:                                    RecordField::from_bytes(input, 5, 1)?,
            helipad_identifier:                         RecordField::from_bytes(input, 7, 4)?,
            helipad_icao_code:                          RecordField::from_bytes(input, 11, 2)?,
            subsection:                                 RecordField::from_bytes(input, 13, 1)?,
            approach_identifier:                        RecordField::from_bytes(input, 14, 6)?,
            final_approach_course_as_runway:            RecordField::from_bytes(input, 20, 2)?,
            operation_type:                             RecordField::from_bytes(input, 25, 2)?,
            continuation_record_number:                 RecordField::from_bytes(input, 27, 1)?,
            application_type:                           RecordField::from_bytes(input, 28, 1)?,
            fpap_ellipsoid_height:                      RecordField::from_bytes(input, 29, 6)?,
            fpap_orthometric_height:                    RecordField::from_bytes(input, 35, 6)?,
            ltp_orthometric_height:                     RecordField::from_bytes(input, 41, 6)?,
            approach_type_identifier:                   RecordField::from_bytes(input, 47, 10)?,
            gbas_sbas_channel_number:                   RecordField::from_bytes(input, 57, 5)?,
            sbas_final_approach_course:                 RecordField::from_bytes(input, 62, 4)?,
            file_record_number:                         RecordField::from_bytes(input, 124, 5)?,
            cycle_date:                                 RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
