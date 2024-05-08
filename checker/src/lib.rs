//! The OpenPowerlifting data checker library.

// Suppress clippy warnings for date literals.
#![allow(clippy::inconsistent_digit_grouping)]
#![allow(clippy::zero_prefixed_literal)]

#[macro_use]
extern crate serde_derive; // Provides struct serialization and deserialization.
#[macro_use]
extern crate strum_macros; // Used for iterating over enums.

pub mod checklib;
pub use crate::checklib::config::{check_config, Config};
pub use crate::checklib::consistency;
pub use crate::checklib::entries::{
    check_entries, check_entries_from_string, EntriesCheckResult, Entry,
};
pub use crate::checklib::lifterdata::{
    check_lifterdata, LifterData, LifterDataCheckResult, LifterDataMap,
};
pub use crate::checklib::meet::{check_meet, check_meet_from_string, Meet, MeetCheckResult};
pub use crate::checklib::CheckResult;

pub mod compiler;
pub mod disambiguator;

mod meetdata;
use meetdata::EntryIndex;
pub use meetdata::{AllMeetData, LifterMap, SingleMeetData};

mod report;
pub use report::{Message, Report, Severity};

pub mod report_count;

use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

/// Checks a directory with meet data.
pub fn check(
    reader: &csv::ReaderBuilder,
    meetdir: &Path,
    config: Option<&Config>,
    lifterdata: Option<&LifterDataMap>,
) -> Result<CheckResult, Box<dyn Error>> {
    let mut acc = Vec::new();

    // Check the meet.csv.
    let meetresult = check_meet(reader, meetdir.join("meet.csv"), config)?;
    if !meetresult.report.messages.is_empty() {
        acc.push(meetresult.report);
    }

    // Only check the entries.csv if the meet.csv passed all tests.
    //
    // Previously, we would check the entries.csv with a default config,
    // but the errors were often nonsense. Because the nonsense errors appeared
    // after the original errors, this confused new contributors.
    // Check the entries.csv.
    let entries = if let Some(meet) = meetresult.meet.as_ref() {
        let entriesresult = check_entries(
            reader,
            meetdir.join("entries.csv"),
            meet,
            config,
            lifterdata,
        )?;
        if !entriesresult.report.messages.is_empty() {
            acc.push(entriesresult.report);
        }
        entriesresult.entries
    } else {
        None
    };

    // Check for commonly-misnamed files.
    if meetdir.join("URL.txt").exists() {
        let mut report = Report::new(meetdir.join("URL.txt"));
        report.error("Must be named 'URL' with no extension");
        acc.push(report);
    }

    // Recursively look for files that may be misnamed or have disallowed extensions.
    for entry in walkdir::WalkDir::new(meetdir)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_invalid_entry)
    {
        let mut report = Report::new(meetdir.join(entry.path()));

        // The file was flagged as invalid. Try to emit a helpful error message.
        let extension: &str = entry
            .path()
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();

        if extension == "pdf" {
            report.error("PDF files are disallowed, please use a program to convert it to text");
        } else if extension.contains("xls") {
            report.error("Binary spreadsheet files are disallowed, please resave as CSV");
        } else {
            report.error("Unknown file, please check spelling and extension");
        }

        acc.push(report);
    }

    Ok(CheckResult {
        reports: acc,
        meet: meetresult.meet,
        entries,
    })
}

/// Returns whether an entry is invalid, and therefore should be flagged for error.
fn is_invalid_entry(entry: &walkdir::DirEntry) -> bool {
    let filename = entry.file_name();

    // Approve any files that match the list of known entities.
    let valid_filenames = &[
        OsStr::new("URL"),
        OsStr::new("entries.csv"),
        OsStr::new("meet.csv"),
        OsStr::new("INFO"),
        OsStr::new(".DS_Store"), // Ignored in the .gitignore, but macOS makes lots of these files.
    ];
    if valid_filenames.contains(&filename) {
        return false;
    }

    // Allow files containing the name "original", such as "original1.txt", in textual formats.
    if let Some(utf8) = filename.to_str() {
        if utf8.starts_with("original")
            && (utf8.ends_with(".csv") || utf8.ends_with(".txt") || utf8.ends_with(".html"))
        {
            return false;
        }
    }

    // The entry didn't match any known pattern, and therefore should raise an error.
    true
}
