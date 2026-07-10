//! # ARINC 424 Common types in the scope of records when parsing
//! This module contains the common record parsing types for the ARINC 424 data. These are used in all versions of the parser

use crate::parsers::arinc424::types::fields::{BLANK, FieldParseError, ParseableField};
use std::any::type_name;

#[derive(Debug)]
pub struct RecordParseError {
    pub message: String,
}
impl From<FieldParseError> for RecordParseError {
    fn from(error: FieldParseError) -> Self {
        Self {
            message: if let (Some(column), Some(column_type)) = (error.column, error.column_type) {
                format!(
                    "Record parse error: {} at column {} of type {}",
                    error.message,
                    column,
                    column_type.split("::").last().unwrap_or(&column_type)
                )
            } else {
                format!("Record parse error: {}", error.message)
            },
        }
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
