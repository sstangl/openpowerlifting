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

use std::path::Path;

// Automatically generates the `main()` function for running data-based tests.
datatest_stable::harness! {
    { test = csvtest, root = "tests/testcases/" }
}

/// Runs a single test case.
fn csvtest(csvpath: &Path) -> datatest_stable::Result<()> {
    datatest_stable::Result::Ok(())
}
