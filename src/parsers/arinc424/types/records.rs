//! # ARINC 424 Common types in the scope of records when parsing
//! This module contains the common record parsing types for the ARINC 424 data. These are used in all versions of the parser
//!
//! The is_primary_record function is used to determine if a record is a primary record but is also
//! fairly misplaced imo. Will leave it here for now as it is not the highest of prios to refactor this.
//! Core prio is stability across revisions.

use crate::parsers::arinc424::types::fields::{BLANK, FieldParseError, ParseableField};
use std::any::type_name;

#[derive(Debug)]
pub struct RecordParseError {
    pub message: String,
    pub raw_line: String,
}

impl RecordParseError {
    pub fn new(message: String, raw_line: Option<String>) -> Self {
        Self {
            message,
            raw_line: raw_line.unwrap_or_else(|| "".to_string()),
        }
    }
}
impl From<FieldParseError> for RecordParseError {
    fn from(error: FieldParseError) -> Self {
        let message = if let (Some(column), Some(column_type)) = (error.column, error.column_type) {
            format!(
                "Record parse error: {} at column {} of type {}",
                error.message,
                column,
                column_type.split("::").last().unwrap_or(&column_type)
            )
        } else {
            format!("Record parse error: {}", error.message)
        };
        Self::new(message, None)
    }
}
#[derive(Debug)]
pub struct RecordField<'a, T> {
    pub raw_bytes: &'a [u8],
    pub value: Option<T>,
}
impl<'a, T: ParseableField> RecordField<'a, T> {
    pub fn from_bytes(
        input: &'a [u8],
        column: usize,
        length: usize,
    ) -> Result<Self, FieldParseError> {
        // to make it 1:1 with the spec, let's use 1-indexed columns
        let value = T::from_bytes(&input[column - 1..column - 1 + length])
            .map_err(|e| e.with_context(type_name::<T>(), column))?;
        Ok(Self {
            raw_bytes: &input[column - 1..column - 1 + length],
            value,
        })
    }
}

// Layout dispatch helpers
pub fn is_primary_record(input: &[u8], continuation_column: usize) -> bool {
    matches!(input[continuation_column - 1], b'0' | b'1' | BLANK)
}
