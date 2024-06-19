//! The OpenPowerlifting in-memory database.
//!
//! Because our data is read-only at runtime, we can lay out data structures
//! better than a "real" database like SQLite3 or PostgreSQL. Additionally,
//! by storing all the data in formats native to Rust, we avoid copy overhead.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate strum_macros;

use itertools::Itertools;

use std::error::Error;
use std::mem;
use std::path::Path;

use crate::cache::*;

// Modules.
pub mod algorithms;
mod cache;
mod data;
mod metafederation;
pub mod query;
mod yesno;

// Re-exports.
pub use crate::data::{Entry, Lifter, Meet};
pub use crate::metafederation::*;

/// The collection of data stores that constitute the complete dataset.
///
/// The data structure is immutable. To prevent the owner from modifying
/// owned data, the struct contents are private and accessed through getters.
#[derive(Debug)]
pub struct OplDb {
    /// The LifterID is implicit in the backing vector, as the index.
    ///
    /// The order of the lifters is arbitrary.
    lifters: Vec<Lifter>,

    /// The MeetID is implicit in the backing vector, as the index.
    ///
    /// The order of the meets is arbitrary.
    meets: Vec<Meet>,

    /// The EntryID is implicit in the backing vector, as the index.
    ///
    /// The order of the entries is by increasing lifter_id.
    /// Within the entries of a single lifter_id, the order is arbitrary.
    entries: Vec<Entry>,

    /// Precalculated caches.
    cache: StaticCache,
    metafed_cache: MetaFederationCache,
}

/// Reads the `lifters.csv` file into a Vec<Lifter>.
fn import_lifters_csv(file: &Path) -> Result<Vec<Lifter>, Box<dyn Error>> {
    let mut vec = Vec::with_capacity(1_000_000);

    let mut rdr = csv::ReaderBuilder::new().quoting(false).from_path(file)?;
    for lifter in rdr.deserialize() {
        let lifter: Lifter = lifter?;
        vec.push(lifter);
    }

    vec.shrink_to_fit();
    Ok(vec)
}

/// Reads the `meet.csv` file into a Vec<Meet>.
fn import_meets_csv(file: &Path) -> Result<Vec<Meet>, Box<dyn Error>> {
    let mut vec = Vec::with_capacity(50_000);

    let mut rdr = csv::ReaderBuilder::new().quoting(false).from_path(file)?;
    for meet in rdr.deserialize() {
        let meet: Meet = meet?;
        vec.push(meet);
    }

    vec.shrink_to_fit();
    Ok(vec)
}

/// Reads the `entries.csv` file into a Vec<Entry>.
///
/// Also fills in metadata about each Meet.
fn import_entries_csv(
    file: &Path,
    meets: &mut [Meet],
) -> Result<(Vec<Entry>, MetaFederationCache), Box<dyn Error>> {
    let mut vec = Vec::with_capacity(3_000_000);

    let mut rdr = csv::ReaderBuilder::new().quoting(false).from_path(file)?;
    for entry in rdr.deserialize() {
        let entry: Entry = entry?;
        vec.push(entry);
    }

    // Initially, the entries CSV is sorted by meet_id.
    // This ordering can be used to efficiently calculate meet metadata.
    let metafed_cache = MetaFederationCache::make(meets, &vec);

    // Calculate num_unique_lifters.
    for (meet_id, meet) in meets.iter_mut().enumerate() {
        meet.num_unique_lifters = precompute_num_unique_lifters(&vec, meet_id as u32);
    }

    // Sort the entries database by lifter_id.
    // This invariant allows for extremely efficient lifter-uniqueness
    // filtering without constructing additional data structures.
    vec.sort_unstable_by_key(|e| e.lifter_id);

    vec.shrink_to_fit();
    Ok((vec, metafed_cache))
}

/// Counts how many unique LifterIDs competed in a given meet.
///
/// Assumes that the entries vector is sorted by meet_id --
/// so this is only callable from within `import_entries_csv()`.
fn precompute_num_unique_lifters(entries: &[Entry], meet_id: u32) -> u16 {
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
    for (i, entry) in entries.iter().enumerate().skip(found_index) {
        if entry.meet_id == meet_id {
            last_index = i;
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
    lifter_ids.into_iter().group_by(|x| *x).into_iter().count() as u16
}

impl OplDb {
    /// Constructs the `OplDb` from CSV files produced by the project
    /// build script.
    pub fn from_csv(
        lifters_csv: &Path,
        meets_csv: &Path,
        entries_csv: &Path,
    ) -> Result<OplDb, Box<dyn Error>> {
        let lifters = import_lifters_csv(lifters_csv)?;
        let mut meets = import_meets_csv(meets_csv)?;
        let (entries, metafed_cache) = import_entries_csv(entries_csv, &mut meets)?;

        let cache = StaticCache::new(&lifters, &meets, &entries);

        Ok(OplDb {
            lifters,
            meets,
            entries,
            cache,
            metafed_cache,
        })
    }

    /// Returns the size of owned data structures.
    pub fn size_bytes(&self) -> usize {
        // Size of owned vectors.
        let lifters_size = mem::size_of::<Lifter>() * self.lifters.len();
        let meets_size = mem::size_of::<Meet>() * self.meets.len();
        let entries_size = mem::size_of::<Entry>() * self.entries.len();
        let owned_vectors = lifters_size + meets_size + entries_size;

        // Size of owned Strings in those objects.
        let mut owned_strings: usize = 0;
        for lifter in &self.lifters {
            owned_strings += mem::size_of::<String>() + lifter.name.len();
            owned_strings += mem::size_of::<String>() + lifter.username.len();
            if let Some(ref instagram) = lifter.instagram {
                owned_strings += mem::size_of::<String>() + instagram.len();
            }
        }
        for meet in &self.meets {
            owned_strings += mem::size_of::<String>() + meet.path.len();
            if let Some(ref state) = meet.state {
                owned_strings += mem::size_of::<String>() + state.len();
            }
            if let Some(ref town) = meet.town {
                owned_strings += mem::size_of::<String>() + town.len();
            }
            owned_strings += mem::size_of::<String>() + meet.name.len();
        }
        // TODO(sstangl): Don't know how to account for the global symbol table here.

        mem::size_of::<OplDb>() + owned_vectors + owned_strings
    }

    /// Borrows the lifters vector.
    #[inline]
    pub fn lifters(&self) -> &[Lifter] {
        self.lifters.as_slice()
    }

    /// Borrows the meets vector.
    #[inline]
    pub fn meets(&self) -> &[Meet] {
        self.meets.as_slice()
    }

    /// Borrows the entries vector.
    #[inline]
    pub fn entries(&self) -> &[Entry] {
        self.entries.as_slice()
    }

    /// Borrows a `Lifter` by index.
    #[inline]
    pub fn lifter(&self, n: u32) -> &Lifter {
        &self.lifters[n as usize]
    }

    /// Borrows a `Meet` by index.
    #[inline]
    pub fn meet(&self, n: u32) -> &Meet {
        &self.meets[n as usize]
    }

    /// Borrows an `Entry` by index.
    #[inline]
    pub fn entry(&self, n: u32) -> &Entry {
        &self.entries[n as usize]
    }

    /// Borrows the static cache. It's static!
    #[inline]
    pub(crate) fn cache(&self) -> &StaticCache {
        &self.cache
    }

    /// Borrows the MetaFederationCache.
    #[inline]
    pub fn metafed_cache(&self) -> &MetaFederationCache {
        &self.metafed_cache
    }

    /// Look up the lifter_id by username.
    pub fn lifter_id(&self, username: &str) -> Option<u32> {
        self.cache.username_map.get(username).cloned()
    }

    /// Get a list of all lifters that have the same username base,
    /// which doesn't include numbers for disambiguation.
    ///
    /// For example, "johndoe" matches "johndoe" and "johndoe1",
    /// but does not match "johndoenut".
    pub fn lifters_under_username_base(&self, base: &str) -> Vec<u32> {
        // Disambiguations end with a digit.
        // Some lifters may have failed to be merged with their disambiguated username.
        // Therefore, for usernames without a digit, it cannot be assumed that they are
        // *not* a disambiguation.
        let is_already_disambiguated: bool =
            base.chars().last().map_or(false, |c| c.is_ascii_digit());
        if is_already_disambiguated {
            if let Some(id) = self.lifter_id(base) {
                return vec![id]; // The input base was an exact lifter.
            }
            return vec![]; // The input base was exact, but with no matches.
        }

        let mut acc = vec![];

        // Look up the name directly.
        if let Some(id) = self.lifter_id(base) {
            acc.push(id);
        }

        // Look up each possible disambiguation value, stopping when one is missing.
        for i in 1.. {
            let disambig = format!("{base}{i}");
            if let Some(id) = self.lifter_id(&disambig) {
                acc.push(id);
            } else {
                break;
            }
        }
        acc
    }

    /// Looks up the meet_id by MeetPath.
    pub fn meet_id(&self, meetpath: &str) -> Option<u32> {
        for i in 0..self.meets.len() {
            if self.meets[i].path == meetpath {
                return Some(i as u32);
            }
        }
        None
    }

    /// Returns all entry_ids with the given lifter_id.
    ///
    /// The vector of entries is sorted by lifter_id. This function uses binary
    /// search followed by a bi-directional linear scan.
    ///
    /// Panics if the lifter_id is not found.
    pub fn entry_ids_for_lifter(&self, lifter_id: u32) -> Vec<u32> {
        // Perform a binary search on lifter_id.
        let found_index = self
            .entries()
            .binary_search_by_key(&lifter_id, |e| e.lifter_id)
            .unwrap();

        // All entries for a lifter are contiguous, so scan backwards to find the first.
        let mut first_index = found_index;
        for index in (0..found_index).rev() {
            if self.entry(index as u32).lifter_id == lifter_id {
                first_index = index;
            } else {
                break;
            }
        }

        // Scan forwards to find the last.
        let mut last_index = found_index;
        for index in (found_index + 1)..self.entries().len() {
            if self.entry(index as u32).lifter_id == lifter_id {
                last_index = index;
            } else {
                break;
            }
        }
        assert!(first_index <= last_index);

        // Collect entries between first_index and last_index, inclusive.
        (first_index..=last_index).map(|i| i as u32).collect()
    }

    /// Returns all entries with the given lifter_id.
    ///
    /// The vector of entries is sorted by lifter_id. This function uses binary
    /// search followed by a bi-directional linear scan.
    ///
    /// Panics if the lifter_id is not found.
    pub fn entries_for_lifter(&self, lifter_id: u32) -> Vec<&Entry> {
        self.entry_ids_for_lifter(lifter_id)
            .into_iter()
            .map(|i| self.entry(i))
            .collect()
    }

    /// Returns all entries with the given meet_id.
    ///
    /// Those entries could be located anywhere in the entries vector,
    /// so they are found using a linear scan.
    pub fn entries_for_meet(&self, meet_id: u32) -> Vec<&Entry> {
        self.entries()
            .iter()
            .filter(|&e| e.meet_id == meet_id)
            .collect()
    }

    /// Returns all entry_ids with the given meet_id.
    ///
    /// Those entries could be located anywhere in the entries vector,
    /// so they are found using a linear scan.
    pub fn entry_ids_for_meet(&self, meet_id: u32) -> Vec<u32> {
        self.entries()
            .iter()
            .enumerate()
            .filter(|&(_i, e)| e.meet_id == meet_id)
            .map(|(i, _e)| i as u32)
            .collect()
    }

    /// Returns all lifter IDs that competed at the given meet_id.
    pub fn lifter_ids_for_meet(&self, meet_id: u32) -> Vec<u32> {
        self.entries()
            .iter()
            .filter(|&e| e.meet_id == meet_id)
            .group_by(|e| e.lifter_id)
            .into_iter()
            .map(|(lifter_id, _group)| lifter_id)
            .collect()
    }

    /// Returns the StaticCache.
    ///
    /// This endpoint is intended only for use in benchmark code, for testing against real data.
    pub fn cache_for_benchmarks(&self) -> &cache::StaticCache {
        &self.cache
    }
}
