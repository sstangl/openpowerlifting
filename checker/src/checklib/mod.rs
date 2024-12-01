//! The checker is responsible for parsing the project data into structs
//! and applying data validation checks.

pub mod config;
pub mod consistency;
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
    pub entries: Option<Box<[Entry]>>,
}

/// Creates a [csv::ReaderBuilder], used to read CSV files.
///
/// Creating the ReaderBuilder in a central location and sharing it is
/// an optimization: internally, each time a ReaderBuilder is created,
/// it must construct a new DFA. Building that again and again for
/// each file took about 5% of total program execution.
pub fn compile_csv_reader(allow_crlf: bool) -> csv::ReaderBuilder {
    let mut reader = csv::ReaderBuilder::new();
    reader.quoting(false);
    if !allow_crlf {
        reader.terminator(csv::Terminator::Any(b'\n'));
    }
    reader
}
