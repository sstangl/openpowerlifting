//! CSV data export for the Meet page.

use opldb::{self, Entry};

use std::error;

use crate::pages::api_liftercsv;

/// Export single-meet data as a CSV file.
pub fn export_csv(
    opldb: &opldb::OplDb,
    meet_id: u32,
    entry_filter: Option<fn(&opldb::OplDb, &Entry) -> bool>,
) -> Result<String, Box<dyn error::Error>> {
    let meet = opldb.meet(meet_id);
    let mut entries = opldb.entries_for_meet(meet_id);

    if let Some(f) = entry_filter {
        entries.retain(|e| f(opldb, e));
    }

    // Build the CSV output.
    let mut wtr = csv::Writer::from_writer(vec![]);
    for entry in entries.into_iter().rev() {
        let lifter = opldb.lifter(entry.lifter_id);
        wtr.serialize(api_liftercsv::make_export_row(lifter, entry, meet))?;
    }

    Ok(String::from_utf8(wtr.into_inner()?)?)
}
