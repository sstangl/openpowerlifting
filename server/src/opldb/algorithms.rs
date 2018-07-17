//! Shared algorithms that operate on StaticCache data.

use std::cmp;

use opldb::fields::*;
use opldb::{Entry, Meet};

/// Whether an `Entry` should be part of `BySquat` rankings and records.
#[inline]
pub fn filter_squat(entry: &Entry) -> bool {
    entry.highest_squatkg() > WeightKg(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByBench` rankings and records.
#[inline]
pub fn filter_bench(entry: &Entry) -> bool {
    entry.highest_benchkg() > WeightKg(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByDeadlift` rankings and records.
#[inline]
pub fn filter_deadlift(entry: &Entry) -> bool {
    entry.highest_deadliftkg() > WeightKg(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByTotal` rankings and records.
#[inline]
pub fn filter_total(entry: &Entry) -> bool {
    // TotalKg is defined to be zero if DQ.
    entry.totalkg > WeightKg(0)
}

/// Whether an `Entry` should be part of `ByMcCulloch` rankings and records.
#[inline]
pub fn filter_mcculloch(entry: &Entry) -> bool {
    // McCulloch points are defined to be zero if DQ.
    entry.mcculloch > Points(0)
}

/// Whether an `Entry` should be part of `ByWilks` rankings and records.
#[inline]
pub fn filter_wilks(entry: &Entry) -> bool {
    // Wilks is defined to be zero if DQ.
    entry.wilks > Points(0)
}

/// Defines an `Ordering` of Entries by Squat.
#[inline]
pub fn cmp_squat(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by SquatKg, higher first.
    a.highest_squatkg().cmp(&b.highest_squatkg()).reverse()
        // If equal, sort by Bodyweight, lower first.
        .then(a.bodyweightkg.cmp(&b.bodyweightkg))
        // If that's equal too, sort by Date, earlier first.
        .then(meets[a.meet_id as usize].date.cmp(&meets[b.meet_id as usize].date))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Bench.
#[inline]
pub fn cmp_bench(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by BenchKg, higher first.
    a.highest_benchkg().cmp(&b.highest_benchkg()).reverse()
        // If equal, sort by Bodyweight, lower first.
        .then(a.bodyweightkg.cmp(&b.bodyweightkg))
        // If that's equal too, sort by Date, earlier first.
        .then(meets[a.meet_id as usize].date.cmp(&meets[b.meet_id as usize].date))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Deadlift.
#[inline]
pub fn cmp_deadlift(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by DeadliftKg, higher first.
    a.highest_deadliftkg().cmp(&b.highest_deadliftkg()).reverse()
        // If equal, sort by Bodyweight, lower first.
        .then(a.bodyweightkg.cmp(&b.bodyweightkg))
        // If that's equal too, sort by Date, earlier first.
        .then(meets[a.meet_id as usize].date.cmp(&meets[b.meet_id as usize].date))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Total.
#[inline]
pub fn cmp_total(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by TotalKg, higher first.
    a.totalkg.cmp(&b.totalkg).reverse()
        // If equal, sort by Bodyweight, lower first.
        .then(a.bodyweightkg.cmp(&b.bodyweightkg))
        // If that's equal too, sort by Date, earlier first.
        .then(meets[a.meet_id as usize].date.cmp(&meets[b.meet_id as usize].date))
}

/// Defines an `Ordering` of Entries by McCulloch points.
#[inline]
pub fn cmp_mcculloch(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by McCulloch, higher first.
    a.mcculloch.cmp(&b.mcculloch).reverse()
        // If equal, sort by Date, earlier first.
        .then(meets[a.meet_id as usize].date.cmp(&meets[b.meet_id as usize].date))
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Wilks.
#[inline]
pub fn cmp_wilks(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by Wilks, higher first.
    a.wilks.cmp(&b.wilks).reverse()
        // If equal, sort by Date, earlier first.
        .then(meets[a.meet_id as usize].date.cmp(&meets[b.meet_id as usize].date))
        // If that's equal too, sort by Total, highest first.
        .then(a.totalkg.cmp(&b.totalkg).reverse())
}
