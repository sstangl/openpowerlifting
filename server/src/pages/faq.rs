//! Logic for the faq page.

/// The context object passed to `templates/faq.html.tera`
#[derive(Serialize)]
pub struct Context {
    pub urlprefix: &'static str,
    pub page_title: &'static str,
    pub page_description: &'static str,
    pub language: langpack::Language,
    pub strings: &'static langpack::Translations,
    pub units: opltypes::WeightUnits,
}

impl Context {
    pub fn new(locale: &langpack::Locale) -> Context {
        Context {
            urlprefix: "/",
            page_title: locale.strings.header.faq,
            page_description: locale.strings.html_header.description,
            strings: locale.strings,
            language: locale.language,
            units: locale.units,
        }
    }
}
