//! Logic for the project status page.

use opltypes::*;
use strum::IntoEnumIterator;

use crate::langpack::{self, Language, Locale};
use crate::opldb;

/// The context object passed to `templates/status.html.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: &'a str,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: WeightUnits,
    pub fed_statuses: Vec<FederationStatus>,
    pub num_entries: u32,
    pub num_meets: u32,
    pub num_lifters: u32,
}

#[derive(Serialize)]
pub struct FederationStatus {
    pub fed: Federation,
    pub status: &'static str,
    pub meet_count: usize,
}

impl FederationStatus {
    fn new(fed: Federation) -> FederationStatus {
        FederationStatus {
            fed,
            status: "Incomplete",
            meet_count: 0,
        }
    }
}

impl<'a> Context<'a> {
    pub fn new(opldb: &'a opldb::OplDb, locale: &'a Locale) -> Context<'a> {
        let mut statuses: Vec<FederationStatus> =
            Federation::iter().map(FederationStatus::new).collect();

        for meet in opldb.get_meets() {
            let idx = meet.federation as usize;
            statuses[idx].meet_count += 1;
        }

        Context {
            page_title: &locale.strings.header.status,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            fed_statuses: statuses,
            num_entries: opldb.get_entries().len() as u32,
            num_meets: opldb.get_meets().len() as u32,
            num_lifters: opldb.get_lifters().len() as u32,
        }
    }
}
