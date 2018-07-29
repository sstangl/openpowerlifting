extern crate csv;

pub mod check_entries;
use check_entries::check_entries;

use std::error::Error;
use std::path::{Path, PathBuf};

/// A data error or warning message that should be reported.
pub enum Message {
    Error(String),
    Warning(String),
}

/// Accumulates messages that should be reported as a single batch.
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

    /// Reports a warning, which allows checks to pass with a note.
    pub fn warning(&mut self, message: impl ToString) {
        self.messages.push(Message::Warning(message.to_string()));
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
}

/// Checks a directory with meet data.
pub fn check(meetdir: &Path) -> Result<Vec<Report>, Box<Error>> {
    let mut acc = Vec::new();

    let report = check_entries(meetdir.join("entries.csv"))?;
    if !report.messages.is_empty() {
        acc.push(report);
    }

    Ok(acc)
}
