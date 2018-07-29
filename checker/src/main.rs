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
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

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

/// Outputs a final summary line.
fn print_summary(error_count: usize, warning_count: usize) {
    let error_str = format!("{} errors", error_count);
    let warning_str = format!("{} warnings", warning_count);

    let error_str = if error_count > 0 {
        error_str.bold().red().to_string()
    } else {
        error_str.bold().green().to_string()
    };

    let warning_str = if warning_count > 0 {
        warning_str.bold().yellow().to_string()
    } else {
        warning_str.bold().green().to_string()
    };

    println!("Summary: {}, {}", error_str, warning_str);
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

    let error_count = Arc::new(AtomicUsize::new(0));
    let warning_count = Arc::new(AtomicUsize::new(0));

    // Unexpected errors that occurred while reading files.
    let internal_error_count = Arc::new(AtomicUsize::new(0));

    meetdirs
        .into_par_iter()
        .for_each(|dir| {
            match checker::check(dir.path()) {
                Ok(reports) => {
                    // Acquire a mutex around stdout.
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();

                    for report in reports {
                        // Update the error and warning counts.
                        let (errors, warnings) = report.count_messages();
                        if errors > 0 {
                            error_count.fetch_add(errors, Ordering::SeqCst);
                        }
                        if warnings > 0 {
                            warning_count.fetch_add(warnings, Ordering::SeqCst);
                        }

                        write_report(&mut handle, report);
                    }
                }
                Err(e) => {
                    internal_error_count.fetch_add(1, Ordering::SeqCst);
                    let stderr = io::stderr();
                    let mut handle = stderr.lock();
                    let _ = handle.write_fmt(
                        format_args!("{}\n", dir.path().to_str().unwrap())
                    );
                    let _ = handle.write_fmt(
                        format_args!(" Internal Error: {}\n",
                                     e.to_string().bold().purple())
                    );
                }
            };
        });

    let error_count = Arc::try_unwrap(error_count).unwrap().into_inner();
    let warning_count = Arc::try_unwrap(warning_count).unwrap().into_inner();
    print_summary(error_count, warning_count);

    let internal_error_count = Arc::try_unwrap(internal_error_count).unwrap().into_inner();
    if error_count > 0 || internal_error_count > 0 {
        process::exit(1);
    }
    Ok(())
}
