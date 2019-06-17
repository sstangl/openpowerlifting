//! Defines Rocket handlers for the OpenIPF distribution.
//!
//! On openpowerlifting.org, these handlers are mounted under /dist/openipf/.
//! The openipf.org site works by using the same server as openpowerlifting.org,
//! with Nginx rewriting URLs based on domain.

use opltypes::*;

use rocket::http::Cookies;
use rocket::request::Form;
use rocket::State;
use rocket_contrib::templates::Template;

use server::langpack::{Language, Locale};
use server::opldb::MetaFederation;
use server::pages;

use std::path::PathBuf;

use crate::common::*;

/// URL prefix used when accessing OpenIPF through OpenPowerlifting.org or
/// localhost.
pub const LOCAL_PREFIX: &'static str = "/dist/openipf/";

/// Default selections used in the OpenIPF rankings.
///
/// This information is also hardcoded in the rankings template.
fn default_openipf_selection() -> pages::selection::Selection {
    use pages::selection::*;
    Selection {
        equipment: EquipmentSelection::Raw,
        federation: FederationSelection::Meta(MetaFederation::IPFAndAffiliates),
        weightclasses: WeightClassSelection::AllClasses,
        sex: SexSelection::AllSexes,
        ageclass: AgeClassSelection::AllAges,
        year: YearSelection::AllYears,
        event: EventSelection::FullPower,
        sort: SortSelection::ByIPFPoints,
    }
}

/// Defines the default rankings used on the site homepage, suitable for the
/// IPF.
#[get("/?<lang>")]
pub fn index(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let selection = default_openipf_selection();
    let mut context = pages::rankings::Context::new(&opldb, &locale, &selection)?;
    context.urlprefix = LOCAL_PREFIX;
    Some(Template::render("openipf/rankings", &context))
}

/// Defines a Rankings sub-page.
///
/// The intention is to reuse as much backend code as possible with
/// OpenPowerlifting, and just swap out the frontend to be IPF-specific so it
/// looks like its own thing.
#[get("/rankings/<selections..>?<lang>")]
pub fn rankings(
    selections: PathBuf,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let default = default_openipf_selection();
    let selection = pages::selection::Selection::from_path(&selections, &default).ok()?;
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let mut context = pages::rankings::Context::new(&opldb, &locale, &selection)?;
    context.urlprefix = LOCAL_PREFIX;
    Some(Template::render("openipf/rankings", &context))
}

/// API endpoint for fetching a slice of rankings data as JSON.
#[get("/api/rankings/<selections..>?<query..>")]
pub fn rankings_api(
    selections: Option<PathBuf>,
    query: Form<RankingsApiQuery>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
) -> Option<JsonString> {
    let default = default_openipf_selection();
    let selection = match selections {
        None => default,
        Some(path) => pages::selection::Selection::from_path(&path, &default).ok()?,
    };

    let language = query.lang.parse::<Language>().ok()?;
    let units = query.units.parse::<WeightUnits>().ok()?;
    let locale = Locale::new(&langinfo, language, units);

    let slice = pages::api_rankings::get_slice(
        &opldb,
        &locale,
        &selection,
        query.start,
        query.end,
    );

    // TODO: Maybe we can use rocket_contrib::Json, but the lifetimes
    // of the values in `slice` outlive this function, which doesn't work.
    Some(JsonString(serde_json::to_string(&slice).ok()?))
}

#[get("/api/rankings?<query..>")]
pub fn default_rankings_api(
    query: Form<RankingsApiQuery>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
) -> Option<JsonString> {
    rankings_api(None, query, opldb, langinfo)
}

/// API endpoint for rankings search.
#[get("/api/search/rankings/<selections..>?<query..>")]
pub fn search_rankings_api<'db>(
    selections: Option<PathBuf>,
    query: Form<SearchRankingsApiQuery>,
    opldb: State<ManagedOplDb>,
) -> Option<JsonString> {
    let default = default_openipf_selection();
    let selection = match selections {
        None => default,
        Some(path) => pages::selection::Selection::from_path(&path, &default).ok()?,
    };

    let result =
        pages::api_search::search_rankings(&opldb, &selection, query.start, &query.q);

    Some(JsonString(serde_json::to_string(&result).ok()?))
}

#[get("/api/search/rankings?<query..>")]
pub fn default_search_rankings_api(
    query: Form<SearchRankingsApiQuery>,
    opldb: State<ManagedOplDb>,
) -> Option<JsonString> {
    search_rankings_api(None, query, opldb)
}
