
use crate::parsers::arinc424::rev23::records::record::ARINCRecord;
use crate::parsers::arinc424::types::records::{RecordField, RecordParseError};
use crate::parsers::arinc424::rev23::definitions::*;
pub(super) struct CommunicationTypeTranslationRecords;
impl CommunicationTypeTranslationRecords {
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        Ok(ARINCRecord::CommunicationTypeTranslationPrimary(
            CommunicationTypeTranslationPrimaryRecord::parse(input)?,
        ))
    }
}

/// 4.1.34.1 Communication Type Translation Primary Record
#[derive(Debug)]
pub struct CommunicationTypeTranslationPrimaryRecord<'a> {
    pub record_type: RecordField<'a, RecordType>,
    pub section: RecordField<'a, Section>,
    pub subsection: RecordField<'a, GenericSubsection>,
    pub communication_type: RecordField<'a, CommunicationsType>,
    pub type_recognized_by: RecordField<'a, CommunicationsTypeRecognizedBy>,
    pub translation: RecordField<'a, CommunicationsTypeTranslation>,
    pub used_on: RecordField<'a, CommunicationsUsedOn>,
    pub communications_class: RecordField<'a, CommunicationsClass>,
    pub file_record_number: RecordField<'a, FileRecordNumber>,
    pub cycle_date: RecordField<'a, CycleDate>,
}

#[rustfmt::skip]
impl<'a> CommunicationTypeTranslationPrimaryRecord<'a> {
    pub fn parse(input: &'a[u8]) -> Result<Self, RecordParseError> {
        Ok(Self {
            record_type:          RecordField::from_bytes(input, 1, 1)?,
            section:              RecordField::from_bytes(input, 5, 1)?,
            subsection:           RecordField::from_bytes(input, 6, 1)?,
            communication_type:   RecordField::from_bytes(input, 7, 3)?,
            type_recognized_by:   RecordField::from_bytes(input, 10, 1)?,
            translation:          RecordField::from_bytes(input, 11, 80)?,
            used_on:              RecordField::from_bytes(input, 91, 1)?,
            communications_class: RecordField::from_bytes(input, 92, 4)?,
            file_record_number:   RecordField::from_bytes(input, 124, 5)?,
            cycle_date:           RecordField::from_bytes(input, 129, 4)?,
        })
    }
}
