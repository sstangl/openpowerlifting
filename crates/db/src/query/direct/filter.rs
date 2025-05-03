//! Logic for efficiently selecting a subset of the database.

use arrayvec::ArrayString;
use opltypes::states::*;
use opltypes::*;
use serde::{self, Serialize};

use std::fmt::Write;
use std::str::FromStr;

use crate::MetaFederation;

/// Query selection descriptor, corresponding to HTML widgets.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EntryFilter {
    pub equipment: EquipmentFilter,
    pub federation: FederationFilter,
    pub weightclasses: WeightClassFilter,
    pub sex: SexFilter,
    pub ageclass: AgeClassFilter,
    pub year: YearFilter,
    pub event: EventFilter,
    pub state: Option<State>,
}

impl Default for EntryFilter {
    fn default() -> Self {
        Self {
            equipment: EquipmentFilter::RawAndWraps,
            federation: FederationFilter::AllFederations,
            weightclasses: WeightClassFilter::AllClasses,
            sex: SexFilter::AllSexes,
            ageclass: AgeClassFilter::AllAges,
            year: YearFilter::AllYears,
            event: EventFilter::AllEvents,
            state: None,
        }
    }
}

/// Limits a query to specific equipment.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum EquipmentFilter {
    Raw,
    Wraps,
    RawAndWraps,
    Single,
    Multi,
    Unlimited,
}

impl Default for EquipmentFilter {
    fn default() -> Self {
        Self::RawAndWraps
    }
}

impl FromStr for EquipmentFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(EquipmentFilter::Raw),
            "wraps" => Ok(EquipmentFilter::Wraps),
            "raw-and-wraps" => Ok(EquipmentFilter::RawAndWraps),
            "single" => Ok(EquipmentFilter::Single),
            "multi" => Ok(EquipmentFilter::Multi),
            "unlimited" => Ok(EquipmentFilter::Unlimited),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FederationFilter {
    AllFederations,
    One(Federation),
    Meta(MetaFederation),
}

/// Controls whether to first try parsing as a MetaFederation or as a
/// Federation.
///
/// The FederationFilter is overloaded and can parse as either a
/// MetaFederation or as a Federation. Unfortunately, in different contexts,
/// different options are preferred.
///
/// For example, the USPA is parseable as both Federation::USPA and
/// MetaFederation::USPA. The MetaFederation includes Federation::IPL events,
/// allowing USPA records and rankings to be set in their international
/// affiliate.
///
/// 1. When showing rankings and records, we want the MetaFederation::USPA.
/// 2. When showing the meet list, we want the Federation::USPA. Otherwise, it
///    gets cluttered with international events. This is particularly bad for
///    IPF affiliates.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FedPreference {
    PreferMetaFederation,
    PreferFederation,
}

pub struct FromStrError;

impl FederationFilter {
    pub fn from_str_preferring(s: &str, preference: FedPreference) -> Result<Self, FromStrError> {
        match preference {
            FedPreference::PreferMetaFederation => {
                if let Ok(meta) = s.parse::<MetaFederation>() {
                    return Ok(FederationFilter::Meta(meta));
                }
                if let Ok(fed) = s.parse::<Federation>() {
                    return Ok(FederationFilter::One(fed));
                }
                Err(FromStrError)
            }
            FedPreference::PreferFederation => {
                if let Ok(fed) = s.parse::<Federation>() {
                    return Ok(FederationFilter::One(fed));
                }
                if let Ok(meta) = s.parse::<MetaFederation>() {
                    return Ok(FederationFilter::Meta(meta));
                }
                Err(FromStrError)
            }
        }
    }
}

impl Serialize for FederationFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Care must be taken that the same string isn't used by both
        // Federation and MetaFederation.
        match self {
            FederationFilter::AllFederations => serializer.serialize_str("All"),
            FederationFilter::One(fed) => fed.serialize(serializer),
            FederationFilter::Meta(meta) => meta.serialize(serializer),
        }
    }
}

/// The weight class selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum WeightClassFilter {
    AllClasses,

    // Traditional classes.
    T44,
    T48,
    TUnder52, // Only used for records. Not directly queryable.
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

    // Extended classes.
    TOver110,

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
    IpfF69,
    IpfF76,
    IpfF84,
    IpfFOver84,

    // Para Men.
    ParaM49,
    ParaM54,
    ParaM59,
    ParaM65,
    ParaM72,
    ParaM80,
    ParaM88,
    ParaM97,
    ParaM107,
    ParaMOver107,

    // Para Women.
    ParaF41,
    ParaF45,
    ParaF50,
    ParaF55,
    ParaF61,
    ParaF67,
    ParaF73,
    ParaF79,
    ParaF86,
    ParaFOver86,

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
    (WeightKg::from_f32(lower), WeightKg::MAX)
}

impl WeightClassFilter {
    /// Returns the bounds of the selected weight class.
    ///
    /// The lower bound is always exclusive.
    /// The upper bound is always inclusive.
    pub fn to_bounds(self) -> (WeightKg, WeightKg) {
        match self {
            WeightClassFilter::AllClasses => make_bound_over(0.0),

            WeightClassFilter::T44 => make_bounds(0.0, 44.0),
            WeightClassFilter::T48 => make_bounds(44.0, 48.0),
            WeightClassFilter::TUnder52 => make_bounds(0.0, 52.0),
            WeightClassFilter::T52 => make_bounds(48.0, 52.0),
            WeightClassFilter::T56 => make_bounds(52.0, 56.0),
            WeightClassFilter::T60 => make_bounds(56.0, 60.0),
            WeightClassFilter::T67_5 => make_bounds(60.0, 67.5),
            WeightClassFilter::T75 => make_bounds(67.5, 75.0),
            WeightClassFilter::T82_5 => make_bounds(75.0, 82.5),
            WeightClassFilter::T90 => make_bounds(82.5, 90.0),
            WeightClassFilter::TOver90 => make_bound_over(90.0),
            WeightClassFilter::T100 => make_bounds(90.0, 100.0),
            WeightClassFilter::T110 => make_bounds(100.0, 110.0),
            WeightClassFilter::T125 => make_bounds(110.0, 125.0),
            WeightClassFilter::T140 => make_bounds(125.0, 140.0),
            WeightClassFilter::TOver140 => make_bound_over(140.0),

            WeightClassFilter::TOver110 => make_bound_over(110.0),

            WeightClassFilter::IpfM53 => make_bounds(0.0, 53.0),
            WeightClassFilter::IpfM59 => make_bounds(53.0, 59.0),
            WeightClassFilter::IpfM66 => make_bounds(59.0, 66.0),
            WeightClassFilter::IpfM74 => make_bounds(66.0, 74.0),
            WeightClassFilter::IpfM83 => make_bounds(74.0, 83.0),
            WeightClassFilter::IpfM93 => make_bounds(83.0, 93.0),
            WeightClassFilter::IpfM105 => make_bounds(93.0, 105.0),
            WeightClassFilter::IpfM120 => make_bounds(105.0, 120.0),
            WeightClassFilter::IpfMOver120 => make_bound_over(120.0),

            WeightClassFilter::IpfF43 => make_bounds(0.0, 43.0),
            WeightClassFilter::IpfF47 => make_bounds(43.0, 47.0),
            WeightClassFilter::IpfF52 => make_bounds(47.0, 52.0),
            WeightClassFilter::IpfF57 => make_bounds(52.0, 57.0),
            WeightClassFilter::IpfF63 => make_bounds(57.0, 63.0),
            WeightClassFilter::IpfF69 => make_bounds(63.0, 69.0),
            WeightClassFilter::IpfF76 => make_bounds(69.0, 76.0),
            WeightClassFilter::IpfF84 => make_bounds(76.0, 84.0),
            WeightClassFilter::IpfFOver84 => make_bound_over(84.0),

            WeightClassFilter::ParaM49 => make_bounds(0.0, 49.0),
            WeightClassFilter::ParaM54 => make_bounds(49.0, 54.0),
            WeightClassFilter::ParaM59 => make_bounds(54.0, 59.0),
            WeightClassFilter::ParaM65 => make_bounds(59.0, 65.0),
            WeightClassFilter::ParaM72 => make_bounds(65.0, 72.0),
            WeightClassFilter::ParaM80 => make_bounds(72.0, 80.0),
            WeightClassFilter::ParaM88 => make_bounds(80.0, 88.0),
            WeightClassFilter::ParaM97 => make_bounds(88.0, 97.0),
            WeightClassFilter::ParaM107 => make_bounds(97.0, 107.0),
            WeightClassFilter::ParaMOver107 => make_bound_over(107.0),

            WeightClassFilter::ParaF41 => make_bounds(0.0, 41.0),
            WeightClassFilter::ParaF45 => make_bounds(41.0, 45.0),
            WeightClassFilter::ParaF50 => make_bounds(45.0, 50.0),
            WeightClassFilter::ParaF55 => make_bounds(50.0, 55.0),
            WeightClassFilter::ParaF61 => make_bounds(55.0, 61.0),
            WeightClassFilter::ParaF67 => make_bounds(61.0, 67.0),
            WeightClassFilter::ParaF73 => make_bounds(67.0, 73.0),
            WeightClassFilter::ParaF79 => make_bounds(73.0, 79.0),
            WeightClassFilter::ParaF86 => make_bounds(79.0, 86.0),
            WeightClassFilter::ParaFOver86 => make_bound_over(86.0),

            WeightClassFilter::WpM62 => make_bounds(0.0, 62.0),
            WeightClassFilter::WpM69 => make_bounds(62.0, 69.0),
            WeightClassFilter::WpM77 => make_bounds(69.0, 77.0),
            WeightClassFilter::WpM85 => make_bounds(77.0, 85.0),
            WeightClassFilter::WpM94 => make_bounds(85.0, 94.0),
            WeightClassFilter::WpM105 => make_bounds(94.0, 105.0),
            WeightClassFilter::WpM120 => make_bounds(105.0, 120.0),
            WeightClassFilter::WpMOver120 => make_bound_over(120.0),

            WeightClassFilter::WpF48 => make_bounds(0.0, 48.0),
            WeightClassFilter::WpF53 => make_bounds(48.0, 53.0),
            WeightClassFilter::WpF58 => make_bounds(53.0, 58.0),
            WeightClassFilter::WpF64 => make_bounds(58.0, 64.0),
            WeightClassFilter::WpF72 => make_bounds(64.0, 72.0),
            WeightClassFilter::WpF84 => make_bounds(72.0, 84.0),
            WeightClassFilter::WpF100 => make_bounds(84.0, 100.0),
            WeightClassFilter::WpFOver100 => make_bound_over(100.0),
        }
    }

    /// Returns the exact WeightClassKg this refers to.
    pub fn to_weightclasskg(self) -> WeightClassKg {
        match self {
            WeightClassFilter::AllClasses => WeightClassKg::Over(WeightKg::from_i32(0)),

            WeightClassFilter::T44 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(44)),
            WeightClassFilter::T48 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(48)),
            WeightClassFilter::TUnder52 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(52)),
            WeightClassFilter::T52 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(52)),
            WeightClassFilter::T56 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(56)),
            WeightClassFilter::T60 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(60)),
            WeightClassFilter::T67_5 => WeightClassKg::UnderOrEqual(WeightKg::from_f32(67.5)),
            WeightClassFilter::T75 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(75)),
            WeightClassFilter::T82_5 => WeightClassKg::UnderOrEqual(WeightKg::from_f32(82.5)),
            WeightClassFilter::T90 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(90)),
            WeightClassFilter::TOver90 => WeightClassKg::Over(WeightKg::from_i32(90)),
            WeightClassFilter::T100 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(100)),
            WeightClassFilter::T110 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(110)),
            WeightClassFilter::T125 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(125)),
            WeightClassFilter::T140 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(140)),
            WeightClassFilter::TOver140 => WeightClassKg::Over(WeightKg::from_i32(140)),

            WeightClassFilter::TOver110 => WeightClassKg::Over(WeightKg::from_i32(110)),

            WeightClassFilter::IpfM53 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(53)),
            WeightClassFilter::IpfM59 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(59)),
            WeightClassFilter::IpfM66 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(66)),
            WeightClassFilter::IpfM74 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(74)),
            WeightClassFilter::IpfM83 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(83)),
            WeightClassFilter::IpfM93 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(93)),
            WeightClassFilter::IpfM105 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(105)),
            WeightClassFilter::IpfM120 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(120)),
            WeightClassFilter::IpfMOver120 => WeightClassKg::Over(WeightKg::from_i32(120)),

            WeightClassFilter::IpfF43 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(43)),
            WeightClassFilter::IpfF47 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(47)),
            WeightClassFilter::IpfF52 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(52)),
            WeightClassFilter::IpfF57 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(57)),
            WeightClassFilter::IpfF63 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(63)),
            WeightClassFilter::IpfF69 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(69)),
            WeightClassFilter::IpfF76 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(76)),
            WeightClassFilter::IpfF84 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(84)),
            WeightClassFilter::IpfFOver84 => WeightClassKg::Over(WeightKg::from_i32(84)),

            WeightClassFilter::ParaM49 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(49)),
            WeightClassFilter::ParaM54 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(54)),
            WeightClassFilter::ParaM59 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(59)),
            WeightClassFilter::ParaM65 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(65)),
            WeightClassFilter::ParaM72 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(72)),
            WeightClassFilter::ParaM80 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(80)),
            WeightClassFilter::ParaM88 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(88)),
            WeightClassFilter::ParaM97 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(97)),
            WeightClassFilter::ParaM107 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(107)),
            WeightClassFilter::ParaMOver107 => WeightClassKg::Over(WeightKg::from_i32(107)),

            WeightClassFilter::ParaF41 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(41)),
            WeightClassFilter::ParaF45 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(45)),
            WeightClassFilter::ParaF50 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(50)),
            WeightClassFilter::ParaF55 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(55)),
            WeightClassFilter::ParaF61 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(61)),
            WeightClassFilter::ParaF67 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(67)),
            WeightClassFilter::ParaF73 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(73)),
            WeightClassFilter::ParaF79 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(79)),
            WeightClassFilter::ParaF86 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(86)),
            WeightClassFilter::ParaFOver86 => WeightClassKg::Over(WeightKg::from_i32(86)),

            WeightClassFilter::WpM62 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(62)),
            WeightClassFilter::WpM69 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(69)),
            WeightClassFilter::WpM77 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(77)),
            WeightClassFilter::WpM85 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(85)),
            WeightClassFilter::WpM94 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(94)),
            WeightClassFilter::WpM105 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(105)),
            WeightClassFilter::WpM120 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(120)),
            WeightClassFilter::WpMOver120 => WeightClassKg::Over(WeightKg::from_i32(120)),

            WeightClassFilter::WpF48 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(48)),
            WeightClassFilter::WpF53 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(53)),
            WeightClassFilter::WpF58 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(58)),
            WeightClassFilter::WpF64 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(64)),
            WeightClassFilter::WpF72 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(72)),
            WeightClassFilter::WpF84 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(84)),
            WeightClassFilter::WpF100 => WeightClassKg::UnderOrEqual(WeightKg::from_i32(100)),
            WeightClassFilter::WpFOver100 => WeightClassKg::Over(WeightKg::from_i32(100)),
        }
    }
}

impl FromStr for WeightClassFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "44" => Ok(WeightClassFilter::T44),
            "48" => Ok(WeightClassFilter::T48),
            "52" => Ok(WeightClassFilter::T52),
            "56" => Ok(WeightClassFilter::T56),
            "60" => Ok(WeightClassFilter::T60),
            "67.5" => Ok(WeightClassFilter::T67_5),
            "75" => Ok(WeightClassFilter::T75),
            "82.5" => Ok(WeightClassFilter::T82_5),
            "90" => Ok(WeightClassFilter::T90),
            "over90" => Ok(WeightClassFilter::TOver90),
            "100" => Ok(WeightClassFilter::T100),
            "110" => Ok(WeightClassFilter::T110),
            "125" => Ok(WeightClassFilter::T125),
            "140" => Ok(WeightClassFilter::T140),
            "over140" => Ok(WeightClassFilter::TOver140),

            "over110" => Ok(WeightClassFilter::TOver110),

            "ipf53" => Ok(WeightClassFilter::IpfM53),
            "ipf59" => Ok(WeightClassFilter::IpfM59),
            "ipf66" => Ok(WeightClassFilter::IpfM66),
            "ipf74" => Ok(WeightClassFilter::IpfM74),
            "ipf83" => Ok(WeightClassFilter::IpfM83),
            "ipf93" => Ok(WeightClassFilter::IpfM93),
            "ipf105" => Ok(WeightClassFilter::IpfM105),
            "ipf120" => Ok(WeightClassFilter::IpfM120),
            "ipfover120" => Ok(WeightClassFilter::IpfMOver120),

            "ipf43" => Ok(WeightClassFilter::IpfF43),
            "ipf47" => Ok(WeightClassFilter::IpfF47),
            "ipf52" => Ok(WeightClassFilter::IpfF52),
            "ipf57" => Ok(WeightClassFilter::IpfF57),
            "ipf63" => Ok(WeightClassFilter::IpfF63),
            "ipf69" => Ok(WeightClassFilter::IpfF69),
            "ipf76" => Ok(WeightClassFilter::IpfF76),
            "ipf84" => Ok(WeightClassFilter::IpfF84),
            "ipfover84" => Ok(WeightClassFilter::IpfFOver84),

            "para49" => Ok(WeightClassFilter::ParaM49),
            "para54" => Ok(WeightClassFilter::ParaM54),
            "para59" => Ok(WeightClassFilter::ParaM59),
            "para65" => Ok(WeightClassFilter::ParaM65),
            "para72" => Ok(WeightClassFilter::ParaM72),
            "para80" => Ok(WeightClassFilter::ParaM80),
            "para88" => Ok(WeightClassFilter::ParaM88),
            "para97" => Ok(WeightClassFilter::ParaM97),
            "para107" => Ok(WeightClassFilter::ParaM107),
            "paraover107" => Ok(WeightClassFilter::ParaMOver107),

            "para41" => Ok(WeightClassFilter::ParaF41),
            "para45" => Ok(WeightClassFilter::ParaF45),
            "para50" => Ok(WeightClassFilter::ParaF50),
            "para55" => Ok(WeightClassFilter::ParaF55),
            "para61" => Ok(WeightClassFilter::ParaF61),
            "para67" => Ok(WeightClassFilter::ParaF67),
            "para73" => Ok(WeightClassFilter::ParaF73),
            "para79" => Ok(WeightClassFilter::ParaF79),
            "para86" => Ok(WeightClassFilter::ParaF86),
            "paraover86" => Ok(WeightClassFilter::ParaFOver86),

            "wp62" => Ok(WeightClassFilter::WpM62),
            "wp69" => Ok(WeightClassFilter::WpM69),
            "wp77" => Ok(WeightClassFilter::WpM77),
            "wp85" => Ok(WeightClassFilter::WpM85),
            "wp94" => Ok(WeightClassFilter::WpM94),
            "wp105" => Ok(WeightClassFilter::WpM105),
            "wp120" => Ok(WeightClassFilter::WpM120),
            "wpover120" => Ok(WeightClassFilter::WpMOver120),

            "wp48" => Ok(WeightClassFilter::WpF48),
            "wp53" => Ok(WeightClassFilter::WpF53),
            "wp58" => Ok(WeightClassFilter::WpF58),
            "wp64" => Ok(WeightClassFilter::WpF64),
            "wp72" => Ok(WeightClassFilter::WpF72),
            "wp84" => Ok(WeightClassFilter::WpF84),
            "wp100" => Ok(WeightClassFilter::WpF100),
            "wpover100" => Ok(WeightClassFilter::WpFOver100),

            _ => Err(()),
        }
    }
}

/// The sex selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum SexFilter {
    AllSexes,
    Men,
    Women,
}

impl FromStr for SexFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // No entry for AllSexes, since it's default.
            "men" => Ok(SexFilter::Men),
            "women" => Ok(SexFilter::Women),
            _ => Err(()),
        }
    }
}

/// The AgeClass selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum AgeClassFilter {
    AllAges,
    Youth512,
    Teenage1315,
    Teenage1617,
    Teenage1819,
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

    // BirthYear-based classes.
    SubJuniorsY14Y18,
    JuniorsY14Y23,
    SeniorsY24Y39,
    MastersOverY40,
    MastersOverY50,
    MastersOverY60,
    MastersOverY70,
}

impl FromStr for AgeClassFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // No entry for AllAges, since it's default.
            "5-12" => Ok(AgeClassFilter::Youth512),
            "13-15" => Ok(AgeClassFilter::Teenage1315),
            "16-17" => Ok(AgeClassFilter::Teenage1617),
            "18-19" => Ok(AgeClassFilter::Teenage1819),
            "20-23" => Ok(AgeClassFilter::Juniors2023),
            "24-34" => Ok(AgeClassFilter::Seniors2434),
            "35-39" => Ok(AgeClassFilter::Submasters3539),
            "40-49" => Ok(AgeClassFilter::Masters4049),
            "50-59" => Ok(AgeClassFilter::Masters5059),
            "60-69" => Ok(AgeClassFilter::Masters6069),
            "70-79" => Ok(AgeClassFilter::Masters7079),
            "over80" => Ok(AgeClassFilter::MastersOver80),
            "40-44" => Ok(AgeClassFilter::Masters4044),
            "45-49" => Ok(AgeClassFilter::Masters4549),
            "50-54" => Ok(AgeClassFilter::Masters5054),
            "55-59" => Ok(AgeClassFilter::Masters5559),
            "60-64" => Ok(AgeClassFilter::Masters6064),
            "65-69" => Ok(AgeClassFilter::Masters6569),
            "70-74" => Ok(AgeClassFilter::Masters7074),
            "75-79" => Ok(AgeClassFilter::Masters7579),

            // BirthYear-based classes.
            "y14-y18" => Ok(AgeClassFilter::SubJuniorsY14Y18),
            "y14-y23" => Ok(AgeClassFilter::JuniorsY14Y23),
            "y24-y39" => Ok(AgeClassFilter::SeniorsY24Y39),
            "over-y40" => Ok(AgeClassFilter::MastersOverY40),
            "over-y50" => Ok(AgeClassFilter::MastersOverY50),
            "over-y60" => Ok(AgeClassFilter::MastersOverY60),
            "over-y70" => Ok(AgeClassFilter::MastersOverY70),

            _ => Err(()),
        }
    }
}

/// The year selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum YearFilter {
    AllYears,
    OneYear(u16),
}

impl FromStr for YearFilter {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(YearFilter::OneYear(s.parse::<u16>()?))
    }
}

impl YearFilter {
    #[inline]
    pub fn as_u32(self) -> Option<u32> {
        match self {
            YearFilter::AllYears => None,
            YearFilter::OneYear(year) => Some(year as u32),
        }
    }
}

impl Serialize for YearFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            YearFilter::AllYears => serializer.serialize_str("AllYears"),
            YearFilter::OneYear(y) => {
                let mut buf = ArrayString::<32>::new();
                write!(buf, "Year{y}").expect("ArrayString overflow");
                serializer.serialize_str(&buf)
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum EventFilter {
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

impl FromStr for EventFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all-events" => Ok(EventFilter::AllEvents),
            "full-power" => Ok(EventFilter::FullPower),
            "push-pull" => Ok(EventFilter::PushPull),
            "squat-only" => Ok(EventFilter::SquatOnly),
            "bench-only" => Ok(EventFilter::BenchOnly),
            "deadlift-only" => Ok(EventFilter::DeadliftOnly),
            _ => Err(()),
        }
    }
}

/// The sort selector widget.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub enum OrderBy {
    Squat,
    Bench,
    Deadlift,
    Total,
    Dots,
    Glossbrenner,
    Goodlift,
    McCulloch,
    Wilks,
}

impl Default for OrderBy {
    fn default() -> Self {
        Self::Dots
    }
}

impl FromStr for OrderBy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "by-squat" => Ok(OrderBy::Squat),
            "by-bench" => Ok(OrderBy::Bench),
            "by-deadlift" => Ok(OrderBy::Deadlift),
            "by-total" => Ok(OrderBy::Total),
            "by-dots" => Ok(OrderBy::Dots),
            "by-glossbrenner" => Ok(OrderBy::Glossbrenner),
            "by-goodlift" => Ok(OrderBy::Goodlift),
            "by-mcculloch" => Ok(OrderBy::McCulloch),
            "by-wilks" => Ok(OrderBy::Wilks),
            _ => Err(()),
        }
    }
}

impl From<OrderBy> for PointsSystem {
    fn from(selection: OrderBy) -> PointsSystem {
        match selection {
            // Weight sorts convert to Total.
            OrderBy::Squat => PointsSystem::Total,
            OrderBy::Bench => PointsSystem::Total,
            OrderBy::Deadlift => PointsSystem::Total,
            OrderBy::Total => PointsSystem::Total,

            // Point sorts are taken directly.
            OrderBy::Dots => PointsSystem::Dots,
            OrderBy::Glossbrenner => PointsSystem::Glossbrenner,
            OrderBy::Goodlift => PointsSystem::Goodlift,
            OrderBy::McCulloch => PointsSystem::McCulloch,
            OrderBy::Wilks => PointsSystem::Wilks,
        }
    }
}

impl OrderBy {
    /// Returns true if the OrderBy is by points, instead of by weight.
    pub fn is_by_points(self) -> bool {
        match self {
            // Weight sorts.
            OrderBy::Squat => false,
            OrderBy::Bench => false,
            OrderBy::Deadlift => false,
            OrderBy::Total => false,

            // Point sorts.
            OrderBy::Dots => true,
            OrderBy::Glossbrenner => true,
            OrderBy::Goodlift => true,
            OrderBy::McCulloch => true,
            OrderBy::Wilks => true,
        }
    }
}
