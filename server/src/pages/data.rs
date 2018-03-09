//! Logic for the data page.

use opldb;
use langpack::{self, Language};

#[derive(Serialize)]
pub struct HeaderContext {
    pub num_entries: u32,
    pub num_meets: u32,
}

/// The context object passed to `templates/status.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: &'a str,
    pub header: HeaderContext,
    pub language: Language,
    pub strings: &'a langpack::Translations,
}

impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        language: Language,
        langinfo: &'a langpack::LangInfo,
    ) -> Context<'a> {
        let strings = langinfo.get_translations(language);
        let page_title = &strings.header.data;

        Context {
            page_title: page_title,
            header: HeaderContext {
                num_entries: opldb.get_entries().len() as u32,
                num_meets: opldb.get_meets().len() as u32,
            },
            strings: strings,
            language: language,
        }
    }
}
