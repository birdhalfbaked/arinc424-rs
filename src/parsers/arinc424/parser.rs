//! # ARINC 424 Parser

//! This module contains the parser for the ARINC 424 data.
//! It is used to parse a line of ARINC 424 data into a single record.
//!
//! ARINC data is inherently sequential, so there is no concern with this approach
//! Future work will likely be in processing large chunks of data or files instead of single lines.
//! Since it's early, I deem this acceptable for now.
//!
use crate::parsers::arinc424::records::{ARINCRecord, RecordParseError};

/// Arinc424Parser is the main parser structure for the ARINC 424 data.
pub struct Arinc424Parser;

impl Arinc424Parser {
    pub fn parse(input: &[u8]) -> Result<ARINCRecord<'_>, RecordParseError> {
        // record types are the
        match ARINCRecord::parse(input) {
            Ok(record) => Ok(record),
            Err(error) => {
                return Err(error);
            }
        }
    }
}

#[test]
fn test_parse_vhf_navaid_primary_record() {
    let input = b"SUSAD KFATK2 IRPW  K2011130 ITWN                   IRPWN36471081W119435663E0130003470     NARFRESNO YOSEMITE INTL          261851713";
    let record = Arinc424Parser::parse(input).unwrap();
    assert!(matches!(record, ARINCRecord::VHFNavaidPrimary(_)));
    println!("{:?}", record);
}
