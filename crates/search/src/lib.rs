//! Search engine for all of powerlifting.

use opldb::query::direct::RankingsQuery;
use opldb::{algorithms, OplDb};
use opltypes::*;

/// Searches the given rankings by lifter information.
///
/// Returns the index of the first match from `start_row`, or `None`.
pub fn search_rankings(
    db: &OplDb,
    rankings: &RankingsQuery,
    start_row: usize, // Inclusive.
    query: &str,
) -> Option<usize> {
    // FIXME: Hacky solution to "#" ,"'"" & "." being replaced by underscores
    // in the query string. The client code makes that replacement in order
    // to ensure that the URL is valid, since this is accessed via a GET parameter.
    // We could do something craftier, like base-64 encode it.
    let query = query.replace('_', "");

    let system = infer_writing_system(&query);

    // Convert the query string to a normalized form.
    // This tries to make it look like a username, since we're
    // just doing comparisons on the username.
    let normalized_latin: String = match Username::from_name(&query) {
        Ok(u) => u.into(),
        Err(_) => String::new(),
    };

    // Disallow bogus queries.
    if normalized_latin.is_empty() && system == WritingSystem::Latin {
        return None;
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
    let list = algorithms::full_sorted_uniqued(rankings, db);

    // Handle out-of-bounds requests.
    if start_row >= list.0.len() {
        return None;
    }

    // TODO: Use a better algorithm; this is really a MVP.
    for i in start_row..list.0.len() {
        let entry = db.entry(list.0[i]);
        let lifter = db.lifter(entry.lifter_id);

        // First, check if there's a match based on the username or IG.
        if !normalized_latin.is_empty()
            && (lifter.username.as_str().contains(&normalized_latin)
                || lifter.username.as_str().contains(&backwards)
                || lifter
                    .instagram
                    .as_ref()
                    .is_some_and(|ig| ig.contains(&normalized_latin)))
        {
            return Some(i);
        }

        // Otherwise, check based on the writing system.
        let localized_name: Option<&str> = match system {
            WritingSystem::CJK if lifter.japanese_name.is_some() => lifter.japanese_name.as_deref(),
            WritingSystem::CJK if lifter.korean_name.is_some() => lifter.korean_name.as_deref(),
            WritingSystem::CJK => lifter.chinese_name.as_deref(),
            WritingSystem::Cyrillic => lifter.cyrillic_name.as_deref(),
            WritingSystem::Greek => lifter.greek_name.as_deref(),
            WritingSystem::Japanese => lifter.japanese_name.as_deref(),
            WritingSystem::Korean => lifter.korean_name.as_deref(),
            WritingSystem::Latin => Some(&lifter.name),
        };

        if let Some(name) = localized_name {
            if name.contains(&query) || name.contains(&backwards_with_space) {
                return Some(i);
            }
        }
    }

    None
}
