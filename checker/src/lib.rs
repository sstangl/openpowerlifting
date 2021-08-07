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

mod meetdata;
use meetdata::EntryIndex;
pub use meetdata::{AllMeetData, LifterMap, SingleMeetData};

pub mod disambiguator;

use std::error::Error;
use std::path::{Path, PathBuf};

/// A data error or warning message that should be reported.
#[derive(Debug, Serialize)]
pub enum Message {
    Error(String),
    Warning(String),
}

/// Accumulates messages that should be reported as a single batch.
#[derive(Debug, Serialize)]
pub struct Report {
    pub path: PathBuf,
    pub messages: Vec<Message>,
}

impl Report {
    /// Creates a new Report.
    pub fn new(path: PathBuf) -> Self {
        Report {
            path,
            messages: Vec::new(),
        }
    }

    /// Reports an error, which causes checks to fail.
    pub fn error(&mut self, message: impl ToString) {
        self.messages.push(Message::Error(message.to_string()));
    }

    /// Reports an error on a specific line.
    pub fn error_on(&mut self, line: u64, message: impl ToString) {
        let msg = format!(" Line {}: {}", line, message.to_string());
        self.messages.push(Message::Error(msg));
    }

    /// Reports a warning, which allows checks to pass with a note.
    pub fn warning(&mut self, message: impl ToString) {
        self.messages.push(Message::Warning(message.to_string()));
    }

    /// Reports a warning on a specific line.
    pub fn warning_on(&mut self, line: u64, message: impl ToString) {
        let msg = format!(" Line {}: {}", line, message.to_string());
        self.messages.push(Message::Warning(msg));
    }

    /// Whether a report has any messages.
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    /// Returns how many messages there are of (errors, warnings).
    pub fn count_messages(&self) -> (usize, usize) {
        let mut errors = 0;
        let mut warnings = 0;

        for message in &self.messages {
            match message {
                Message::Error(_) => errors += 1,
                Message::Warning(_) => warnings += 1,
            }
        }

        (errors, warnings)
    }

    /// Returns how many errors there are.
    pub fn count_errors(&self) -> usize {
        let (errors, _) = self.count_messages();
        errors
    }

    /// Returns how many warnings there are.
    pub fn count_warnings(&self) -> usize {
        let (_, warnings) = self.count_messages();
        warnings
    }

    /// Returns the name of the parent folder of the given file.
    pub fn parent_folder(&self) -> Result<&str, &str> {
        self.path
            .as_path()
            .parent()
            .and_then(|p| p.file_name().and_then(std::ffi::OsStr::to_str))
            .ok_or("Insufficient parent directories")
    }
}

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
