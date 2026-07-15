use crate::rev18_faa::definitions::*;
use crate::types::fields::ParseableField;
use crate::types::records::*;

#[cfg(test)]
use crate::types::fields::LengthLimitedIdentifier;

/// Validates the reference, section, and subsection combination to ensure they are a
/// possible combination that is allowed.
pub fn is_valid_reference(
    reference: &RecordField<impl ParseableField>,
    section: &RecordField<Section>,
    subsection: &RecordField<impl ParseableField>,
) -> Vec<RecordValidationMessage> {
    let mut messages = Vec::new();
    let field_positions = vec![
        (reference.start_column, reference.end_column),
        (section.start_column, section.end_column),
        (subsection.start_column, subsection.end_column),
    ];
    if reference.value.is_none() && (section.value.is_some() || subsection.value.is_some()) {
        messages.push(RecordValidationMessage::new(
            "Reference is required when section and subsection are provided",
            field_positions.clone(),
        ));
    } else {
        let subsection_bytes: &[u8] = subsection.raw_bytes;
        let is_valid = match section.value {
            Some(Section::Airport) => AirportSubsection::from_bytes(subsection_bytes).is_ok(),
            Some(Section::MORA) => GridMora::from_bytes(subsection_bytes).is_ok(),
            Some(Section::Navaid) => NavaidSubsection::from_bytes(subsection_bytes).is_ok(),
            Some(Section::Tables) => TablesSubsection::from_bytes(subsection_bytes).is_ok(),
            Some(Section::Enroute) => EnrouteSubsection::from_bytes(subsection_bytes).is_ok(),
            Some(Section::CompanyRoutes) => {
                CompanyRoutesSubsection::from_bytes(subsection_bytes).is_ok()
            }
            Some(Section::Airspace) => AirspaceSubsection::from_bytes(subsection_bytes).is_ok(),
            Some(Section::Heliport) => HeliportSubsection::from_bytes(subsection_bytes).is_ok(),
            _ => false,
        };
        if !is_valid {
            messages.push(RecordValidationMessage::new(
                "Invalid section and subsection combination",
                field_positions.clone(),
            ));
        }
    }
    messages
}

#[test]
fn test_is_valid_reference() {
    let reference = RecordField::<LengthLimitedIdentifier<3, true>> {
        value: LengthLimitedIdentifier::<3, true>::from_bytes(b"ABC").unwrap(),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    // Happy case
    let section = RecordField::<Section> {
        value: Some(Section::Airport),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let subsection = RecordField::<GenericSubsection> {
        value: GenericSubsection::from_bytes(b"A").unwrap(),
        raw_bytes: b"A",
        start_column: 0,
        end_column: 0,
    };
    let messages = is_valid_reference(&reference, &section, &subsection);
    assert_eq!(
        messages.len(),
        0,
        "Expected no messages, got {}",
        messages.len()
    );
    // Invalid case
    let section = RecordField::<Section> {
        value: Some(Section::Airport),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let subsection = RecordField::<GenericSubsection> {
        value: GenericSubsection::from_bytes(b" ").unwrap(),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let messages = is_valid_reference(&reference, &section, &subsection);
    assert_eq!(
        messages.len(),
        1,
        "Expected 1 message, got {}",
        messages.len()
    );
}

/// Validates the route type and qualifier combinations to ensure they are a possible combination
/// that is allowed for an airport heliport approach route.
pub fn is_valid_route_type_and_qualifier_combination<'a>(
    route_type: &RecordField<AirportHeliportApproachRouteType>,
    qualifier1: &RecordField<RouteTypeQualifier1>,
    qualifier2: &RecordField<RouteTypeQualifier2>,
) -> Vec<RecordValidationMessage> {
    let mut messages = Vec::new();
    let field_positions = vec![
        (route_type.start_column, route_type.end_column),
        (qualifier1.start_column, qualifier1.end_column),
        (qualifier2.start_column, qualifier2.end_column),
    ];
    if route_type.value.is_none() && (qualifier1.value.is_some() || qualifier2.value.is_some()) {
        messages.push(RecordValidationMessage::new(
            "Invalid route type and qualifier combination: Route type is required",
            field_positions.clone(),
        ));
    }

    // validate qualifier 1
    match route_type.value {
        Some(AirportHeliportApproachRouteType::RNAVApproach) => {
            if matches!(
                qualifier1.value,
                Some(RouteTypeQualifier1::DMERequired | RouteTypeQualifier1::DMENotRequired)
            ) {
                messages.push(
                    RecordValidationMessage::new(
                        "Invalid route type and qualifier combination: Route Qualifier 1 incompatible with RNAV Approach",
                        field_positions.clone(),
                    ),
                );
            }
        }
        _ => {}
    }
    // validate qualifier 2
    match route_type.value {
        Some(AirportHeliportApproachRouteType::MissedApproach) => {
            if matches!(
                qualifier2.value,
                Some(
                    RouteTypeQualifier2::ProcedureWithStraightInMinimums
                        | RouteTypeQualifier2::ProcedureWithoutStraightInMinimums
                )
            ) {
                messages.push(
                    RecordValidationMessage::new(
                        "Invalid route type and qualifier combination: Route Qualifier 2 not allowed for missed approach",
                        field_positions.clone(),
                    ),
                );
            }
        }
        _ => {
            if matches!(
                qualifier2.value,
                Some(
                    RouteTypeQualifier2::PrimaryMissedApproach
                        | RouteTypeQualifier2::SecondaryMissedApproach
                        | RouteTypeQualifier2::EngineOutMissedApproach
                )
            ) {
                messages.push(
                    RecordValidationMessage::new(
                        "Invalid route type and qualifier combination: Route Qualifier 2 not allowed except on missed approach",
                        field_positions.clone(),
                    ),
                );
            }
        }
    }
    messages
}

#[test]
fn test_is_valid_route_type_and_qualifier_combination() {
    // Happy case
    let route_type = RecordField::<AirportHeliportApproachRouteType> {
        value: Some(AirportHeliportApproachRouteType::MissedApproach),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let qualifier1 = RecordField::<RouteTypeQualifier1> {
        value: Some(RouteTypeQualifier1::DMERequired),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let qualifier2 = RecordField::<RouteTypeQualifier2> {
        value: Some(RouteTypeQualifier2::PrimaryMissedApproach),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let result =
        is_valid_route_type_and_qualifier_combination(&route_type, &qualifier1, &qualifier2);
    assert_eq!(
        result.len(),
        0,
        "Expected no messages, got {}",
        result.len()
    );

    // Invalid case to test both qualifier errors
    let route_type = RecordField::<AirportHeliportApproachRouteType> {
        value: Some(AirportHeliportApproachRouteType::RNAVApproach),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let qualifier1 = RecordField::<RouteTypeQualifier1> {
        value: Some(RouteTypeQualifier1::DMERequired),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let qualifier2 = RecordField::<RouteTypeQualifier2> {
        value: Some(RouteTypeQualifier2::PrimaryMissedApproach),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let result =
        is_valid_route_type_and_qualifier_combination(&route_type, &qualifier1, &qualifier2);
    assert_eq!(result.len(), 2, "Expected 2 messages, got {}", result.len());
}

/// Validates the area code and boundary code combination to ensure they are a possible combination
/// that is allowed. Note that this should only check boundaries, that is, bordering regions that can touch
pub fn is_valid_boundary_code(
    area_code: &RecordField<CustomerAreaCode>,
    boundary_code: &RecordField<BoundaryCode>,
) -> Vec<RecordValidationMessage> {
    let mut messages = Vec::new();
    if boundary_code.value.is_none() {
        return messages;
    }
    let field_positions = vec![
        (area_code.start_column, area_code.end_column),
        (boundary_code.start_column, boundary_code.end_column),
    ];
    if !matches!(
        (&area_code.value, &boundary_code.value),
        (
            Some(CustomerAreaCode::USA),
            Some(
                BoundaryCode::CanadaAlaska
                    | BoundaryCode::LatinAmerica
                    | BoundaryCode::Pacific
                    | BoundaryCode::Europe
                    | BoundaryCode::Africa
            )
        ) | (
            Some(CustomerAreaCode::Canada),
            Some(
                BoundaryCode::USA
                    | BoundaryCode::EasternEurope
                    | BoundaryCode::Europe
                    | BoundaryCode::Pacific
            )
        ) | (
            Some(CustomerAreaCode::Pacific),
            Some(
                BoundaryCode::CanadaAlaska
                    | BoundaryCode::USA
                    | BoundaryCode::EasternEurope
                    | BoundaryCode::Europe
                    | BoundaryCode::MiddleEastSouthAsia
            )
        ) | (
            Some(CustomerAreaCode::LatinAmerica),
            Some(
                BoundaryCode::Pacific
                    | BoundaryCode::USA
                    | BoundaryCode::SouthPacific
                    | BoundaryCode::SouthAmerica
                    | BoundaryCode::Africa
            )
        ) | (
            Some(CustomerAreaCode::SouthPacific),
            Some(
                BoundaryCode::LatinAmerica
                    | BoundaryCode::Pacific
                    | BoundaryCode::SouthAmerica
                    | BoundaryCode::MiddleEastSouthAsia
                    | BoundaryCode::Africa
            )
        ) | (
            Some(CustomerAreaCode::SouthAmerica),
            Some(BoundaryCode::SouthPacific | BoundaryCode::LatinAmerica | BoundaryCode::Africa)
        ) | (
            Some(CustomerAreaCode::Europe),
            Some(
                BoundaryCode::EasternEurope
                    | BoundaryCode::CanadaAlaska
                    | BoundaryCode::USA
                    | BoundaryCode::Africa
                    | BoundaryCode::MiddleEastSouthAsia
            )
        ) | (
            Some(CustomerAreaCode::EasternEurope),
            Some(
                BoundaryCode::Europe
                    | BoundaryCode::MiddleEastSouthAsia
                    | BoundaryCode::Pacific
                    | BoundaryCode::CanadaAlaska
            )
        ) | (
            Some(CustomerAreaCode::MiddleEast),
            Some(
                BoundaryCode::Africa
                    | BoundaryCode::SouthPacific
                    | BoundaryCode::EasternEurope
                    | BoundaryCode::Pacific
                    | BoundaryCode::Europe
            )
        ) | (
            Some(CustomerAreaCode::Africa),
            Some(
                BoundaryCode::SouthAmerica
                    | BoundaryCode::SouthPacific
                    | BoundaryCode::LatinAmerica
                    | BoundaryCode::USA
                    | BoundaryCode::Europe
                    | BoundaryCode::MiddleEastSouthAsia
            )
        )
    ) {
        messages.push(RecordValidationMessage::new(
            "Invalid boundary code for area code",
            field_positions.clone(),
        ));
    }

    messages
}

#[test]
fn test_is_valid_boundary_code() {
    let area_code = RecordField::<CustomerAreaCode> {
        value: Some(CustomerAreaCode::USA),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let boundary_code = RecordField::<BoundaryCode> {
        value: Some(BoundaryCode::LatinAmerica),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let result = is_valid_boundary_code(&area_code, &boundary_code);
    assert_eq!(
        result.len(),
        0,
        "Expected no messages, got {}",
        result.len()
    );

    let area_code = RecordField::<CustomerAreaCode> {
        value: Some(CustomerAreaCode::Canada),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let boundary_code = RecordField::<BoundaryCode> {
        value: Some(BoundaryCode::SouthPacific),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let result = is_valid_boundary_code(&area_code, &boundary_code);
    assert_eq!(result.len(), 1, "Expected 1 message, got {}", result.len());
}

/// Validates the altitude description to ensure we don't have values that are invalid based on the
/// record type
pub fn is_valid_altitude_description(
    record_section: &RecordField<Section>,
    record_subsection: &RecordField<impl ParseableField>,
    altitude_description: &RecordField<CrossingAltitudeDescription>,
) -> Vec<RecordValidationMessage> {
    let mut messages = Vec::new();
    let field_positions = vec![
        (record_section.start_column, record_section.end_column),
        (record_subsection.start_column, record_subsection.end_column),
        (
            altitude_description.start_column,
            altitude_description.end_column,
        ),
    ];

    match altitude_description.value {
        Some(CrossingAltitudeDescription::Between) => match record_section.value {
            Some(Section::Airport) => {
                if !matches!(
                    AirportSubsection::from_bytes(record_subsection.raw_bytes).unwrap_or_default(),
                    Some(
                        AirportSubsection::ApproachProcedures
                            | AirportSubsection::SIDS
                            | AirportSubsection::STARS
                            | AirportSubsection::Communications
                    )
                ) {
                    messages.push(RecordValidationMessage::new(
                        "Invalid altitude description for approach",
                        field_positions.clone(),
                    ));
                }
            }
            Some(Section::Heliport) => {
                if !matches!(
                    HeliportSubsection::from_bytes(record_subsection.raw_bytes).unwrap_or_default(),
                    Some(
                        HeliportSubsection::ApproachProcedures
                            | HeliportSubsection::SIDS
                            | HeliportSubsection::STARS
                            | HeliportSubsection::Communications
                    )
                ) {
                    messages.push(RecordValidationMessage::new(
                        "Invalid altitude description for approach",
                        field_positions.clone(),
                    ));
                }
            }
            Some(Section::Navaid) => {
                if !matches!(
                    NavaidSubsection::from_bytes(record_subsection.raw_bytes).unwrap_or_default(),
                    Some(NavaidSubsection::VHFNavaid)
                ) {
                    messages.push(RecordValidationMessage::new(
                        "Invalid altitude description for approach",
                        field_positions.clone(),
                    ));
                }
            }
            Some(Section::Enroute) => {
                if !matches!(
                    EnrouteSubsection::from_bytes(record_subsection.raw_bytes).unwrap_or_default(),
                    Some(EnrouteSubsection::PreferredRoutes)
                ) {
                    messages.push(RecordValidationMessage::new(
                        "Invalid altitude description for approach",
                        field_positions.clone(),
                    ));
                }
            }
            _ => {}
        },
        Some(CrossingAltitudeDescription::AtOrAboveSecondAltitude) => match record_section.value {
            Some(Section::Airport) => {
                if !matches!(
                    AirportSubsection::from_bytes(record_subsection.raw_bytes).unwrap_or_default(),
                    Some(AirportSubsection::SIDS)
                ) {
                    messages.push(RecordValidationMessage::new(
                        "Invalid altitude description for approach",
                        field_positions.clone(),
                    ));
                }
            }
            Some(Section::Heliport) => {
                if !matches!(
                    HeliportSubsection::from_bytes(record_subsection.raw_bytes).unwrap_or_default(),
                    Some(HeliportSubsection::SIDS)
                ) {
                    messages.push(RecordValidationMessage::new(
                        "Invalid altitude description for approach",
                        field_positions.clone(),
                    ));
                }
            }
            _ => {}
        },
        _ => {}
    }
    messages
}

#[test]
fn test_is_valid_altitude_description() {
    let record_section = RecordField::<Section> {
        value: Some(Section::Airport),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let record_subsection = RecordField::<AirportSubsection> {
        value: Some(AirportSubsection::SIDS),
        raw_bytes: b"D",
        start_column: 0,
        end_column: 0,
    };
    let altitude_description = RecordField::<CrossingAltitudeDescription> {
        value: Some(CrossingAltitudeDescription::Between),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let result =
        is_valid_altitude_description(&record_section, &record_subsection, &altitude_description);
    assert_eq!(
        result.len(),
        0,
        "Expected no messages, got {}",
        result.len()
    );

    let record_section = RecordField::<Section> {
        value: Some(Section::Airport),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let record_subsection = RecordField::<CompanyRoutesSubsection> {
        value: Some(CompanyRoutesSubsection::CompanyRoutes),
        raw_bytes: b" ",
        start_column: 0,
        end_column: 0,
    };

    let altitude_description = RecordField::<CrossingAltitudeDescription> {
        value: Some(CrossingAltitudeDescription::AtOrAboveSecondAltitude),
        raw_bytes: b"",
        start_column: 0,
        end_column: 0,
    };
    let result =
        is_valid_altitude_description(&record_section, &record_subsection, &altitude_description);
    assert_eq!(result.len(), 1, "Expected 1 messages, got {}", result.len());
}
