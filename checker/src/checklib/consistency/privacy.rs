//! Checks for Sex consistency errors.

use crate::{EntryMap, LifterData, Report};

/// Checks privacy consistency for all lifters.
///
/// Every lifter marked for privacy should appear in the data,
/// and there should be unambiguous interaction with disambiguations.
pub fn check_privacy_all(entrymap: &EntryMap, lifterdata: &LifterData, reports: &mut Vec<Report>) {
    let mut report = Report::new("[Privacy Consistency]".into());

    // TODO: This is an inefficient way to filter for all lifters with privacy.
    for (username, data) in lifterdata.iter_usernames() {
        if !data.privacy {
            continue;
        }

        // Every lifter with privacy marked should exist in the data.
        if !entrymap.contains_key(username) {
            let msg = format!("Lifter '{username}' marked for privacy, but not in data");
            report.error(msg);
        }

        // If a privacy-redacted lifter is not disambiguated, there should be no variants.
        // Otherwise, automatic disambiguation may lift results out of privacy.
        if !username.has_variant() {
            let variants = entrymap.get_variants_of(username);
            if !variants.is_empty() {
                let msg = format!(
                    "Lifter '{username}' marked for privacy, but {} variants exist",
                    variants.len()
                );
                report.error(msg);
            }
        }
    }

    if report.has_messages() {
        reports.push(report);
    }
}
