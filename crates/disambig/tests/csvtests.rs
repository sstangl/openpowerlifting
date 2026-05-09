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

use opltypes::*;
use serde::{Deserialize, de::Error};
use serde_derive::Deserialize;

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

/// Deserialization helper that rejects empty strings.
///
/// This is useful to make sure that AssertGroup is non-empty.
fn disallow_empty_string<'de, D: serde::Deserializer<'de>>(de: D) -> Result<String, D::Error> {
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
    let _rows = read_testdata(csvpath);

    datatest_stable::Result::Ok(())
}
