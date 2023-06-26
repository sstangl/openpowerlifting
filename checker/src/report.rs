//! Defines the interface for human-readable errors and warnings.
//!
//! Unlike most libraries that surface error messages, this one explicitly
//! does not intend to visually show the context, other than by reporting
//! the exact line number on which the error occurred. This is because:
//!
//! 1. Each line is logically self-consistent.
//! 2. There are usually an extreme number of errors, and vertical density
//!    helps scan through them all.
//! 3. The context itself is generally not helpful, because the data is CSV.

use std::path::PathBuf;

use crate::report_count::ReportCount;

/// A data error or warning message that should be reported.
#[derive(Debug, Serialize)]
pub enum Message {
    Error(String),
    Warning(String),
}

/// Accumulates messages that should be reported as a single batch.
#[derive(Debug, Serialize)]
pub struct Report {
    /// Each report represents errors/warnings from a single file. This is its path.
    pub path: PathBuf,
    /// Any errors or warnings generated while reading that file.
    pub messages: Vec<Message>,
}

impl Report {
    /// Creates a new `Report` about the file at `path`.
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
        let msg = format!(" Line {line}: {}", message.to_string());
        self.messages.push(Message::Error(msg));
    }

    /// Reports a warning, which allows checks to pass with a note.
    pub fn warning(&mut self, message: impl ToString) {
        self.messages.push(Message::Warning(message.to_string()));
    }

    /// Reports a warning on a specific line.
    pub fn warning_on(&mut self, line: u64, message: impl ToString) {
        let msg = format!(" Line {line}: {}", message.to_string());
        self.messages.push(Message::Warning(msg));
    }

    /// Whether a report has any messages.
    pub fn has_messages(&self) -> bool {
        !self.messages.is_empty()
    }

    /// Returns how many messages there are of (errors, warnings).
    pub fn count_messages(&self) -> ReportCount {
        let mut errors = 0;
        let mut warnings = 0;

        for message in &self.messages {
            match message {
                Message::Error(_) => errors += 1,
                Message::Warning(_) => warnings += 1,
            }
        }

        ReportCount::new(errors, warnings)
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
