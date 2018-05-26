//! Precalculated cache of data, such as sorts.

use itertools::Itertools;
use opldb::fields::*;
use opldb::{Entry, Meet};
use std::cmp::Ordering;

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
pub struct NonSortedNonUnique(Vec<u32>);

/// List of indices into the opldb.entries vector,
/// in some sorted order, but with each lifter potentially
/// occurring multiple times.
///
/// This is useful to get `O(n)` lookup, since it stores
/// the filter/sort algorithm in an intermediate output,
/// where further filtering and uniqueness can be applied.
pub struct SortedNonUnique(Vec<u32>);

/// List of indices into the opldb.entries vector,
/// in some sorted order, with each lifter occurring at
/// most once.
///
/// This is useful to get `O(1)` lookup, since it stores
/// the filter/sort/unique algorithm in its final output.
pub struct SortedUnique(pub Vec<u32>);

/// Owning structure of all precomputed data.
pub struct StaticCache {
    pub constant_time: ConstantTimeCache,
    pub linear_time: LinearTimeCache,
    pub log_linear_time: LogLinearTimeCache,
}

impl StaticCache {
    pub fn new(meets: &Vec<Meet>, entries: &Vec<Entry>) -> StaticCache {
        let loglin = LogLinearTimeCache::new(entries);

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

/// Owning structure of all `O(1)` lookup data.
pub struct ConstantTimeCache {
    // Weight comparisons.
    pub squat: ConstantTimeBy,
    pub bench: ConstantTimeBy,
    pub deadlift: ConstantTimeBy,
    pub total: ConstantTimeBy,

    // Points comparisons.
    pub wilks: ConstantTimeBy,
}

impl ConstantTimeCache {
    /// Sorts and uniques the data with reference to a comparator.
    ///
    /// The comparator should return greatest-first, in sorted order
    /// by however it should show up in the final database.
    ///
    /// TODO: Filter out zero entries (like lifters with no squat for by-squat,
    /// etc.)
    fn sort_and_unique_by<F>(
        idxl: &NonSortedNonUnique,
        entries: &Vec<Entry>,
        compare: F,
    ) -> SortedUnique
    where
        F: Fn(u32, u32) -> Ordering,
    {
        // First, group contiguous entries by lifter_id, so only the best
        // entry for each lifter is counted.
        // The group_by() operation is lazy and does not perform any action yet.
        let groups = idxl
            .0
            .iter()
            .group_by(|idx| entries[**idx as usize].lifter_id);

        // Perform the grouping operation, generating a new vector.
        let mut vec: Vec<u32> = groups
            .into_iter()
            // `min_by()` takes the best entry due to comparator ordering.
            .map(|(_key, group)| *group.min_by(|&x, &y| compare(*x, *y)).unwrap())
            .collect();

        vec.sort_by(|&x, &y| compare(x, y));
        vec.shrink_to_fit();
        SortedUnique(vec)
    }

    pub fn new(
        loglin: &LogLinearTimeCache,
        meets: &Vec<Meet>,
        entries: &Vec<Entry>,
    ) -> ConstantTimeCache {
        let by_squat = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by SquatKg, highest first.
            entries[x].highest_squatkg().cmp(&entries[y].highest_squatkg()).reverse()
                // If equal, sort by Bodyweight, since this is for rankings.
                // (Records would sort by Date before Bodyweight.)
                .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                // If that's equal too, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
        };

        let squat = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_squat),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_squat),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_squat),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_squat),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_squat),
        };

        let by_bench = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by SquatKg, highest first.
            entries[x].highest_benchkg().cmp(&entries[y].highest_benchkg()).reverse()
                // If equal, sort by Bodyweight, since this is for rankings.
                // (Records would sort by Date before Bodyweight.)
                .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                // If that's equal too, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
        };

        let bench = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_bench),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_bench),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_bench),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_bench),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_bench),
        };

        let by_deadlift = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by SquatKg, highest first.
            entries[x].highest_deadliftkg().cmp(
                    &entries[y].highest_deadliftkg()).reverse()
                // If equal, sort by Bodyweight, since this is for rankings.
                // (Records would sort by Date before Bodyweight.)
                .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                // If that's equal too, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
        };

        let deadlift = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_deadlift),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_deadlift),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_deadlift),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_deadlift),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_deadlift),
        };

        let by_total = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by SquatKg, highest first.
            entries[x].totalkg.cmp(&entries[y].totalkg).reverse()
                // If equal, sort by Bodyweight, since this is for rankings.
                // (Records would sort by Date before Bodyweight.)
                .then(entries[x].bodyweightkg.cmp(&entries[y].bodyweightkg))
                // If that's equal too, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
        };

        let total = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_total),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_total),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_total),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_total),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_total),
        };

        let by_wilks = |x: u32, y: u32| {
            let x = x as usize;
            let y = y as usize;

            // First sort by Wilks, highest first.
            entries[x].wilks.cmp(&entries[y].wilks).reverse()
                // If equal, sort by Date, earliest first.
                .then(meets[entries[x].meet_id as usize].date.cmp(
                        &meets[entries[y].meet_id as usize].date))
                // If that's equal too, sort by Total, highest first.
                .then(entries[x].totalkg.cmp(&entries[y].totalkg))
        };

        let wilks = ConstantTimeBy {
            raw: Self::sort_and_unique_by(&loglin.raw, entries, by_wilks),
            wraps: Self::sort_and_unique_by(&loglin.wraps, entries, by_wilks),
            raw_wraps: Self::sort_and_unique_by(&loglin.raw_wraps, entries, by_wilks),
            single: Self::sort_and_unique_by(&loglin.single, entries, by_wilks),
            multi: Self::sort_and_unique_by(&loglin.multi, entries, by_wilks),
        };

        ConstantTimeCache {
            squat,
            bench,
            deadlift,
            total,
            wilks,
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
}

impl LogLinearTimeCache {
    fn filter_entries<F>(entries: &Vec<Entry>, select: F) -> NonSortedNonUnique
    where
        F: Fn(&Entry) -> bool,
    {
        let mut vec = Vec::new();
        for i in 0..entries.len() {
            if select(&entries[i]) {
                vec.push(i as u32);
            }
        }
        vec.shrink_to_fit();
        NonSortedNonUnique(vec)
    }

    pub fn new(entries: &Vec<Entry>) -> LogLinearTimeCache {
        LogLinearTimeCache {
            raw: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.equipment == Equipment::Raw
            }),
            wraps: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.equipment == Equipment::Wraps
            }),
            raw_wraps: Self::filter_entries(entries, |e| {
                !e.place.is_dq()
                    && (e.equipment == Equipment::Raw || e.equipment == Equipment::Wraps)
            }),
            single: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.equipment == Equipment::Single
            }),
            multi: Self::filter_entries(entries, |e| {
                !e.place.is_dq() && e.equipment == Equipment::Multi
            }),
        }
    }
}
