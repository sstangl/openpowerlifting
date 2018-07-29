//! Tests for entries.csv files.

extern crate checker;
extern crate csv;

use checker::check_entries::do_check;
use checker::Report;

use std::path::PathBuf;

/// Executes checks against a string representation of a CSV,
/// returning the number of errors.
fn check(csv: &str) -> usize {
    let report = Report::new(PathBuf::from("[inline]"));
    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let (errors, _warnings) = do_check(&mut rdr, report).unwrap().count_messages();
    errors
}

#[test]
fn test_empty_file() {
    assert!(check("") > 0);
}
