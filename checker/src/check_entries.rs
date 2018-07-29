//! Checks for entries.csv files.

use csv;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use Report;

/// Checks a single entries.csv file from an open CSV `Reader`.
///
/// Extracting this out into a `Reader`-specific function is useful
/// for creating tests that do not have a backing CSV file.
pub fn do_check<R>(
    rdr: &mut csv::Reader<R>,
    mut report: Report,
) -> Result<Report, Box<Error>>
    where R: io::Read
{
    // Succeeds even on the empty file.
    let headers = rdr.headers()?;
    if headers.is_empty() {
        report.error("No column headers");
        return Ok(report);
    }

    Ok(report)
}

/// Checks a single entries.csv file by path.
pub fn check_entries(entries_csv: PathBuf) -> Result<Report, Box<Error>> {
    // Allow the pending Report to own the PathBuf.
    let mut report = Report::new(entries_csv);

    // The entries.csv file must exist.
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(report);
    }

    let mut rdr = csv::Reader::from_path(&report.path)?;
    Ok(do_check(&mut rdr, report)?)
}
