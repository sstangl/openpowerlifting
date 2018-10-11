//! Logic for the contact page.

use langpack::{self, Locale};
use opltypes;

/// The context object passed to `templates/contact.html.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: &'a str,
    pub language: langpack::Language,
    pub strings: &'a langpack::Translations,
    pub units: opltypes::WeightUnits,
}

impl<'a> Context<'a> {
    pub fn new(locale: &'a Locale) -> Context<'a> {
        Context {
            page_title: &locale.strings.header.contact,
            strings: locale.strings,
            language: locale.language,
            units: locale.units,
        }
    }
}
