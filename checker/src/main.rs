//! Checks CSV data files for validity.

extern crate checker;
extern crate colored;
extern crate csv;
extern crate rayon;
extern crate serde;
extern crate walkdir;

use colored::*;
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use std::env;
use std::io::{self, Write};
use std::error::Error;
use std::path::PathBuf;

// For purposes of testing, a meet directory is any directory containing
// either of the files "entries.csv" or "meet.csv".
fn is_meetdir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
        && (entry.path().join("entries.csv").exists()
            || entry.path().join("meet.csv").exists())
}

/// Determines the project root from the binary path.
fn get_project_root() -> Result<PathBuf, Box<Error>> {
    const ERR: &str = "get_project_root() ran out of parent directories";
    Ok(env::current_exe()?     // root/target/release/binary
        .parent().ok_or(ERR)?  // root/target/release
        .parent().ok_or(ERR)?  // root/target
        .parent().ok_or(ERR)?  // root
        .to_path_buf())
}

/// Outputs a report to stdout with the StdoutLock held for atomicity.
///
/// Failure to write a report is not itself an error that needs reporting.
fn write_report(handle: &mut io::StdoutLock, report: checker::Report) {
    // Output the full name of the file in default coloring.
    let _ = handle.write_fmt(format_args!("{}\n", report.path.to_str().unwrap()));

    // Output each message with some festive coloring.
    for message in report.messages {
        match message {
            checker::Message::Error(s) => {
                let _ = handle.write_fmt(format_args!(" {}\n", s.bold().red()));
            }
            checker::Message::Warning(s) => {
                let _ = handle.write_fmt(format_args!(" {}\n", s.bold().yellow()));
            }
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    // Get handles to various parts of the project.
    let project_root = get_project_root()?;
    let meet_data_root = project_root.join("meet-data");

    // Build a list of every directory containing meet results.
    let meetdirs: Vec<DirEntry> = WalkDir::new(&meet_data_root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_meetdir(entry))
        .collect();

    meetdirs
        .into_par_iter()
        .for_each(|dir| {
            match checker::check(dir.path()) {
                Ok(reports) => {
                    // Acquire a mutex around stdout.
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();

                    for report in reports {
                        write_report(&mut handle, report);
                    }
                }
                Err(_e) => (),
            };
        });

    Ok(())
}
