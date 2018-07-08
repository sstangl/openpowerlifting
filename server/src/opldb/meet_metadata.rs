//! Defines a metadata cache for individual meets.

use itertools::Itertools;
use strum::IntoEnumIterator;

use opldb::{Entry, Meet, MetaFederation};

/// Counts how many unique LifterIDs competed in a given meet.
///
/// Assumes that the entries vector is sorted by meet_id --
/// so this is only callable from within `import_entries_csv()`.
pub fn precompute_num_unique_lifters(entries: &Vec<Entry>, meet_id: u32) -> u32 {
    let found_index = entries
        .binary_search_by_key(&meet_id, |e| e.meet_id)
        .unwrap();

    // All entries for a meet are contiguous, so scan linearly to find the first.
    let mut first_index = found_index;
    for index in (0..found_index).rev() {
        if entries[index].meet_id == meet_id {
            first_index = index;
        } else {
            break;
        }
    }

    // Scan to find the last.
    let mut last_index = found_index;
    for index in found_index..entries.len() {
        if entries[index].meet_id == meet_id {
            last_index = index;
        } else {
            break;
        }
    }
    assert!(first_index <= last_index);

    // Gather all the lifter_ids.
    let mut lifter_ids: Vec<u32> = (first_index..last_index + 1)
        .map(|i| entries[i].lifter_id)
        .collect();

    lifter_ids.sort_unstable();
    *(&lifter_ids.into_iter().group_by(|x| *x).into_iter().count()) as u32
}

/// Pre-computed list of meets in a MetaFederation.
///
/// A meet is part of the MetaFederation if it contains
/// at least one entry such that `MetaFederation::contains(entry)`.
pub struct MetaFederationCache {
    /// Uses (MetaFederation as usize) as index to a list of meet_ids
    /// for that MetaFederation.
    cache: Vec<Vec<u32>>,
}

impl MetaFederationCache {
    pub fn get_meet_ids_for<'a>(&'a self, meta: MetaFederation) -> &'a Vec<u32> {
        &self.cache[meta as usize]
    }

    /// Fill in the MetaFederationCache during CSV importation.
    ///
    /// The `entries` vector should be sorted by `entry.meet_id`,
    /// not by `entry.lifter_id` as it is post-importation.
    pub fn make(meets: &Vec<Meet>, entries: &Vec<Entry>) -> MetaFederationCache {
        let num_metafeds: usize = MetaFederation::iter().count();

        // Vector of list of meets for each MetaFederation.
        let mut ret: Vec<Vec<u32>> = Vec::with_capacity(num_metafeds);
        for _ in 0..num_metafeds {
            ret.push(vec![]);
        }

        // Vector of whether each meet has a match for the
        // given MetaFederation (accessed via index).
        let mut contains: Vec<bool> = Vec::with_capacity(num_metafeds);
        for _ in 0..num_metafeds {
            contains.push(false);
        }

        let mut last_meet_id = 0;

        // Iterate by grouping entries from the same Meet.
        for (meet_id, meet_entries) in entries.iter().group_by(|e| e.meet_id).into_iter()
        {
            // Sanity checking that the entries argument is sorted by meet_id.
            assert!(last_meet_id <= meet_id);
            last_meet_id = meet_id;

            // Check whether any entries are part of each MetaFederation.
            for entry in meet_entries {
                for meta in MetaFederation::iter() {
                    if meta.contains(&entry, &meets) {
                        contains[meta as usize] = true;
                    }
                }
            }

            // If any match, add to that MetaFederation's meet list.
            for i in 0..num_metafeds {
                if contains[i] {
                    ret[i].push(meet_id);
                }
                // Reset the vector for the next iteration.
                contains[i] = false;
            }
        }

        // Since we're here already, sort the return vector by Date
        // in reverse order -- that's usually how it's consumed.
        for v in &mut ret {
            v.sort_unstable_by(|&a, &b| {
                meets[a as usize]
                    .date
                    .cmp(&meets[b as usize].date)
                    .reverse()
            });
        }

        MetaFederationCache { cache: ret }
    }
}
