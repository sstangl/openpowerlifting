//! Checks that bodyweight changes over time are plausible.

use opltypes::Date;

use crate::checklib::consistency::{self, date, ConsistencyResult};
use crate::{AllMeetData, Entry, EntryIndex, LifterDataMap, LifterMap, Report};

/// Get the average change in bodyweight from `a` to `b` as a percentage per
/// day.
fn calc_average_percentage_change(a: &Entry, b: &Entry, a_date: Date, b_date: Date) -> f32 {
    // Handle division-by-zero cases early.
    if a.bodyweightkg.is_zero() || b.bodyweightkg.is_zero() || a_date == b_date {
        return 0.0;
    }

    // Get the absolute change in bodyweight over the interval.
    let a_bw = f32::from(a.bodyweightkg);
    let b_bw = f32::from(b.bodyweightkg);
    let bw_delta = f32::abs(a_bw - b_bw);

    // Express that delta as a percentage change with respect to Entry `a`.
    let as_percentage = (bw_delta / a_bw) * 100.0;

    // Get the average change in percentage over the given time interval.
    // Note that if `b_date` is earlier that `a_date`, `interval_days` can be
    // negative.
    let interval_days = (b_date - a_date) as f32;

    as_percentage / interval_days
}

/// Checks bodyweight consistency for a single lifter.
pub fn check_bodyweight_one(
    indices: &[EntryIndex],
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
    report: &mut Report,
) -> ConsistencyResult {
    if consistency::should_skip_lifter(meetdata.entry(indices[0])) {
        return ConsistencyResult::Skipped;
    }

    // Allow manually excluding lifters through `lifter-data/bw-exemptions.csv`.
    let username = &meetdata.entry(indices[0]).username;
    if let Some(data) = lifterdata.get(username) {
        if data.exempt_bodyweight {
            return ConsistencyResult::Skipped;
        }
    }

    // Entries in the LifterMap are already sorted by date.
    // Sort the entries by date.
    let entries: Vec<&Entry> = indices.iter().map(|i| meetdata.entry(*i)).collect();

    let mut prev: &Entry = entries[0];
    for entry in entries.iter().skip(1) {
        // Ignore entries with missing bodyweight.
        if entry.bodyweightkg.is_zero() {
            continue;
        }

        let prev_date = date(prev);
        let this_date = date(entry);

        let average_per_day = calc_average_percentage_change(prev, entry, prev_date, this_date);

        // Chosen to only produce a few warnings.
        // The intention is that this be tightened-up over time.
        const BODYWEIGHT_PERCENTAGE_CHANGE_PER_DAY_THRESHOLD: f32 = 55.0;

        if average_per_day.abs() > BODYWEIGHT_PERCENTAGE_CHANGE_PER_DAY_THRESHOLD {
            let days = this_date - prev_date;
            let plural = if days > 1 { "s" } else { "" };
            let msg = format!(
                "www.openpowerlifting.org/u/{} ranged [{}, {}] in {} day{}",
                entry.username, prev.bodyweightkg, entry.bodyweightkg, days, plural
            );
            report.warning(msg);
            return ConsistencyResult::Inconsistent;
        }

        prev = entry;
    }

    ConsistencyResult::Consistent
}

/// Checks bodyweight consistency for all lifters.
pub fn check_bodyweight_all(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
    reports: &mut Vec<Report>,
) {
    let mut report = Report::new("[Bodyweight Consistency]".into());

    for lifter_indices in liftermap.values() {
        check_bodyweight_one(lifter_indices, meetdata, lifterdata, &mut report);
    }

    if report.has_messages() {
        reports.push(report);
    }
}
