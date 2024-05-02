//! Checks for duplicate meets.

use crate::checklib::consistency::{self, date, ConsistencyResult};
use crate::{AllMeetData, Entry, EntryIndex, LifterMap, Meet, Report};

/// Checks for duplicate meets with different MeetPaths for one lifter.
pub fn check_duplicates_one(
    indices: &[EntryIndex],
    meetdata: &AllMeetData,
    report: &mut Report,
    scratch: &mut Vec<EntryIndex>,
) -> ConsistencyResult {
    if consistency::should_skip_lifter(meetdata.entry(indices[0])) {
        return ConsistencyResult::Skipped;
    }

    let mut result = ConsistencyResult::Consistent;
    let username = &meetdata.entry(indices[0]).username;

    let ei_by_date = scratch;
    ei_by_date.clear();
    for index in indices.iter() {
        ei_by_date.push(*index);
    }

    let date_sort_closure = |ei_a: &EntryIndex, ei_b: &EntryIndex| {
        let date_a = date(meetdata.entry(*ei_a));
        let date_b = date(meetdata.entry(*ei_b));
        date_a.cmp(&date_b)
    };

    ei_by_date.sort_unstable_by(date_sort_closure);

    // Compare all pairs of EntryIndexes that occur on the same date.
    //
    // The outer iteration excludes the last index since there are no
    // more indices for the inner loop to form a pair.
    for i in 0..ei_by_date.len() - 1 {
        let cur_entry: &Entry = meetdata.entry(ei_by_date[i]);
        let cur_meet: &Meet = meetdata.meet(ei_by_date[i]);

        if cur_meet.allow_duplicates {
            continue; // `ExemptDuplicates` was set in this meet's CONFIG.toml.
        }

        if cur_entry.totalkg.is_zero() {
            continue; // DQs can give false positives.
        }

        for &index in ei_by_date.iter().skip(i + 1) {
            let match_entry: &Entry = meetdata.entry(index);
            let match_meet: &Meet = meetdata.meet(index);

            if cur_meet.date != match_meet.date {
                break; // No more pairs on this date with cur_entry.
            }

            if match_meet.allow_duplicates {
                continue; // `ExemptDuplicates` was set in this meet's CONFIG.toml.
            }

            // Check whether (cur_entry, match_entry) is a duplicate pair.
            if (cur_meet.path != match_meet.path)
                && (cur_entry.totalkg == match_entry.totalkg)
                && (cur_entry.event == match_entry.event)
                && (cur_entry.equipment == match_entry.equipment)
            {
                let msg = format!(
                    "www.openpowerlifting.org/u/{username} on {}: {} and {}",
                    cur_meet.date, cur_meet.path, match_meet.path,
                );
                report.error(msg);
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
    let mut report = Report::new("[Duplicate Meets]".into());
    let mut scratch: Vec<EntryIndex> = Vec::new();

    for lifter_indices in liftermap.values() {
        check_duplicates_one(lifter_indices, meetdata, &mut report, &mut scratch);
    }

    if report.has_messages() {
        reports.push(report);
    }
}
