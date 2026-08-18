//! Implementation of Country (and State) interpolation.

use colored::*;
use opltypes::{Country, Username};

use crate::{AllMeetData, EntryIndex, LifterMap};

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_found_initial(debug: bool, country: Country, path: &Option<String>) {
    if debug {
        println!(
            "{} {:#?} {} {}",
            "Found initial Country".green(),
            country,
            "in".green(),
            path.as_ref().unwrap()
        );
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_matched(debug: bool, country: Country, path: &Option<String>) {
    if debug {
        println!(
            "{} {:#?} {} {}",
            "Matched Country".green(),
            country,
            "in".green(),
            path.as_ref().unwrap()
        );
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_conflict(debug: bool, country: Country, path: &Option<String>) {
    if debug {
        println!(
            "{} {:#?} {} {}",
            "Conflict with Country".bold().red(),
            country,
            "in".bold().red(),
            path.as_ref().unwrap()
        );
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_inferred(debug: bool, country: Country, path: &Option<String>) {
    if debug {
        println!(
            "{} {:#?} {} {}",
            "Inferred Country".bold().green(),
            country,
            "in".bold().green(),
            path.as_ref().unwrap()
        );
    }
}

/// Returns a single Country that is consistent for all the Entries.
fn consistent_country(
    meetdata: &AllMeetData,
    indices: &[EntryIndex],
    debug: bool,
) -> Option<Country> {
    // There are two common arrangements for reporting a lifter's country:
    //  1. When a lifter competes domestically, it is typical to not mark their country,
    //     since it is obvious from context.
    //  2. When a lifter competes internationally, it is typical to mark their country.
    let mut explicit_acc: Option<Country> = None; // Where the Country is marked explicitly.
    let mut implicit_acc: Option<Country> = None; // Where the Country is implicit from the fed.
    let mut found_implicit_conflict = false;

    // First, check for any entries where the lifter's country is explicitly marked.
    for &index in indices {
        let meet = meetdata.meet(index);
        let entry = meetdata.entry(index);

        // Get the MeetPath for more helpful debugging output.
        let path: Option<String> = debug.then(|| meet.path.clone());

        // Check for consistency with the explicit results.
        if let Some(country) = entry.country {
            if let Some(acc_country) = explicit_acc {
                if country == acc_country || country.contains(acc_country) {
                    trace_matched(debug, country, &path);
                } else if acc_country.contains(country) {
                    trace_matched(debug, country, &path);
                    explicit_acc = Some(country); // This country is more specific.
                } else {
                    trace_conflict(debug, country, &path);
                    return None; // Two explicit results contradict: can't be fixed.
                }
            } else {
                trace_found_initial(debug, country, &path);
                explicit_acc = Some(country);
            }
        }

        // Check for consistency with the implicit results.
        if !found_implicit_conflict
            && let Some(fed_country) = meet.federation.home_country()
            && (fed_country == meet.country || fed_country.contains(meet.country))
        {
            if let Some(acc_country) = implicit_acc {
                if fed_country == acc_country || fed_country.contains(acc_country) {
                    continue;
                } else if acc_country.contains(fed_country) {
                    implicit_acc = Some(fed_country);
                } else {
                    found_implicit_conflict = true;
                    implicit_acc = None;
                }
            } else {
                implicit_acc = Some(fed_country);
            }
        }
    }

    // Prefer explicit data when returning.
    if explicit_acc.is_some() {
        return explicit_acc;
    }

    // But if there is no explicit data, prefer consistent implicit data.
    if debug && found_implicit_conflict {
        println!("{}", "No consistent federation.home_country()".bold().red());
        return None;
    }
    implicit_acc
}

/// Country interpolation for a single lifter's entries.
fn interpolate_country_single_lifter(
    meetdata: &mut AllMeetData,
    indices: &[EntryIndex],
    debug: bool,
) {
    if let Some(country) = consistent_country(meetdata, indices, debug) {
        for &index in indices {
            // Get the MeetPath for more helpful debugging output.
            let path: Option<String> = if debug {
                Some(meetdata.meet(index).path.clone())
            } else {
                None
            };

            trace_inferred(debug, country, &path);
            meetdata.entry_mut(index).country = Some(country);
        }
    }
}

/// Public-facing entry point for debugging a single lifter's interpolation.
pub fn interpolate_country_debug_for(
    meetdata: &mut AllMeetData,
    liftermap: &LifterMap,
    username: &Username,
) {
    match liftermap.get(username) {
        Some(indices) => interpolate_country_single_lifter(meetdata, indices, true),
        None => println!("Username '{username}' not found"),
    }
}

/// Attempts to infer a Country for a lifter from surrounding Entry data.
pub fn interpolate_country(meetdata: &mut AllMeetData, liftermap: &LifterMap) {
    for indices in liftermap.values() {
        interpolate_country_single_lifter(meetdata, indices, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SingleMeetData;
    use crate::checklib::{Entry, Meet};

    use opltypes::Federation;

    /// Helper function to generate a single-meet AllMeetData struct
    /// from a list of entries.
    fn meetdata_from_vec(entries: Vec<Entry>) -> AllMeetData {
        let meet = Meet::test_default();
        let entries = entries.into_boxed_slice();
        let singlemeetdata = SingleMeetData { meet, entries };
        AllMeetData::from(vec![singlemeetdata])
    }

    /// If no entries have a set Country, interpolation should take the host country.
    #[test]
    fn all_none() {
        let entries = vec![Entry::default(), Entry::default()];
        let mut meetdata = meetdata_from_vec(entries);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.entry_at(0, 0).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 1).country, Some(Country::USA));
    }

    /// If there is a single entry in a regional affiliate but no explicit
    /// country, infer that the lifter is from that affiliate's country.
    #[test]
    fn one_entry_implied_country() {
        let meet = Meet {
            federation: Federation::AusPL,
            country: Country::Australia,
            ..Meet::test_default()
        };
        let entry = Entry::default();

        let mut meetdata = AllMeetData::from(vec![SingleMeetData {
            meet,
            entries: vec![entry].into(),
        }]);
        let liftermap = meetdata.create_liftermap();

        assert_eq!(meetdata.entry_at(0, 0).country, None);
        interpolate_country(&mut meetdata, &liftermap);
        assert_eq!(meetdata.entry_at(0, 0).country, Some(Country::Australia));
    }

    /// BP lifters should be marked as UK if the MeetCountry is England.
    #[test]
    fn allow_england_in_uk() {
        let meet = Meet {
            federation: Federation::BP, // home_country() == UK.
            country: Country::England,  // Is in the UK.
            ..Meet::test_default()
        };
        let entry = Entry::default();

        let mut meetdata = AllMeetData::from(vec![SingleMeetData {
            meet,
            entries: vec![entry].into(),
        }]);
        let liftermap = meetdata.create_liftermap();

        assert_eq!(meetdata.entry_at(0, 0).country, None);
        interpolate_country(&mut meetdata, &liftermap);
        assert_eq!(meetdata.entry_at(0, 0).country, Some(Country::UK));
    }

    /// If only one entry has a set Country, propagate that Country.
    #[test]
    fn one_some() {
        let usa = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let mut meetdata = meetdata_from_vec(vec![Entry::default(), usa, Entry::default()]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.entry_at(0, 0).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 1).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 2).country, Some(Country::USA));
    }

    /// If a regional fed inexplicitly holds a meet in another country,
    /// don't infer that the lifter is necessarily from the fed's country.
    #[test]
    fn fed_home_meet_mismatch() {
        let meet = Meet {
            federation: Federation::AusPL,
            country: Country::Norway,
            ..Meet::test_default()
        };
        let entry = Entry::default();

        let mut meetdata = AllMeetData::from(vec![SingleMeetData {
            meet,
            entries: vec![entry].into(),
        }]);
        let liftermap = meetdata.create_liftermap();

        assert_eq!(meetdata.entry_at(0, 0).country, None);
        interpolate_country(&mut meetdata, &liftermap);
        assert_eq!(meetdata.entry_at(0, 0).country, None);
    }

    /// If two entries conflict, don't propagate a Country.
    #[test]
    fn conflict() {
        let usa = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };
        let russia = Entry {
            country: Some(Country::Russia),
            ..Entry::default()
        };

        let mut meetdata = meetdata_from_vec(vec![Entry::default(), usa, Entry::default(), russia]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.entry_at(0, 0).country, None);
        assert_eq!(meetdata.entry_at(0, 1).country, Some(Country::USA));
        assert_eq!(meetdata.entry_at(0, 2).country, None);
        assert_eq!(meetdata.entry_at(0, 3).country, Some(Country::Russia));
    }

    /// Countries within the UK are compatible with Country:UK.
    #[test]
    fn uk_subsets() {
        let uk = Entry {
            country: Some(Country::UK),
            ..Entry::default()
        };
        let scotland = Entry {
            country: Some(Country::Scotland),
            ..Entry::default()
        };

        let mut meetdata =
            meetdata_from_vec(vec![Entry::default(), uk, Entry::default(), scotland]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.entry_at(0, 0).country, Some(Country::Scotland));
        assert_eq!(meetdata.entry_at(0, 1).country, Some(Country::Scotland));
        assert_eq!(meetdata.entry_at(0, 2).country, Some(Country::Scotland));
        assert_eq!(meetdata.entry_at(0, 3).country, Some(Country::Scotland));
    }
}
