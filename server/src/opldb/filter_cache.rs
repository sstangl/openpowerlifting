//! This file defines the filter cache.

use opldb::filter::Filter;
use opldb::fields::*;
use opldb::{Entry,Meet,Lifter};

/// List of precomputed filters.
pub struct FilterCache {
    // Equipment filters.
    pub equipment_raw: Filter,
    pub equipment_wraps: Filter,
    pub equipment_single: Filter,
    pub equipment_multi: Filter,

    // Sex filter.
    pub sex_male: Filter,
    pub sex_female: Filter,

    // Year filter.
    pub year_2018: Filter,
    pub year_2017: Filter,
    pub year_2016: Filter,
    pub year_2015: Filter,
    pub year_2014: Filter,
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
