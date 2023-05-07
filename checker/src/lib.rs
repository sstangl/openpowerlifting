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
pub use report::{Message, Report};

use std::error::Error;
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

    // Check the entries.csv.
    let entriesresult = check_entries(
        reader,
        meetdir.join("entries.csv"),
        meetresult.meet.as_ref(),
        config,
        lifterdata,
    )?;
    if !entriesresult.report.messages.is_empty() {
        acc.push(entriesresult.report);
    }

    // Check for commonly-misnamed files.
    if meetdir.join("URL.txt").exists() {
        let mut report = Report::new(meetdir.join("URL.txt"));
        report.error("Must be named 'URL' with no extension");
        acc.push(report);
    }
    if meetdir.join("results.csv").exists() {
        let mut report = Report::new(meetdir.join("results.csv"));
        report.error("'results.csv' files should now be named 'original.csv'");
        acc.push(report);
    }
    if meetdir.join("results.txt").exists() {
        let mut report = Report::new(meetdir.join("results.txt"));
        report.error("'results.txt' files should now be named 'original.txt'");
        acc.push(report);
    }

    Ok(CheckResult {
        reports: acc,
        meet: meetresult.meet,
        entries: entriesresult.entries,
    })
}
