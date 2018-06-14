//! Logic for the display of the rankings page.

use serde_json;

use langpack::{self, Language, Locale};
use opldb;

use pages::api_rankings::get_slice;
use pages::selection::Selection;

/// The context object passed to `templates/rankings.html.tera`.
#[derive(Serialize)]
pub struct Context<'db, 'a> {
    pub page_title: String,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: opldb::WeightUnits,

    pub selection: &'a Selection,
    pub data: String,
}

impl<'db, 'a> Context<'db, 'a> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        locale: &'db Locale,
        selection: &'a Selection,
    ) -> Option<Context<'db, 'a>> {
        // Inline the top 100 to avoid another round-trip.
        let slice = get_slice(&opldb, &locale, &selection, 0, 99)?;

        Some(Context {
            page_title: "Rankings".to_string(),
            language: locale.language,
            strings: locale.strings,
            units: locale.units,

            selection: selection,
            data: serde_json::to_string(&slice).ok()?,
        })
    }
}
