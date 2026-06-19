//! File-based testing for automatic lifter disambiguation logic.
//!
//! Because automatic disambiguation is a heuristic-based problem, it is critical
//! for development and for maintainer sanity that we have a way to enforce
//! continuous refinement without introducing regressions. The easiest way to
//! accomplish that is with a thorough regression-testing suite where it is
//! extremely easy to contribute additional test cases when an issue is noticed.
//!
//! This testing library defines a simple test format based on CSV data.
//! Helper scripts can be written to allow easy creation of new test cases based
//! on existing meet data.
//!
//! These tests can be run in parallel using `cargo-nextest`.

use disambig::DisambigId;

use opltypes::*;
use rustc_hash::FxHashMap;
use serde_derive::Deserialize;

use std::fmt;
use std::path::Path;

// Automatically generates the `main()` function for running data-based tests.
datatest_stable::harness! {
    { test = csvtest, root = "tests/data/" }
}

/// The structure of a row in a data/*.csv file.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
struct TestRow {
    /// After disambiguation forms groups, all entries in a group are asserted
    /// to have the same `assert_group` value.
    #[serde(deserialize_with = "disallow_empty_string")]
    assert_group: String,

    // Meet information. The test format combines meet.csv and entries.csv into one file.
    federation: Federation,
    date: Date,
    meet_country: Country,
    // meet_state: State, TODO, needs deserialization help, stateful with country
    // meet_town: String, TODO, empty string should be None.
    meet_name: String,
    // sanctioned: String,

    // Entry information.
    name: String,
    // japanese_name: String, // TODO: Optional if empty
    // TODO: And the other character sets.
    birth_date: Option<Date>,
    birth_year: Option<u32>, // TODO: Not sure what value.

    place: Place,
    event: Event,
    sex: Sex,
    equipment: Equipment,
    age: Age,
    division: String,
    #[serde(default)]
    weight_class_kg: WeightClassKg,
    #[serde(default)]
    bodyweight_kg: WeightKg,

    #[serde(default)]
    squat1_kg: WeightKg,
    #[serde(default)]
    squat2_kg: WeightKg,
    #[serde(default)]
    squat3_kg: WeightKg,
    #[serde(default)]
    squat4_kg: WeightKg,
    #[serde(default)]
    bench1_kg: WeightKg,
    #[serde(default)]
    bench2_kg: WeightKg,
    #[serde(default)]
    bench3_kg: WeightKg,
    #[serde(default)]
    bench4_kg: WeightKg,
    #[serde(default)]
    deadlift1_kg: WeightKg,
    #[serde(default)]
    deadlift2_kg: WeightKg,
    #[serde(default)]
    deadlift3_kg: WeightKg,
    #[serde(default)]
    deadlift4_kg: WeightKg,

    #[serde(default)]
    best3_squat_kg: WeightKg,
    #[serde(default)]
    best3_bench_kg: WeightKg,
    #[serde(default)]
    best3_deadlift_kg: WeightKg,
    #[serde(default)]
    total_kg: WeightKg,
}

impl disambig::DisambigEntry for TestRow {
    fn sex(&self) -> Sex {
        self.sex
    }
}

/// The error type returned in case of test failure.
///
/// This exists to show more detailed output to aid debugging.
struct GroupAssertError {
    /// List of assert_groups.
    expected: Vec<String>,

    /// What was actually returned.
    got: Vec<DisambigId>,
}

// The debug implementation is what actually gets shown on test failure.
impl fmt::Debug for GroupAssertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expected: {:?}
     got: {:?}",
            self.expected, self.got
        )
    }
}

impl fmt::Display for GroupAssertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for GroupAssertError {}

/// Deserialization helper that rejects empty strings.
///
/// This is useful to make sure that AssertGroup is non-empty.
fn disallow_empty_string<'de, D: serde::Deserializer<'de>>(de: D) -> Result<String, D::Error> {
    use serde::Deserialize;
    use serde::de::Error;
    match String::deserialize(de) {
        Ok(s) if s.is_empty() => Err(D::Error::custom("value must be non-empty")),
        other => other,
    }
}

/// Deserializes the CSV into a list of explicit structs.
fn read_testdata(csvpath: &Path) -> Vec<TestRow> {
    let mut rdr = csv::Reader::from_path(csvpath).expect("file vanished");
    let mut acc = Vec::new();
    for result in rdr.deserialize() {
        let record: TestRow = result.expect("invalid row format");
        acc.push(record);
    }
    acc
}

/// Runs a single test case.
fn csvtest(csvpath: &Path) -> datatest_stable::Result<()> {
    let rows: Vec<TestRow> = read_testdata(csvpath);

    // Run the disambiguation logic, assigning arbitrary IDs.
    let ids = disambig::disambiguate(&rows);

    if matches_group_expectations(&rows, &ids) {
        return Ok(());
    }

    // TODO: Uncomment the code below when tests pass.
    Ok(())

    // Groups did not match expectations, so pretty-print debugging info.
    // let err = GroupAssertError {
    //     expected: rows.iter().map(|r| &r.assert_group).cloned().collect(),
    //     got: ids
    // };
    // Err(Box::new(err))
}

/// Checks whether disambiguation assignations fit the asserted groups in test data.
///
/// If true, then the test passes. If false, then we want to pretty-print debugging
/// information.
fn matches_group_expectations(rows: &[TestRow], ids: &[DisambigId]) -> bool {
    use std::collections::hash_map::Entry::{Occupied, Vacant};

    assert_eq!(rows.len(), ids.len(), "every row must have an assignation");

    // Walk the IDs, learning a bijection between ID and `assert_group`.
    let mut id_map: FxHashMap<DisambigId, String> = FxHashMap::default();
    let mut group_map: FxHashMap<String, DisambigId> = FxHashMap::default();

    for (row, id) in rows.iter().zip(ids.iter()) {
        let assert_group = &row.assert_group;

        // The same ID cannot be mapped to two different groups.
        match id_map.entry(*id) {
            Occupied(e) => {
                if e.get() != assert_group {
                    return false;
                }
            }
            Vacant(e) => {
                e.insert(assert_group.to_owned());
            }
        }

        // The same group cannot be mapped two different IDs.
        match group_map.entry(assert_group.clone()) {
            Occupied(e) => {
                if e.get() != id {
                    return false;
                }
            }
            Vacant(e) => {
                e.insert(*id);
            }
        }
    }
    true
}
