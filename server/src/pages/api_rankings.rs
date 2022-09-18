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

    // TODO: Use a better algorithm, don't generate everything.
    let list = algorithms::full_sorted_uniqued(selection, opldb);
    let total_length = list.0.len();

    let req_length = usize::min(
        end_row.saturating_sub(start_row).saturating_add(1),
        ROW_LIMIT,
    );

    // Figure out the points system to be used.
    let points_system = if selection.order_by.is_by_points() {
        PointsSystem::from(selection.order_by)
    } else {
        // The selection is by-weight, so get the points from the default.
        PointsSystem::from(defaults.order_by)
    };

    let slice_iter = list.0.iter().enumerate().skip(start_row).take(req_length);
    let rows: Vec<JsEntryRow> = slice_iter
        .map(|(i, n)| JsEntryRow::from(opldb, locale, opldb.entry(*n), i as u32, points_system))
        .collect();

    RankingsSlice { total_length, rows }
}
