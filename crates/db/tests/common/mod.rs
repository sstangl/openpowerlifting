//! Common code for database integration tests.

use opldb::OplDb;

use std::path::Path;
use std::sync::Once;

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = Once::new();

pub fn db() -> &'static OplDb {
    const LIFTERS_CSV: &str = "../../build/lifters.csv";
    const MEETS_CSV: &str = "../../build/meets.csv";
    const ENTRIES_CSV: &str = "../../build/entries.csv";

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
