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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checklib::{Entry, Meet};
    use crate::SingleMeetData;

    /// Helper function to generate a single-meet AllMeetData struct
    /// from a list of entries.
    fn meetdata_from_vec(entries: Vec<Entry>) -> AllMeetData {
        let meet = Meet::test_default();
        let singlemeetdata = SingleMeetData { meet, entries };
        AllMeetData::from(vec![singlemeetdata])
    }

    /// If no entries have a set Country, interpolation should not do anything.
    #[test]
    fn all_none() {
        let entries = vec![Entry::default(), Entry::default()];
        let mut meetdata = meetdata_from_vec(entries);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.get_entry_at(0, 0).country, None);
        assert_eq!(meetdata.get_entry_at(0, 1).country, None);
    }

    /// If only one entry has a set Country, propagate that Country.
    #[test]
    fn one_some() {
        let usa = Entry {
            country: Some(Country::USA),
            ..Entry::default()
        };

        let mut meetdata =
            meetdata_from_vec(vec![Entry::default(), usa, Entry::default()]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.get_entry_at(0, 0).country, Some(Country::USA));
        assert_eq!(meetdata.get_entry_at(0, 1).country, Some(Country::USA));
        assert_eq!(meetdata.get_entry_at(0, 2).country, Some(Country::USA));
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

        let mut meetdata =
            meetdata_from_vec(vec![Entry::default(), usa, Entry::default(), russia]);
        let liftermap = meetdata.create_liftermap();
        interpolate_country(&mut meetdata, &liftermap);

        assert_eq!(meetdata.get_entry_at(0, 0).country, None);
        assert_eq!(meetdata.get_entry_at(0, 1).country, Some(Country::USA));
        assert_eq!(meetdata.get_entry_at(0, 2).country, None);
        assert_eq!(meetdata.get_entry_at(0, 3).country, Some(Country::Russia));
    }
}
