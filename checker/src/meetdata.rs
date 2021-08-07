//! Defines MeetData, the owner of all meet-related data produced by the
//! Checker.

use fxhash::{FxBuildHasher, FxHashMap};
use opltypes::Username;

use crate::checklib::{Entry, Meet};

/// All checker-generated data structures for a single meet.
pub struct SingleMeetData {
    pub meet: Meet,
    pub entries: Vec<Entry>,
}

/// Permanent owner of all data from all meets.
pub struct AllMeetData {
    /// An owned vector of all meet data.
    ///
    /// Once assigned, the Vec may not be resized. To enforce this invariant,
    /// getters are used that only provide a slice.
    meets: Vec<SingleMeetData>,
}

/// Directions to a specific `Entry` within the `AllMeetData`.
///
/// Once the `AllMeetData` is created by the checker, each `SingleMeetData`
/// and each contained `Entry` has been assigned a unique index. Taking
/// these indices together gives each `Entry` a unique `EntryIndex`.
///
/// These indices are used to create the equivalent of a singly-linked list
/// of `Entry` structs referring to the same lifter.
#[derive(Copy, Clone)]
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
pub type LifterMap = FxHashMap<Username, Vec<EntryIndex>>;

impl From<Vec<SingleMeetData>> for AllMeetData {
    fn from(v: Vec<SingleMeetData>) -> AllMeetData {
        AllMeetData { meets: v }
    }
}

impl AllMeetData {
    /// Borrows the meet data immutably.
    pub fn meets(&self) -> &[SingleMeetData] {
        self.meets.as_slice()
    }

    /// Borrows the meet data mutably. The underlying vector remains immutable.
    pub fn meets_mut(&mut self) -> &mut [SingleMeetData] {
        self.meets.as_mut_slice()
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

    /// Generates a LifterMap for grouping Entries for the same lifter.
    ///
    /// As a side-effect, also stores each Entry's EntryIndex within itself,
    /// so that the Entry can know its `SingleMeetData` context.
    pub fn create_liftermap(&mut self) -> LifterMap {
        // Initialize the LifterMap to be fairly large to avoid reallocation.
        let mut map = LifterMap::with_capacity_and_hasher(800_000, FxBuildHasher::default());

        for (meet_index, singlemeet) in self.meets.iter_mut().enumerate() {
            for (entry_index, entry) in singlemeet.entries.iter_mut().enumerate() {
                // Tell each Entry its EntryIndex in the AllMeetData.
                let index = EntryIndex::at(meet_index, entry_index);
                entry.index = Some(index);

                // Add the EntryIndex to the appropriate vector.
                if let Some(vec) = map.get_mut(&entry.username) {
                    vec.push(index);
                } else {
                    // TODO: Because the username Strings are cloned, this takes about
                    // 400ms. If we could use a Symbol-like interface, we would get O(1)
                    // comparison and would save quite a lot of memory.
                    let username = entry.username.clone();
                    map.insert(username, vec![index]);
                }
            }
        }

        // Sort EntryIndex vectors by EntryDate.
        // Consistency-enforcing code depends on this ordering.
        for (_key, indices) in map.iter_mut() {
            if indices.len() >= 2 {
                indices.sort_unstable_by_key(|ei| self.entry(*ei).entrydate);
            }
        }

        map
    }
}
