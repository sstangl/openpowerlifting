//! Implementation of server functionality.

// External dependencies.
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate strum_macros;

// Exported modules.
pub mod langpack;
pub mod pages;

mod query_parser;
pub use query_parser::*;
