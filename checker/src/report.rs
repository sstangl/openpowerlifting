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

/// The severity of the report message.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
pub enum Severity {
    /// Errors are shown in red and halt compilation.
    ///
    /// They represent critical errors and must be fixed for compilation to proceed.
    Error,

    /// Warnings are shown in yellow and do not halt compilation.
    ///
    /// They are intended as observations for things that you should fix, but it's fine
    /// to fix them later. Warnings become obnoxious as they increase in number, which
    /// provides pressure to fix them quickly.
    Warning,
}

/// A data error or warning message that should be reported.
#[derive(Clone, Debug, Serialize)]
pub struct Message {
    pub severity: Severity,
    pub text: String,
}

/// Accumulates messages that should be reported as a single batch.
///
/// Serialization must match the deserialization format in "checker.ts".
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
    pub fn error(&mut self, text: impl ToString) {
        let message = Message {
            severity: Severity::Error,
            text: text.to_string(),
        };
        self.messages.push(message);
    }

    /// Reports an error on a specific line.
    pub fn error_on(&mut self, line: u64, message: impl ToString) {
        let text = format!(" Line {line}: {}", message.to_string());
        self.error(text);
    }

    /// Reports a warning, which allows checks to pass with a note.
    pub fn warning(&mut self, text: impl ToString) {
        let message = Message {
            severity: Severity::Warning,
            text: text.to_string(),
        };
        self.messages.push(message);
    }

    /// Reports a warning on a specific line.
    pub fn warning_on(&mut self, line: u64, message: impl ToString) {
        let text = format!(" Line {line}: {}", message.to_string());
        self.warning(text);
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
            match message.severity {
                Severity::Error => errors += 1,
                Severity::Warning => warnings += 1,
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
