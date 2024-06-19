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
    /// The path to the meet folder from the root of `meet-data/`.
    ///
    /// For example, the file `meet-data/uspa/1234/meet.csv` would have a `path` of `uspa/1234`.
    pub path: String,

    pub federation: Federation,
    pub date: Date,
    pub country: Country,
    pub state: Option<State>,
    pub town: Option<String>,
    pub name: String,
    pub ruleset: RuleSet,

    /// Whether the meet is sanctioned by a recognized federation.
    pub sanctioned: bool,

    /// If true, allows entries in this meet to appear in other meets.
    ///
    /// This is enabled by setting `ExemptDuplicates` for the meet in a `CONFIG.toml`.
    pub allow_duplicates: bool,
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
            sanctioned: true,
            allow_duplicates: false,
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
const OPTIONAL_HEADERS: [&str; 2] = ["RuleSet", "Sanctioned"];

/// Checks that the headers are fixed and correct.
fn check_headers(headers: &csv::StringRecord, report: &mut Report) {
    // Check header length.
    let minheaders = REQUIRED_HEADERS.len();
    let maxheaders = REQUIRED_HEADERS.len() + OPTIONAL_HEADERS.len();

    if headers.len() < minheaders {
        report.error(format!("There must be at least {minheaders} columns"));
        return;
    } else if headers.len() > maxheaders {
        report.error(format!("There can be at most {maxheaders} columns"));
        return;
    }

    // Quickly check for Windows line endings, producing a more helpful error message.
    if let Some(last_header) = headers.get(headers.len() - 1) {
        // The CSV Reader already stripped the '\n' in "\r\n".
        if last_header.ends_with('\r') {
            report.error("Windows line endings detected: run `dos2unix` to convert");
            return;
        }
    }

    // Check required headers.
    for (i, header) in headers.iter().take(REQUIRED_HEADERS.len()).enumerate() {
        if header != REQUIRED_HEADERS[i] {
            report.error(format!("Column {i} must be '{}'", REQUIRED_HEADERS[i]));
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
                "Unknown federation '{s}'. \
                 Add to crates/opltypes/src/federation.rs?",
            ));
            None
        }
    }
}

/// Checks the Date column.
pub fn check_date(s: &str, report: &mut Report) -> Option<Date> {
    let date = s.parse::<Date>();
    if date.is_err() {
        report.error(format!("Invalid date '{s}'. Must be YYYY-MM-DD"));
        return None;
    }
    let date = date.unwrap();

    // The date should not be implausibly long ago.
    if date.year() < 1945 {
        report.error(format!("Implausible year in '{s}'"));
    }

    // This is sufficiently fast to call that caching is of no practical benefit.
    //
    // Tomorrow's date is used as the "now" cutoff. This solves a timezone issue
    // where contributors in Australia might add meets "in the future" relative
    // to someone living in the USA.
    let now = chrono::Local::now() + chrono::naive::Days::new(1);

    // The date should not be in the future.
    let (y, m, d) = (now.year() as u32, now.month(), now.day());
    if (date.year() > y)
        || (date.year() == y && date.month() > m)
        || (date.year() == y && date.month() == m && date.day() > d)
    {
        report.error(format!("Meet occurs in the future in '{s}'"));
    }

    // The date should exist in the Gregorian calendar.
    if !date.is_valid() {
        let msg = format!("Date '{s}' does not exist in the Gregorian calendar");
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
                "Unknown country '{s}'. \
                 Add to crates/opltypes/src/country.rs?"
            ));

            // Emit some helpful warnings.
            if s.contains("Chin") {
                report.warning(format!("Should '{s}' be 'Taiwan'?"));
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

    let country = match country {
        Some(value) => value,
        None => {
            report.warning(format!(
                "Couldn't check MeetState '{s}' due to invalid MeetCountry"
            ));

            return None;
        }
    };

    match State::from_str_and_country(s, country) {
        Ok(s) => Some(s),
        Err(_) => {
            let cstr = country.to_string();
            let mut error = format!("Unknown state '{s}' for country '{cstr}'");

            if let Some(available) = State::get_available_for_country(country) {
                let concat = available.join(", ");
                error += &format!(", available values: [{concat}]");
            }

            report.error(error);
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
            report.error(format!("Illegal character in MeetTown '{s}'"));
            break;
        }
    }

    // Check for excessive spacing.
    if s.contains("  ") || s.starts_with(' ') || s.ends_with(' ') {
        report.error(format!("Excessive whitespace in MeetTown '{s}'"));
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
            report.error(format!("Illegal character in MeetName '{s}'"));
            break;
        }
    }

    // Check for excessive spacing.
    if s.contains("  ") || s.starts_with(' ') || s.ends_with(' ') {
        report.error(format!("Excessive whitespace in MeetName '{s}'"));
    }

    // The federation shouldn't be part of the name.
    if !fedstr.is_empty() && s.contains(fedstr) {
        report.error(format!("MeetName '{s}' must not contain the federation"));
    }

    // The year shouldn't be part of the name.
    if let Some(idx) = datestr.find('-') {
        let year = &datestr[0..idx];
        if s.contains(year) {
            report.error(format!("MeetName '{s}' must not contain the year"));
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
            report.error(format!("Failed parsing RuleSet '{s}'"));
            RuleSet::default()
        }
    }
}

/// Checks the optional Sanctioned column.
fn check_sanctioned(s: &str, report: &mut Report) -> bool {
    match s {
        "Yes" => true,
        "No" => false,
        other => {
            let msg = format!("Sanctioned column must be either Yes or No, found '{other}'");
            report.error(msg);
            true
        }
    }
}

/// Checks a single meet.csv file from an open `csv::Reader`.
///
/// Extracting this out into a `Reader`-specific function is useful
/// for creating tests that do not have a backing CSV file.
pub fn do_check<R: io::Read>(
    rdr: &mut csv::Reader<R>,
    config: Option<&Config>,
    mut report: Report,
    meetpath: String,
) -> Result<MeetCheckResult, Box<dyn Error>> {
    // Remember the number of errors at the start.
    // If the number increased during checking, don't return a parsed Meet struct.
    let initial_errors = report.count_messages().errors();

    // Verify column headers. Only continue if they're valid.
    let headers = rdr.headers()?;
    check_headers(headers, &mut report);
    if !report.messages.is_empty() {
        return Ok(MeetCheckResult { report, meet: None });
    }
    let num_columns = headers.len();

    // If there are optional columns, remember their indices for later.
    let ruleset_idx: Option<usize>;
    let sanctioned_idx: Option<usize>;

    if num_columns > REQUIRED_HEADERS.len() {
        ruleset_idx = headers.iter().position(|hdr| hdr == "RuleSet");
        sanctioned_idx = headers.iter().position(|hdr| hdr == "Sanctioned");
    } else {
        ruleset_idx = None;
        sanctioned_idx = None;
    }

    // Read a single row.
    let mut record = csv::StringRecord::new();
    if !rdr.read_record(&mut record)? {
        report.error("The second row is missing");
        return Ok(MeetCheckResult { report, meet: None });
    }

    // Check that the number of columns in the second row matches the headers row.
    if record.len() != num_columns {
        report.error("The second row is missing columns");
        return Ok(MeetCheckResult { report, meet: None });
    }

    // Check the required columns.
    let federation = check_federation(&record[0], &mut report);
    let date = check_date(&record[1], &mut report);
    let country = check_meetcountry(&record[2], &mut report);
    let state = check_meetstate(&record[3], &mut report, country);
    let town = check_meettown(&record[4], &mut report);
    let name = check_meetname(&record[5], &mut report, &record[0], &record[1]);

    // Check the optional columns. They come at the end, in any order.
    let ruleset = if let Some(idx) = ruleset_idx {
        check_ruleset(&record[idx], &mut report)
    } else {
        configured_ruleset(config, date)
    };

    // Check the optional "Sanctioned" column.
    let sanctioned = if let Some(idx) = sanctioned_idx {
        check_sanctioned(&record[idx], &mut report)
    } else {
        true
    };

    // Attempt to read another row -- but there shouldn't be one.
    if rdr.read_record(&mut record)? {
        report.error("Too many rows");
    }

    // If there were errors, return early.
    if initial_errors != report.count_messages().errors() {
        return Ok(MeetCheckResult { report, meet: None });
    }

    // Check the CONFIG.toml to see if this meet should be excluded from duplicate checking.
    // If true, this means that identical entries in this meet can appear in other meets.
    //
    // Normally, that happens because a meet was entered twice erroneously. Sometimes however
    // it's intentional, when a single entry counts toward multiple different federations.
    //
    // TODO(sstangl): This is ugly. It would be better to make Config non-optional, and always
    // pass around a default Config, rather than hardcoding things. We could even make that
    // present in the data itself, by using a `meet-data/CONFIG.toml`.
    let allow_duplicates = if let Some(config) = config {
        if let Some((_fedname, meetname)) = meetpath.rsplit_once('/') {
            if let Some(exemptions) = config.exemptions_for(meetname) {
                exemptions.contains(&super::config::Exemption::ExemptDuplicates)
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

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
                sanctioned,
                allow_duplicates,
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

    let meetpath = check_meetpath(&mut report).unwrap_or_default();

    let mut rdr = reader.from_path(&report.path)?;
    do_check(&mut rdr, config, report, meetpath)
}
