//! Checks for meet.csv files.

use chrono::{self, Datelike};
use csv;
use opltypes::*;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use Report;

/// Product of a successful parse.
pub struct Meet {
    pub federation: Federation,
    pub date: Date,
    pub country: Country,
    pub state: Option<State>,
    pub town: Option<String>,
    pub name: String,
}

pub struct CheckResult {
    pub report: Report,
    pub meet: Option<Meet>,
}

/// Every meet.csv must have exactly these headers in the same order.
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
            // The MeetPath is just the stuff after "meet-data/".
            let pathstart: usize = s.rfind("meet-data").unwrap_or(0);

            // Each character may only be alphanumeric ASCII or "/".
            for c in s.chars().skip(pathstart) {
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
pub fn check_federation(s: &str, report: &mut Report) -> Option<Federation> {
    match s.parse::<Federation>() {
        Ok(f) => Some(f),
        Err(_) => {
            report.error(format!(
                "Unknown federation '{}'. \
                 Add to modules/opltypes/src/federation.rs?",
                s
            ));
            None
        }
    }
}

/// Checks the Date column.
pub fn check_date(s: &str, report: &mut Report) -> Option<Date> {
    let date = s.parse::<Date>();
    if date.is_err() {
        report.error(format!("Invalid date '{}'. Must be YYYY-MM-DD", s));
        return None;
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
    if (date.year() > y)
        || (date.year() == y && date.month() > m)
        || (date.year() == y && date.month() == m && date.day() > d)
    {
        report.error(format!("Meet occurs in the future in '{}'", s));
    }

    Some(date)
}

/// Checks the MeetCountry column.
pub fn check_meetcountry(s: &str, report: &mut Report) -> Option<Country> {
    match s.parse::<Country>() {
        Ok(c) => Some(c),
        Err(_) => {
            report.error(format!(
                "Unknown country '{}'. \
                 Add to modules/opltypes/src/country.rs?",
                s
            ));

            // Emit some helpful warnings.
            if s.contains("Chin") {
                report.warning(format!("Should '{}' be 'Taiwan'?", s));
            }

            None
        }
    }
}

/// Checks the optional MeetState column.
pub fn check_meetstate(s: &str, report: &mut Report, country: Option<Country>) -> Option<State> {
    if s.is_empty() {
        return None;
    }

    if country.is_none() {
        report.warning(
            format!("Couldn't check MeetState '{}' due to invalid MeetCountry", s)
        );
        return None;
    }
    let country = country.unwrap();

    match State::from_str_and_country(s, country) {
        Ok(s) => Some(s),
        Err(_) => {
            let cstr = country.to_string();
            report.error(format!("Unknown state '{}' for country '{}'", s, cstr));
            None
        }
    }
}

/// Checks the optional MeetTown column.
pub fn check_meettown(s: &str, report: &mut Report) -> Option<String> {
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

    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

/// Checks the mandatory MeetName column.
pub fn check_meetname(s: &str, report: &mut Report, fedstr: &str, datestr: &str) -> Option<String> {
    if s.is_empty() {
        report.error("MeetName cannot be empty");
        return None;
    }

    for c in s.chars() {
        // Non-ASCII characters are allowed.
        if !c.is_alphanumeric() && !" -&.'/".contains(c) {
            report.error(format!("Illegal character in MeetName '{}'", s));
            break;
        }
    }

    // Check for excessive spacing.
    if s.contains("  ") || s.starts_with(' ') || s.ends_with(' ') {
        report.error(format!("Excessive whitespace in MeetTown '{}'", s));
    }

    // The federation shouldn't be part of the name.
    if !fedstr.is_empty() && s.contains(fedstr) {
        report.error(format!("MeetName '{}' must not contain the federation", s));
    }

    // The year shouldn't be part of the name.
    if let Some(idx) = datestr.find('-') {
        let year = &datestr[0..idx];
        if s.contains(year) {
            report.error(format!("MeetName '{}' must not contain the year", s));
        }
    }

    Some(s.to_string())
}

/// Checks a single meet.csv file from an open `csv::Reader`.
///
/// Extracting this out into a `Reader`-specific function is useful
/// for creating tests that do not have a backing CSV file.
pub fn do_check<R>(
    rdr: &mut csv::Reader<R>,
    mut report: Report,
) -> Result<CheckResult, Box<Error>>
where
    R: io::Read,
{
    // Remember the number of errors at the start.
    // If the number increased during checking, don't return a parsed Meet struct.
    let initial_errors = report.count_errors();

    // Verify column headers. Only continue if they're valid.
    check_headers(rdr.headers()?, &mut report);
    if !report.messages.is_empty() {
        return Ok(CheckResult {report, meet: None});
    }

    // Read a single row.
    let mut record = csv::StringRecord::new();
    if !rdr.read_record(&mut record)? {
        report.error("The second row is missing");
        return Ok(CheckResult {report, meet: None});
    }

    let federation = check_federation(record.get(0).unwrap(), &mut report);
    let date = check_date(record.get(1).unwrap(), &mut report);
    let country = check_meetcountry(record.get(2).unwrap(), &mut report);
    let state = check_meetstate(
        record.get(3).unwrap(),
        &mut report,
        country
    );
    let town = check_meettown(record.get(4).unwrap(), &mut report);
    let name = check_meetname(
        record.get(5).unwrap(),
        &mut report,
        record.get(0).unwrap(),
        record.get(1).unwrap(),
    );

    // Attempt to read another row -- but there shouldn't be one.
    if rdr.read_record(&mut record)? {
        report.error("Too many rows");
    }

    // If all mandatory data is present, and there were no errors,
    // forward a post-parsing Meet struct to the Entry-parsing phase.
    if initial_errors == report.count_errors() &&
        federation.is_some() &&
        date.is_some() &&
        country.is_some() &&
        name.is_some()
    {
        let meet = Meet {
            federation: federation.unwrap(),
            date: date.unwrap(),
            country: country.unwrap(),
            state,
            town,
            name: name.unwrap(),
        };
        Ok(CheckResult {report, meet: Some(meet)})
    } else {
        Ok(CheckResult {report, meet: None})
    }
}

/// Checks a single meet.csv file by path.
pub fn check_meet(meet_csv: PathBuf) -> Result<CheckResult, Box<Error>> {
    // Allow the pending Report to own the PathBuf.
    let mut report = Report::new(meet_csv);

    // The meet.csv file must exist.
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(CheckResult {report, meet: None});
    }

    check_meetpath(&mut report);

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(&report.path)?;

    Ok(do_check(&mut rdr, report)?)
}
