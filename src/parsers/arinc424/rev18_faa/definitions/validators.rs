use crate::parsers::arinc424::rev18_faa::definitions::*;
use crate::parsers::arinc424::types::fields::ParseableField;
use crate::parsers::arinc424::types::records::*;

#[cfg(test)]
use crate::parsers::arinc424::types::fields::LengthLimitedIdentifier;

pub fn is_valid_reference(
    reference: &RecordField<impl ParseableField>,
    section: &RecordField<Section>,
    subsection: &RecordField<GenericSubsection>,
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
fn test_is_valid_section_subsection_combination() {
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
        raw_bytes: b"",
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
