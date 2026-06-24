//! # ARINC 424 Field Parsers
//! This module contains the parsers for the ARINC 424 field types.
//!
//! ## Field Types
//! - Alpha
//! - Alphanumeric
//! - Numeric
//!
//! ## Field Parsers
//! - FieldRaw
//! - FieldParseError
//!
//! ## Raw Fields
//! We define raw field types that do a minimum data validation for first pass data loading.
//! Since Generative LLMs were used to help generate the field names, there may be errors in what the field is supposed to be.
//! To ensure quality, human-verified raw fields are denoted with ✅
//!
//! ### Variants
//!
//! Variants are denoted with (A), (B), (C), (D), etc.
//! They are used ONLY when the length between records is necessarily different.
//! If there are conditionally Numeric AND Alpha fields, it is preferred to use the Alphanumeric field type.
//! and validate on record level later on.
#![allow(non_camel_case_types)]

pub const BLANK: u8 = b' ';
#[derive(Debug, PartialEq, Eq)]
pub struct FieldParseError {
    pub message: String,
}
