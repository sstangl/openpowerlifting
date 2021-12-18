//! Interface for efficiently querying rankings.

use std::ffi::OsStr;
use std::path::Path;

use crate::query::direct::*;

/// A query for rankings information.
#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize)]
pub struct RankingsQuery {
    pub filter: EntryFilter,
    pub order_by: OrderBy,
}

/// Error type for `from_path()` impls.
#[derive(Debug)]
pub enum FromPathError {
    /// Utf8 parsing failed.
    NotUtf8,
    /// Some part of the path contained no information.
    EmptyComponent,
    /// Some component kind occurred more than once.
    ConflictingComponent,
    /// Some component could not be parsed into any type.
    UnknownComponent,
}

impl RankingsQuery {
    /// Parses a URL path into a #[RankingsQuery].
    ///
    /// The `default` parameter is provided explicitly instead of by calling
    /// `Self::default()` because the default is context-dependent. For example,
    /// OpenIPF and OpenPowerlifting have different selector defaults.
    pub fn from_url_path(p: &Path, default: &Self) -> Result<Self, FromPathError> {
        let mut ret: RankingsQuery = *default;

        // Disallow empty path components.
        if let Some(s) = p.to_str() {
            if s.contains("//") {
                return Err(FromPathError::EmptyComponent);
            }
        } else {
            // Failed parsing UTF-8.
            return Err(FromPathError::NotUtf8);
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
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.filter.equipment = e;
                parsed_equipment = true;
            // Check whether this is federation information.
            } else if let Ok(f) =
                FederationFilter::from_str_preferring(segment, FedPreference::PreferMetaFederation)
            {
                if parsed_federation {
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.filter.federation = f;
                parsed_federation = true;
            // Check whether this is weight class information.
            } else if let Ok(w) = segment.parse::<WeightClassFilter>() {
                if parsed_weightclasses {
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.filter.weightclasses = w;
                parsed_weightclasses = true;
            // Check whether this is sex information.
            } else if let Ok(s) = segment.parse::<SexFilter>() {
                if parsed_sex {
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.filter.sex = s;
                parsed_sex = true;
            // Check whether this is age class information.
            } else if let Ok(c) = segment.parse::<AgeClassFilter>() {
                if parsed_ageclass {
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.filter.ageclass = c;
                parsed_ageclass = true;
            // Check whether this is year information.
            } else if let Ok(y) = segment.parse::<YearFilter>() {
                if parsed_year {
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.filter.year = y;
                parsed_year = true;
            // Check whether this is sort information.
            } else if let Ok(s) = segment.parse::<OrderBy>() {
                if parsed_sort {
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.order_by = s;
                parsed_sort = true;
            // Check whether this is event information.
            } else if let Ok(e) = segment.parse::<EventFilter>() {
                if parsed_event {
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.filter.event = e;
                parsed_event = true;
            // Check whether this is a Country-State code.
            } else if let Ok(s) = opltypes::states::State::from_full_code(segment) {
                if parsed_state {
                    return Err(FromPathError::ConflictingComponent);
                }
                ret.filter.state = Some(s);
                parsed_state = true;
            // Unknown string, therefore malformed URL.
            } else {
                return Err(FromPathError::UnknownComponent);
            }
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MetaFederation;

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
        assert_eq!(
            s.filter.federation,
            FederationFilter::Meta(MetaFederation::USPA)
        );
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
