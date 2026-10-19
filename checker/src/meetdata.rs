//! Defines MeetData, the owner of all meet-related data produced by the
//! Checker.

use std::{
    collections::BTreeSet,
    ops::Bound::{Excluded, Included},
};

use ascii::{AsciiString, IntoAsciiString};
use opltypes::Username;
use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::checklib::{Entry, Meet};

/// All checker-generated data structures for a single meet.
pub struct SingleMeetData {
    pub meet: Meet,
    pub entries: Box<[Entry]>,
}

/// Permanent owner of all data from all meets.
pub struct AllMeetData {
    /// An owned array referencing all meet data.
    meets: Box<[SingleMeetData]>,
}

/// Directions to a specific `Entry` within the `AllMeetData`.
///
/// Once the `AllMeetData` is created by the checker, each `SingleMeetData`
/// and each contained `Entry` has been assigned a unique index. Taking
/// these indices together gives each `Entry` a unique `EntryIndex`.
///
/// These indices are used to create the equivalent of a singly-linked list
/// of `Entry` structs referring to the same lifter.
#[derive(Copy, Clone, Default)]
pub struct EntryIndex {
    /// Index of the parent `SingleMeetData` within the `AllMeetData.meets`
    /// vector.
    pub meetdata_index: u32,

    /// Index of the `Entry` within the `SingleMeetData.entries` vector.
    pub entry_index: u32,
}

impl EntryIndex {
    /// Constructor.
    pub fn at(meetdata_index: usize, entry_index: usize) -> EntryIndex {
        EntryIndex {
            meetdata_index: meetdata_index as u32,
            entry_index: entry_index as u32,
        }
    }
}

/// Map from Username to a vector of EntryIndex references, representing all
/// Entries corresponding to the same lifter.
///
/// Each vector of indices is ordered by date.
///
/// The original intention was to use a doubly-linked list with memory in each
/// Entry, but unfortunately Rust makes it basically impossible to mutably
/// iterate over the meet data while simultaneously mutating external meet data.
///
/// TODO: Creating all the vectors takes about 200ms.
/// TODO: It's possible that we could do better with unsafe blocks and a linked
/// list.
///
/// Note that because internal self-references are disallowed, this map
/// must maintain a copy of the String and EntryIndex.
pub struct EntryMap {
    map: FxHashMap<Username, Vec<EntryIndex>>,

    /// The set of all username variants with disambiguation numbers.
    ///
    /// A BTreeSet is used to be able to do range queries, for querying string prefixes.
    /// Although this contains Usernames, it's stored as AsciiString for range queries.
    ///
    /// It's preferable to split out the HashMap and BTreeMap for speed: using just a
    /// BTreeSet instead of two data structures causes building the data structure to increase
    /// 4x, from 0.8s to 3.2s, which is unacceptably slow.
    variants: BTreeSet<AsciiString>,
}

impl EntryMap {
    // HashMap-like read-only interface.
    pub fn get(&self, u: &Username) -> Option<&Vec<EntryIndex>> {
        self.map.get(u)
    }
    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, Username, Vec<EntryIndex>> {
        self.map.keys()
    }
    pub fn values(&self) -> std::collections::hash_map::Values<'_, Username, Vec<EntryIndex>> {
        self.map.values()
    }

    pub fn insert(&mut self, u: Username, entries: Vec<EntryIndex>) -> Option<Vec<EntryIndex>> {
        if u.has_variant() {
            self.variants.insert(u.as_ascii_str().to_owned());
        }
        self.map.insert(u, entries)
    }
    pub fn remove(&mut self, u: &Username) -> Option<Vec<EntryIndex>> {
        self.variants.remove(u.as_ascii_str());
        self.map.remove(u)
    }

    /// Looks up all lifters sharing the same disambiguation base.
    pub fn get_variants_of(&self, u: &Username) -> Vec<Username> {
        let low_inclusive = format!("{u}0").into_ascii_string().unwrap(); // >= all numbers.
        let high_exclusive = format!("{u}:").into_ascii_string().unwrap(); // <= all numbers (`:` is the next ASCII char after `9`).

        self.variants
            .range((Included(low_inclusive), Excluded(high_exclusive)))
            .map(|ascii| Username::from_trusted_str(ascii.as_str()).unwrap())
            .collect()
    }

    /// Inserts an entry, maintaining sorted order.
    ///
    /// This is used by the disambiguation code. It's inefficient to do it like this, but it's fine
    /// for an initial version.
    ///
    /// TODO: Many lookups and redundant sorting can be avoided.
    pub fn insert_entry_for(&mut self, u: Username, e: EntryIndex, meetdata: &AllMeetData) {
        if u.has_variant() && !self.variants.contains(u.as_ascii_str()) {
            self.variants.insert(u.as_ascii_str().to_owned());
        }

        let indices = self.map.entry(u).or_default();
        indices.push(e);
        indices.sort_unstable_by_key(|ei| meetdata.entry(*ei).entrydate);
    }
}

impl From<Vec<SingleMeetData>> for AllMeetData {
    fn from(v: Vec<SingleMeetData>) -> AllMeetData {
        AllMeetData {
            meets: v.into_boxed_slice(),
        }
    }
}

impl AllMeetData {
    /// Borrows the meet data immutably.
    pub fn meets(&self) -> &[SingleMeetData] {
        &self.meets
    }

    /// Borrows the meet data mutably.
    pub fn meets_mut(&mut self) -> &mut [SingleMeetData] {
        &mut self.meets
    }

    /// Borrows a `Meet` by `EntryIndex`.
    pub fn meet(&self, index: EntryIndex) -> &Meet {
        &self.meets[index.meetdata_index as usize].meet
    }

    /// Borrows an `Entry` by `EntryIndex`.
    ///
    /// Note that because the lifetime of each Entry is equal to the lifetime
    /// of the AllMeetData, it is only possible to refer to a single Entry
    /// at a time.
    pub fn entry(&self, index: EntryIndex) -> &Entry {
        &self.meets[index.meetdata_index as usize].entries[index.entry_index as usize]
    }

    /// Shorthand for use in tests: constructs an EntryIndex inline.
    #[cfg(test)]
    pub fn entry_at(&self, meetdata_index: usize, entry_index: usize) -> &Entry {
        &self.meets[meetdata_index].entries[entry_index]
    }

    /// Borrows an `Entry` mutably by `EntryIndex`.
    ///
    /// Note that because the lifetime of each Entry is equal to the lifetime
    /// of the AllMeetData, it is only possible to refer to a single Entry
    /// at a time.
    pub fn entry_mut(&mut self, index: EntryIndex) -> &mut Entry {
        &mut self.meets[index.meetdata_index as usize].entries[index.entry_index as usize]
    }

    /// Generates an EntryMap for grouping Entries for the same lifter.
    ///
    /// As a side-effect, also stores each Entry's EntryIndex within itself,
    /// so that the Entry can know its `SingleMeetData` context.
    pub fn create_entrymap(&mut self) -> EntryMap {
        // Initialize the LifterMap to be fairly large to avoid reallocation.
        let mut entrymap = EntryMap {
            map: FxHashMap::with_capacity_and_hasher(1_500_000, FxBuildHasher),
            variants: BTreeSet::new(),
        };

        for (meet_index, singlemeet) in self.meets.iter_mut().enumerate() {
            for (entry_index, entry) in singlemeet.entries.iter_mut().enumerate() {
                // Tell each Entry its EntryIndex in the AllMeetData.
                let index = EntryIndex::at(meet_index, entry_index);
                entry.index = index;

                // Add the EntryIndex to the appropriate vector.
                if let Some(vec) = entrymap.map.get_mut(&entry.username) {
                    vec.push(index);
                } else {
                    // TODO: Because the username Strings are cloned, this takes about
                    // 400ms. If we could use a Symbol-like interface, we would get O(1)
                    // comparison and would save quite a lot of memory.
                    if entry.username.has_variant() {
                        entrymap
                            .variants
                            .insert(entry.username.as_ascii_str().to_owned());
                    }
                    entrymap.map.insert(entry.username.clone(), vec![index]);
                }
            }
        }

        // Sort EntryIndex vectors by EntryDate.
        // Consistency-enforcing code depends on this ordering.
        for indices in entrymap.map.values_mut() {
            if indices.len() >= 2 {
                indices.sort_unstable_by_key(|ei| self.entry(*ei).entrydate);
            }
        }

        entrymap
    }
}
