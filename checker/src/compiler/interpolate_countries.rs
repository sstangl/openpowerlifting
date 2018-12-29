use crate::check_entries::Entry;
use opltypes::*;

fn is_country_consistent(entries: &[Entry]) -> bool {
    let mut curr_country = None;
    for entry in entries {
        if entry.country.is_some() {
            if curr_country.is_some() && entry.country != curr_country {
                return false;
            }
            curr_country = entry.country;
        }
    }

    true
}

fn interpolate_array(entries: &mut [Entry]) {
    let lifter_country = entries.iter().find_map(|e| e.country);
    for entry in entries {
        entry.country = lifter_country;
    }
}

pub fn interpolate(entries: &mut [Entry]) {
    if is_country_consistent(entries) {
        interpolate_array(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for generating test data
    fn entry(country: Option<Country>) -> Entry {
        Entry {
            country,
            ..Entry::default()
        }
    }

    #[test]
    fn test_interp_start() {
        let a = entry(None);
        let b = entry(None);
        let c = entry(Some(Country::USA));
        let d = entry(Some(Country::USA));

        let e = entry(Some(Country::USA));
        let f = entry(Some(Country::USA));
        let g = entry(Some(Country::USA));
        let h = entry(Some(Country::USA));

        let mut interp_arr = [a, b, c, d];
        let old_arr = [e, f, g, h];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

    #[test]
    fn test_interp_end() {
        let a = entry(Some(Country::USA));
        let b = entry(Some(Country::USA));
        let c = entry(None);
        let d = entry(None);

        let e = entry(Some(Country::USA));
        let f = entry(Some(Country::USA));
        let g = entry(Some(Country::USA));
        let h = entry(Some(Country::USA));

        let mut interp_arr = [a, b, c, d];
        let old_arr = [e, f, g, h];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

    #[test]
    fn test_interp_gaps() {
        let a = entry(Some(Country::USA));
        let b = entry(None);
        let c = entry(None);
        let d = entry(Some(Country::USA));

        let e = entry(Some(Country::USA));
        let f = entry(Some(Country::USA));
        let g = entry(Some(Country::USA));
        let h = entry(Some(Country::USA));

        let mut interp_arr = [a, b, c, d];
        let old_arr = [e, f, g, h];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

    #[test]
    fn test_invalid_interp() {
        let a = entry(Some(Country::USA));
        let b = entry(Some(Country::Estonia));
        let c = entry(Some(Country::USA));

        let d = entry(Some(Country::USA));
        let e = entry(Some(Country::Estonia));
        let f = entry(Some(Country::USA));

        let mut interp_arr = [a, b, c];
        let old_arr = [d, e, f];

        interpolate(&mut interp_arr);

        assert!(interp_arr.iter().eq(old_arr.iter()));
    }

}
