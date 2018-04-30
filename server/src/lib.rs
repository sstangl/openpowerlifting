//! Implementation of server functionality.

#![feature(path_ancestors)]

// External dependencies.
extern crate csv;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate strum_macros;

// Exported modules.
pub mod langpack;
pub mod opldb;
pub mod pages;
