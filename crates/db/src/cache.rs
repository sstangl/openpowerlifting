//! Precalculated cache of data, such as sorts.

use fxhash::{FxBuildHasher, FxHashMap};
use itertools::Itertools;
use opltypes::*;
use smartstring::alias::CompactString;

use std::cmp::Ordering;

use crate::algorithms::*;
use crate::{Entry, Lifter, Meet};

/// List of indices into the opldb.entries vector,
/// in no particular order, but such that entries from the same
/// lifter are next to each other (sorted by LifterID).
///
/// This is useful to get `O(n log n)` lookup, which allows for
/// performing a uniqueness operation without constructing
/// a HashMap.
///
/// Because it's non-sorted, that also means that there doesn't
/// need to be a version of the data stored for each way in
/// which the data can be sorted, so there's memory savings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonSortedNonUnique(pub Vec<u32>);

/// List of indices into the opldb.entries vector,
/// in some sorted order, with each lifter occurring at
/// most once.
///
/// This is useful to get `O(1)` lookup, since it stores
/// the filter/sort/unique algorithm in its final output.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortedUnique(pub Vec<u32>);

impl NonSortedNonUnique {
    /// Unions the indices from both source inputs.
    pub fn union(&self, other: &NonSortedNonUnique) -> NonSortedNonUnique {
        debug_assert!(self.maintains_invariants());
        debug_assert!(other.maintains_invariants());

        // March and add the least element to the list.
        let mut acc = Vec::<u32>::with_capacity(self.0.len().max(other.0.len()));

        let mut self_index = 0;
        let mut other_index = 0;

        while self_index < self.0.len() && other_index < other.0.len() {
            let a = self.0[self_index];
            let b = other.0[other_index];

            match a.cmp(&b) {
                Ordering::Equal => {
                    acc.push(a);
                    self_index += 1;
                    other_index += 1;
                }
                Ordering::Less => {
                    acc.push(a);
                    self_index += 1;
                }
                Ordering::Greater => {
                    acc.push(b);
                    other_index += 1;
                }
            }
        }

        // One of the lists is depleted.
        // Accumulate what remains of the other list.
        // FIXME: Does this re-iterate over the iterator again?
        for &n in self.0.iter().skip(self_index) {
            acc.push(n);
        }
        for &n in other.0.iter().skip(other_index) {
            acc.push(n);
        }

        NonSortedNonUnique(acc)
    }

    /// Intersects the indices from both source inputs.
    pub fn intersect(&self, other: &NonSortedNonUnique) -> NonSortedNonUnique {
        debug_assert!(self.maintains_invariants());
        debug_assert!(other.maintains_invariants());

        // March and matching elements to the list.
        let mut acc = Vec::<u32>::new();

        if self.0.is_empty() || other.0.is_empty() {
            return NonSortedNonUnique(acc);
        }

        let mut self_iter = self.0.iter();
        let mut other_iter = other.0.iter();

        let mut a: u32 = *self_iter.next().unwrap();
        let mut b: u32 = *other_iter.next().unwrap();

        loop {
            match a.cmp(&b) {
                Ordering::Equal => {
                    acc.push(a);
                    a = match self_iter.next() {
                        Some(a) => *a,
                        None => {
                            return NonSortedNonUnique(acc);
                        }
                    };
                    b = match other_iter.next() {
                        Some(b) => *b,
                        None => {
                            return NonSortedNonUnique(acc);
                        }
                    };
                }
                Ordering::Less => {
                    a = match self_iter.next() {
                        Some(a) => *a,
                        None => {
                            return NonSortedNonUnique(acc);
                        }
                    };
                }
                Ordering::Greater => {
                    b = match other_iter.next() {
                        Some(b) => *b,
                        None => {
                            return NonSortedNonUnique(acc);
                        }
                    };
                }
            }
        }
    }

    /// Sorts and uniques the data with reference to a comparator.
    pub fn sort_and_unique_by<F, G>(
        &self,
        entries: &[Entry],
        meets: &[Meet],
        compare: F,
        belongs: G,
    ) -> SortedUnique
    where
        F: Fn(&[Meet], &Entry, &Entry) -> Ordering,
        G: Fn(&Entry) -> bool,
    {
        // First, group contiguous entries by lifter_id, so only the best
        // entry for each lifter is counted.
        // Entries are filtered at this step, so that the grouping operation
        // below does not include (and possibly output) an entry that does
        // not belong.
        // The group_by() operation is lazy and does not perform any action yet.
        let groups = self
            .0
            .iter()
            .filter(|x| belongs(&entries[**x as usize]))
            .group_by(|idx| entries[**idx as usize].lifter_id);

        // Perform the grouping operation, generating a new vector.
        let mut vec: Vec<u32> = groups
            .into_iter()
            // `min_by()` takes the best entry due to comparator ordering.
            .map(|(_key, group)| {
                *group
                    .min_by(|&a, &b| compare(meets, &entries[*a as usize], &entries[*b as usize]))
                    .unwrap()
            })
            .collect();

        vec.sort_by(|&a, &b| compare(meets, &entries[a as usize], &entries[b as usize]));
        vec.shrink_to_fit();
        SortedUnique(vec)
    }

    /// Tests that the list is monotonically increasing.
    pub fn maintains_invariants(&self) -> bool {
        if self.0.is_empty() {
            return true;
        }

        let mut prev = self.0[0];
        for &i in self.0.iter().skip(1) {
            if prev >= i {
                return false;
            }
            prev = i;
        }
        true
    }
}

/// Owning structure of all precomputed data.
#[derive(Debug, Serialize, Deserialize)]
pub struct StaticCache {
    // Precalculated data for Rankings.
    pub constant_time: ConstantTimeCache,
    pub log_linear_time: LogLinearTimeCache,

    /// Precalculated map of Lifter Username to Lifter ID.
    pub username_map: FxHashMap<CompactString, u32>,

    /// Precalculated table of EntryIDs sorted by their underlying MeetID.
    ///
    /// This facilitates a quick binary search lookup for all entries under a given MeetID.
    pub entry_ids_sorted_by_meet_id: Vec<u32>,
}

impl StaticCache {
    pub fn new(lifters: &[Lifter], meets: &[Meet], entries: &[Entry]) -> StaticCache {
        // Calculate the map from Username to ID.
        let mut username_map = FxHashMap::with_hasher(FxBuildHasher::default());
        for (i, lifter) in lifters.iter().enumerate() {
            let cloned = CompactString::from(lifter.username.as_str());
            username_map.insert(cloned, i as u32);
        }
        username_map.shrink_to_fit();

        // Calculate Entry IDs sorted by Meet ID, for quick entries-in-meet lookup.
        let mut entry_ids_sorted_by_meet_id: Vec<u32> = Vec::with_capacity(entries.len());
        for i in 0..entries.len() {
            entry_ids_sorted_by_meet_id.push(i as u32);
        }
        entry_ids_sorted_by_meet_id
            .sort_unstable_by_key(|entry_id| entries[*entry_id as usize].meet_id);
        entry_ids_sorted_by_meet_id.shrink_to_fit();

        // Calculate the rankings and records caches.
        let loglin = LogLinearTimeCache::new(meets, entries);

        StaticCache {
            constant_time: ConstantTimeCache::new(&loglin, meets, entries),
            log_linear_time: loglin,
            username_map,
            entry_ids_sorted_by_meet_id,
        }
    }
}

/// Stores all sorts for a given equipment type.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantTimeBy {
    pub raw: SortedUnique,
    pub wraps: SortedUnique,
    pub raw_wraps: SortedUnique,
    pub single: SortedUnique,
    pub multi: SortedUnique,
    pub unlimited: SortedUnique,
}

impl ConstantTimeBy {
    pub fn new<F, G>(
        loglin: &LogLinearTimeCache,
        mv: &[Meet],
        ev: &[Entry],
        compare: &F,
        belongs: &G,
    ) -> ConstantTimeBy
    where
        F: Fn(&[Meet], &Entry, &Entry) -> Ordering,
        G: Fn(&Entry) -> bool,
    {
        ConstantTimeBy {
            raw: loglin.raw.sort_and_unique_by(ev, mv, compare, belongs),
            wraps: loglin.wraps.sort_and_unique_by(ev, mv, compare, belongs),
            raw_wraps: loglin
                .raw_wraps
                .sort_and_unique_by(ev, mv, compare, belongs),
            single: loglin.single.sort_and_unique_by(ev, mv, compare, belongs),
            multi: loglin.multi.sort_and_unique_by(ev, mv, compare, belongs),
            unlimited: loglin
                .unlimited
                .sort_and_unique_by(ev, mv, compare, belongs),
        }
    }
}

/// Owning structure of all `O(1)` lookup data.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantTimeCache {
    // Weight comparisons.
    pub squat: ConstantTimeBy,
    pub bench: ConstantTimeBy,
    pub deadlift: ConstantTimeBy,
    pub total: ConstantTimeBy,

    // Points comparisons.
    pub wilks: ConstantTimeBy,
    pub mcculloch: ConstantTimeBy,
    pub glossbrenner: ConstantTimeBy,
    pub goodlift: ConstantTimeBy,
    pub dots: ConstantTimeBy,
}

impl ConstantTimeCache {
    pub fn new(loglin: &LogLinearTimeCache, mv: &[Meet], ev: &[Entry]) -> ConstantTimeCache {
        ConstantTimeCache {
            squat: ConstantTimeBy::new(loglin, mv, ev, &cmp_squat, &filter_squat),
            bench: ConstantTimeBy::new(loglin, mv, ev, &cmp_bench, &filter_bench),
            deadlift: ConstantTimeBy::new(loglin, mv, ev, &cmp_deadlift, &filter_deadlift),
            total: ConstantTimeBy::new(loglin, mv, ev, &cmp_total, &filter_total),
            wilks: ConstantTimeBy::new(loglin, mv, ev, &cmp_wilks, &filter_wilks),
            mcculloch: ConstantTimeBy::new(loglin, mv, ev, &cmp_mcculloch, &filter_mcculloch),
            glossbrenner: ConstantTimeBy::new(
                loglin,
                mv,
                ev,
                &cmp_glossbrenner,
                &filter_glossbrenner,
            ),
            goodlift: ConstantTimeBy::new(loglin, mv, ev, &cmp_goodlift, &filter_goodlift),
            dots: ConstantTimeBy::new(loglin, mv, ev, &cmp_dots, &filter_dots),
        }
    }
}

/// Owning structure of all `O(n log n)` lookup data.
#[derive(Debug, Serialize, Deserialize)]
pub struct LogLinearTimeCache {
    /// List of all non-DQ Raw entry indices by LifterID.
    pub raw: NonSortedNonUnique,
    /// List of all non-DQ Wraps entry incides by LifterID.
    pub wraps: NonSortedNonUnique,
    /// List of all non-DQ Raw+Wraps entry indices by LifterID.
    pub raw_wraps: NonSortedNonUnique,
    /// List of all non-DQ Single-ply entry indices by LifterID.
    pub single: NonSortedNonUnique,
    /// List of all non-DQ Multi-ply entry indices by LifterID.
    pub multi: NonSortedNonUnique,
    /// List of all non-DQ Multi-ply and Unlimited entry indices by LifterID.
    pub unlimited: NonSortedNonUnique,

    /// List of all non-DQ Male entry indices by LifterID.
    pub male: NonSortedNonUnique,
    /// List of all non-DQ Female entry indices by LifterID.
    pub female: NonSortedNonUnique,

    pub year2025: NonSortedNonUnique,
    pub year2024: NonSortedNonUnique,
    pub year2023: NonSortedNonUnique,
    pub year2022: NonSortedNonUnique,
    pub year2021: NonSortedNonUnique,
    pub year2020: NonSortedNonUnique,
    pub year2019: NonSortedNonUnique,
    pub year2018: NonSortedNonUnique,
    pub year2017: NonSortedNonUnique,
    pub year2016: NonSortedNonUnique,
    pub year2015: NonSortedNonUnique,
}

impl LogLinearTimeCache {
    fn filter_entries<F>(entries: &[Entry], meets: &[Meet], select: F) -> NonSortedNonUnique
    where
        F: Fn(&Entry) -> bool,
    {
        let mut vec = Vec::new();
        for (i, entry) in entries.iter().enumerate() {
            // Filter out unsanctioned meets. We pretend they don't exist for rankings or records.
            if select(entry) && meets[entry.meet_id as usize].sanctioned {
                vec.push(i as u32);
            }
        }
        vec.shrink_to_fit();
        NonSortedNonUnique(vec)
    }

    pub fn new(meets: &[Meet], entries: &[Entry]) -> LogLinearTimeCache {
        LogLinearTimeCache {
            raw: Self::filter_entries(entries, meets, |e| e.equipment == Equipment::Raw),
            wraps: Self::filter_entries(entries, meets, |e| e.equipment == Equipment::Wraps),
            raw_wraps: Self::filter_entries(entries, meets, |e| {
                e.equipment == Equipment::Raw || e.equipment == Equipment::Wraps
            }),
            single: Self::filter_entries(entries, meets, |e| e.equipment == Equipment::Single),
            multi: Self::filter_entries(entries, meets, |e| e.equipment == Equipment::Multi),
            unlimited: Self::filter_entries(entries, meets, |e| {
                matches!(
                    e.equipment,
                    Equipment::Single | Equipment::Multi | Equipment::Unlimited
                )
            }),

            male: Self::filter_entries(entries, meets, |e| e.sex == Sex::M),
            female: Self::filter_entries(entries, meets, |e| e.sex == Sex::F),
            
            year2025: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2025
            }),
            year2024: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2024
            }),
            year2023: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2023
            }),
            year2022: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2022
            }),
            year2021: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2021
            }),
            year2020: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2020
            }),
            year2019: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2019
            }),
            year2018: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2018
            }),
            year2017: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2017
            }),
            year2016: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2016
            }),
            year2015: Self::filter_entries(entries, meets, |e| {
                meets[e.meet_id as usize].date.year() == 2015
            }),
        }
    }

    /// Looks up a year cache by integer.
    pub fn year_cache(&self, year: u32) -> Option<&NonSortedNonUnique> {
        match year {
            2025 => Some(&self.year2025),
            2024 => Some(&self.year2024),
            2023 => Some(&self.year2023),
            2022 => Some(&self.year2022),
            2021 => Some(&self.year2021),
            2020 => Some(&self.year2020),
            2019 => Some(&self.year2019),
            2018 => Some(&self.year2018),
            2017 => Some(&self.year2017),
            2016 => Some(&self.year2016),
            2015 => Some(&self.year2015),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_nonmonotonic() {
        let f = NonSortedNonUnique(vec![1, 2, 3, 5, 4]);
        assert!(!f.maintains_invariants());
        let f = NonSortedNonUnique(vec![1, 2, 3, 4, 5]);
        assert!(f.maintains_invariants());
    }

    #[test]
    fn test_union_basic() {
        let f1 = NonSortedNonUnique(vec![1, 2, 3]);
        assert_eq!(f1.union(&f1), f1);

        let f1 = NonSortedNonUnique(vec![0, 2, 6]);
        let f2 = NonSortedNonUnique(vec![1, 2, 7]);
        let expected = NonSortedNonUnique(vec![0, 1, 2, 6, 7]);
        assert_eq!(f1.union(&f2), expected);
        assert_eq!(f2.union(&f1), expected);
    }

    #[test]
    fn test_union_empty() {
        let empty = NonSortedNonUnique(vec![]);
        assert_eq!(empty.union(&empty), empty);

        let f2 = NonSortedNonUnique(vec![1, 2, 3]);
        assert_eq!(empty.union(&f2), f2);
        assert_eq!(f2.union(&empty), f2);
    }

    #[test]
    fn test_intersect_basic() {
        let f1 = NonSortedNonUnique(vec![1, 2, 3]);
        assert_eq!(f1.intersect(&f1), f1);

        let f1 = NonSortedNonUnique(vec![0, 2, 4, 6, 8]);
        let f2 = NonSortedNonUnique(vec![0, 3, 4, 8, 10, 12]);
        let expected = NonSortedNonUnique(vec![0, 4, 8]);
        assert_eq!(f1.intersect(&f2), expected);
        assert_eq!(f2.intersect(&f1), expected);
    }

    #[test]
    fn test_intersect_empty() {
        let empty = NonSortedNonUnique(vec![]);
        assert_eq!(empty.intersect(&empty), empty);

        let f2 = NonSortedNonUnique(vec![1, 2, 3]);
        assert_eq!(empty.intersect(&f2), empty);
        assert_eq!(f2.intersect(&empty), empty);
    }
}
