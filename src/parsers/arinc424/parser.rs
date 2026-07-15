//! # ARINC 424 Parser
//! This module contains the parser for the ARINC 424 data. It is a wrapper around the various revisions
//! we support in the package for processing data. This allows for easy revision switching without
//! making us dynamically allocate a bunch more heap objects to handle dynamicity within a trait-based approach,
//! so while it might be a bit more verbose, it is more efficient.

use crate::parsers::arinc424::rev18::records::record::ARINCRecord as Rev18ArincRecord;
use crate::parsers::arinc424::rev18_faa::records::record::ARINCRecord as Rev18FAAArincRecord;
use crate::parsers::arinc424::rev23::records::record::ARINCRecord as Rev23ArincRecord;

use crate::parsers::arinc424::types::records::{RecordError, RecordValidationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arinc424Version {
    Rev18,
    Rev18FAA,
    Rev23,
}

#[derive(Debug)]
pub enum Arinc424VersionedRecord<'a> {
    Rev18(Rev18ArincRecord<'a>),
    Rev18FAA(Rev18FAAArincRecord<'a>),
    Rev23(Rev23ArincRecord<'a>),
}

impl<'a> Arinc424VersionedRecord<'a> {
    pub fn validate(&self) -> Result<(), RecordValidationError> {
        match self {
            Self::Rev18(_) => Ok(()),
            Self::Rev18FAA(record) => record.validate(),
            Self::Rev23(_) => Ok(()),
        }
    }
}

#[derive(Debug)]
pub enum Arinc424Parser {
    Rev18,
    Rev18FAA,
    Rev23,
}

impl Arinc424Parser {
    pub fn new(version: Arinc424Version) -> Self {
        match version {
            Arinc424Version::Rev18 => Self::Rev18,
            Arinc424Version::Rev18FAA => Self::Rev18FAA,
            Arinc424Version::Rev23 => Self::Rev23,
        }
    }

    pub fn parse<'a>(&self, input: &'a [u8]) -> Result<Arinc424VersionedRecord<'a>, RecordError> {
        let record = match self {
            Self::Rev18 => Ok(Arinc424VersionedRecord::Rev18(Rev18ArincRecord::parse(
                input,
            )?)),
            Self::Rev18FAA => Ok(Arinc424VersionedRecord::Rev18FAA(
                Rev18FAAArincRecord::parse(input)?,
            )),
            Self::Rev23 => Ok(Arinc424VersionedRecord::Rev23(Rev23ArincRecord::parse(
                input,
            )?)),
        };
        match record {
            Ok(record) => match record.validate() {
                Ok(()) => Ok(record),
                Err(e) => Err(RecordError::from(
                    e.with_raw_line(String::from_utf8_lossy(input).to_string()),
                )),
            },
            Err(e) => Err(e),
        }
    }
}

#[test]
fn test_parse_rev23_vhf_navaid_primary_record() {
    let input = b"SUSAD KFATK2 IRPW  K2011130 ITWN                   IRPWN36471081W119435663E0130003470     NARFRESNO YOSEMITE INTL          261851713";
    let parser = Arinc424Parser::new(Arinc424Version::Rev23);
    let record = parser.parse(input).unwrap();
    assert!(matches!(
        record,
        Arinc424VersionedRecord::Rev23(Rev23ArincRecord::VHFNavaidPrimary(_))
    ));
}
