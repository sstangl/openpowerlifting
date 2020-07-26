//! Implementation of server functionality.

// External dependencies.
#[macro_use]
extern crate serde_derive;

// Exported modules.
pub mod pages;

mod query_parser;
pub use query_parser::*;
