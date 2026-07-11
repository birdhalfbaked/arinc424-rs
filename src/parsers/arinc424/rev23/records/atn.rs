use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError, is_primary_record};
use crate::parsers::arinc424::rev23::definitions::*;
pub(super) struct ATNRecords;
impl ATNRecords {
    const CONTINUATION_COLUMN: usize = 19;

    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        if is_primary_record(input, Self::CONTINUATION_COLUMN) {
            Ok(ARINCRecord::ATNDataPrimary(ATNDataPrimaryRecord::parse(
                input,
            )?))
        } else {
            Err(RecordParseError {
                message: "Invalid record type".to_string(),
            })
        }
    }
}

/// 4.1.37.1 ATN Data Record Primary
#[derive(Debug)]
pub struct ATNDataPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub ground_facility_identifier: RecordField<'a, ATNGroundFacilityIdentifier>,
    pub continuation_record_number: RecordField<'a, ContinuationRecordNumber>,
    pub authority_and_format_identifier: RecordField<'a, ATNAuthorityFormatIdentifier>,
    pub initial_domain_identifier: RecordField<'a, ATNInitialDomainIdentifier>,
    pub version_identifier: RecordField<'a, ATNVersion>,
    pub administrative_identifier: RecordField<'a, ATNAdministration>,
    pub routing_domain_format: RecordField<'a, ATNRoutingDomainFormat>,
    pub administrative_region_selector: RecordField<'a, ATNAdministrativeRegionSelector>,
    pub location_identifier: RecordField<'a, ATNRoutingLocation>,
    pub system_identifier: RecordField<'a, ATNSystemIdentifier>,
    pub network_service_access_point_selector:
        RecordField<'a, ATNNetworkServiceAccessPointSelector>,
    pub context_management_transport_selector:
        RecordField<'a, ATNContextManagementTransportSelector>,
    pub use_indicator: RecordField<'a, ATNATSUGroundFacilityUseIndicator>,
    pub fir_uir_name: RecordField<'a, FirUirName>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

impl<'a> ATNDataPrimaryRecord<'a> {
    pub fn parse(input: &'a [u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type: RecordField::from_bytes(input, 1, 1)?,
            section: RecordField::from_bytes(input, 5, 1)?,
            subsection: RecordField::from_bytes(input, 6, 1)?,
            ground_facility_identifier: RecordField::from_bytes(input, 7, 8)?,
            continuation_record_number: RecordField::from_bytes(input, 19, 1)?,
            authority_and_format_identifier: RecordField::from_bytes(input, 20, 2)?,
            initial_domain_identifier: RecordField::from_bytes(input, 22, 4)?,
            version_identifier: RecordField::from_bytes(input, 26, 2)?,
            administrative_identifier: RecordField::from_bytes(input, 28, 6)?,
            routing_domain_format: RecordField::from_bytes(input, 34, 2)?,
            administrative_region_selector: RecordField::from_bytes(input, 36, 6)?,
            location_identifier: RecordField::from_bytes(input, 42, 4)?,
            system_identifier: RecordField::from_bytes(input, 46, 12)?,
            network_service_access_point_selector: RecordField::from_bytes(input, 58, 2)?,
            context_management_transport_selector: RecordField::from_bytes(input, 60, 4)?,
            use_indicator: RecordField::from_bytes(input, 64, 1)?,
            fir_uir_name: RecordField::from_bytes(input, 65, 25)?,
            file_record_number: RecordField::from_bytes(input, 124, 5)?,
            cycle_date: RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
