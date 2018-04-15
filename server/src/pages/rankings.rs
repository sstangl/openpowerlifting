//! Logic for the display of the rankings page.

use serde_json;

use langpack::{self, Language};
use opldb;
use opldb::CachedFilter;

use pages::jsdata::JsEntryRow;

/// The context object passed to `templates/rankings.html.tera`.
#[derive(Serialize)]
pub struct Context<'db> {
    pub page_title: String,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: opldb::WeightUnits,

    pub data: String,
}

impl<'db> Context<'db> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        language: Language,
        langinfo: &'db langpack::LangInfo,
        units: opldb::WeightUnits,
    ) -> Context<'db> {
        let strings = langinfo.get_translations(language);
        let number_format = language.number_format();

        // Get a list of raw + wraps.
        let filter_raw = opldb.get_filter(CachedFilter::EquipmentRaw);
        let filter_wraps = opldb.get_filter(CachedFilter::EquipmentWraps);
        let raw_or_wraps = filter_raw.union(filter_wraps);

        let rankings = raw_or_wraps.sort_and_unique_by_wilks(&opldb);

        // Send over the top 100 by default.
        let top_100: Vec<JsEntryRow> = rankings.list[0..100]
            .into_iter()
            .zip(0..)
            .map(|(&n, i)| {
                JsEntryRow::from(
                    opldb,
                    strings,
                    number_format,
                    units,
                    opldb.get_entry(n),
                    i,
                )
            })
            .collect();

        Context {
            page_title: "Rankings".to_string(),
            language: language,
            strings: strings,
            units: units,

            /// FIXME: Handle failure.
            data: serde_json::to_string(&top_100).unwrap(),
        }
    }
}
