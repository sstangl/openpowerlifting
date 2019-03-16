//! The checker is responsible for parsing the project data into structs
//! and applying data validation checks.

pub mod config;
pub mod entries;
pub mod meet;

use crate::Report;
use entries::Entry;
use meet::Meet;

/// Returns the generated structures and any associated reports.
pub struct CheckResult {
    pub reports: Vec<Report>,
    pub meet: Option<Meet>,
    pub entries: Option<Vec<Entry>>,
}
