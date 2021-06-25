//! Logic for a page disambiguating multiple lifters.
//!
//! This happens for "/u/johndoe", when there exist "/u/johndoe1" and
//! "/u/johndoe2".

use langpack::{get_localized_name, Language, Locale};
use opltypes::*;

use crate::pages::lifter::MeetResultsRow;
use crate::pages::meet::points_column_title;

/// The context object passed to `templates/disambiguation.tera`
#[derive(Serialize)]
pub struct Context<'db> {
    pub urlprefix: &'static str,
    pub page_title: &'db str,
    pub page_description: &'db str,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: WeightUnits,
    pub points_column_title: &'db str,

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
        points_system: PointsSystem,
        username_base: &str,
        lifter_ids: &[u32],
    ) -> Context<'db> {
        // For each ID, create a LifterResults struct.
        let mut variants: Vec<LifterResults<'db>> = lifter_ids
            .iter()
            .map(|&lifter_id| {
                let lifter = opldb.get_lifter(lifter_id);
                let localized_name = get_localized_name(lifter, locale.language);

                // Get a list of the entries for this lifter, oldest entries first.
                let mut entries = opldb.get_entries_for_lifter(lifter_id);
                entries.sort_unstable_by_key(|e| &opldb.get_meet(e.meet_id).date);

                let lifter_sex = locale.strings.translate_sex(entries[0].sex);

                // Display the meet results, most recent first.
                let meet_results = entries
                    .into_iter()
                    .map(|e| MeetResultsRow::from(opldb, locale, points_system, e))
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
            let (_, number) = v.lifter.username.as_str().split_at(username_base.len());
            number.parse::<u32>().unwrap_or(0)
        });

        Context {
            urlprefix: "/",
            page_title: "Disambiguation",
            page_description: &locale.strings.html_header.description,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            points_column_title: points_column_title(points_system, locale, points_system),
            variants,
        }
    }
}
