//! Checks that disambiguated names match the name-disambiguation.csv file.

use opltypes::Username;

use std::fmt::Write;

use crate::{LifterDataMap, LifterMap, Report};

/// Checks disambiguation consistency for all disambiguated lifters.
///
/// Consistency means that the expected number of username variants are
/// all present, there are none above that number, and there are no
/// missing numbers between 1 and the maximum.
pub fn check_disambiguations_all(
    liftermap: &LifterMap,
    lifterdatamap: &LifterDataMap,
    reports: &mut Vec<Report>,
) {
    let mut report = Report::new("[Disambiguation Consistency]".into());

    // Check that all variant usernames are marked for disambiguation.
    for username in liftermap.keys() {
        let (base, variant) = username.to_parts();
        if variant == 0 {
            continue;
        }

        let base_username = Username::from_name(base.into()).unwrap();

        let mut marked = false;
        if let Some(data) = lifterdatamap.get(&base_username) {
            if data.disambiguation_count != 0 {
                marked = true;
            }

            // While we're here, also check that the variant does not
            // overflow the maximum number.
            if data.disambiguation_count < variant {
                let msg = format!(
                    "{} is variant {}, but only {} variants are defined",
                    username.as_str(),
                    variant,
                    data.disambiguation_count
                );
                report.error(msg);
            }
        }

        if !marked {
            let msg = format!("{} not marked for disambiguation", username.as_str());
            report.error(msg);
        }
    }

    // Check that usernames marked for disambiguation appear in the liftermap.
    let mut scratch = String::with_capacity(128);
    for (username, data) in lifterdatamap.iter() {
        if data.disambiguation_count == 0 {
            continue;
        }

        // The previous loop already checked each username to guarantee that
        // variants above the maximum do not occur.
        //
        // This loop then checks that all variants up to and including the
        // maximum do occur. This completes the checking.
        for variant in 1..=data.disambiguation_count {
            scratch.clear();
            if write!(&mut scratch, "{}{}", username.as_str(), variant).is_err() {
                let msg = format!("Failed write for {}, {}", username.as_str(), variant);
                report.error(msg);
            }

            if let Ok(u) = Username::from_name(&scratch) {
                if liftermap.get(&u).is_none() {
                    let msg = format!("{} missing for {}", u.as_str(), username.as_str());
                    report.error(msg);
                }
            }
        }
    }

    if report.has_messages() {
        reports.push(report);
    }
}
