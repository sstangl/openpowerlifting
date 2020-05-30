//! Implements the /api/search endpoints.

use opldb::query::direct::RankingsQuery;
use opldb::{algorithms, OplDb};
use usernames::*;

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

pub fn search_rankings<'db>(
    opldb: &'db OplDb,
    selection: &RankingsQuery,
    start_row: usize, // Inclusive.
    query: &str,
) -> SearchRankingsResult {
    // FIXME: Hacky solution to "#" ,"'"" & "." being replaced by underscores
    // in the query string. The client code makes that replacement in order
    // to ensure that the URL is valid, since this is accessed via a GET parameter.
    // We could do something craftier, like base-64 encode it.
    let query = query.replace("_", "");

    let system = infer_writing_system(&query);

    // Convert the query string to a normalized form.
    // This tries to make it look like a username, since we're
    // just doing comparisons on the username.
    let normalized_latin: String = match make_username(&query) {
        Ok(s) => s,
        Err(_) => String::new(),
    };

    // Disallow bogus queries.
    if normalized_latin.is_empty() && system == WritingSystem::Latin {
        return SearchRankingsResult { next_index: None };
    }

    let backwards: String = query
        .to_ascii_lowercase()
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join("");

    let backwards_with_space: String = query
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ");

    // TODO: Use a better algorithm, don't generate everything.
    let list = algorithms::get_full_sorted_uniqued(selection, opldb);

    // Handle out-of-bounds requests.
    if start_row >= list.0.len() {
        return SearchRankingsResult { next_index: None };
    }

    // TODO: Use a better algorithm; this is really a MVP.
    for i in start_row..list.0.len() {
        let entry = opldb.get_entry(list.0[i]);
        let lifter = opldb.get_lifter(entry.lifter_id);

        // First, check if there's a match based on the username or IG.
        if !normalized_latin.is_empty()
            && (lifter.username.contains(&normalized_latin)
                || lifter.username.contains(&backwards)
                || lifter
                    .instagram
                    .as_ref()
                    .map_or(false, |ig| ig.contains(&normalized_latin)))
        {
            return SearchRankingsResult::index(i);
        }

        // Otherwise, check based on the writing system.
        let localized_name: Option<&String> = match system {
            WritingSystem::Cyrillic => lifter.cyrillic_name.as_ref(),
            WritingSystem::Greek => lifter.greek_name.as_ref(),
            WritingSystem::Japanese => lifter.japanese_name.as_ref(),
            WritingSystem::Korean => lifter.korean_name.as_ref(),
            WritingSystem::Latin => Some(&lifter.name),
        };

        if let Some(name) = localized_name {
            if name.contains(&query) || name.contains(&backwards_with_space) {
                return SearchRankingsResult::index(i);
            }
        }
    }

    SearchRankingsResult { next_index: None }
}
