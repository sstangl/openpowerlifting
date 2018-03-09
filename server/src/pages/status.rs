//! Logic for the project status page.

use opldb;
use opldb::fields::Federation;
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
    pub fed_statuses: Vec<FederationStatus<'a>>,
    pub num_entries: u32,
    pub num_meets: u32,
}

#[derive(Serialize)]
pub struct FederationStatus<'a> {
    pub fed: Federation,
    pub status: &'a str,
    pub meet_count: usize,
}

impl<'a> FederationStatus<'a> {
    fn from(fed: Federation, status: &'a str, meet_count: usize) -> FederationStatus<'a> {
        FederationStatus {
            fed: fed,
            status: status,
            meet_count: meet_count,
        }
    }
}

impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        language: Language,
        langinfo: &'a langpack::LangInfo,
    ) -> Context<'a> {
        let strings = langinfo.get_translations(language);
        let page_title = &strings.header.status;
        let mut fed_statuses: Vec<FederationStatus> = vec![];

        for federation in Federation::iter_variants() {
            let fed_status = "Incomplete";
            // TODO: Make this more efficient
            let fed_meet_count = opldb
                .get_meets()
                .iter()
                .filter(|m| m.federation == federation)
                .count();
            let fed_status = FederationStatus::from(federation, fed_status, fed_meet_count);
            fed_statuses.push(fed_status);
        }

        Context {
            page_title: page_title,
            header: HeaderContext {
                num_entries: opldb.get_entries().len() as u32,
                num_meets: opldb.get_meets().len() as u32,
            },
            strings: strings,
            language: language,
            fed_statuses: fed_statuses,
            num_entries: opldb.get_entries().len() as u32,
            num_meets: opldb.get_meets().len() as u32,
        }
    }
}
