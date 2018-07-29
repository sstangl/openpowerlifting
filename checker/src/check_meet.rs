//! Checks for meet.csv files.

use csv;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use Report;

/// Every meet.csv must have excatly these headers in the same order.
const KNOWN_HEADERS: [&str; 6] = [
    "Federation",
    "Date",
    "MeetCountry",
    "MeetState",
    "MeetTown",
    "MeetName",
];

/// Checks that the headers are fixed and correct.
fn check_headers(headers: &csv::StringRecord, report: &mut Report) {
    // Check header length.
    if headers.len() != KNOWN_HEADERS.len() {
        report.error(format!("There must be {} columns", KNOWN_HEADERS.len()));
        return;
    }

    // Check header exact value.
    for (i, header) in headers.iter().enumerate() {
        if header != KNOWN_HEADERS[i] {
            report.error(format!("Column {} must be '{}'", i, KNOWN_HEADERS[i]));
        }
    }
}

/// Checks that the MeetPath contains only characters valid in a URL.
pub fn check_meetpath(report: &mut Report) {
    // Because the report owns the path, we can't call mutable methods
    // like error() and warning() until the path reference is dropped.
    // Instead of allocating a heap-vec, just remember what errors occurred.
    let mut ascii_error = false;
    let mut parent_error = false;
    let mut utf8_error = false;

    // The original Path is to the file, so get the parent directory.
    if let Some(parent) = report.path.parent() {
        if let Some(s) = parent.to_str() {
            // Each character may only be alphanumeric ASCII or "/".
            for c in s.chars() {
                if !c.is_ascii_alphanumeric() && c != '/' && c != '-' {
                    ascii_error = true;
                }
            }
        } else {
            utf8_error = true;
        }
    } else {
        parent_error = true;
    }

    // With the reference to report.path dropped, report any errors.
    if utf8_error {
        report.error("Path contains non-UTF8 characters");
    }
    if ascii_error {
        report.error("Path must only contain alphanumeric ASCII or '/-' characters");
    }
    if parent_error {
        report.error("Path had insufficient parent directories");
    }
}

/// Checks a single meet.csv file from an open `csv::Reader`.
///
/// Extracting this out into a `Reader`-specific function is useful
/// for creating tests that do not have a backing CSV file.
pub fn do_check<R>(
    rdr: &mut csv::Reader<R>,
    mut report: Report,
) -> Result<Report, Box<Error>>
where
    R: io::Read,
{
    // Verify column headers. Only continue if they're valid.
    check_headers(rdr.headers()?, &mut report);
    if !report.messages.is_empty() {
        return Ok(report);
    }

    Ok(report)
}

/// Checks a single meet.csv file by path.
pub fn check_meet(meet_csv: PathBuf) -> Result<Report, Box<Error>> {
    // Allow the pending Report to own the PathBuf.
    let mut report = Report::new(meet_csv);

    // The meet.csv file must exist.
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(report);
    }

    check_meetpath(&mut report);

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .from_path(&report.path)?;

    Ok(do_check(&mut rdr, report)?)
}
