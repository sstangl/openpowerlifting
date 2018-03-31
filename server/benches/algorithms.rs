#![feature(test)]

extern crate server;
use server::opldb::CachedFilter;
use server::opldb::OplDb;

use std::sync::{Once, ONCE_INIT};

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = ONCE_INIT;

fn db() -> &'static OplDb {
    const LIFTERS_CSV: &str = "../build/bench-data/lifters.csv";
    const MEETS_CSV: &str = "../build/bench-data/meets.csv";
    const ENTRIES_CSV: &str = "../build/bench-data/openpowerlifting.csv";

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

    #[bench]
    fn bench_get_entries_for_lifter(b: &mut Bencher) {
        let opldb = db();
        b.iter(|| {
            opldb.get_entries_for_lifter(1);
        });
    }

    #[bench]
    fn bench_get_entries_for_meet(b: &mut Bencher) {
        let opldb = db();
        b.iter(|| {
            opldb.get_entries_for_meet(0);
        });
    }

    #[bench]
    fn bench_filter_raw_intersect_2017(b: &mut Bencher) {
        let opldb = db();
        let filter_raw = opldb.get_filter(CachedFilter::EquipmentRaw);
        let filter_2017 = opldb.get_filter(CachedFilter::Year2017);

        b.iter(|| {
            filter_raw.intersect(filter_2017);
        });
    }
}
