//! Implementation of server functionality.

// Suppress clippy warnings for date literals.
#![allow(clippy::inconsistent_digit_grouping)]
#![allow(clippy::zero_prefixed_literal)]

// External dependencies.
#[macro_use]
extern crate serde_derive;

// Exported modules.
pub mod pages;
pub mod referring_path;
