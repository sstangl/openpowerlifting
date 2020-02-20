//! Logic for the display of the rankings page.

use opltypes;
use serde_json;

use crate::langpack::{self, Language};
use crate::opldb;
use crate::pages::api_rankings::get_slice;
use crate::pages::selection::Selection;

/// The context object passed to `templates/rankings.html.tera`.
#[derive(Serialize)]
pub struct Context<'db, 'a> {
    /// Prefix used for constructing URLs. Needed because of distributions.
    ///
    /// Defaults to "/", but can be mutated by the Context owner.
    ///
    /// # Examples
    ///
    /// For OpenPowerlifting.org, the prefix is "/".
    /// For OpenIPF.org, the prefix is "/". Nginx rewrites to "/dist/openipf/".
    /// For local development of OpenIPF.org, the prefix is "/dist/openipf/".
    pub urlprefix: &'static str,

    pub page_title: String,
    pub page_description: &'a str,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: opltypes::WeightUnits,
    pub selection: &'a Selection,
    pub default_selection: &'a Selection,
    pub data: String,
}

impl<'db, 'a> Context<'db, 'a> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        locale: &'db langpack::Locale<'a>,
        selection: &'a Selection,
        defaults: &'a Selection,
        use_ipf_equipment: bool,
    ) -> Option<Context<'db, 'a>> {
        // Inline the top 100 to avoid another round-trip.
        let mut slice = get_slice(&opldb, &locale, &selection, &defaults, 0, 99);

        // If this is for the IPF, use different names for some equipment.
        if use_ipf_equipment {
            for row in &mut slice.rows {
                if row.equipment == &locale.strings.equipment.raw {
                    row.equipment = &locale.strings.equipment.classic;
                }
                if row.equipment == &locale.strings.equipment.single {
                    row.equipment = &locale.strings.equipment.equipped;
                }
            }
        }

        Some(Context {
            urlprefix: "/",
            page_title: "Rankings".to_string(),
            page_description: &locale.strings.html_header.description,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            selection,
            default_selection: defaults,
            data: serde_json::to_string(&slice).ok()?,
        })
    }
}
