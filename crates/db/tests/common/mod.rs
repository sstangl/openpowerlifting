//! Common code for database integration tests.

use opldb::OplDb;

use std::path::Path;
use std::sync::LazyLock;

static OPLDB_GLOBAL: LazyLock<OplDb> = LazyLock::new(|| {
    OplDb::from_csv(
        Path::new("../../build/lifters.csv"),
        Path::new("../../build/meets.csv"),
        Path::new("../../build/entries.csv"),
        Path::new("../../build/username_map.fst"),
    )
    .unwrap()
});

pub fn db() -> &'static OplDb {
    &*OPLDB_GLOBAL
}
