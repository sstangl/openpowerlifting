//! Implements the /api/rankings endpoint, used for dynamic loading of the
//! rankings table via AJAX calls. Not intended for external use.

use pages::jsdata::JsEntryRow;
use pages::selection::Selection;

use langpack::Locale;
use opldb::OplDb;

#[derive(Serialize)]
pub struct RankingsSlice<'db> {
    /// The total length of the full ranking (not the length of this slice).
    pub total_length: usize,
    /// Some selection of rows.
    pub rows: Vec<JsEntryRow<'db>>,
}

pub fn get_slice<'db>(
    opldb: &'db OplDb,
    locale: &'db Locale,
    selection: &Selection,
    start_row: usize, // Inclusive.
    end_row: usize,   // Inclusive. Can be out-of-bounds.
) -> RankingsSlice<'db> {
    const ROW_LIMIT: usize = 100;
    let mut end_row = end_row;

    // TODO: Use a better algorithm, don't generate everything.
    let list = opldb
        .get_static_cache()
        .get_full_sorted_uniqued(selection, opldb);

    let total_length = list.0.len();

    // The request must be in-bounds.
    if end_row >= total_length {
        end_row = total_length - 1;
    }
    if (start_row > end_row) || (start_row >= total_length) {
        return RankingsSlice {
            total_length,
            rows: vec![],
        };
    }

    // Limit the request size to something sane.
    // Arithmetic can't overflow since it's already been compared to total_length.
    if end_row - start_row + 1 > ROW_LIMIT {
        end_row = start_row + (ROW_LIMIT - 1);
    }

    let rows: Vec<JsEntryRow> = list.0[start_row..(end_row + 1)]
        .iter()
        .zip(start_row..)
        .map(|(&n, i)| {
            JsEntryRow::from(opldb, locale, opldb.get_entry(n), i as u32, selection.sort)
        })
        .collect();

    RankingsSlice { total_length, rows }
}
