//! Checks for duplicate meets.

use crate::checklib::consistency::{self, get_date, ConsistencyResult};
use crate::{AllMeetData, Entry, EntryIndex, LifterMap, Meet, Report};

/// Checks for duplicate meets with different MeetPaths for one lifter.
pub fn check_duplicates_one(
    indices: &[EntryIndex],
    meetdata: &AllMeetData,
    report: &mut Report,
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
        let date_a = get_date(&meetdata.get_entry(*ei_a));
        let date_b = get_date(&meetdata.get_entry(*ei_b));
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

        for &index in ei_by_date.iter().skip(i + 1) {
            let match_entry: &Entry = meetdata.get_entry(index);
            let match_meet: &Meet = meetdata.get_meet(index);

            if cur_meet.date != match_meet.date {
                break; // No more pairs on this date with cur_entry.
            }

            // Check whether (cur_entry, match_entry) is a duplicate pair.
            if (cur_meet.path != match_meet.path)
                && (cur_entry.totalkg == match_entry.totalkg)
                && (cur_entry.event == match_entry.event)
                && (cur_entry.equipment == match_entry.equipment)
            {
                // FIXME: a USAPL meet was intentionally broken up into two
                //        separate meets. This is instead of some exemption.
                if cur_meet.date == opltypes::Date::from_parts(2016, 12, 9) {
                    continue;
                }
                // gpc-can/2003 and wrpf-can/2001 is a single meet, dual-sanctioned.
                if cur_meet.date == opltypes::Date::from_parts(2020, 11, 7) {
                    continue;
                }
                // usapl-archive/MO-2001-04-21-A and usapl-archive/MO-2001-04-21-B.
                if cur_meet.date == opltypes::Date::from_parts(2001, 4, 21) {
                    continue;
               }
                // mags/ip/IP-2002-01-16-C and mags/ip/IP-2002-01-17-A.
                if cur_meet.date == opltypes::Date::from_parts(2001, 11, 25) {
                    continue;
                }
                // mags/ip/IP-2002-02-16-C and mags/ip/IP-2002-02-16-B.
                if cur_meet.date == opltypes::Date::from_parts(2002, 02, 10) {
                    continue;
                }
                // usapl/NS-2021-07 and usapl/NS-2021-07-B.
                if cur_meet.date == opltypes::Date::from_parts(2021, 03, 20) {
                    continue;
                }
                // uspc/2118 and wp-usa/2103.
                if cur_meet.date == opltypes::Date::from_parts(2021, 06, 26) {
                    continue;
                }

                let msg = format!(
                    "www.openpowerlifting.org/u/{} on {}: {} and {}",
                    username, cur_meet.date, cur_meet.path, match_meet.path,
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
