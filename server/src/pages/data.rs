//! Logic for the data page.

use langpack::{self, Language};

/// The context object passed to `templates/data.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: &'a str,
    pub language: Language,
    pub strings: &'a langpack::Translations,
}

impl<'a> Context<'a> {
    pub fn new(language: Language, langinfo: &'a langpack::LangInfo) -> Context<'a> {
        let strings = langinfo.get_translations(language);
        let page_title = &strings.header.data;

        Context {
            page_title: page_title,
            strings: strings,
            language: language,
        }
    }
}
