#![feature(test)]

extern crate server;
use server::opldb::CachedFilter;
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

    #[bench]
    fn bench_union_2016_2017(b: &mut Bencher) {
        let opldb = db();
        let filter_2016 = opldb.get_filter(CachedFilter::Year2016);
        let filter_2017 = opldb.get_filter(CachedFilter::Year2017);

        b.iter(|| {
            filter_2016.union(filter_2017);
        });
    }

    #[bench]
    fn bench_sort_and_unique_by(b: &mut Bencher) {
        let opldb = db();
        let filter = opldb.get_filter(CachedFilter::EquipmentRaw);

        b.iter(|| {
            filter.sort_and_unique_by(&opldb, |x, y| {
                let x_w = opldb.get_entry(x).wilks;
                let y_w = opldb.get_entry(y).wilks;
                x_w.cmp(&y_w)
            });
        });
    }

    #[bench]
    fn bench_sort_and_unique_by_wilks(b: &mut Bencher) {
        let opldb = db();
        let filter = opldb.get_filter(CachedFilter::EquipmentRaw);

        b.iter(|| {
            filter.sort_and_unique_by_wilks(&opldb);
        });
    }
}
