//! CLI driver for developing the search interface.

use rustyline::error::ReadlineError;

use std::path::Path;

const READLINE_PROMPT: &str = ">>> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const LIFTERS_CSV: &str = "../../build/lifters.csv";
    const MEETS_CSV: &str = "../../build/meets.csv";
    const ENTRIES_CSV: &str = "../../build/entries.csv";

    let _db = opldb::OplDb::from_csv(
        Path::new(LIFTERS_CSV),
        Path::new(MEETS_CSV),
        Path::new(ENTRIES_CSV),
    )?;

    let mut rl = rustyline::DefaultEditor::new().unwrap();
    loop {
        match rl.readline(READLINE_PROMPT) {
            Ok(line) => {
                if !line.is_empty() {
                    rl.add_history_entry(line.as_str()).unwrap();
                    println!("{line}");
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }

    std::process::exit(0); // Dropping the database takes a while.
}
