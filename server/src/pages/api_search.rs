//! Implements the /api/search endpoints.

use opldb::query::direct::RankingsQuery;
use opldb::OplDb;

/// JSON return from the /api/search/rankings/ endpoint.
#[derive(Serialize)]
pub struct SearchRankingsResult {
    /// The next index of a search result to which the viewport should update.
    pub next_index: Option<usize>,
}

impl SearchRankingsResult {
    pub fn index(n: usize) -> SearchRankingsResult {
        SearchRankingsResult {
            next_index: Some(n),
        }
    }
}

pub fn search_rankings(
    db: &OplDb,
    rankings: &RankingsQuery,
    start_row: usize, // Inclusive.
    query: &str,
) -> SearchRankingsResult {
    SearchRankingsResult {
        next_index: search::search_rankings(db, rankings, start_row, query),
    }
}
