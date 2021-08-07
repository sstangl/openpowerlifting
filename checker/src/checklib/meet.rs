//! Checks for meet.csv files.

use chrono::{self, Datelike};
use opltypes::states::*;
use opltypes::*;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use crate::checklib::config::Config;
use crate::Report;

/// Product of a successful parse.
pub struct Meet {
    pub path: String,
    pub federation: Federation,
    pub date: Date,
    pub country: Country,
    pub state: Option<State>,
    pub town: Option<String>,
    pub name: String,
    pub ruleset: RuleSet,
}

impl Meet {
    /// Generates a default Meet for purposes of testing.
    #[cfg(test)]
    pub fn test_default() -> Meet {
        Meet {
            path: "test/1901".to_string(),
            federation: Federation::WRPF,
            date: Date::from_parts(2019, 03, 01),
            country: opltypes::Country::USA,
            state: None,
            town: None,
            name: "Test Meet".to_string(),
            ruleset: RuleSet::default(),
        }
    }
}

pub struct MeetCheckResult {
    pub report: Report,
    pub meet: Option<Meet>,
}

/// Every meet.csv must have exactly these headers in the same order.
const REQUIRED_HEADERS: [&str; 6] = [
    "Federation",
    "Date",
    "MeetCountry",
    "MeetState",
    "MeetTown",
    "MeetName",
];

/// Optional headers may appear after the required ones.
const OPTIONAL_HEADERS: [&str; 1] = ["RuleSet"];

/// Checks that the headers are fixed and correct.
fn check_headers(headers: &csv::StringRecord, report: &mut Report) {
    // Check header length.
    let minheaders = REQUIRED_HEADERS.len();
    let maxheaders = REQUIRED_HEADERS.len() + OPTIONAL_HEADERS.len();

    if headers.len() < minheaders {
        report.error(format!("There must be at least {} columns", minheaders));
        return;
    } else if headers.len() > maxheaders {
        report.error(format!("There can be at most {} columns", maxheaders));
        return;
    }

    // Check required headers.
    for (i, header) in headers.iter().take(REQUIRED_HEADERS.len()).enumerate() {
        if header != REQUIRED_HEADERS[i] {
            report.error(format!("Column {} must be '{}'", i, REQUIRED_HEADERS[i]));
        }
    }

    // Check optional headers.
    for header in headers.iter().skip(REQUIRED_HEADERS.len()) {
        if !OPTIONAL_HEADERS.contains(&header) {
            report.error(format!("Unknown optional column '{}'", &header));
        }
    }
}

/// Checks that the MeetPath contains only characters valid in a URL.
pub fn check_meetpath(report: &mut Report) -> Option<String> {
    match opltypes::file_to_meetpath(&report.path) {
        Ok(s) => Some(s),
        Err(MeetPathError::NonAsciiError) => {
            report.error("Path must only contain alphanumeric ASCII or '/-' characters");
            None
        }
        Err(MeetPathError::FilesystemUTF8Error) => {
            report.error("Path contains non-UTF8 characters");
            None
        }
        Err(MeetPathError::ParentLookupError) => {
            report.error("Path had insufficient parent directories");
            None
        }
        Err(MeetPathError::MeetDataDirNotFoundError) => {
            report.error("Could not find the meet data directory");
            None
        }
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

    // The date should exist in the Gregorian calendar.
    if !date.is_valid() {
        let msg = format!("Date '{}' does not exist in the Gregorian calendar", s);
        report.error(msg);
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
        report.warning(format!(
            "Couldn't check MeetState '{}' due to invalid MeetCountry",
            s
        ));
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
        if !c.is_alphanumeric() && !" -&.'/°%:".contains(c) {
            report.error(format!("Illegal character in MeetName '{}'", s));
            break;
        }
    }

    // Check for excessive spacing.
    if s.contains("  ") || s.starts_with(' ') || s.ends_with(' ') {
        report.error(format!("Excessive whitespace in MeetName '{}'", s));
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

/// Gets the default RuleSet for this meet.
fn configured_ruleset(config: Option<&Config>, date: Option<Date>) -> RuleSet {
    // If there is incomplete specification, just use the defaults.
    if config.is_none() || date.is_none() {
        return RuleSet::default();
    }

    let config = config.unwrap();
    let date = date.unwrap();

    // Take the first RuleSet that matches the given Date.
    for section in config.rulesets.iter() {
        if date >= section.date_min && date <= section.date_max {
            return section.ruleset;
        }
    }

    // If none match, just use the default.
    RuleSet::default()
}

/// Checks the optional RuleSet column.
fn check_ruleset(s: &str, report: &mut Report) -> RuleSet {
    match s.parse::<RuleSet>() {
        Ok(ruleset) => ruleset,
        Err(_) => {
            report.error(format!("Failed parsing RuleSet '{}'", s));
            RuleSet::default()
        }
    }
}

/// Checks a single meet.csv file from an open `csv::Reader`.
///
/// Extracting this out into a `Reader`-specific function is useful
/// for creating tests that do not have a backing CSV file.
pub fn do_check<R>(
    rdr: &mut csv::Reader<R>,
    config: Option<&Config>,
    mut report: Report,
    meetpath: String,
) -> Result<MeetCheckResult, Box<dyn Error>>
where
    R: io::Read,
{
    // Remember the number of errors at the start.
    // If the number increased during checking, don't return a parsed Meet struct.
    let initial_errors = report.count_errors();

    // Verify column headers. Only continue if they're valid.
    check_headers(rdr.headers()?, &mut report);
    if !report.messages.is_empty() {
        return Ok(MeetCheckResult { report, meet: None });
    }

    // Read a single row.
    let mut record = csv::StringRecord::new();
    if !rdr.read_record(&mut record)? {
        report.error("The second row is missing");
        return Ok(MeetCheckResult { report, meet: None });
    }

    // Check the required columns.
    let federation = check_federation(record.get(0).unwrap(), &mut report);
    let date = check_date(record.get(1).unwrap(), &mut report);
    let country = check_meetcountry(record.get(2).unwrap(), &mut report);
    let state = check_meetstate(record.get(3).unwrap(), &mut report, country);
    let town = check_meettown(record.get(4).unwrap(), &mut report);
    let name = check_meetname(
        record.get(5).unwrap(),
        &mut report,
        record.get(0).unwrap(),
        record.get(1).unwrap(),
    );

    // Check the optional columns.
    // The RuleSet is set to the federation default, unless it's overridden.
    let ruleset = if record.len() > REQUIRED_HEADERS.len() {
        check_ruleset(record.get(REQUIRED_HEADERS.len()).unwrap(), &mut report)
    } else {
        configured_ruleset(config, date)
    };

    // Attempt to read another row -- but there shouldn't be one.
    if rdr.read_record(&mut record)? {
        report.error("Too many rows");
    }

    // If there were errors, return early.
    if initial_errors != report.count_errors() {
        return Ok(MeetCheckResult { report, meet: None });
    }

    match (federation, date, country, name) {
        (Some(federation), Some(date), Some(country), Some(name)) => {
            let meet = Some(Meet {
                path: meetpath,
                federation,
                date,
                country,
                state,
                town,
                name,
                ruleset,
            });
            Ok(MeetCheckResult { report, meet })
        }
        _ => Ok(MeetCheckResult { report, meet: None }),
    }
}

/// Checks a single meet.csv string, used by the server.
pub fn check_meet_from_string(
    reader: &csv::ReaderBuilder,
    meet_csv: &str,
) -> Result<MeetCheckResult, Box<dyn Error>> {
    let report = Report::new(PathBuf::from("uploaded/content"));
    let mut rdr = reader.from_reader(meet_csv.as_bytes());
    do_check(&mut rdr, None, report, "upload".to_string())
}

/// Checks a single meet.csv file by path.
pub fn check_meet(
    reader: &csv::ReaderBuilder,
    meet_csv: PathBuf,
    config: Option<&Config>,
) -> Result<MeetCheckResult, Box<dyn Error>> {
    // Allow the pending Report to own the PathBuf.
    let mut report = Report::new(meet_csv);

    // The meet.csv file must exist.
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(MeetCheckResult { report, meet: None });
    }

    let meetpath = check_meetpath(&mut report).unwrap_or_else(String::new);

    let mut rdr = reader.from_path(&report.path)?;
    do_check(&mut rdr, config, report, meetpath)
}
