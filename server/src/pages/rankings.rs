//! Logic for the display of the rankings page.

use serde_json;

use langpack::{self, Language};
use opldb;
use opldb::fields::{Federation, WeightKg};
use opldb::CachedFilter;
use opldb::Filter;

use pages::jsdata::JsEntryRow;

use std::ffi::OsStr;
use std::path;
use std::str::FromStr;

/// Query selection descriptor, corresponding to HTML widgets.
#[derive(PartialEq, Serialize)]
pub struct Selection {
    pub equipment: EquipmentSelection,
    pub federations: Option<FederationSelection>,
    pub weightclasses: WeightClassSelection,
    pub sex: SexSelection,
}

impl Selection {
    pub fn new_default() -> Self {
        Selection {
            equipment: EquipmentSelection::RawAndWraps,
            federations: None,
            weightclasses: WeightClassSelection::AllClasses,
            sex: SexSelection::AllSexes,
        }
    }

    pub fn from_path(p: &path::Path) -> Result<Self, ()> {
        let mut ret = Selection::new_default();

        // Disallow empty path components.
        if let Some(s) = p.to_str() {
            if s.contains("//") {
                return Err(());
            }
        } else {
            // Failed parsing UTF-8;
            return Err(());
        }

        // Prevent fields from being overwritten or redundant.
        let mut parsed_equipment: bool = false;
        let mut parsed_federations: bool = false;
        let mut parsed_weightclasses: bool = false;
        let mut parsed_sex: bool = false;

        // Iterate over each component of the path, attempting to
        // determine what kind of data it is.
        for segment in p.ancestors()
            .filter_map(|a| a.file_name().and_then(OsStr::to_str))
        {
            // Check whether this is equipment information.
            if let Ok(e) = segment.parse::<EquipmentSelection>() {
                if parsed_equipment {
                    return Err(());
                }
                ret.equipment = e;
                parsed_equipment = true;
            // Check whether this is federation information.
            } else if let Ok(f) = segment.parse::<FederationSelection>() {
                if parsed_federations {
                    return Err(());
                }
                ret.federations = Some(f);
                parsed_federations = true;
            // Check whether this is weight class information.
            } else if let Ok(w) = segment.parse::<WeightClassSelection>() {
                if parsed_weightclasses {
                    return Err(());
                }
                ret.weightclasses = w;
                parsed_weightclasses = true;
            // Check whether this is sex information.
            } else if let Ok(s) = segment.parse::<SexSelection>() {
                if parsed_sex {
                    return Err(());
                }
                ret.sex = s;
                parsed_sex = true;
            // Unknown string, therefore malformed URL.
            } else {
                return Err(());
            }
        }

        Ok(ret)
    }
}

/// The equipment selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum EquipmentSelection {
    Raw,
    Wraps,
    /// Default selection.
    RawAndWraps,
    Single,
    Multi,
}

impl EquipmentSelection {
    pub fn to_cached_filter(self) -> CachedFilter {
        match self {
            EquipmentSelection::Raw => CachedFilter::EquipmentRaw,
            EquipmentSelection::Wraps => CachedFilter::EquipmentWraps,
            EquipmentSelection::RawAndWraps => CachedFilter::EquipmentRawAndWraps,
            EquipmentSelection::Single => CachedFilter::EquipmentSingle,
            EquipmentSelection::Multi => CachedFilter::EquipmentMulti,
        }
    }
}

impl FromStr for EquipmentSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(EquipmentSelection::Raw),
            "wraps" => Ok(EquipmentSelection::Wraps),
            // No entry for RawAndWraps, since it's default.
            "single" => Ok(EquipmentSelection::Single),
            "multi" => Ok(EquipmentSelection::Multi),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Serialize)]
pub struct FederationSelection(Vec<Federation>);

impl FromStr for FederationSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("+");

        // Check if the first part parses as a federation.
        // If it doesn't, there's no need to heap-allocate a vector.
        if let Some(s) = iter.next() {
            let fed = match Federation::from_url_str(s) {
                Ok(f) => f,
                Err(_) => return Err(()),
            };

            let mut acc = Vec::<Federation>::new();
            acc.push(fed);

            for part in iter {
                let fed = match Federation::from_url_str(part) {
                    Ok(f) => f,
                    Err(_) => return Err(()),
                };

                // Federations should occur at most once.
                if acc.contains(&fed) {
                    return Err(());
                }

                acc.push(fed);
            }

            Ok(FederationSelection(acc))
        } else {
            Err(())
        }
    }
}

/// The weight class selector widget.
#[derive(Copy, Clone, PartialEq, Serialize)]
pub enum WeightClassSelection {
    AllClasses,

    // Traditional classes.
    T44,
    T48,
    T52,
    T56,
    T60,
    T67_5,
    T75,
    T82_5,
    T90,
    TOver90,
    T100,
    T110,
    T125,
    T140,
    TOver140,

    // IPF Men.
    M53,
    M59,
    M66,
    M74,
    M83,
    M93,
    M105,
    M120,
    MOver120,

    // IPF Women.
    F43,
    F47,
    F52,
    F57,
    F63,
    F72,
    F84,
    FOver84,
}

/// Helper function to save repetition.
fn make_bounds(lower: f32, upper: f32) -> (WeightKg, WeightKg) {
    (WeightKg::from_f32(lower), WeightKg::from_f32(upper))
}

impl WeightClassSelection {
    /// Returns the bounds of the selected weight class.
    ///
    /// The lower bound is always exclusive.
    /// The upper bound is always inclusive.
    pub fn to_bounds(self) -> (WeightKg, WeightKg) {
        match self {
            WeightClassSelection::AllClasses => make_bounds(0.0, 999.0),

            WeightClassSelection::T44 => make_bounds(0.0, 44.0),
            WeightClassSelection::T48 => make_bounds(44.0, 48.0),
            WeightClassSelection::T52 => make_bounds(48.0, 52.0),
            WeightClassSelection::T56 => make_bounds(52.0, 56.0),
            WeightClassSelection::T60 => make_bounds(56.0, 60.0),
            WeightClassSelection::T67_5 => make_bounds(60.0, 67.5),
            WeightClassSelection::T75 => make_bounds(67.5, 75.0),
            WeightClassSelection::T82_5 => make_bounds(75.0, 82.5),
            WeightClassSelection::T90 => make_bounds(82.5, 90.0),
            WeightClassSelection::TOver90 => make_bounds(90.0, 999.0),
            WeightClassSelection::T100 => make_bounds(90.0, 100.0),
            WeightClassSelection::T110 => make_bounds(100.0, 110.0),
            WeightClassSelection::T125 => make_bounds(110.0, 125.0),
            WeightClassSelection::T140 => make_bounds(125.0, 140.0),
            WeightClassSelection::TOver140 => make_bounds(140.0, 999.0),

            WeightClassSelection::M53 => make_bounds(0.0, 53.0),
            WeightClassSelection::M59 => make_bounds(53.0, 59.0),
            WeightClassSelection::M66 => make_bounds(59.0, 66.0),
            WeightClassSelection::M74 => make_bounds(66.0, 74.0),
            WeightClassSelection::M83 => make_bounds(74.0, 83.0),
            WeightClassSelection::M93 => make_bounds(83.0, 93.0),
            WeightClassSelection::M105 => make_bounds(93.0, 105.0),
            WeightClassSelection::M120 => make_bounds(105.0, 120.0),
            WeightClassSelection::MOver120 => make_bounds(120.0, 999.0),

            WeightClassSelection::F43 => make_bounds(0.0, 43.0),
            WeightClassSelection::F47 => make_bounds(43.0, 47.0),
            WeightClassSelection::F52 => make_bounds(47.0, 52.0),
            WeightClassSelection::F57 => make_bounds(52.0, 57.0),
            WeightClassSelection::F63 => make_bounds(57.0, 63.0),
            WeightClassSelection::F72 => make_bounds(63.0, 72.0),
            WeightClassSelection::F84 => make_bounds(72.0, 84.0),
            WeightClassSelection::FOver84 => make_bounds(84.0, 999.0),
        }
    }
}

impl FromStr for WeightClassSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "44" => Ok(WeightClassSelection::T44),
            "48" => Ok(WeightClassSelection::T48),
            "52" => Ok(WeightClassSelection::T52),
            "56" => Ok(WeightClassSelection::T56),
            "60" => Ok(WeightClassSelection::T60),
            "67.5" => Ok(WeightClassSelection::T67_5),
            "75" => Ok(WeightClassSelection::T75),
            "82.5" => Ok(WeightClassSelection::T82_5),
            "90" => Ok(WeightClassSelection::T90),
            "over90" => Ok(WeightClassSelection::TOver90),
            "100" => Ok(WeightClassSelection::T100),
            "110" => Ok(WeightClassSelection::T110),
            "125" => Ok(WeightClassSelection::T125),
            "140" => Ok(WeightClassSelection::T140),
            "over140" => Ok(WeightClassSelection::TOver140),

            "ipf53" => Ok(WeightClassSelection::M53),
            "ipf59" => Ok(WeightClassSelection::M59),
            "ipf66" => Ok(WeightClassSelection::M66),
            "ipf74" => Ok(WeightClassSelection::M74),
            "ipf83" => Ok(WeightClassSelection::M83),
            "ipf93" => Ok(WeightClassSelection::M93),
            "ipf105" => Ok(WeightClassSelection::M105),
            "ipf120" => Ok(WeightClassSelection::M120),
            "ipfover120" => Ok(WeightClassSelection::MOver120),

            "ipf43" => Ok(WeightClassSelection::F43),
            "ipf47" => Ok(WeightClassSelection::F47),
            "ipf52" => Ok(WeightClassSelection::F52),
            "ipf57" => Ok(WeightClassSelection::F57),
            "ipf63" => Ok(WeightClassSelection::F63),
            "ipf72" => Ok(WeightClassSelection::F72),
            "ipf84" => Ok(WeightClassSelection::F84),
            "ipfover84" => Ok(WeightClassSelection::FOver84),

            _ => Err(()),
        }
    }
}

/// The sex selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum SexSelection {
    AllSexes,
    Men,
    Women,
}

impl SexSelection {
    pub fn to_cached_filter(self) -> Option<CachedFilter> {
        match self {
            SexSelection::AllSexes => None,
            SexSelection::Men => Some(CachedFilter::SexMale),
            SexSelection::Women => Some(CachedFilter::SexFemale),
        }
    }
}

impl FromStr for SexSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // No entry for AllSexes, since it's default.
            "men" => Ok(SexSelection::Men),
            "women" => Ok(SexSelection::Women),
            _ => Err(()),
        }
    }
}

/// The context object passed to `templates/rankings.html.tera`.
#[derive(Serialize)]
pub struct Context<'db, 'a> {
    pub page_title: String,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: opldb::WeightUnits,

    pub selection: &'a Selection,
    pub data: String,
}

impl<'db, 'a> Context<'db, 'a> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        language: Language,
        langinfo: &'db langpack::LangInfo,
        units: opldb::WeightUnits,
        selection: &'a Selection,
    ) -> Context<'db, 'a> {
        let strings = langinfo.get_translations(language);
        let number_format = language.number_format();

        // An equipment filter is always used.
        let filter_equipment: &Filter =
            opldb.get_filter(selection.equipment.to_cached_filter());
        let filter_sex: Option<&Filter> = selection
            .sex
            .to_cached_filter()
            .and_then(|c| Some(opldb.get_filter(c)));

        // If there is intersection to be done, perform it here.
        let rankings = if *selection != Selection::new_default() {
            let mut filter = if filter_sex.is_some() {
                filter_equipment.intersect(filter_sex.unwrap())
            } else {
                // FIXME: this sucks, but we need it such that |filter|
                // is always dropped at the end of this block... so we
                // have to unnecessarily allocate memory here.
                filter_equipment.clone()
            };

            // Filter by federation manually.
            if selection.federations.is_some() {
                let fedlist: &Vec<Federation> =
                    &selection.federations.as_ref().unwrap().0;

                // TODO -- this could be cleaner.
                filter = Filter {
                    list: filter
                        .list
                        .into_iter()
                        .filter(|&i| {
                            fedlist.contains(
                                &opldb.get_meet(opldb.get_entry(i).meet_id).federation,
                            )
                        })
                        .collect(),
                };
            }

            // Filter by weight class manually.
            if selection.weightclasses != WeightClassSelection::AllClasses {
                let (lower, upper) = selection.weightclasses.to_bounds();

                filter = Filter {
                    list: filter
                        .list
                        .into_iter()
                        .filter(|&i| {
                            opldb.get_entry(i).bodyweightkg > lower
                                && opldb.get_entry(i).bodyweightkg <= upper
                        })
                        .collect(),
                };
            }

            filter.sort_and_unique_by_wilks(&opldb)
        } else {
            filter_equipment.sort_and_unique_by_wilks(&opldb)
        };

        // Send over the top 100 by default.
        let top_100: Vec<JsEntryRow> = rankings.list[0..rankings.list.len().min(100)]
            .into_iter()
            .zip(0..)
            .map(|(&n, i)| {
                JsEntryRow::from(
                    opldb,
                    strings,
                    number_format,
                    units,
                    opldb.get_entry(n),
                    i,
                )
            })
            .collect();

        Context {
            page_title: "Rankings".to_string(),
            language: language,
            strings: strings,
            units: units,

            selection: selection,
            /// FIXME: Handle failure.
            data: serde_json::to_string(&top_100).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_selection_from_path() {
        let s = Selection::from_path(Path::new("/raw/men")).unwrap();
        assert_eq!(s.equipment, EquipmentSelection::Raw);
        assert_eq!(s.sex, SexSelection::Men);

        let s = Selection::from_path(Path::new("/wraps/women")).unwrap();
        assert_eq!(s.equipment, EquipmentSelection::Wraps);
        assert_eq!(s.sex, SexSelection::Women);

        let s = Selection::from_path(Path::new("/uspa/raw")).unwrap();
        assert!(s.federations.unwrap().0.contains(&Federation::USPA));
        assert_eq!(s.equipment, EquipmentSelection::Raw);

        let s = Selection::from_path(Path::new("/uspa+usapl+spf/raw")).unwrap();
        let fedlist = s.federations.unwrap().0;
        assert!(fedlist.contains(&Federation::USPA));
        assert!(fedlist.contains(&Federation::USAPL));
        assert!(fedlist.contains(&Federation::SPF));
        assert!(!fedlist.contains(&Federation::RPS));
        assert_eq!(s.equipment, EquipmentSelection::Raw);
    }

    #[test]
    fn test_selection_from_path_errors() {
        // Selectors should not be applied more than once per category.
        assert!(Selection::from_path(Path::new("/raw/raw")).is_err());
        assert!(Selection::from_path(Path::new("/wraps/raw")).is_err());
        assert!(Selection::from_path(Path::new("/women/men")).is_err());
        assert!(Selection::from_path(Path::new("/women/women/women/raw")).is_err());

        // Disallow stupid URLs that would ordinarily work fine.
        assert!(Selection::from_path(Path::new("/raw///////")).is_err());
        assert!(Selection::from_path(Path::new("////raw////")).is_err());
        assert!(Selection::from_path(Path::new("////////raw")).is_err());

        // Disallow redundant federations.
        assert!(Selection::from_path(Path::new("/nipf+spf+nipf/")).is_err());

        // Disallow nonsense.
        assert!(Selection::from_path(Path::new("912h3h123h12ch39")).is_err());
        assert!(Selection::from_path(Path::new(".......")).is_err());
        assert!(Selection::from_path(Path::new("/menwomen")).is_err());
    }
}
