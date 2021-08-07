//! Checks for Sex consistency errors.

use crate::checklib::consistency::{self, ConsistencyResult};
use crate::{AllMeetData, EntryIndex, LifterDataMap, LifterMap, Report};

/// Checks sex consistency for a single lifter.
pub fn check_sex_one(
    indices: &[EntryIndex],
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
    report: &mut Report,
) -> ConsistencyResult {
    if consistency::should_skip_lifter(meetdata.entry(indices[0])) {
        return ConsistencyResult::Skipped;
    }

    // Allow manually excluding lifters through `lifter-data/sex-exemptions.csv`.
    let username = &meetdata.entry(indices[0]).username;
    if let Some(data) = lifterdata.get(username) {
        if data.exempt_sex {
            return ConsistencyResult::Skipped;
        }
    }

    // Check that all the Sex values are identical.
    let expected_sex = meetdata.entry(indices[0]).sex;
    for index in indices.iter().skip(1) {
        if meetdata.entry(*index).sex != expected_sex {
            let url = format!("www.openpowerlifting.org/u/{}", username);
            let name = &meetdata.entry(*index).name;
            let msg = format!("Sex conflict for '{}' - {}", name, url);
            report.error(msg);
            return ConsistencyResult::Inconsistent;
        }
    }

    ConsistencyResult::Consistent
}

/// Checks sex consistency for all lifters.
pub fn check_sex_all(
    liftermap: &LifterMap,
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
    reports: &mut Vec<Report>,
) {
    let mut report = Report::new("[Sex Consistency]".into());

    for lifter_indices in liftermap.values() {
        check_sex_one(lifter_indices, meetdata, lifterdata, &mut report);
    }

    if report.has_messages() {
        reports.push(report);
    }
}
