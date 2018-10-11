//! Logic for the data page.

use langpack::{self, Language, Locale};
use opltypes;

/// The context object passed to `templates/data.html.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: &'a str,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: opltypes::WeightUnits,
}

impl<'a> Context<'a> {
    pub fn new(locale: &'a Locale) -> Context<'a> {
        Context {
            page_title: &locale.strings.header.data,
            strings: locale.strings,
            language: locale.language,
            units: locale.units,
        }
    }
}
