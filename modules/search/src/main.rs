//! CLI driver for developing the search interface.

use rustyline::error::ReadlineError;
use rustyline::Editor;

const READLINE_PROMPT: &str = ">>> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const LIFTERS_CSV: &str = "../../build/lifters.csv";
    const MEETS_CSV: &str = "../../build/meets.csv";
    const ENTRIES_CSV: &str = "../../build/entries.csv";

    let _db = opldb::OplDb::from_csv(LIFTERS_CSV, MEETS_CSV, ENTRIES_CSV)?;

    let mut rl = Editor::<()>::new();
    loop {
        match rl.readline(READLINE_PROMPT) {
            Ok(line) => {
                if !line.is_empty() {
                    rl.add_history_entry(line.as_str());
                    println!("{}", line);
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    std::process::exit(0); // Dropping the database takes a while.
}
