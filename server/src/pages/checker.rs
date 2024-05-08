//! Logic for the checker page.

use opldb::OplDb;

use checker::{EntriesCheckResult, Meet, MeetCheckResult, Message};
use std::error::Error;

/// Incoming data format for the checker, sent via JSON POST.
///
/// The client-side code constructing this is in
/// server/client/scripts/checker.ts.
#[derive(Debug, Deserialize)]
pub struct CheckerInput {
    meet: String,
    entries: String,
}

/// Outgoing data format from the checker, returned to the client.
///
/// The client-side code constructing this is in
/// server/client/scripts/checker.ts.
#[derive(Default, Serialize)]
pub struct CheckerOutput {
    /// Text of the first I/O error that occurred during checking.
    io_error: Option<String>,

    /// Messages from checking the meet.csv file.
    meet_messages: Vec<Message>,

    /// Messages from checking the entries.csv file.
    entries_messages: Vec<Message>,
}

impl CheckerOutput {
    /// Constructor for just an I/O error.
    pub fn with_io_error(err: impl ToString) -> Self {
        Self {
            io_error: Some(err.to_string()),
            ..Self::default()
        }
    }

    /// Constructor with messages (empty or not) from checking a meet.
    pub fn with_meet_messages(messages: Vec<Message>) -> Self {
        Self {
            meet_messages: messages,
            ..Self::default()
        }
    }
}

/// Checks a meet.csv encoded as a string.
fn check_meet(
    reader: &csv::ReaderBuilder,
    input: &CheckerInput,
) -> Result<MeetCheckResult, Box<dyn Error>> {
    checker::check_meet_from_string(reader, &input.meet)
}

/// Checks an entries.csv encoded as a string.
fn check_entries(
    reader: &csv::ReaderBuilder,
    opldb: &OplDb,
    input: &CheckerInput,
    meet: &Meet,
) -> Result<EntriesCheckResult, Box<dyn Error>> {
    let EntriesCheckResult {
        mut report,
        entries,
    } = checker::check_entries_from_string(reader, &input.entries, meet)?;

    match entries {
        Some(entries) => {
            // Ensure that the username and name do not introduce a conflict.
            for entry in entries.iter() {
                if let Some(id) = opldb.lifter_id(entry.username.as_str()) {
                    let lifter = opldb.lifter(id);
                    if lifter.name != entry.name {
                        report.error(format!(
                            "Conflict for '{}': '{}' vs '{}'",
                            &entry.username, lifter.name, &entry.name
                        ));
                    }
                }
            }

            Ok(EntriesCheckResult {
                report,
                entries: Some(entries),
            })
        }
        None => Ok(EntriesCheckResult { report, entries }),
    }
}

/// Checks a CheckerInput, returning a JSON-serializable CheckerOutput.
pub fn check(opldb: &OplDb, input: &CheckerInput) -> CheckerOutput {
    // Compile the DFA that reads the CSV.
    let reader: csv::ReaderBuilder = checker::checklib::compile_csv_reader();

    // First check the meet.csv, because entries.csv date checking is dependent.
    match check_meet(&reader, input) {
        Ok(MeetCheckResult { report, meet }) => {
            let mut output = CheckerOutput::with_meet_messages(report.messages);

            // If the meet.csv parsed successfully, also parse the entries.csv.
            if let Some(meet) = meet.as_ref() {
                match check_entries(&reader, opldb, input, meet) {
                    Ok(EntriesCheckResult { report, .. }) => {
                        output.entries_messages = report.messages;
                    }
                    Err(err) => output.io_error = Some(err.to_string()),
                }
            }

            output
        }
        Err(err) => CheckerOutput::with_io_error(err),
    }
}
