extern crate chrono;
extern crate csv;
extern crate opltypes;
extern crate strum;
#[macro_use]
extern crate strum_macros;
extern crate toml;

pub mod check_config;
pub use check_config::{check_config, Config};
pub mod check_entries;
use check_entries::check_entries;
pub mod check_meet;
use check_meet::check_meet;

use std::error::Error;
use std::path::{Path, PathBuf};

/// A data error or warning message that should be reported.
#[derive(Debug)]
pub enum Message {
    Error(String),
    Warning(String),
}

/// Accumulates messages that should be reported as a single batch.
#[derive(Debug)]
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
}

/// Checks a directory with meet data.
pub fn check(meetdir: &Path, config: Option<&Config>) -> Result<Vec<Report>, Box<Error>> {
    let mut acc = Vec::new();

    // Check the meet.csv.
    let meetresult = check_meet(meetdir.join("meet.csv"))?;
    if !meetresult.report.messages.is_empty() {
        acc.push(meetresult.report);
    }

    // Check the entries.csv.
    let report = check_entries(meetdir.join("entries.csv"), meetresult.meet.as_ref())?;
    if !report.messages.is_empty() {
        acc.push(report);
    }

    // Check for commonly-misnamed files.
    if meetdir.join("URL.txt").exists() {
        let mut report = Report::new(meetdir.join("URL.txt"));
        report.error("Must be named 'URL' with no extension");
        acc.push(report);
    }

    Ok(acc)
}
