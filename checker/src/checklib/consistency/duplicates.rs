//! Checks for duplicate meets.

use crate::checklib::consistency::{self, get_date, ConsistencyResult};
use crate::{AllMeetData, Entry, EntryIndex, LifterMap, Meet, Report};

/// Checks for duplicate meets with different MeetPaths for one lifter.
pub fn check_duplicates_one(
    indices: &[EntryIndex],
    meetdata: &AllMeetData,
    reports: &mut Vec<Report>,
    scratch: &mut Vec<EntryIndex>,
) -> ConsistencyResult {
    if consistency::should_skip_lifter(&meetdata.get_entry(indices[0])) {
        return ConsistencyResult::Skipped;
    }

    let mut result = ConsistencyResult::Consistent;
    let username = &meetdata.get_entry(indices[0]).username;

    let ei_by_date = scratch;
    ei_by_date.clear();
    for index in indices.iter() {
        ei_by_date.push(*index);
    }

    let date_sort_closure = |ei_a: &EntryIndex, ei_b: &EntryIndex| {
        let date_a = get_date(meetdata, &meetdata.get_entry(*ei_a));
        let date_b = get_date(meetdata, &meetdata.get_entry(*ei_b));
        date_a.cmp(&date_b)
    };

    ei_by_date.sort_unstable_by(|a, b| date_sort_closure(a, b));

    // Compare all pairs of EntryIndexes that occur on the same date.
    //
    // The outer iteration excludes the last index since there are no
    // more indices for the inner loop to form a pair.
    for i in 0..ei_by_date.len() - 1 {
        let cur_entry: &Entry = meetdata.get_entry(ei_by_date[i]);
        let cur_meet: &Meet = meetdata.get_meet(ei_by_date[i]);

        if cur_entry.totalkg.is_zero() {
            continue; // DQs can give false positives.
        }

        for inner_i in i + 1..ei_by_date.len() {
            let match_entry: &Entry = meetdata.get_entry(ei_by_date[inner_i]);
            let match_meet: &Meet = meetdata.get_meet(ei_by_date[inner_i]);

            if cur_meet.date != match_meet.date {
                break; // No more pairs on this date with cur_entry.
            }

            // Check whether (cur_entry, match_entry) is a duplicate pair.
            if (cur_meet.path != match_meet.path)
                && (cur_entry.totalkg == match_entry.totalkg)
                && (cur_entry.event == match_entry.event)
            {
                let msg = format!(
                    "Duplicate meet for {} on {}: {} and {}",
                    username, cur_meet.date, cur_meet.path, match_meet.path,
                );
                let mut report = Report::new("[Consistency]".into());
                report.warning(msg);
                reports.push(report);
                result = ConsistencyResult::Inconsistent;
            }
        }
    }

    result
}

/// Checks for duplicate meets with different MeetPaths for all lifters.
pub fn check_duplicates_all(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    reports: &mut Vec<Report>,
) {
    let mut scratch: Vec<EntryIndex> = Vec::new();

    for lifter_indices in liftermap.values() {
        check_duplicates_one(lifter_indices, meetdata, reports, &mut scratch);
    }
}
