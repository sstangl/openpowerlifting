//! Logic for the display of the rankings page.

use serde_json;
use opltypes;

use langpack::{self, Language, Locale};
use opldb;

use pages::api_rankings::get_slice;
use pages::selection::{Selection, SortSelection};

/// The context object passed to `templates/rankings.html.tera`.
#[derive(Serialize)]
pub struct Context<'db, 'a> {
    pub page_title: String,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: opltypes::WeightUnits,

    /// The title of the points column, which changes based on SortSelection.
    pub points_column_title: &'db str,

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
        let slice = get_slice(&opldb, &locale, &selection, 0, 99);

        Some(Context {
            page_title: "Rankings".to_string(),
            language: locale.language,
            strings: locale.strings,
            units: locale.units,

            // This should mirror the logic in JsEntryRow::from().
            points_column_title: match selection.sort {
                SortSelection::BySquat
                | SortSelection::ByBench
                | SortSelection::ByDeadlift
                | SortSelection::ByTotal
                | SortSelection::ByWilks => &locale.strings.columns.wilks,
                SortSelection::ByMcCulloch => &locale.strings.columns.mcculloch,
                SortSelection::ByGlossbrenner => &locale.strings.columns.glossbrenner,
            },

            selection,
            data: serde_json::to_string(&slice).ok()?,
        })
    }
}
