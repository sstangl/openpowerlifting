//! Shared algorithms that operate on StaticCache data.

use opltypes::*;

use std::borrow::Cow;
use std::cmp;

use crate::cache::NonSortedNonUnique;
use crate::cache::SortedUnique;
use crate::query::direct::*;
use crate::{Entry, Meet, OplDb};

/// Whether an `Entry` should be part of `BySquat` rankings and records.
#[inline]
pub fn filter_squat(entry: &Entry) -> bool {
    entry.highest_squatkg() > WeightKg::from_i32(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByBench` rankings and records.
#[inline]
pub fn filter_bench(entry: &Entry) -> bool {
    entry.highest_benchkg() > WeightKg::from_i32(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByDeadlift` rankings and records.
#[inline]
pub fn filter_deadlift(entry: &Entry) -> bool {
    entry.highest_deadliftkg() > WeightKg::from_i32(0) && !entry.place.is_dq()
}

/// Whether an `Entry` should be part of `ByTotal` rankings and records.
#[inline]
pub fn filter_total(entry: &Entry) -> bool {
    // TotalKg is defined to be zero if DQ.
    entry.totalkg > WeightKg::from_i32(0)
}

/// Whether an `Entry` should be part of `ByMcCulloch` rankings and records.
#[inline]
pub fn filter_mcculloch(entry: &Entry) -> bool {
    // McCulloch points are defined to be zero if DQ.
    entry.points(PointsSystem::McCulloch, WeightUnits::Kg) > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByWilks` rankings and records.
#[inline]
pub fn filter_wilks(entry: &Entry) -> bool {
    // Wilks is defined to be zero if DQ.
    entry.points(PointsSystem::Wilks, WeightUnits::Kg) > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByWilks2020` rankings and records.
#[inline]
pub fn filter_wilks2020(entry: &Entry) -> bool {
    // Wilks2020 is defined to be zero if DQ.
    entry.points(PointsSystem::Wilks2020, WeightUnits::Kg) > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByGlossbrenner` rankings and records.
#[inline]
pub fn filter_glossbrenner(entry: &Entry) -> bool {
    // Glossbrenner is defined to be zero if DQ.
    entry.points(PointsSystem::Glossbrenner, WeightUnits::Kg) > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByGoodlift` rankings and records.
#[inline]
pub fn filter_goodlift(entry: &Entry) -> bool {
    // Goodlift Points are defined to be zero if DQ.
    entry.points(PointsSystem::Goodlift, WeightUnits::Kg) > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByIPFPoints` rankings and records.
#[inline]
pub fn filter_ipfpoints(entry: &Entry) -> bool {
    // IPF Points are defined to be zero if DQ.
    entry.points(PointsSystem::IPFPoints, WeightUnits::Kg) > Points::from_i32(0)
}

/// Whether an `Entry` should be part of `ByDots` rankings and records.
#[inline]
pub fn filter_dots(entry: &Entry) -> bool {
    // Dots points are defined to be zero if DQ.
    entry.points(PointsSystem::Dots, WeightUnits::Kg) > Points::from_i32(0)
}

/// Defines an `Ordering` of Entries by Squat.
#[inline]
pub fn cmp_squat(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by SquatKg, higher first.
    a.highest_squatkg()
        .cmp(&b.highest_squatkg())
        .reverse()
        // If equal, sort by Date, earlier first.
        .then_with(|| {
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date)
        })
        // If equal, sort by Bodyweight, lower first.
        .then_with(|| a.bodyweightkg.cmp(&b.bodyweightkg))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then_with(|| a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Bench.
#[inline]
pub fn cmp_bench(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by BenchKg, higher first.
    a.highest_benchkg()
        .cmp(&b.highest_benchkg())
        .reverse()
        // If equal, sort by Date, earlier first.
        .then_with(|| {
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date)
        })
        // If equal, sort by Bodyweight, lower first.
        .then_with(|| a.bodyweightkg.cmp(&b.bodyweightkg))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then_with(|| a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Deadlift.
#[inline]
pub fn cmp_deadlift(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by DeadliftKg, higher first.
    a.highest_deadliftkg()
        .cmp(&b.highest_deadliftkg())
        .reverse()
        // If equal, sort by Date, earlier first.
        .then_with(|| {
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date)
        })
        // If equal, sort by Bodyweight, lower first.
        .then_with(|| a.bodyweightkg.cmp(&b.bodyweightkg))
        // If for the same lifter on the same day, prefer Entry with largest Total.
        .then_with(|| a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by Total.
#[inline]
pub fn cmp_total(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    // First sort by TotalKg, higher first.
    a.totalkg
        .cmp(&b.totalkg)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then_with(|| {
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date)
        })
        // If equal, sort by Bodyweight, lower first.
        .then_with(|| a.bodyweightkg.cmp(&b.bodyweightkg))
}

/// Defines a generic `Ordering` of Entries by some points.
#[inline(always)]
fn cmp_generic_points(meets: &[Meet], a: &Entry, b: &Entry, system: PointsSystem) -> cmp::Ordering {
    let a_points = a.points(system, WeightUnits::Kg);
    let b_points = b.points(system, WeightUnits::Kg);

    // First sort by points, higher first.
    a_points
        .cmp(&b_points)
        .reverse()
        // If equal, sort by Date, earlier first.
        .then_with(|| {
            meets[a.meet_id as usize]
                .date
                .cmp(&meets[b.meet_id as usize].date)
        })
        // If that's equal too, sort by Total, highest first.
        .then_with(|| a.totalkg.cmp(&b.totalkg).reverse())
}

/// Defines an `Ordering` of Entries by McCulloch points.
#[inline]
pub fn cmp_mcculloch(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::McCulloch)
}

/// Defines an `Ordering` of Entries by Wilks.
#[inline]
pub fn cmp_wilks(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::Wilks)
}

/// Defines an `Ordering` of Entries by Dots points.
#[inline]
pub fn cmp_dots(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::Dots)
}

/// Defines an `Ordering` of Entries by Glossbrenner.
#[inline]
pub fn cmp_glossbrenner(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::Glossbrenner)
}

/// Defines an `Ordering` of Entries by Goodlift.
#[inline]
pub fn cmp_goodlift(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::Goodlift)
}

/// Defines an `Ordering` of Entries by IPF Points.
#[inline]
pub fn cmp_ipfpoints(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::IPFPoints)
}

/// Defines an `Ordering` of Entries by NASA Points.
#[inline]
pub fn cmp_nasa(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::NASA)
}

/// Defines an `Ordering` of Entries by Wilks2020 Points.
#[inline]
pub fn cmp_wilks2020(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::Wilks2020)
}

/// Defines an `Ordering` of Entries by Reshel points.
#[inline]
pub fn cmp_reshel(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::Reshel)
}

/// Defines an `Ordering` of Entries by Schwartz/Malone points.
#[inline]
pub fn cmp_schwartzmalone(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::SchwartzMalone)
}

/// Defines an `Ordering` of Entries by AH (Haleczko) points.
#[inline]
pub fn cmp_ah(meets: &[Meet], a: &Entry, b: &Entry) -> cmp::Ordering {
    cmp_generic_points(meets, a, b, PointsSystem::AH)
}

/// Gets a list of all entry indices matching the given selection.
pub fn entry_indices_for<'db>(
    selection: &EntryFilter,
    opldb: &'db OplDb,
) -> Cow<'db, NonSortedNonUnique> {
    let cache = opldb.cache();

    // Use the NonSortedNonUnique cached data.
    let equipment: &NonSortedNonUnique = match selection.equipment {
        EquipmentFilter::Raw => &cache.log_linear_time.raw,
        EquipmentFilter::Wraps => &cache.log_linear_time.wraps,
        EquipmentFilter::RawAndWraps => &cache.log_linear_time.raw_wraps,
        EquipmentFilter::Single => &cache.log_linear_time.single,
        EquipmentFilter::Multi => &cache.log_linear_time.multi,
        EquipmentFilter::Unlimited => &cache.log_linear_time.unlimited,
    };
    let mut cur = Cow::Borrowed(equipment);

    // Apply the Sex filter.
    cur = match selection.sex {
        SexFilter::AllSexes => cur,
        SexFilter::Men => Cow::Owned(cur.intersect(&cache.log_linear_time.male)),
        SexFilter::Women => Cow::Owned(cur.intersect(&cache.log_linear_time.female)),
    };

    // Apply the Year filter.
    cur = match selection.year {
        YearFilter::AllYears => cur,
        YearFilter::OneYear(year) => {
            if let Some(year_cache) = cache.log_linear_time.year_cache(year as u32) {
                Cow::Owned(cur.intersect(year_cache))
            } else {
                let year = year as u32;
                let filter = NonSortedNonUnique(
                    cur.0
                        .iter()
                        .filter_map(|&i| {
                            match opldb.meet(opldb.entry(i).meet_id).date.year() == year {
                                true => Some(i),
                                false => None,
                            }
                        })
                        .collect(),
                );
                Cow::Owned(filter)
            }
        }
    };

    // Filter by State manually.
    if selection.state.is_some() {
        let filter = NonSortedNonUnique(
            cur.0
                .iter()
                .filter_map(|&i| match opldb.entry(i).lifter_state == selection.state {
                    true => Some(i),
                    false => None,
                })
                .collect(),
        );
        cur = Cow::Owned(filter);
    }

    // Filter by federation manually.
    if selection.federation != FederationFilter::AllFederations {
        if let FederationFilter::One(fed) = selection.federation {
            let filter = NonSortedNonUnique(
                cur.0
                    .iter()
                    .filter_map(
                        |&i| match opldb.meet(opldb.entry(i).meet_id).federation == fed {
                            true => Some(i),
                            false => None,
                        },
                    )
                    .collect(),
            );
            cur = Cow::Owned(filter);
        } else if let FederationFilter::Meta(metafed) = selection.federation {
            let meets = opldb.meets();
            let filter = NonSortedNonUnique(
                cur.0
                    .iter()
                    .filter_map(|&i| match metafed.contains(opldb.entry(i), meets) {
                        true => Some(i),
                        false => None,
                    })
                    .collect(),
            );
            cur = Cow::Owned(filter);
        }
    }

    // Filter by AgeClass manually.
    if selection.ageclass != AgeClassFilter::AllAges {
        let filter = NonSortedNonUnique(
            cur.0
                .iter()
                .filter_map(|&i| {
                    use AgeClass::*;
                    use AgeClassFilter::*;

                    let class = opldb.entry(i).ageclass;
                    let byclass = opldb.entry(i).birthyearclass;

                    let matches: bool = match selection.ageclass {
                        AllAges => true,

                        // Age-based classes.
                        Youth512 => class == Class5_12,
                        Teenage1315 => class == Class13_15,
                        Teenage1617 => class == Class16_17,
                        Teenage1819 => class == Class18_19,
                        Juniors2023 => class == Class20_23,
                        Seniors2434 => class == Class24_34,
                        Submasters3539 => class == Class35_39,
                        Masters4049 => class == Class40_44 || class == Class45_49,
                        Masters5059 => class == Class50_54 || class == Class55_59,
                        Masters6069 => class == Class60_64 || class == Class65_69,
                        Masters7079 => class == Class70_74 || class == Class75_79,
                        Masters4044 => class == Class40_44,
                        Masters4549 => class == Class45_49,
                        Masters5054 => class == Class50_54,
                        Masters5559 => class == Class55_59,
                        Masters6064 => class == Class60_64,
                        Masters6569 => class == Class65_69,
                        Masters7074 => class == Class70_74,
                        Masters7579 => class == Class75_79,
                        MastersOver80 => class == Class80_999,

                        // BirthYear-based classes.
                        SubJuniorsY14Y18 => byclass == BirthYearClass::ClassY14Y18,
                        JuniorsY14Y23 => {
                            byclass == BirthYearClass::ClassY19Y23
                                || byclass == BirthYearClass::ClassY14Y18
                        }
                        SeniorsY24Y39 => byclass == BirthYearClass::ClassY24Y39,
                        MastersOverY40 => byclass >= BirthYearClass::ClassY40Y49,
                        MastersOverY50 => byclass >= BirthYearClass::ClassY50Y59,
                        MastersOverY60 => byclass >= BirthYearClass::ClassY60Y69,
                        MastersOverY70 => byclass == BirthYearClass::ClassY70Y999,
                    };
                    if matches {
                        Option::Some(i)
                    } else {
                        Option::None
                    }
                })
                .collect(),
        );
        cur = Cow::Owned(filter);
    }

    // Filter by event manually.
    if selection.event != EventFilter::AllEvents {
        let filter = NonSortedNonUnique(
            cur.0
                .iter()
                .filter_map(|&i| {
                    let ev = opldb.entry(i).event;
                    let matches: bool = match selection.event {
                        EventFilter::AllEvents => true,
                        EventFilter::FullPower => ev.is_full_power(),
                        EventFilter::PushPull => ev.is_push_pull(),
                        EventFilter::SquatOnly => ev.is_squat_only(),
                        EventFilter::BenchOnly => ev.is_bench_only(),
                        EventFilter::DeadliftOnly => ev.is_deadlift_only(),
                    };
                    if matches {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect(),
        );
        cur = Cow::Owned(filter);
    }

    // Filter by weight class manually.
    if selection.weightclasses != WeightClassFilter::AllClasses {
        let (lower, upper) = selection.weightclasses.to_bounds();

        let filter = NonSortedNonUnique(
            cur.0
                .iter()
                .filter_map(|&i| {
                    let e = opldb.entry(i);

                    // Handle cases with explicit bodyweight.
                    if e.bodyweightkg > lower && e.bodyweightkg <= upper {
                        return Some(i);
                    }

                    // Handle SHW classes with unspecified bodyweight.
                    if upper == WeightKg::MAX {
                        if let WeightClassKg::Over(over) = e.weightclasskg {
                            if over >= lower {
                                return Some(i);
                            }
                        }
                    }

                    None
                })
                .collect(),
        );
        cur = Cow::Owned(filter);
    }

    cur
}

/// Gets a full sorted list for the given selection.
///
/// In almost every case it's not necessary to generate the full list,
/// but doing so can be useful for debugging.
pub fn full_sorted_uniqued<'db>(
    query: &RankingsQuery,
    opldb: &'db OplDb,
) -> Cow<'db, SortedUnique> {
    let cache = opldb.cache();

    // First, try to use the constant-time cache.
    if query.filter.federation == FederationFilter::AllFederations
        && query.filter.weightclasses == WeightClassFilter::AllClasses
        && query.filter.year == YearFilter::AllYears
        && query.filter.ageclass == AgeClassFilter::AllAges
        && query.filter.event == EventFilter::AllEvents
        && query.filter.state.is_none()
    {
        let by_sort = match query.order_by {
            OrderBy::Squat => &cache.constant_time.squat,
            OrderBy::Bench => &cache.constant_time.bench,
            OrderBy::Deadlift => &cache.constant_time.deadlift,
            OrderBy::Total => &cache.constant_time.total,
            OrderBy::Dots => &cache.constant_time.dots,
            OrderBy::Glossbrenner => &cache.constant_time.glossbrenner,
            OrderBy::Goodlift => &cache.constant_time.goodlift,
            OrderBy::McCulloch => &cache.constant_time.mcculloch,
            OrderBy::Wilks => &cache.constant_time.wilks,
        };

        let sorted_uniqued = match query.filter.equipment {
            EquipmentFilter::Raw => &by_sort.raw,
            EquipmentFilter::Wraps => &by_sort.wraps,
            EquipmentFilter::RawAndWraps => &by_sort.raw_wraps,
            EquipmentFilter::Single => &by_sort.single,
            EquipmentFilter::Multi => &by_sort.multi,
            EquipmentFilter::Unlimited => &by_sort.unlimited,
        };

        // Since each lifter is only one sex, sex selections
        // can just be an O(n) filter.
        if query.filter.sex != SexFilter::AllSexes {
            return Cow::Owned(SortedUnique(
                sorted_uniqued
                    .0
                    .iter()
                    .filter_map(|&n| {
                        let sex = opldb.entry(n).sex;
                        match (query.filter.sex == SexFilter::Men && sex == Sex::M)
                            || (query.filter.sex == SexFilter::Women && sex == Sex::F)
                        {
                            true => Some(n),
                            false => None,
                        }
                    })
                    .collect(),
            ));
        }

        return Cow::Borrowed(sorted_uniqued);
    }

    // If the ConstantTime cache fails, use the NonSortedNonUnique cache data.
    let cur = entry_indices_for(&query.filter, opldb);

    let entries = opldb.entries();
    let meets = opldb.meets();

    // TODO: Common out sort code with ConstantTimeCache::new()
    Cow::Owned(match query.order_by {
        OrderBy::Squat => cur.sort_and_unique_by(entries, meets, cmp_squat, filter_squat),
        OrderBy::Bench => cur.sort_and_unique_by(entries, meets, cmp_bench, filter_bench),
        OrderBy::Deadlift => cur.sort_and_unique_by(entries, meets, cmp_deadlift, filter_deadlift),
        OrderBy::Total => cur.sort_and_unique_by(entries, meets, cmp_total, filter_total),
        OrderBy::Dots => cur.sort_and_unique_by(entries, meets, cmp_dots, filter_dots),
        OrderBy::Glossbrenner => {
            cur.sort_and_unique_by(entries, meets, cmp_glossbrenner, filter_glossbrenner)
        }
        OrderBy::Goodlift => cur.sort_and_unique_by(entries, meets, cmp_goodlift, filter_goodlift),
        OrderBy::McCulloch => {
            cur.sort_and_unique_by(entries, meets, cmp_mcculloch, filter_mcculloch)
        }
        OrderBy::Wilks => cur.sort_and_unique_by(entries, meets, cmp_wilks, filter_wilks),
    })
}
