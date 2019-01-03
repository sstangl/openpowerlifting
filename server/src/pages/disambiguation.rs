//! Logic for a page disambiguating multiple lifters.
//!
//! This happens for "/u/johndoe", when there exist "/u/johndoe1" and
//! "/u/johndoe2".

use opltypes::*;

use crate::langpack::{self, get_localized_name, Language, Locale};
use crate::opldb;
use crate::pages::lifter::MeetResultsRow;

/// The context object passed to `templates/disambiguation.tera`
#[derive(Serialize)]
pub struct Context<'db> {
    pub page_title: &'db str,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: WeightUnits,

    pub variants: Vec<LifterResults<'db>>,
}

#[derive(Serialize)]
pub struct LifterResults<'db> {
    pub lifter: &'db opldb::Lifter,
    pub localized_name: &'db str,
    pub lifter_sex: &'db str,
    pub meet_results: Vec<MeetResultsRow<'db>>,
}

impl<'db> Context<'db> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        locale: &'db Locale,
        username_base: &str,
        lifter_ids: &[u32],
    ) -> Context<'db> {
        // For each ID, create a LifterResults struct.
        let mut variants: Vec<LifterResults<'db>> = lifter_ids
            .iter()
            .map(|&lifter_id| {
                let lifter = opldb.get_lifter(lifter_id);
                let localized_name = get_localized_name(&lifter, locale.language);

                // Get a list of the entries for this lifter, oldest entries first.
                let mut entries = opldb.get_entries_for_lifter(lifter_id);
                entries.sort_unstable_by_key(|e| &opldb.get_meet(e.meet_id).date);

                let lifter_sex = locale.strings.translate_sex(entries[0].sex);

                // Display the meet results, most recent first.
                let meet_results = entries
                    .into_iter()
                    .map(|e| MeetResultsRow::from(opldb, locale, e))
                    .rev()
                    .collect();

                LifterResults {
                    lifter,
                    localized_name,
                    lifter_sex,
                    meet_results,
                }
            })
            .collect();

        // Sort the variants in order of ascending number.
        variants.sort_unstable_by_key(|v| {
            // Sort by disambiguation number.
            let (_, number) = v.lifter.username.split_at(username_base.len());
            number.parse::<u32>().unwrap_or(0)
        });

        Context {
            page_title: "Disambiguation",
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            variants,
        }
    }
}
