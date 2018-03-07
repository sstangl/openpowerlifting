//! Logic for the project status page.

use opldb;
use opldb::fields;
use langpack::{self, Language};

#[derive(Serialize)]
pub struct HeaderContext {
    pub num_entries: u32,
    pub num_meets: u32,
}

/// The context object passed to `templates/status.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: String,
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

        Context {
            page_title: "Status".to_string(),
            header: HeaderContext {
                num_entries: opldb.get_entries().len() as u32,
                num_meets: opldb.get_meets().len() as u32,
            },
            language: language,
            strings: strings
        }
    }
}
