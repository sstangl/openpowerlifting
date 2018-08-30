//! Logic for efficiently selecting a subset of the database.

use opltypes::*;
use serde::{self, Serialize};

use std::ffi::OsStr;
use std::path;
use std::str::FromStr;

use opldb::MetaFederation;

/// Query selection descriptor, corresponding to HTML widgets.
#[derive(PartialEq, Serialize)]
pub struct Selection {
    pub equipment: EquipmentSelection,
    pub federation: FederationSelection,
    pub weightclasses: WeightClassSelection,
    pub sex: SexSelection,
    pub ageclass: AgeClassSelection,
    pub year: YearSelection,
    pub event: EventSelection,
    pub sort: SortSelection,
}

impl Default for Selection {
    fn default() -> Selection {
        Selection {
            equipment: EquipmentSelection::RawAndWraps,
            federation: FederationSelection::AllFederations,
            weightclasses: WeightClassSelection::AllClasses,
            sex: SexSelection::AllSexes,
            ageclass: AgeClassSelection::AllAges,
            year: YearSelection::AllYears,
            event: EventSelection::AllEvents,
            sort: SortSelection::ByWilks,
        }
    }
}

impl Selection {
    pub fn from_path(p: &path::Path) -> Result<Self, ()> {
        let mut ret = Selection::default();

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

        // Iterate over each path component, attempting to determine
        // what kind of data it is.
        for segment in p
            .ancestors()
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
                if parsed_federation {
                    return Err(());
                }
                ret.federation = f;
                parsed_federation = true;
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
            } else if let Ok(s) = segment.parse::<AgeClassSelection>() {
                if parsed_ageclass {
                    return Err(());
                }
                ret.ageclass = s;
                parsed_ageclass = true;
            // Check whether this is year information.
            } else if let Ok(y) = segment.parse::<YearSelection>() {
                if parsed_year {
                    return Err(());
                }
                ret.year = y;
                parsed_year = true;
            // Check whether this is sort information.
            } else if let Ok(s) = segment.parse::<SortSelection>() {
                if parsed_sort {
                    return Err(());
                }
                ret.sort = s;
                parsed_sort = true;
            // Check whether this is event information.
            } else if let Ok(e) = segment.parse::<EventSelection>() {
                if parsed_event {
                    return Err(());
                }
                ret.event = e;
                parsed_event = true;
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FederationSelection {
    AllFederations,
    One(Federation),
    Meta(MetaFederation),
}

impl FromStr for FederationSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(fed) = s.parse::<Federation>() {
            return Ok(FederationSelection::One(fed));
        }

        if let Ok(meta) = s.parse::<MetaFederation>() {
            return Ok(FederationSelection::Meta(meta));
        }

        Err(())
    }
}

impl Serialize for FederationSelection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Care must be taken that the same string isn't used by both
        // Federation and MetaFederation.
        match self {
            FederationSelection::AllFederations => serializer.serialize_str("All"),
            FederationSelection::One(fed) => fed.serialize(serializer),
            FederationSelection::Meta(meta) => meta.serialize(serializer),
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
    IpfM53,
    IpfM59,
    IpfM66,
    IpfM74,
    IpfM83,
    IpfM93,
    IpfM105,
    IpfM120,
    IpfMOver120,

    // IPF Women.
    IpfF43,
    IpfF47,
    IpfF52,
    IpfF57,
    IpfF63,
    IpfF72,
    IpfF84,
    IpfFOver84,

    // WP Men.
    WpM62,
    WpM69,
    WpM77,
    WpM85,
    WpM94,
    WpM105,
    WpM120,
    WpMOver120,

    // WP Women.
    WpF48,
    WpF53,
    WpF58,
    WpF64,
    WpF72,
    WpF84,
    WpF100,
    WpFOver100,
}

/// Helper function to save repetition.
fn make_bounds(lower: f32, upper: f32) -> (WeightKg, WeightKg) {
    (WeightKg::from_f32(lower), WeightKg::from_f32(upper))
}

/// Helper function for SHW classes.
fn make_bound_over(lower: f32) -> (WeightKg, WeightKg) {
    (WeightKg::from_f32(lower), WeightKg::max_value())
}

impl WeightClassSelection {
    /// Returns the bounds of the selected weight class.
    ///
    /// The lower bound is always exclusive.
    /// The upper bound is always inclusive.
    pub fn to_bounds(self) -> (WeightKg, WeightKg) {
        match self {
            WeightClassSelection::AllClasses => make_bound_over(0.0),

            WeightClassSelection::T44 => make_bounds(0.0, 44.0),
            WeightClassSelection::T48 => make_bounds(44.0, 48.0),
            WeightClassSelection::T52 => make_bounds(48.0, 52.0),
            WeightClassSelection::T56 => make_bounds(52.0, 56.0),
            WeightClassSelection::T60 => make_bounds(56.0, 60.0),
            WeightClassSelection::T67_5 => make_bounds(60.0, 67.5),
            WeightClassSelection::T75 => make_bounds(67.5, 75.0),
            WeightClassSelection::T82_5 => make_bounds(75.0, 82.5),
            WeightClassSelection::T90 => make_bounds(82.5, 90.0),
            WeightClassSelection::TOver90 => make_bound_over(90.0),
            WeightClassSelection::T100 => make_bounds(90.0, 100.0),
            WeightClassSelection::T110 => make_bounds(100.0, 110.0),
            WeightClassSelection::T125 => make_bounds(110.0, 125.0),
            WeightClassSelection::T140 => make_bounds(125.0, 140.0),
            WeightClassSelection::TOver140 => make_bound_over(140.0),

            WeightClassSelection::IpfM53 => make_bounds(0.0, 53.0),
            WeightClassSelection::IpfM59 => make_bounds(53.0, 59.0),
            WeightClassSelection::IpfM66 => make_bounds(59.0, 66.0),
            WeightClassSelection::IpfM74 => make_bounds(66.0, 74.0),
            WeightClassSelection::IpfM83 => make_bounds(74.0, 83.0),
            WeightClassSelection::IpfM93 => make_bounds(83.0, 93.0),
            WeightClassSelection::IpfM105 => make_bounds(93.0, 105.0),
            WeightClassSelection::IpfM120 => make_bounds(105.0, 120.0),
            WeightClassSelection::IpfMOver120 => make_bound_over(120.0),

            WeightClassSelection::IpfF43 => make_bounds(0.0, 43.0),
            WeightClassSelection::IpfF47 => make_bounds(43.0, 47.0),
            WeightClassSelection::IpfF52 => make_bounds(47.0, 52.0),
            WeightClassSelection::IpfF57 => make_bounds(52.0, 57.0),
            WeightClassSelection::IpfF63 => make_bounds(57.0, 63.0),
            WeightClassSelection::IpfF72 => make_bounds(63.0, 72.0),
            WeightClassSelection::IpfF84 => make_bounds(72.0, 84.0),
            WeightClassSelection::IpfFOver84 => make_bound_over(84.0),

            WeightClassSelection::WpM62 => make_bounds(0.0, 62.0),
            WeightClassSelection::WpM69 => make_bounds(62.0, 69.0),
            WeightClassSelection::WpM77 => make_bounds(69.0, 77.0),
            WeightClassSelection::WpM85 => make_bounds(77.0, 85.0),
            WeightClassSelection::WpM94 => make_bounds(85.0, 94.0),
            WeightClassSelection::WpM105 => make_bounds(94.0, 105.0),
            WeightClassSelection::WpM120 => make_bounds(105.0, 120.0),
            WeightClassSelection::WpMOver120 => make_bound_over(120.0),

            WeightClassSelection::WpF48 => make_bounds(0.0, 48.0),
            WeightClassSelection::WpF53 => make_bounds(48.0, 53.0),
            WeightClassSelection::WpF58 => make_bounds(53.0, 58.0),
            WeightClassSelection::WpF64 => make_bounds(58.0, 64.0),
            WeightClassSelection::WpF72 => make_bounds(64.0, 72.0),
            WeightClassSelection::WpF84 => make_bounds(72.0, 84.0),
            WeightClassSelection::WpF100 => make_bounds(84.0, 100.0),
            WeightClassSelection::WpFOver100 => make_bound_over(100.0),
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

            "ipf53" => Ok(WeightClassSelection::IpfM53),
            "ipf59" => Ok(WeightClassSelection::IpfM59),
            "ipf66" => Ok(WeightClassSelection::IpfM66),
            "ipf74" => Ok(WeightClassSelection::IpfM74),
            "ipf83" => Ok(WeightClassSelection::IpfM83),
            "ipf93" => Ok(WeightClassSelection::IpfM93),
            "ipf105" => Ok(WeightClassSelection::IpfM105),
            "ipf120" => Ok(WeightClassSelection::IpfM120),
            "ipfover120" => Ok(WeightClassSelection::IpfMOver120),

            "ipf43" => Ok(WeightClassSelection::IpfF43),
            "ipf47" => Ok(WeightClassSelection::IpfF47),
            "ipf52" => Ok(WeightClassSelection::IpfF52),
            "ipf57" => Ok(WeightClassSelection::IpfF57),
            "ipf63" => Ok(WeightClassSelection::IpfF63),
            "ipf72" => Ok(WeightClassSelection::IpfF72),
            "ipf84" => Ok(WeightClassSelection::IpfF84),
            "ipfover84" => Ok(WeightClassSelection::IpfFOver84),

            "wp62" => Ok(WeightClassSelection::WpM62),
            "wp69" => Ok(WeightClassSelection::WpM69),
            "wp77" => Ok(WeightClassSelection::WpM77),
            "wp85" => Ok(WeightClassSelection::WpM85),
            "wp94" => Ok(WeightClassSelection::WpM94),
            "wp105" => Ok(WeightClassSelection::WpM105),
            "wp120" => Ok(WeightClassSelection::WpM120),
            "wpover120" => Ok(WeightClassSelection::WpMOver120),

            "wp48" => Ok(WeightClassSelection::WpF48),
            "wp53" => Ok(WeightClassSelection::WpF53),
            "wp58" => Ok(WeightClassSelection::WpF58),
            "wp64" => Ok(WeightClassSelection::WpF64),
            "wp72" => Ok(WeightClassSelection::WpF72),
            "wp84" => Ok(WeightClassSelection::WpF84),
            "wp100" => Ok(WeightClassSelection::WpF100),
            "wpover100" => Ok(WeightClassSelection::WpFOver100),

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

/// The AgeClass selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum AgeClassSelection {
    AllAges,
    Youth512,
    Juniors1315,
    Juniors1617,
    Juniors1819,
    Juniors2023,
    Seniors2434,
    Submasters3539,

    // By 10s.
    Masters4049,
    Masters5059,
    Masters6069,
    Masters7079,

    // By 5s.
    Masters4044,
    Masters4549,
    Masters5054,
    Masters5559,
    Masters6064,
    Masters6569,
    Masters7074,
    Masters7579,

    MastersOver80,
}

impl FromStr for AgeClassSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // No entry for AllAges, since it's default.
            "5-12" => Ok(AgeClassSelection::Youth512),
            "13-15" => Ok(AgeClassSelection::Juniors1315),
            "16-17" => Ok(AgeClassSelection::Juniors1617),
            "18-19" => Ok(AgeClassSelection::Juniors1819),
            "20-23" => Ok(AgeClassSelection::Juniors2023),
            "24-34" => Ok(AgeClassSelection::Seniors2434),
            "35-39" => Ok(AgeClassSelection::Submasters3539),
            "40-49" => Ok(AgeClassSelection::Masters4049),
            "50-59" => Ok(AgeClassSelection::Masters5059),
            "60-69" => Ok(AgeClassSelection::Masters6069),
            "70-79" => Ok(AgeClassSelection::Masters7079),
            "over80" => Ok(AgeClassSelection::MastersOver80),
            "40-44" => Ok(AgeClassSelection::Masters4044),
            "45-49" => Ok(AgeClassSelection::Masters4549),
            "50-54" => Ok(AgeClassSelection::Masters5054),
            "55-59" => Ok(AgeClassSelection::Masters5559),
            "60-64" => Ok(AgeClassSelection::Masters6064),
            "65-69" => Ok(AgeClassSelection::Masters6569),
            "70-74" => Ok(AgeClassSelection::Masters7074),
            "75-79" => Ok(AgeClassSelection::Masters7579),
            _ => Err(()),
        }
    }
}

/// The year selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum YearSelection {
    AllYears,
    Year2018,
    Year2017,
    Year2016,
    Year2015,
    Year2014,
    Year2013,
    Year2012,
    Year2011,
    Year2010,
    Year2009,
    Year2008,
    Year2007,
    Year2006,
    Year2005,
    Year2004,
    Year2003,
    Year2002,
    Year2001,
    Year2000,
    Year1999,
    Year1998,
    Year1997,
    Year1996,
    Year1995,
    Year1994,
    Year1993,
    Year1992,
    Year1991,
    Year1990,
    Year1989,
    Year1988,
    Year1987,
    Year1986,
    Year1985,
    Year1984,
    Year1983,
    Year1982,
    Year1981,
    Year1980,
    Year1979,
    Year1978,
    Year1977,
    Year1976,
    Year1975,
    Year1974,
    Year1973,
    Year1972,
    Year1971,
    Year1970,
    Year1969,
    Year1968,
    Year1967,
    Year1966,
    Year1965,
}

impl FromStr for YearSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // No entry for AllYears, since it's default.
            "2018" => Ok(YearSelection::Year2018),
            "2017" => Ok(YearSelection::Year2017),
            "2016" => Ok(YearSelection::Year2016),
            "2015" => Ok(YearSelection::Year2015),
            "2014" => Ok(YearSelection::Year2014),
            "2013" => Ok(YearSelection::Year2013),
            "2012" => Ok(YearSelection::Year2012),
            "2011" => Ok(YearSelection::Year2011),
            "2010" => Ok(YearSelection::Year2010),
            "2009" => Ok(YearSelection::Year2009),
            "2008" => Ok(YearSelection::Year2008),
            "2007" => Ok(YearSelection::Year2007),
            "2006" => Ok(YearSelection::Year2006),
            "2005" => Ok(YearSelection::Year2005),
            "2004" => Ok(YearSelection::Year2004),
            "2003" => Ok(YearSelection::Year2003),
            "2002" => Ok(YearSelection::Year2002),
            "2001" => Ok(YearSelection::Year2001),
            "2000" => Ok(YearSelection::Year2000),
            "1999" => Ok(YearSelection::Year1999),
            "1998" => Ok(YearSelection::Year1998),
            "1997" => Ok(YearSelection::Year1997),
            "1996" => Ok(YearSelection::Year1996),
            "1995" => Ok(YearSelection::Year1995),
            "1994" => Ok(YearSelection::Year1994),
            "1993" => Ok(YearSelection::Year1993),
            "1992" => Ok(YearSelection::Year1992),
            "1991" => Ok(YearSelection::Year1991),
            "1990" => Ok(YearSelection::Year1990),
            "1989" => Ok(YearSelection::Year1989),
            "1988" => Ok(YearSelection::Year1988),
            "1987" => Ok(YearSelection::Year1987),
            "1986" => Ok(YearSelection::Year1986),
            "1985" => Ok(YearSelection::Year1985),
            "1984" => Ok(YearSelection::Year1984),
            "1983" => Ok(YearSelection::Year1983),
            "1982" => Ok(YearSelection::Year1982),
            "1981" => Ok(YearSelection::Year1981),
            "1980" => Ok(YearSelection::Year1980),
            "1979" => Ok(YearSelection::Year1979),
            "1978" => Ok(YearSelection::Year1978),
            "1977" => Ok(YearSelection::Year1977),
            "1976" => Ok(YearSelection::Year1976),
            "1975" => Ok(YearSelection::Year1975),
            "1974" => Ok(YearSelection::Year1974),
            "1973" => Ok(YearSelection::Year1973),
            "1972" => Ok(YearSelection::Year1972),
            "1971" => Ok(YearSelection::Year1971),
            "1970" => Ok(YearSelection::Year1970),
            "1969" => Ok(YearSelection::Year1969),
            "1968" => Ok(YearSelection::Year1968),
            "1967" => Ok(YearSelection::Year1967),
            "1966" => Ok(YearSelection::Year1966),
            "1965" => Ok(YearSelection::Year1965),
            _ => Err(()),
        }
    }
}

impl YearSelection {
    #[inline]
    pub fn as_u32(self) -> Option<u32> {
        match self {
            YearSelection::AllYears => None,
            YearSelection::Year2018 => Some(2018),
            YearSelection::Year2017 => Some(2017),
            YearSelection::Year2016 => Some(2016),
            YearSelection::Year2015 => Some(2015),
            YearSelection::Year2014 => Some(2014),
            YearSelection::Year2013 => Some(2013),
            YearSelection::Year2012 => Some(2012),
            YearSelection::Year2011 => Some(2011),
            YearSelection::Year2010 => Some(2010),
            YearSelection::Year2009 => Some(2009),
            YearSelection::Year2008 => Some(2008),
            YearSelection::Year2007 => Some(2007),
            YearSelection::Year2006 => Some(2006),
            YearSelection::Year2005 => Some(2005),
            YearSelection::Year2004 => Some(2004),
            YearSelection::Year2003 => Some(2003),
            YearSelection::Year2002 => Some(2002),
            YearSelection::Year2001 => Some(2001),
            YearSelection::Year2000 => Some(2000),
            YearSelection::Year1999 => Some(1999),
            YearSelection::Year1998 => Some(1998),
            YearSelection::Year1997 => Some(1997),
            YearSelection::Year1996 => Some(1996),
            YearSelection::Year1995 => Some(1995),
            YearSelection::Year1994 => Some(1994),
            YearSelection::Year1993 => Some(1993),
            YearSelection::Year1992 => Some(1992),
            YearSelection::Year1991 => Some(1991),
            YearSelection::Year1990 => Some(1990),
            YearSelection::Year1989 => Some(1989),
            YearSelection::Year1988 => Some(1988),
            YearSelection::Year1987 => Some(1987),
            YearSelection::Year1986 => Some(1986),
            YearSelection::Year1985 => Some(1985),
            YearSelection::Year1984 => Some(1984),
            YearSelection::Year1983 => Some(1983),
            YearSelection::Year1982 => Some(1982),
            YearSelection::Year1981 => Some(1981),
            YearSelection::Year1980 => Some(1980),
            YearSelection::Year1979 => Some(1979),
            YearSelection::Year1978 => Some(1978),
            YearSelection::Year1977 => Some(1977),
            YearSelection::Year1976 => Some(1976),
            YearSelection::Year1975 => Some(1975),
            YearSelection::Year1974 => Some(1974),
            YearSelection::Year1973 => Some(1973),
            YearSelection::Year1972 => Some(1972),
            YearSelection::Year1971 => Some(1971),
            YearSelection::Year1970 => Some(1970),
            YearSelection::Year1969 => Some(1969),
            YearSelection::Year1968 => Some(1968),
            YearSelection::Year1967 => Some(1967),
            YearSelection::Year1966 => Some(1966),
            YearSelection::Year1965 => Some(1965),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum EventSelection {
    /// Any event.
    AllEvents,
    /// Corresponds to SBD.
    FullPower,
    /// Corresponds to BD.
    PushPull,
    /// Corresponds to S.
    SquatOnly,
    /// Corresponds to B.
    BenchOnly,
    /// Corresponds to D.
    DeadliftOnly,
}

impl FromStr for EventSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full-power" => Ok(EventSelection::FullPower),
            "push-pull" => Ok(EventSelection::PushPull),
            "squat-only" => Ok(EventSelection::SquatOnly),
            "bench-only" => Ok(EventSelection::BenchOnly),
            "deadlift-only" => Ok(EventSelection::DeadliftOnly),
            _ => Err(()),
        }
    }
}

/// The sort selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum SortSelection {
    BySquat,
    ByBench,
    ByDeadlift,
    ByTotal,
    // ByAllometric,
    ByGlossbrenner,
    ByMcCulloch,
    ByWilks,
}

impl FromStr for SortSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "by-squat" => Ok(SortSelection::BySquat),
            "by-bench" => Ok(SortSelection::ByBench),
            "by-deadlift" => Ok(SortSelection::ByDeadlift),
            "by-total" => Ok(SortSelection::ByTotal),
            // "by-allometric" => Ok(SortSelection::ByAllometric),
            "by-glossbrenner" => Ok(SortSelection::ByGlossbrenner),
            "by-mcculloch" => Ok(SortSelection::ByMcCulloch),
            "by-wilks" => Ok(SortSelection::ByWilks),
            _ => Err(()),
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
        assert_eq!(s.federation, FederationSelection::One(Federation::USPA));
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

        // Disallow nonsense.
        assert!(Selection::from_path(Path::new("912h3h123h12ch39")).is_err());
        assert!(Selection::from_path(Path::new(".......")).is_err());
        assert!(Selection::from_path(Path::new("/menwomen")).is_err());
    }
}
