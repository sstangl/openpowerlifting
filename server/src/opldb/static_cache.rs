//! Precalculated cache of data, such as sorts.

use itertools::Itertools;
use opltypes::*;

use std::cmp::Ordering;
use std::ops::Deref;

use opldb::algorithms::*;
use opldb::{Entry, Meet};

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
#[derive(Clone, Debug, PartialEq)]
pub struct NonSortedNonUnique(pub Vec<u32>);

/// List of indices into the opldb.entries vector,
/// in some sorted order, but with each lifter potentially
/// occurring multiple times.
///
/// This is useful to get `O(n)` lookup, since it stores
/// the filter/sort algorithm in an intermediate output,
/// where further filtering and uniqueness can be applied.
pub struct SortedNonUnique(pub Vec<u32>);

/// List of indices into the opldb.entries vector,
/// in some sorted order, with each lifter occurring at
/// most once.
///
/// This is useful to get `O(1)` lookup, since it stores
/// the filter/sort/unique algorithm in its final output.
pub struct SortedUnique(pub Vec<u32>);

// TODO: Can we templatize these PossiblyOwned types?
/// Allows remembering whether or not a returned SortedUnique is to be
/// deallocated.
pub enum PossiblyOwnedNonSortedNonUnique<'db> {
    Borrowed(&'db NonSortedNonUnique),
    Owned(NonSortedNonUnique),
}

impl<'db> Deref for PossiblyOwnedNonSortedNonUnique<'db> {
    type Target = NonSortedNonUnique;

    fn deref(&self) -> &NonSortedNonUnique {
        match &self {
            PossiblyOwnedNonSortedNonUnique::Borrowed(x) => x,
            PossiblyOwnedNonSortedNonUnique::Owned(x) => &x,
        }
    }
}

/// Allows remembering whether or not a returned SortedUnique is to be
/// deallocated.
pub enum PossiblyOwnedSortedUnique<'db> {
    Borrowed(&'db SortedUnique),
    Owned(SortedUnique),
}

impl<'db> Deref for PossiblyOwnedSortedUnique<'db> {
    type Target = SortedUnique;

    fn deref(&self) -> &SortedUnique {
        match &self {
            PossiblyOwnedSortedUnique::Borrowed(x) => x,
            PossiblyOwnedSortedUnique::Owned(x) => &x,
        }
    }
}

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

            if a == b {
                acc.push(a);
                self_index += 1;
                other_index += 1;
            } else if a < b {
                acc.push(a);
                self_index += 1;
            } else {
                acc.push(b);
                other_index += 1;
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

        let mut self_index = 0;
        let mut other_index = 0;

        let mut a = self.0[self_index];
        let mut b = other.0[other_index];

        loop {
            if a == b {
                acc.push(a);
                self_index += 1;
                other_index += 1;
                if self_index == self.0.len() || other_index == other.0.len() {
                    break;
                }
                a = self.0[self_index];
                b = other.0[other_index];
            } else if a < b {
                self_index += 1;
                if self_index == self.0.len() {
                    break;
                }
                a = self.0[self_index];
            } else {
                other_index += 1;
                if other_index == other.0.len() {
                    break;
                }
                b = other.0[other_index];
            }
        }

        NonSortedNonUnique(acc)
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
            .map(|(_key, group)| *group.min_by(|&a, &b| compare(meets, &entries[*a as usize], &entries[*b as usize])).unwrap())
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
pub struct StaticCache {
    pub constant_time: ConstantTimeCache,
    pub linear_time: LinearTimeCache,
    pub log_linear_time: LogLinearTimeCache,
}

impl StaticCache {
    pub fn new(meets: &[Meet], entries: &[Entry]) -> StaticCache {
        let loglin = LogLinearTimeCache::new(meets, entries);

        StaticCache {
            constant_time: ConstantTimeCache::new(&loglin, meets, entries),
            linear_time: LinearTimeCache::new(),
            log_linear_time: loglin,
        }
    }
}

/// Stores all sorts for a given equipment type.
pub struct ConstantTimeBy {
    pub raw: SortedUnique,
    pub wraps: SortedUnique,
    pub raw_wraps: SortedUnique,
    pub single: SortedUnique,
    pub multi: SortedUnique,
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
        }
    }
}

/// Owning structure of all `O(1)` lookup data.
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
}

impl ConstantTimeCache {
    pub fn new(
        loglin: &LogLinearTimeCache,
        mv: &[Meet],
        ev: &[Entry],
    ) -> ConstantTimeCache {
        ConstantTimeCache {
            squat: ConstantTimeBy::new(loglin, mv, ev, &cmp_squat, &filter_squat),
            bench: ConstantTimeBy::new(loglin, mv, ev, &cmp_bench, &filter_bench),
            deadlift: ConstantTimeBy::new(
                loglin,
                mv,
                ev,
                &cmp_deadlift,
                &filter_deadlift,
            ),
            total: ConstantTimeBy::new(loglin, mv, ev, &cmp_total, &filter_total),
            wilks: ConstantTimeBy::new(loglin, mv, ev, &cmp_wilks, &filter_wilks),
            mcculloch: ConstantTimeBy::new(
                loglin,
                mv,
                ev,
                &cmp_mcculloch,
                &filter_mcculloch,
            ),
            glossbrenner: ConstantTimeBy::new(
                loglin, mv, ev, &cmp_glossbrenner, &filter_glossbrenner
            ),
        }
    }
}

/// Owning structure of all `O(n)` lookup data.
pub struct LinearTimeCache {}

impl LinearTimeCache {
    pub fn new() -> LinearTimeCache {
        LinearTimeCache {}
    }
}

/// Owning structure of all `O(n log n)` lookup data.
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

    /// List of all non-DQ Male entry indices by LifterID.
    pub male: NonSortedNonUnique,
    /// List of all non-DQ Female entry indices by LifterID.
    pub female: NonSortedNonUnique,

    pub year2018: NonSortedNonUnique,
    pub year2017: NonSortedNonUnique,
    pub year2016: NonSortedNonUnique,
    pub year2015: NonSortedNonUnique,
    pub year2014: NonSortedNonUnique,
}

impl LogLinearTimeCache {
    fn filter_entries<F>(entries: &[Entry], select: F) -> NonSortedNonUnique
    where
        F: Fn(&Entry) -> bool,
    {
        let mut vec = Vec::new();
        for (i, entry) in entries.iter().enumerate() {
            if select(entry) {
                vec.push(i as u32);
            }
        }
        vec.shrink_to_fit();
        NonSortedNonUnique(vec)
    }

    pub fn new(meets: &[Meet], entries: &[Entry]) -> LogLinearTimeCache {
        LogLinearTimeCache {
            raw: Self::filter_entries(entries, |e| e.equipment == Equipment::Raw),
            wraps: Self::filter_entries(entries, |e| e.equipment == Equipment::Wraps),
            raw_wraps: Self::filter_entries(entries, |e| {
                e.equipment == Equipment::Raw || e.equipment == Equipment::Wraps
            }),
            single: Self::filter_entries(entries, |e| e.equipment == Equipment::Single),
            multi: Self::filter_entries(entries, |e| e.equipment == Equipment::Multi),

            male: Self::filter_entries(entries, |e| e.sex == Sex::M),
            female: Self::filter_entries(entries, |e| e.sex == Sex::F),

            year2018: Self::filter_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2018
            }),
            year2017: Self::filter_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2017
            }),
            year2016: Self::filter_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2016
            }),
            year2015: Self::filter_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2015
            }),
            year2014: Self::filter_entries(entries, |e| {
                meets[e.meet_id as usize].date.year() == 2014
            }),
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
