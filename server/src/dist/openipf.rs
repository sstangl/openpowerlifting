//! Defines Rocket handlers for the OpenIPF distribution.
//!
//! On openpowerlifting.org, these handlers are mounted under /dist/openipf/.
//! The openipf.org site works by using the same server as openpowerlifting.org,
//! with Nginx rewriting URLs based on domain.

use langpack::{Language, Locale};
use opldb::{self, Entry, MetaFederation};
use opltypes::*;

use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::State;
use rocket_dyn_templates::Template;

use server::pages;
use server::referring_path::ReferringPath;

use std::path::PathBuf;

use crate::common::*;
use crate::CsvFile;

/// URL prefix used when accessing OpenIPF through OpenPowerlifting.org or
/// localhost.
pub const LOCAL_PREFIX: &str = "/dist/openipf/";

/// Assigns the local prefix based on the Host HTTP header.
///
/// If served from openipf.org, we want it to pretend to be at the root,
/// since Nginx has a rewrite rule that always prepends /dist/openipf.
///
/// If served from elsewhere (localhost or openpowerlifting.org), we want
/// to prepend /dist/openipf/ to allow it to use the same common server.
fn local_prefix(host: &Host) -> &'static str {
    if host.served_from_openipf_org() {
        "/"
    } else {
        LOCAL_PREFIX
    }
}

/// Default selections used in the OpenIPF rankings.
///
/// This information is also hardcoded in the rankings template.
fn default_openipf_rankings_query() -> opldb::query::direct::RankingsQuery {
    use opldb::query::direct::*;
    RankingsQuery {
        filter: EntryFilter {
            equipment: EquipmentFilter::Raw,
            federation: FederationFilter::Meta(MetaFederation::IPFAndAffiliates),
            weightclasses: WeightClassFilter::AllClasses,
            sex: SexFilter::AllSexes,
            ageclass: AgeClassFilter::AllAges,
            year: YearFilter::AllYears,
            event: EventFilter::FullPower,
            state: None,
        },
        order_by: OrderBy::Goodlift,
    }
}

/// Defines the default rankings used on the site homepage, suitable for the
/// IPF.
#[get("/?<lang>")]
pub fn index(
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let locale = make_locale(lang, languages, cookies);
    let default = default_openipf_rankings_query();
    let mut cx = pages::rankings::Context::new(opldb, &locale, &default, &default, true)?;
    cx.urlprefix = local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/rankings", &cx),
        Device::Mobile => Template::render("openipf/mobile/rankings", &cx),
    })
}

/// Defines a Rankings sub-page.
///
/// The intention is to reuse as much backend code as possible with
/// OpenPowerlifting, and just swap out the frontend to be IPF-specific so it
/// looks like its own thing.
#[get("/rankings/<selections..>?<lang>")]
pub fn rankings(
    selections: PathBuf,
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let default = default_openipf_rankings_query();
    let selection =
        opldb::query::direct::RankingsQuery::from_url_path(&selections, &default).ok()?;
    let locale = make_locale(lang, languages, cookies);
    let mut cx = pages::rankings::Context::new(opldb, &locale, &selection, &default, true)?;
    cx.urlprefix = local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/rankings", &cx),
        Device::Mobile => Template::render("openipf/mobile/rankings", &cx),
    })
}

/// API endpoint for fetching a slice of rankings data as JSON.
#[get("/api/rankings/<selections..>?<query..>")]
pub fn rankings_api(
    selections: Option<PathBuf>,
    query: RankingsApiQuery,
    opldb: &State<ManagedOplDb>,
) -> Option<JsonString> {
    let default = default_openipf_rankings_query();
    let selection = match selections {
        None => default,
        Some(path) => opldb::query::direct::RankingsQuery::from_url_path(&path, &default).ok()?,
    };

    let language = query.lang.parse::<Language>().ok()?;
    let units = query.units.parse::<WeightUnits>().ok()?;
    let locale = Locale::new(language, units);

    let mut slice = pages::api_rankings::query_slice(
        opldb,
        &locale,
        &selection,
        &default,
        query.start,
        query.end,
    );

    for row in &mut slice.rows {
        if row.equipment == locale.strings.equipment.raw {
            row.equipment = locale.strings.equipment.classic;
        }
        if row.equipment == locale.strings.equipment.single {
            row.equipment = locale.strings.equipment.equipped;
        }
    }

    // TODO: Maybe we can use rocket_contrib::Json, but the lifetimes
    // of the values in `slice` outlive this function, which doesn't work.
    Some(JsonString(serde_json::to_string(&slice).ok()?))
}

#[get("/api/rankings?<query..>")]
pub fn default_rankings_api(
    query: RankingsApiQuery,
    opldb: &State<ManagedOplDb>,
) -> Option<JsonString> {
    rankings_api(None, query, opldb)
}

/// API endpoint for rankings search.
#[get("/api/search/rankings/<selections..>?<query..>")]
pub fn search_rankings_api(
    selections: Option<PathBuf>,
    query: SearchRankingsApiQuery,
    opldb: &State<ManagedOplDb>,
) -> Option<JsonString> {
    let default = default_openipf_rankings_query();
    let selection = match selections {
        None => default,
        Some(path) => opldb::query::direct::RankingsQuery::from_url_path(&path, &default).ok()?,
    };

    let result = pages::api_search::search_rankings(opldb, &selection, query.start, &query.q);

    Some(JsonString(serde_json::to_string(&result).ok()?))
}

#[get("/api/search/rankings?<query..>")]
pub fn default_search_rankings_api(
    query: SearchRankingsApiQuery,
    opldb: &State<ManagedOplDb>,
) -> Option<JsonString> {
    search_rankings_api(None, query, opldb)
}

#[get("/records/<selections..>?<lang>")]
pub fn records(
    selections: Option<PathBuf>,
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let default_rankings = default_openipf_rankings_query();
    let default = pages::records::RecordsQuery {
        equipment: default_rankings.filter.equipment,
        federation: default_rankings.filter.federation,
        sex: opldb::query::direct::SexFilter::Men,
        classkind: pages::records::ClassKind::IPF,
        ageclass: default_rankings.filter.ageclass,
        year: default_rankings.filter.year,
        state: None,
    };

    let selection = if let Some(sel) = selections {
        pages::records::RecordsQuery::from_path(&sel, &default).ok()?
    } else {
        default
    };
    let locale = make_locale(lang, languages, cookies);
    let mut cx = pages::records::Context::new(
        opldb,
        &locale,
        &selection,
        &default_openipf_rankings_query(),
    );
    cx.urlprefix = local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/records", &cx),
        Device::Mobile => Template::render("openipf/mobile/records", &cx),
    })
}

#[get("/records?<lang>")]
pub fn records_default(
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    records(None, lang, opldb, languages, host, device, cookies)
}

/// Used to show only IPF-sanctioned meets.
fn ipf_only_filter(opldb: &opldb::OplDb, e: &Entry) -> bool {
    let meet = opldb.meet(e.meet_id);
    meet.federation.sanctioning_body(meet.date) == Some(Federation::IPF)
}

#[get("/u/<username>?<lang>")]
pub fn lifter(
    username: &str,
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Result<Template, Redirect>> {
    let locale = make_locale(lang, languages, cookies);

    // Disambiguations end with a digit.
    // Some lifters may have failed to be merged with their disambiguated username.
    // Therefore, for usernames without a digit, it cannot be assumed that they are
    // *not* a disambiguation.
    let is_definitely_disambiguation: bool = username
        .chars()
        .last()
        .map_or(false, |c| c.is_ascii_digit());

    let lifter_ids: Vec<u32> = if is_definitely_disambiguation {
        if let Some(id) = opldb.lifter_id(username) {
            vec![id]
        } else {
            vec![]
        }
    } else {
        opldb.lifters_under_username_base(username)
    };

    match lifter_ids.len() {
        // If no LifterID was found, maybe the name just needs to be lowercased.
        0 => {
            let lowercase = username.to_ascii_lowercase();
            let _guard = opldb.lifter_id(&lowercase)?;
            Some(Err(Redirect::permanent(format!("/u/{lowercase}"))))
        }

        // If a specific lifter was referenced, return the lifter's unique page.
        1 => {
            let mut cx = pages::lifter::Context::new(
                opldb,
                &locale,
                lifter_ids[0],
                PointsSystem::from(default_openipf_rankings_query().order_by),
                Some(ipf_only_filter),
            );
            cx.urlprefix = local_prefix(&host);

            // Change the equipment terminology to be IPF-specific.
            for best in &mut cx.bests {
                if best.equipment == locale.strings.equipment.raw {
                    best.equipment = locale.strings.equipment.classic;
                }
                if best.equipment == locale.strings.equipment.single {
                    best.equipment = locale.strings.equipment.equipped;
                }
            }
            for result in &mut cx.meet_results {
                if result.equipment == locale.strings.equipment.raw {
                    result.equipment = locale.strings.equipment.classic;
                }
                if result.equipment == locale.strings.equipment.single {
                    result.equipment = locale.strings.equipment.equipped;
                }
            }

            Some(Ok(match device {
                Device::Desktop => Template::render("openipf/desktop/lifter", cx),
                Device::Mobile => Template::render("openipf/mobile/lifter", cx),
            }))
        }

        // If multiple lifters were referenced, return a disambiguation page.
        _ => {
            let mut cx = pages::disambiguation::Context::new(
                opldb,
                &locale,
                PointsSystem::from(default_openipf_rankings_query().order_by),
                username,
                &lifter_ids,
            );
            cx.urlprefix = local_prefix(&host);

            Some(Ok(match device {
                Device::Desktop => Template::render("openipf/desktop/disambiguation", cx),
                Device::Mobile => Template::render("openipf/mobile/disambiguation", cx),
            }))
        }
    }
}

#[get("/api/liftercsv/<username>")]
pub fn lifter_csv(username: &str, opldb: &State<ManagedOplDb>) -> Option<CsvFile> {
    let lifter_id = opldb.lifter_id(username)?;
    let content = pages::api_liftercsv::export_csv(opldb, lifter_id, Some(ipf_only_filter)).ok()?;
    let filename = format!("{username}.csv");
    Some(CsvFile { filename, content })
}

/// Exports single-meet data as a CSV file.
#[get("/api/meetcsv/<meetpath..>")]
pub fn meet_csv(meetpath: PathBuf, opldb: &State<ManagedOplDb>) -> Option<CsvFile> {
    let meet_path_str = meetpath.to_str()?;
    let meet_id = opldb.meet_id(meet_path_str)?;
    let content = pages::api_meetcsv::export_csv(opldb, meet_id, Some(ipf_only_filter)).ok()?;
    let filename = format!("{meet_path_str}.csv");
    Some(CsvFile { filename, content })
}

#[get("/mlist/<mselections..>?<lang>")]
pub fn meetlist(
    mselections: Option<PathBuf>,
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let openipf_defaults = default_openipf_rankings_query();
    let defaults = pages::meetlist::MeetListQuery {
        federation: openipf_defaults.filter.federation,
        year: openipf_defaults.filter.year,
    };

    let mselection = match mselections {
        None => defaults,
        Some(p) => pages::meetlist::MeetListQuery::from_path(&p, defaults).ok()?,
    };
    let locale = make_locale(lang, languages, cookies);
    let mut cx = pages::meetlist::Context::new(opldb, &locale, &mselection);
    cx.urlprefix = local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/meetlist", &cx),
        Device::Mobile => Template::render("openipf/mobile/meetlist", &cx),
    })
}

#[get("/mlist?<lang>")]
pub fn meetlist_default(
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    meetlist(None, lang, opldb, languages, host, device, cookies)
}

#[get("/m/<meetpath..>?<lang>")]
pub fn meet(
    meetpath: PathBuf,
    lang: Option<&str>,
    referring_path: Option<ReferringPath>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let mut meetpath_str: &str = meetpath.to_str()?;
    let default_sort = pages::meet::MeetSortSelection::ByDivision;
    let mut sort = default_sort;

    // The meetpath may contain an optional sorting directive.
    // If present, detect and remove that component from the path.
    let component = meetpath.as_path().file_name()?.to_str()?;
    if let Ok(sortselection) = component.parse::<pages::meet::MeetSortSelection>() {
        sort = sortselection;
        meetpath_str = meetpath.as_path().parent()?.to_str()?;
    }

    let referring_username =
        referring_path.and_then(|s| s.strip_prefix("/u/").map(ToString::to_string));

    let meet_id = opldb.meet_id(meetpath_str)?;
    let locale = make_locale(lang, languages, cookies);
    let use_ipf_equipment = true;
    let mut cx = pages::meet::Context::new(
        opldb,
        &locale,
        meet_id,
        sort,
        default_sort,
        use_ipf_equipment,
        referring_username,
    );
    cx.urlprefix = local_prefix(&host);

    // Change the equipment terminology to be IPF-specific.
    for table in &mut cx.tables {
        for row in &mut table.rows {
            if row.equipment == locale.strings.equipment.raw {
                row.equipment = locale.strings.equipment.classic;
            }
            if row.equipment == locale.strings.equipment.single {
                row.equipment = locale.strings.equipment.equipped;
            }
        }
    }

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/meet", &cx),
        Device::Mobile => Template::render("openipf/mobile/meet", &cx),
    })
}

/// Used to show only IPF-sanctioned federations.
fn ipf_fed_filter(fed: Federation) -> bool {
    // Using a maximum date causes the sanctioning_body() logic to return the most
    // current sanctioning information.
    let latest = Date::from_parts(9999, 01, 01);
    fed.sanctioning_body(latest) == Some(Federation::IPF)
}

#[get("/status?<lang>")]
pub fn status(
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let locale = make_locale(lang, languages, cookies);
    let mut cx = pages::status::Context::new(opldb, &locale, Some(ipf_fed_filter));
    cx.urlprefix = local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/status", &cx),
        Device::Mobile => Template::render("openipf/mobile/status", &cx),
    })
}

#[get("/faq?<lang>")]
pub fn faq(
    lang: Option<&str>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let locale = make_locale(lang, languages, cookies);
    let mut cx = pages::faq::Context::new(&locale);
    cx.urlprefix = local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/faq", &cx),
        Device::Mobile => Template::render("openipf/mobile/faq", &cx),
    })
}

#[get("/contact?<lang>")]
pub fn contact(
    lang: Option<&str>,
    languages: AcceptLanguage,
    host: Host,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let locale = make_locale(lang, languages, cookies);
    let instagram_dob_email_template = get_instagram_dob_email_template();
    let name_correction_email_template = get_name_correction_email_template();

    let mut cx = pages::contact::Context::new(
        &locale,
        instagram_dob_email_template,
        name_correction_email_template,
    );
    cx.urlprefix = local_prefix(&host);

    Some(match device {
        Device::Desktop => Template::render("openipf/desktop/contact", &cx),
        Device::Mobile => Template::render("openipf/mobile/contact", &cx),
    })
}
