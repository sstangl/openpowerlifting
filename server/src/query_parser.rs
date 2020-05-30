//! Parses URL paths into database queries.

use opldb::query::direct::*;
use opltypes::states::State;

use std::ffi::OsStr;
use std::path::Path;

/// Trait for parsing a URL path into a database query.
pub trait FromUrlPath: Copy + Clone {
    /// Parses a URL path into a database query.
    ///
    /// The `default` parameter is provided explicitly instead of calling
    /// `Self::default()` because the default is context-dependent by
    /// distribution. For example, OpenIPF and OpenPowerlifting have
    /// different selector defaults.
    fn from_url_path(p: &Path, default: &Self) -> Result<Self, ()>;
}

impl FromUrlPath for RankingsQuery {
    fn from_url_path(p: &Path, default: &Self) -> Result<Self, ()> {
        let mut ret: RankingsQuery = *default;

        // Disallow empty path components.
        if let Some(s) = p.to_str() {
            if s.contains("//") {
                return Err(());
            }
        } else {
            // Failed parsing UTF-8.
            return Err(());
        }

        // Prevent fields from being overwritten or redundant.
        let mut parsed_equipment: bool = false;
        let mut parsed_federation: bool = false;
        let mut parsed_weightclasses: bool = false;
        let mut parsed_sex: bool = false;
        let mut parsed_ageclass: bool = false;
        let mut parsed_year: bool = false;
        let mut parsed_sort: bool = false;
        let mut parsed_event: bool = false;
        let mut parsed_state: bool = false;

        // Iterate over each path component, attempting to determine
        // what kind of data it is.
        for segment in p
            .ancestors()
            .filter_map(|a| a.file_name().and_then(OsStr::to_str))
        {
            // Check whether this is equipment information.
            if let Ok(e) = segment.parse::<EquipmentFilter>() {
                if parsed_equipment {
                    return Err(());
                }
                ret.filter.equipment = e;
                parsed_equipment = true;
            // Check whether this is federation information.
            } else if let Ok(f) = FederationFilter::from_str_preferring(
                segment,
                FedPreference::PreferMetaFederation,
            ) {
                if parsed_federation {
                    return Err(());
                }
                ret.filter.federation = f;
                parsed_federation = true;
            // Check whether this is weight class information.
            } else if let Ok(w) = segment.parse::<WeightClassFilter>() {
                if parsed_weightclasses {
                    return Err(());
                }
                ret.filter.weightclasses = w;
                parsed_weightclasses = true;
            // Check whether this is sex information.
            } else if let Ok(s) = segment.parse::<SexFilter>() {
                if parsed_sex {
                    return Err(());
                }
                ret.filter.sex = s;
                parsed_sex = true;
            // Check whether this is age class information.
            } else if let Ok(c) = segment.parse::<AgeClassFilter>() {
                if parsed_ageclass {
                    return Err(());
                }
                ret.filter.ageclass = c;
                parsed_ageclass = true;
            // Check whether this is year information.
            } else if let Ok(y) = segment.parse::<YearFilter>() {
                if parsed_year {
                    return Err(());
                }
                ret.filter.year = y;
                parsed_year = true;
            // Check whether this is sort information.
            } else if let Ok(s) = segment.parse::<OrderBy>() {
                if parsed_sort {
                    return Err(());
                }
                ret.order_by = s;
                parsed_sort = true;
            // Check whether this is event information.
            } else if let Ok(e) = segment.parse::<EventFilter>() {
                if parsed_event {
                    return Err(());
                }
                ret.filter.event = e;
                parsed_event = true;
            // Check whether this is a Country-State code.
            } else if let Ok(s) = State::from_full_code(segment) {
                if parsed_state {
                    return Err(());
                }
                ret.filter.state = Some(s);
                parsed_state = true;
            // Unknown string, therefore malformed URL.
            } else {
                return Err(());
            }
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opldb::MetaFederation;
    use std::path::Path;

    #[test]
    fn test_rankings_query_from_path() {
        let d = RankingsQuery::default();

        let s = RankingsQuery::from_url_path(Path::new("/raw/men"), &d).unwrap();
        assert_eq!(s.filter.equipment, EquipmentFilter::Raw);
        assert_eq!(s.filter.sex, SexFilter::Men);

        let s = RankingsQuery::from_url_path(Path::new("/wraps/women"), &d).unwrap();
        assert_eq!(s.filter.equipment, EquipmentFilter::Wraps);
        assert_eq!(s.filter.sex, SexFilter::Women);

        let s = RankingsQuery::from_url_path(Path::new("/uspa/raw"), &d).unwrap();
        assert_eq!(s.filter.federation, FederationFilter::Meta(MetaFederation::USPA));
        assert_eq!(s.filter.equipment, EquipmentFilter::Raw);
    }

    #[test]
    fn test_rankings_query_from_path_errors() {
        let d = RankingsQuery::default();

        // Selectors should not be applied more than once per category.
        assert!(RankingsQuery::from_url_path(Path::new("/raw/raw"), &d).is_err());
        assert!(RankingsQuery::from_url_path(Path::new("/wraps/raw"), &d).is_err());
        assert!(RankingsQuery::from_url_path(Path::new("/women/men"), &d).is_err());
        assert!(RankingsQuery::from_url_path(Path::new("/women/women/women/raw"), &d).is_err());

        // Disallow stupid URLs that would ordinarily work fine.
        assert!(RankingsQuery::from_url_path(Path::new("/raw///////"), &d).is_err());
        assert!(RankingsQuery::from_url_path(Path::new("////raw////"), &d).is_err());
        assert!(RankingsQuery::from_url_path(Path::new("////////raw"), &d).is_err());

        // Disallow nonsense.
        assert!(RankingsQuery::from_url_path(Path::new("912h3h123h12ch39"), &d).is_err());
        assert!(RankingsQuery::from_url_path(Path::new("......."), &d).is_err());
        assert!(RankingsQuery::from_url_path(Path::new("/menwomen"), &d).is_err());
    }
}
