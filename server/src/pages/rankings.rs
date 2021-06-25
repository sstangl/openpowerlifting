//! Logic for the display of the rankings page.

use langpack::Language;
use opldb::query::direct::*;
use opltypes::states::State;

use crate::pages::api_rankings::get_slice;

/// Flattened version of the RankingsQuery database object.
///
/// This is parsed by the JS code to determine the active widget status.
#[derive(Serialize)]
pub struct RankingsWidgets {
    pub equipment: EquipmentFilter,
    pub federation: FederationFilter,
    pub weightclasses: WeightClassFilter,
    pub sex: SexFilter,
    pub ageclass: AgeClassFilter,
    pub year: YearFilter,
    pub event: EventFilter,
    pub state: Option<State>,
    pub sort: OrderBy,
}

impl From<&RankingsQuery> for RankingsWidgets {
    fn from(q: &RankingsQuery) -> Self {
        Self {
            equipment: q.filter.equipment,
            federation: q.filter.federation,
            weightclasses: q.filter.weightclasses,
            sex: q.filter.sex,
            ageclass: q.filter.ageclass,
            year: q.filter.year,
            event: q.filter.event,
            state: q.filter.state,
            sort: q.order_by,
        }
    }
}

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

    pub page_title: &'a str,
    pub page_description: &'a str,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: opltypes::WeightUnits,
    pub selection: RankingsWidgets,
    pub default_selection: &'a RankingsQuery,
    pub initial_data: String,
}

impl<'db, 'a> Context<'db, 'a> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        locale: &'db langpack::Locale<'a>,
        selection: &'a RankingsQuery,
        defaults: &'a RankingsQuery,
        use_ipf_equipment: bool,
    ) -> Option<Context<'db, 'a>> {
        // Inline the top 100 to avoid another round-trip.
        let mut slice = get_slice(opldb, locale, selection, defaults, 0, 99);

        // If this is for the IPF, use different names for some equipment.
        if use_ipf_equipment {
            for row in &mut slice.rows {
                if row.equipment == locale.strings.equipment.raw {
                    row.equipment = &locale.strings.equipment.classic;
                }
                if row.equipment == locale.strings.equipment.single {
                    row.equipment = &locale.strings.equipment.equipped;
                }
            }
        }

        Some(Context {
            urlprefix: "/",
            page_title: &locale.strings.page_titles.rankings,
            page_description: &locale.strings.html_header.description,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            selection: RankingsWidgets::from(selection),
            default_selection: defaults,
            initial_data: serde_json::to_string(&slice).ok()?,
        })
    }
}
