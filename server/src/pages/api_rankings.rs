//! Implements the /api/rankings endpoint, used for dynamic loading of the
//! rankings table via AJAX calls. Not intended for external use.

use langpack::Locale;
use opldb::query::direct::RankingsQuery;
use opldb::{algorithms, OplDb};
use opltypes::PointsSystem;

use crate::pages::jsdata::JsEntryRow;

#[derive(Serialize)]
pub struct RankingsSlice<'db> {
    /// The total length of the full ranking (not the length of this slice).
    pub total_length: usize,
    /// Some selection of rows.
    pub rows: Vec<JsEntryRow<'db>>,
}

pub fn query_slice<'db>(
    opldb: &'db OplDb,
    locale: &'db Locale,
    selection: &RankingsQuery,
    defaults: &RankingsQuery,
    start_row: usize, // Inclusive.
    end_row: usize,   // Inclusive. Can be out-of-bounds.
) -> RankingsSlice<'db> {
    const ROW_LIMIT: usize = 100;
    let mut end_row = end_row;

    // TODO: Use a better algorithm, don't generate everything.
    let list = algorithms::full_sorted_uniqued(selection, opldb);
    let total_length = list.0.len();

    // Limit the request size to something sane.
    // Arithmetic can't overflow since it's already been compared to total_length.
    if end_row - start_row + 1 > ROW_LIMIT {
        end_row = start_row + (ROW_LIMIT - 1);
    }

    // The request must be in-bounds.
    if end_row >= total_length {
        if total_length > 0 {
            end_row = total_length - 1;
        } else {
            end_row = 0;
        }
    }
    if (start_row > end_row) || (start_row >= total_length) {
        return RankingsSlice {
            total_length,
            rows: vec![],
        };
    }

    // Figure out the points system to be used.
    let points_system = if selection.order_by.is_by_points() {
        PointsSystem::from(selection.order_by)
    } else {
        // The selection is by-weight, so get the points from the default.
        PointsSystem::from(defaults.order_by)
    };

    let rows: Vec<JsEntryRow> = list.0[start_row..=end_row]
        .iter()
        .zip(start_row..)
        .map(|(&n, i)| JsEntryRow::from(opldb, locale, opldb.entry(n), i as u32, points_system))
        .collect();

    RankingsSlice { total_length, rows }
}
