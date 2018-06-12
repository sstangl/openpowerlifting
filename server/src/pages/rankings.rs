//! Logic for the display of the rankings page.

use serde_json;

use langpack::{self, Language, Locale};
use opldb;

use pages::jsdata::JsEntryRow;
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
    ) -> Context<'db, 'a> {
        // TODO: Don't generate the full list, just generate the top 100.
        let list = opldb
            .get_static_cache()
            .get_full_sorted_uniqued(selection, opldb);

        let top_100: Vec<JsEntryRow> = list.0[0..list.0.len().min(100)]
            .iter()
            .zip(0..)
            .map(|(&n, i)| JsEntryRow::from(opldb, locale, opldb.get_entry(n), i))
            .collect();

        // Send over the top 100 by default.
        Context {
            page_title: "Rankings".to_string(),
            language: locale.language,
            strings: locale.strings,
            units: locale.units,

            selection: selection,
            /// FIXME: Handle failure.
            data: serde_json::to_string(&top_100).unwrap(),
        }
    }
}
