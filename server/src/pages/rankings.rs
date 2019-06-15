//! Logic for the display of the rankings page.

use opltypes;
use serde_json;

use crate::langpack::{self, Language, Locale};
use crate::opldb;
use crate::pages::api_rankings::get_slice;
use crate::pages::selection::Selection;

/// The context object passed to `templates/rankings.html.tera`.
#[derive(Serialize)]
pub struct Context<'db, 'a> {
    /// Prefix used for constructing URLs. Needed because of distributions.
    ///
    /// Defaults to "/", but can be mutated by the Context owner.
    ///
    /// # Examples
    ///
    /// For OpenPowerlifting.org, the prefix is "/".
    /// For OpenIPF.org, the prefix is "/". Nginx rewrites to "/dist/openipf/".
    /// For local development of OpenIPF.org, the prefix is "/dist/openipf/".
    pub urlprefix: &'static str,

    pub page_title: String,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: opltypes::WeightUnits,
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
            urlprefix: "/",
            page_title: "Rankings".to_string(),
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            selection,
            data: serde_json::to_string(&slice).ok()?,
        })
    }
}
