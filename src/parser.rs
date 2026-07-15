//! # ARINC 424 Parser
//! This module contains the parser for the ARINC 424 data. It is a wrapper around the various revisions
//! we support in the package for processing data. This allows for easy revision switching without
//! making us dynamically allocate a bunch more heap objects to handle dynamicity within a trait-based approach,
//! so while it might be a bit more verbose, it is more efficient.

#[cfg(feature = "rev18")]
use crate::rev18::records::record::ARINCRecord as Rev18ArincRecord;
#[cfg(feature = "rev18_faa")]
use crate::rev18_faa::records::record::ARINCRecord as Rev18FAAArincRecord;
#[cfg(feature = "rev23")]
use crate::rev23::records::record::ARINCRecord as Rev23ArincRecord;

use crate::types::records::{RecordError, RecordValidationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arinc424Version {
    #[cfg(feature = "rev18")]
    Rev18,
    #[cfg(feature = "rev18_faa")]
    Rev18FAA,
    #[cfg(feature = "rev23")]
    Rev23,
}

#[derive(Debug)]
pub enum Arinc424VersionedRecord<'a> {
    #[cfg(feature = "rev18")]
    Rev18(Rev18ArincRecord<'a>),
    #[cfg(feature = "rev18_faa")]
    Rev18FAA(Rev18FAAArincRecord<'a>),
    #[cfg(feature = "rev23")]
    Rev23(Rev23ArincRecord<'a>),
}

impl<'a> Arinc424VersionedRecord<'a> {
    pub fn validate(&self) -> Result<(), RecordValidationError> {
        match self {
            #[cfg(feature = "rev18")]
            Self::Rev18(_) => Ok(()),
            #[cfg(feature = "rev18_faa")]
            Self::Rev18FAA(record) => record.validate(),
            #[cfg(feature = "rev23")]
            Self::Rev23(_) => Ok(()),
        }
    }
}

#[derive(Debug)]
pub enum Arinc424Parser {
    #[cfg(feature = "rev18")]
    Rev18,
    #[cfg(feature = "rev18_faa")]
    Rev18FAA,
    #[cfg(feature = "rev23")]
    Rev23,
}

impl Arinc424Parser {
    pub fn new(version: Arinc424Version) -> Self {
        match version {
            #[cfg(feature = "rev18")]
            Arinc424Version::Rev18 => Self::Rev18,
            #[cfg(feature = "rev18_faa")]
            Arinc424Version::Rev18FAA => Self::Rev18FAA,
            #[cfg(feature = "rev23")]
            Arinc424Version::Rev23 => Self::Rev23,
        }
    }

    pub fn parse<'a>(&self, input: &'a [u8]) -> Result<Arinc424VersionedRecord<'a>, RecordError> {
        let record = match self {
            #[cfg(feature = "rev18")]
            Self::Rev18 => Ok(Arinc424VersionedRecord::Rev18(Rev18ArincRecord::parse(
                input,
            )?)),
            #[cfg(feature = "rev18_faa")]
            Self::Rev18FAA => Ok(Arinc424VersionedRecord::Rev18FAA(
                Rev18FAAArincRecord::parse(input)?,
            )),
            #[cfg(feature = "rev23")]
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
