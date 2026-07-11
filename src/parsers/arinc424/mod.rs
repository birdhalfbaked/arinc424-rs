//! # ARINC 424
//! This module contains the core parser for the ARINC 424 data.
//!
//! References are to the various supported revisions of the ARINC 424 standard which currently target:
//! - ARINC 424-18
//! - ARINC 424-23
//!
//! The versions selected are because of the edge revision as of writing this, and to support the FAA CIFP files which
//! rely on the -18 revision.
pub mod rev18;
pub mod rev23;

pub mod parser;
pub mod types;
