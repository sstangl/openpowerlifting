//! Implements the /api/search endpoints.

use pages::selection::Selection;
use opldb::OplDb;

/// JSON return from the /api/search/rankings/ endpoint.
#[derive(Serialize)]
pub struct SearchRankingsResult {
    /// The next index of a search result to which the viewport should update.
    pub next_index: Option<usize>,
}

pub fn search_rankings<'db>(
    opldb: &'db OplDb,
    selection: &Selection,
    start_row: usize, // Inclusive.
    query: &str,
) -> SearchRankingsResult {
    // Convert the query string to a normalized form.
    // This tries to make it look like a username, since we're
    // just doing comparisons on the username.
    // TODO: Handle non-ASCII UTF-8 characters.
    let mut normalized: String = query.to_ascii_lowercase();
    normalized = normalized.replace(" ", "");

    // Disallow bogus searches.
    if normalized.len() == 0 {
        return SearchRankingsResult { next_index: None };
    }

    // TODO: Use a better algorithm, don't generate everything.
    let list = opldb
        .get_static_cache()
        .get_full_sorted_uniqued(selection, opldb);

    // Handle out-of-bounds requests.
    if start_row >= list.0.len() {
        return SearchRankingsResult { next_index: None };
    }

    // TODO: Use a better algorithm; this is really a MVP.
    for i in start_row..list.0.len() {
        let entry = opldb.get_entry(list.0[i]);
        let lifter = opldb.get_lifter(entry.lifter_id);

        if lifter.username.contains(&normalized) {
            return SearchRankingsResult {
                next_index: Some(start_row + i)
            };
        }
    }

    return SearchRankingsResult { next_index: None };
}
