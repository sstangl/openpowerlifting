//! Benchmarks loading the database from data files.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use opldb::query::direct::*;
use opldb::{MetaFederation, OplDb};

use std::path::Path;
use std::sync::Once;

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = Once::new();

const LIFTERS_CSV: &str = "../../build/lifters.csv";
const MEETS_CSV: &str = "../../build/meets.csv";
const ENTRIES_CSV: &str = "../../build/entries.csv";

fn db() -> &'static OplDb {
    unsafe {
        OPLDB_INIT.call_once(|| {
            OPLDB_GLOBAL = Some(
                OplDb::from_csv(
                    Path::new(LIFTERS_CSV),
                    Path::new(MEETS_CSV),
                    Path::new(ENTRIES_CSV),
                )
                .unwrap(),
            );
        });
        OPLDB_GLOBAL.as_ref().unwrap()
    }
}

pub fn query_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("queries");
    let db = db();

    // Taken from default_openipf_rankings_query().
    group.bench_function("openipf full default", |b| {
        let query = RankingsQuery {
            filter: EntryFilter {
                equipment: EquipmentFilter::Raw,
                federation: FederationFilter::Meta(MetaFederation::IPFAndAffiliates),
                weightclasses: WeightClassFilter::AllClasses,
                sex: SexFilter::AllSexes,
                ageclass: AgeClassFilter::AllAges,
                year: YearFilter::AllYears,
                event: EventFilter::FullPower,
                state: None,
            },
            order_by: OrderBy::Goodlift,
        };

        b.iter(|| {
            opldb::algorithms::full_sorted_uniqued(&query, black_box(db));
        });
    });
}

pub fn data_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_structures");
    let db = db();

    let cache = db.cache_for_benchmarks();
    let raw_wraps = &cache.log_linear_time.raw_wraps;
    let male = &cache.log_linear_time.male;

    // Benchmark set intersection on actual data.
    // TODO(sstangl): The throughput is best characterized 3-dimensionally.
    group.throughput(Throughput::Elements(raw_wraps.0.len() as u64));
    group.bench_function("NonSortedNonUnique::intersect()", |b| {
        b.iter(|| black_box(raw_wraps.intersect(black_box(male))));
    });
}

/// Benchmarks dealing with looking up information on specific lifters.
///
/// This is the most common operation on the server because of web crawlers.
pub fn lifter_info(c: &mut Criterion) {
    let mut group = c.benchmark_group("lifter_info");
    let db = db();

    group.bench_function("lifters_under_username_base (many lifters)", |b| {
        b.iter(|| black_box(db.lifters_under_username_base("joserodriguez")))
    });

    group.bench_function("lifters_under_username_base (one lifter)", |b| {
        b.iter(|| black_box(db.lifters_under_username_base("seanstangl")))
    });
}

criterion_group!(benches, query_benchmarks, data_structures, lifter_info);
criterion_main!(benches);
