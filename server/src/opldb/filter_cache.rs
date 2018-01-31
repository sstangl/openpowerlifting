//! This file defines the filter cache.

use opldb::filter::Filter;
use opldb::fields::*;
use opldb::{Entry,Meet,Lifter};

/// The filters cached within the `OplDb`.
///
/// This exists so that the `OplDb::FilterCache` may be private,
/// with filters only accessible as borrowers through a getter.
pub enum CachedFilter {
    EquipmentRaw,
    EquipmentWraps,
    EquipmentSingle,
    EquipmentMulti,

    SexMale,
    SexFemale,

    Year2018,
    Year2017,
    Year2016,
    Year2015,
    Year2014,
}

/// List of precomputed filters.
pub struct FilterCache {
    // Equipment filters.
    equipment_raw: Filter,
    equipment_wraps: Filter,
    equipment_single: Filter,
    equipment_multi: Filter,

    // Sex filter.
    sex_male: Filter,
    sex_female: Filter,

    // Year filter.
    year_2018: Filter,
    year_2017: Filter,
    year_2016: Filter,
    year_2015: Filter,
    year_2014: Filter,
}

impl FilterCache {
    pub fn new(meets: &Vec<Meet>, entries: &Vec<Entry>) -> FilterCache {
        FilterCache {
            equipment_raw: filter_on_entries(entries, |e| {
                e.equipment == Equipment::Raw
            }),

            equipment_wraps: filter_on_entries(entries, |e| {
                e.equipment == Equipment::Wraps
            }),

            equipment_single: filter_on_entries(entries, |e| {
                e.equipment == Equipment::Single
            }),

            equipment_multi: filter_on_entries(entries, |e| {
                e.equipment == Equipment::Multi
            }),

            sex_male: filter_on_entries(entries, |e| {
                e.sex == Sex::M
            }),

            sex_female: filter_on_entries(entries, |e| {
                e.sex == Sex::F
            }),

            year_2018: filter_on_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2018
            }),

            year_2017: filter_on_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2017
            }),

            year_2016: filter_on_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2016
            }),

            year_2015: filter_on_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2015
            }),

            year_2014: filter_on_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2014
            }),
        }
    }

    pub fn from_enum(&self, c: CachedFilter) -> &Filter {
        match c {
            CachedFilter::EquipmentRaw => &self.equipment_raw,
            CachedFilter::EquipmentWraps => &self.equipment_wraps,
            CachedFilter::EquipmentSingle => &self.equipment_single,
            CachedFilter::EquipmentMulti => &self.equipment_multi,

            CachedFilter::SexMale => &self.sex_male,
            CachedFilter::SexFemale => &self.sex_female,

            CachedFilter::Year2018 => &self.year_2018,
            CachedFilter::Year2017 => &self.year_2017,
            CachedFilter::Year2016 => &self.year_2016,
            CachedFilter::Year2015 => &self.year_2015,
            CachedFilter::Year2014 => &self.year_2014,
        }
    }
}

fn filter_on_entries<F>(entries: &Vec<Entry>, select: F) -> Filter
    where F: Fn(&Entry) -> bool
{
    let mut vec = Vec::new();
    for i in 0 .. entries.len() {
        if select(&entries[i]) {
            vec.push(i as u32);
        }
    }
    vec.shrink_to_fit();
    Filter { list: vec }
}
