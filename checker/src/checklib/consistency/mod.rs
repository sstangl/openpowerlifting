//! Checks for consistency errors across entries per lifter.

use crate::{AllMeetData, Entry, LifterDataMap, LifterMap, Report};
use opltypes::Date;

mod bodyweight;
use bodyweight::check_bodyweight_all;

mod disambiguations;
use disambiguations::check_disambiguations_all;

mod duplicates;
use duplicates::check_duplicates_all;

mod name;
use name::check_name_all;

mod sex;
use sex::check_sex_all;

/// Return type for consistency check functions.
pub enum ConsistencyResult {
    Consistent,
    Inconsistent,
    Skipped,
}

/// Helper for getting the date of an [Entry].
pub fn date(entry: &Entry) -> Date {
    entry.entrydate
}

/// Whether the lifter should be skipped for consistency checks.
///
/// There are some names that we already know need disambiguation,
/// but there is not enough information to do so:
///  1. Lifters with initialized first names.
///  2. Lifters with only a surname.
pub fn should_skip_lifter(entry: &Entry) -> bool {
    // Skip lifters with initialized first names.
    if entry.name.chars().skip(1).take(1).collect::<Vec<char>>() == ['.'] {
        return true;
    }

    // Skip lifters with only a surname.
    if !entry.name.contains(' ') {
        return true;
    }

    false
}

/// Check entries for per-lifter consistency.
///
/// The `is_partial` argument is true iff only a subtree of the meet data
/// is being checked. In that case, the `liftermap` does not contain
/// full information from the database: it only contains information
/// from the chosen subtree. Therefore, full-tree disambiguation consistency
/// checks must be disabled.
pub fn check(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
    is_partial: bool,
) -> Vec<Report> {
    let mut reports = Vec::new();

    check_sex_all(liftermap, meetdata, lifterdata, &mut reports);
    check_name_all(liftermap, meetdata, &mut reports);
    check_bodyweight_all(liftermap, meetdata, lifterdata, &mut reports);
    check_duplicates_all(liftermap, meetdata, &mut reports);

    // The checks below require the full meet-data tree, not a subset.
    if !is_partial {
        check_disambiguations_all(liftermap, lifterdata, &mut reports);
    }

    reports
}
