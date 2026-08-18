//! Checks for consistency errors across entries per lifter.

use crate::{AllMeetData, Entry, LifterDataMap, LifterMap, Report};
use opltypes::Date;

mod bodyweight;
mod disambiguations;
mod duplicates;
mod name;
mod sex;

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
    if entry.name.chars().nth(1) == Some('.') {
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
    let mut reports_sex: Vec<Report> = Vec::new();
    let mut reports_name: Vec<Report> = Vec::new();
    let mut reports_dup: Vec<Report> = Vec::new();
    let mut reports_dis: Vec<Report> = Vec::new();

    // Execute checks in parallel.
    rayon::scope(|s| {
        s.spawn(|_| sex::check_sex_all(liftermap, meetdata, lifterdata, &mut reports_sex));
        s.spawn(|_| name::check_name_all(liftermap, meetdata, &mut reports_name));
        s.spawn(|_| duplicates::check_duplicates_all(liftermap, meetdata, &mut reports_dup));

        // bodyweight::check_bodyweight_all(liftermap, meetdata, lifterdata, &mut reports);

        // The checks below require the full meet-data tree, not a subset.
        if !is_partial {
            s.spawn(|_| {
                disambiguations::check_disambiguations_all(liftermap, lifterdata, &mut reports_dis)
            });
        }
    });

    let mut reports = Vec::new();
    reports.append(&mut reports_sex);
    reports.append(&mut reports_name);
    reports.append(&mut reports_dup);
    reports.append(&mut reports_dis);
    reports
}
