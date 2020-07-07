//! Checks for consistency errors across entries per lifter.

use crate::{AllMeetData, Entry, LifterDataMap, LifterMap, Report};
use opltypes::Date;

mod bodyweight;
use bodyweight::check_bodyweight_all;
mod name;
use name::check_name_all;
mod sex;
use sex::check_sex_all;
mod duplicates;
use duplicates::check_duplicates_all;

/// Return type for consistency check functions.
pub enum ConsistencyResult {
    Consistent,
    Inconsistent,
    Skipped,
}

/// Helper for getting the date of an [Entry].
pub fn get_date(meetdata: &AllMeetData, entry: &Entry) -> Date {
    if let Some(date) = entry.entrydate {
        date
    } else {
        meetdata
            .get_meet(entry.index.expect("Unassigned EntryIndex"))
            .date
    }
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
pub fn check(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
) -> Vec<Report> {
    let mut reports = Vec::new();

    check_sex_all(liftermap, meetdata, lifterdata, &mut reports);
    check_name_all(liftermap, meetdata, &mut reports);
    check_bodyweight_all(liftermap, meetdata, lifterdata, &mut reports);
    check_duplicates_all(liftermap, meetdata, &mut reports);

    reports
}
