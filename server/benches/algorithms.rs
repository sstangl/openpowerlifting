//! Defines benchmarks for internal OplDb data structures and algorithms.

#![feature(test)]

extern crate server;
use server::opldb::algorithms::*;
use server::opldb::OplDb;

use std::sync::{Once, ONCE_INIT};

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = ONCE_INIT;

fn db() -> &'static OplDb {
    const LIFTERS_CSV: &str = "../build/lifters.csv";
    const MEETS_CSV: &str = "../build/meets.csv";
    const ENTRIES_CSV: &str = "../build/entries.csv";

    unsafe {
        OPLDB_INIT.call_once(|| {
            OPLDB_GLOBAL =
                Some(OplDb::from_csv(LIFTERS_CSV, MEETS_CSV, ENTRIES_CSV).unwrap())
        });

        OPLDB_GLOBAL.as_ref().unwrap()
    }
}

mod benches {
    use super::*;

    extern crate test;
    use self::test::Bencher;

    /// Time taken to enumerate all the entries for a single lifter.
    #[bench]
    fn bench_get_entries_for_lifter(b: &mut Bencher) {
        let opldb = db();
        b.iter(|| {
            opldb.get_entries_for_lifter(1);
        });
    }

    /// Time taken to enumerate all the entries for a single meet.
    #[bench]
    fn bench_get_entries_for_meet(b: &mut Bencher) {
        let opldb = db();
        b.iter(|| {
            opldb.get_entries_for_meet(0);
        });
    }

    /// Time taken to sort and unique all Raw and Wraps entries by Wilks.
    #[bench]
    fn raw_wraps_sort_and_unique_by_wilks(b: &mut Bencher) {
        let opldb = db();
        let cache = opldb.get_static_cache();

        b.iter(|| {
            cache.log_linear_time.raw_wraps.sort_and_unique_by(
                opldb.get_entries(),
                opldb.get_meets(),
                &cmp_wilks,
                &filter_wilks,
            );
        });
    }
}
