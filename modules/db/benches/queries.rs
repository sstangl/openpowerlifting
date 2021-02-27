//! Benchmarks loading the database from data files.

use criterion::{criterion_group, criterion_main, Criterion};
use opldb::query::direct::*;
use opldb::{MetaFederation, OplDb};

use std::sync::Once;

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = Once::new();

const LIFTERS_CSV: &str = "../../build/lifters.csv";
const MEETS_CSV: &str = "../../build/meets.csv";
const ENTRIES_CSV: &str = "../../build/entries.csv";

fn db() -> &'static OplDb {
    unsafe {
        OPLDB_INIT.call_once(|| {
            OPLDB_GLOBAL = Some(OplDb::from_csv(LIFTERS_CSV, MEETS_CSV, ENTRIES_CSV).unwrap());
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
            opldb::algorithms::get_full_sorted_uniqued(&query, &db);
        });
    });
}

criterion_group!(benches, query_benchmarks);
criterion_main!(benches);
