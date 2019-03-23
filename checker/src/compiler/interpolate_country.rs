//! Implementation of Country interpolation.

use opltypes::Country;

use crate::{AllMeetData, EntryIndex, LifterMap};

/// Returns a single Country that is consistent for all the Entries.
fn get_consistent_country(
    meetdata: &AllMeetData,
    indices: &[EntryIndex],
) -> Option<Country> {
    let mut country = None;
    for index in indices {
        let entry = meetdata.get_entry(*index);
        if entry.country.is_some() {
            if country.is_some() && country != entry.country {
                return None;
            }
            country = entry.country;
        }
    }
    country
}

/// Attempts to infer a Country for a lifter from surrounding Entry data.
pub fn interpolate_country(meetdata: &mut AllMeetData, liftermap: &LifterMap) {
    for (_username, indexvec) in liftermap {
        // Interpolation requires multiple entries.
        if indexvec.len() < 2 {
            continue;
        }

        if let Some(country) = get_consistent_country(&meetdata, &indexvec) {
            for index in indexvec {
                meetdata.get_entry_mut(*index).country = Some(country);
            }
        }
    }
}
