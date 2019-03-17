//! The checker is responsible for parsing the project data into structs
//! and applying data validation checks.

pub mod config;
pub mod entries;
pub mod lifterdata;
pub mod meet;

pub use crate::Report;
pub use entries::Entry;
pub use lifterdata::{LifterData, LifterDataMap};
pub use meet::Meet;

/// Returns the generated structures and any associated reports.
pub struct CheckResult {
    pub reports: Vec<Report>,
    pub meet: Option<Meet>,
    pub entries: Option<Vec<Entry>>,
}
