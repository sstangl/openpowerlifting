//! Checks for meet.csv files.

use chrono::{self, Datelike};
use csv;
use opltypes::*;

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

/// Checks the Federation column.
pub fn check_federation(s: &str, report: &mut Report) {
    if s.parse::<Federation>().is_err() {
        report.error(format!("Unknown federation '{}'. \
                              Add to modules/opltypes/src/federation.rs?", s));
    }
}

/// Checks the Date column.
pub fn check_date(s: &str, report: &mut Report) {
    let date = s.parse::<Date>();
    if date.is_err() {
        report.error(format!("Invalid date '{}'. Must be YYYY-MM-DD", s));
        return;
    }
    let date = date.unwrap();

    // The date should not be implausibly long ago.
    if date.year() < 1945 {
        report.error(format!("Implausible year in '{}'", s));
    }

    // This is sufficiently fast to call that caching is of no practical benefit.
    let now = chrono::Local::now();

    // The date should not be in the future.
    let (y, m, d) = (now.year() as u32, now.month() as u32, now.day() as u32);
    if (date.year() > y) ||
       (date.year() == y && date.month() > m) ||
       (date.year() == y && date.month() == m && date.day() > d)
    {
        report.error(format!("Meet occurs in the future in '{}'", s));
    }
}

/// Checks the MeetCountry column.
pub fn check_country(s: &str, report: &mut Report) {
    if s.parse::<Country>().is_err() {
        report.error(format!("Unknown country '{}'. \
                              Add to modules/opltypes/src/country.rs?", s));

        // Emit some helpful warnings.
        if s.contains("Chin") {
            report.warning(format!("Should '{}' be 'Taiwan'?", s));
        }
    }
}

/// Checks the optional MeetTown column.
pub fn check_town(s: &str, report: &mut Report) {
    // Check each character for validity.
    for c in s.chars() {
        // Non-ASCII characters are allowed.
        if !c.is_alphabetic() && !" -.'".contains(c) {
            report.error(format!("Illegal character in MeetTown '{}'", s));
            break;
        }
    }

    // Check for excessive spacing.
    if s.contains("  ") || s.starts_with(' ') || s.ends_with(' ') {
        report.error(format!("Excessive whitespace in MeetTown '{}'", s));
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

    // Read a single row.
    let mut record = csv::StringRecord::new();
    if !rdr.read_record(&mut record)? {
        report.error("The second row is missing");
        return Ok(report);
    }

    check_federation(record.get(0).unwrap(), &mut report);
    check_date(record.get(1).unwrap(), &mut report);
    check_country(record.get(2).unwrap(), &mut report);
    check_town(record.get(4).unwrap(), &mut report);

    // Attempt to read another row -- but there shouldn't be one.
    if rdr.read_record(&mut record)? {
        report.error("Too many rows");
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
