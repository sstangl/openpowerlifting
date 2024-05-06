//! The OpenPowerlifting server!

// Suppress clippy warnings for date literals.
#![allow(clippy::inconsistent_digit_grouping)]
#![allow(clippy::zero_prefixed_literal)]
// Allow Rocket endpoints with a lot of arguments.
#![allow(clippy::too_many_arguments)]

use opltypes::{Federation, WeightUnits};

#[macro_use]
extern crate rocket;

// Distributions, like OpenIPF.
mod dist;

// Shared Rocket code between the main server and distributions.
mod common;
use common::*;

#[cfg(test)]
mod tests;

use langpack::{Language, Locale};
use opltypes::Username;

use rocket::fs::NamedFile;
use rocket::http::{ContentType, CookieJar, Status};
use rocket::request::Request;
use rocket::response::{Redirect, Responder, Response};
use rocket::serde::json::Json;
use rocket::State;
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;
use server::referring_path::ReferringPath;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

use server::pages;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Kubernetes-style readiness check handler.
///
/// A machine that returns `200 OK` signifies that it is ready to take on more traffic.
#[get("/readyz")]
async fn readyz() -> Status {
    Status::Ok
}

/// A file served from /static.
enum StaticFile {
    /// PathBuf is the path to the non-gz version of the file.
    Gzipped(PathBuf, File),
    Plain(NamedFile),
}

impl<'r> Responder<'r, 'static> for StaticFile {
    fn respond_to(self, req: &'r Request) -> Result<Response<'static>, Status> {
        let mut response = match self {
            StaticFile::Gzipped(p, f) => {
                let mut r = f.respond_to(req)?;
                r.set_raw_header("Content-Encoding", "gzip");
                if let Some(ext) = p.extension() {
                    if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy()) {
                        r.set_header(ct);
                    }
                }
                r
            }
            StaticFile::Plain(f) => f.respond_to(req)?,
        };
        // Set to 1 year -- effectively forever.
        response.set_raw_header("Cache-Control", "public, max-age=31556926");
        Ok(response)
    }
}

#[get("/static/<file..>")]
async fn statics(file: PathBuf, encoding: AcceptEncoding) -> Option<StaticFile> {
    let staticdir = env::var("STATICDIR").ok()?;
    let filepath = Path::new(&staticdir).join(&file);

    // Prefer returning a compressed variant (same filename plus ".gz").
    if encoding.supports_gzip() {
        let gzfilename = format!("{}.gz", file.file_name()?.to_str()?);
        let gzfilepath = filepath.with_file_name(gzfilename);
        if let Ok(gzfile) = File::open(gzfilepath) {
            return Some(StaticFile::Gzipped(filepath, gzfile));
        }
    }

    let namedfile = NamedFile::open(filepath).await.ok()?;
    Some(StaticFile::Plain(namedfile))
}

/// Actually store the favicon in static/images/,
/// but allow serving from the root.
#[get("/favicon.ico")]
async fn root_favicon(encoding: AcceptEncoding) -> Option<StaticFile> {
    statics(PathBuf::from("images/favicon.ico"), encoding).await
}

#[get("/apple-touch-icon.png")]
async fn root_apple_touch_icon(encoding: AcceptEncoding) -> Option<StaticFile> {
    statics(PathBuf::from("images/apple-touch-icon.png"), encoding).await
}

#[get("/rankings/<selections..>?<lang>")]
fn rankings(
    selections: PathBuf,
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let defaults = opldb::query::direct::RankingsQuery::default();
    let selection =
        opldb::query::direct::RankingsQuery::from_url_path(&selections, &defaults).ok()?;
    let locale = make_locale(lang, languages, cookies);
    let cx = pages::rankings::Context::new(opldb, &locale, &selection, &defaults, false)?;

    Some(match device {
        Device::Desktop => Template::render("openpowerlifting/desktop/rankings", &cx),
        Device::Mobile => Template::render("openpowerlifting/mobile/rankings", &cx),
    })
}

#[get("/rankings")]
async fn rankings_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/records/<selections..>?<lang>")]
fn records(
    selections: Option<PathBuf>,
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let default = pages::records::RecordsQuery::default();
    let selection = if let Some(sel) = selections {
        pages::records::RecordsQuery::from_path(&sel, &default).ok()?
    } else {
        default
    };
    let locale = make_locale(lang, languages, cookies);
    let context = pages::records::Context::new(
        opldb,
        &locale,
        &selection,
        &opldb::query::direct::RankingsQuery::default(),
    );

    Some(match device {
        Device::Desktop => Template::render("openpowerlifting/desktop/records", &context),
        Device::Mobile => Template::render("openpowerlifting/mobile/records", &context),
    })
}

#[get("/records?<lang>")]
fn records_default(
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    records(None, lang, opldb, languages, device, cookies)
}

#[get("/u/<username>?<lang>")]
fn lifter(
    username: &str,
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
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
        opldb.lifters_under_username(username)
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
            let cx = pages::lifter::Context::new(
                opldb,
                &locale,
                lifter_ids[0],
                opltypes::PointsSystem::from(
                    opldb::query::direct::RankingsQuery::default().order_by,
                ),
                None,
            );
            Some(Ok(match device {
                Device::Desktop => Template::render("openpowerlifting/desktop/lifter", cx),
                Device::Mobile => Template::render("openpowerlifting/mobile/lifter", cx),
            }))
        }

        // If multiple lifters were referenced, return a disambiguation page.
        _ => {
            let cx = pages::disambiguation::Context::new(
                opldb,
                &locale,
                opltypes::PointsSystem::from(
                    opldb::query::direct::RankingsQuery::default().order_by,
                ),
                username,
                &lifter_ids,
            );
            Some(Ok(match device {
                Device::Desktop => Template::render("openpowerlifting/desktop/disambiguation", cx),
                Device::Mobile => Template::render("openpowerlifting/mobile/disambiguation", cx),
            }))
        }
    }
}

/// Wrapper for a CSV file as a String, to give it a Responder impl.
pub struct CsvFile {
    pub filename: String,
    pub content: String,
}

impl<'r> Responder<'r, 'static> for CsvFile {
    fn respond_to(self, req: &'r Request) -> Result<Response<'static>, Status> {
        let mut r = self.content.respond_to(req)?;
        r.set_header(ContentType::CSV);

        // The filename is controlled by the "Content-Disposition" header.
        let disp = format!(r#"attachment; filename="{}""#, self.filename);
        r.set_raw_header("Content-Disposition", disp);
        Ok(r)
    }
}

/// Exports single-lifter data as a CSV file.
#[get("/api/liftercsv/<username>")]
fn lifter_csv(username: &str, opldb: &State<ManagedOplDb>) -> Option<CsvFile> {
    let lifter_id = opldb.lifter_id(username)?;
    let content = pages::api_liftercsv::export_csv(opldb, lifter_id, None).ok()?;
    let filename = format!("{username}.csv");
    Some(CsvFile { filename, content })
}

/// Exports single-meet data as a CSV file.
#[get("/api/meetcsv/<meetpath..>")]
fn meet_csv(meetpath: PathBuf, opldb: &State<ManagedOplDb>) -> Option<CsvFile> {
    let meet_path_str = meetpath.to_str()?;
    let meet_id = opldb.meet_id(meet_path_str)?;
    let content = pages::api_meetcsv::export_csv(opldb, meet_id, None).ok()?;
    let filename = format!("{meet_path_str}.csv");
    Some(CsvFile { filename, content })
}

#[get("/mlist/<mselections..>?<lang>")]
fn meetlist(
    mselections: Option<PathBuf>,
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let defaults = pages::meetlist::MeetListQuery::default();
    let mselection = match mselections {
        None => defaults,
        Some(p) => pages::meetlist::MeetListQuery::from_path(&p, defaults).ok()?,
    };
    let locale = make_locale(lang, languages, cookies);
    let cx = pages::meetlist::Context::new(opldb, &locale, &mselection);

    Some(match device {
        Device::Desktop => Template::render("openpowerlifting/desktop/meetlist", &cx),
        Device::Mobile => Template::render("openpowerlifting/mobile/meetlist", &cx),
    })
}

#[get("/mlist?<lang>")]
fn meetlist_default(
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    meetlist(None, lang, opldb, languages, device, cookies)
}

#[get("/m/<meetpath..>?<lang>")]
fn meet(
    meetpath: PathBuf,
    lang: Option<&str>,
    referring_path: Option<ReferringPath>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let mut meetpath_str: &str = meetpath.to_str()?;
    let default_sort = pages::meet::MeetSortSelection::ByFederationDefault;
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
    let use_ipf_equipment = false;
    let context = pages::meet::Context::new(
        opldb,
        &locale,
        meet_id,
        sort,
        default_sort,
        use_ipf_equipment,
        referring_username,
    );

    Some(match device {
        Device::Desktop => Template::render("openpowerlifting/desktop/meet", &context),
        Device::Mobile => Template::render("openpowerlifting/mobile/meet", &context),
    })
}

#[get("/status?<lang>")]
fn status(
    lang: Option<&str>,
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let locale = make_locale(lang, languages, cookies);
    let context = pages::status::Context::new(opldb, &locale, None);

    Some(match device {
        Device::Desktop => Template::render("openpowerlifting/desktop/status", &context),
        Device::Mobile => Template::render("openpowerlifting/mobile/status", &context),
    })
}

#[get("/faq?<lang>")]
fn faq(
    lang: Option<&str>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let locale = make_locale(lang, languages, cookies);
    let context = pages::faq::Context::new(&locale);

    Some(match device {
        Device::Desktop => Template::render("openpowerlifting/desktop/faq", &context),
        Device::Mobile => Template::render("openpowerlifting/mobile/faq", &context),
    })
}

#[get("/contact?<lang>")]
fn contact(
    lang: Option<&str>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<Template> {
    let locale = make_locale(lang, languages, cookies);
    let instagram_dob_email_template = common::get_instagram_dob_email_template();
    let name_correction_email_template = common::get_name_correction_email_template();

    let context = pages::contact::Context::new(
        &locale,
        instagram_dob_email_template,
        name_correction_email_template,
    );

    Some(match device {
        Device::Desktop => Template::render("openpowerlifting/desktop/contact", &context),
        Device::Mobile => Template::render("openpowerlifting/mobile/contact", &context),
    })
}

#[derive(Responder)]
#[allow(clippy::large_enum_variant)] // The Redirect type is 339 bytes; Template is zero.
enum IndexReturn {
    Redirect(Redirect),
    Template(Template),
}

#[get("/?<lang>&<fed>")]
fn index(
    lang: Option<&str>,
    fed: Option<&str>, // For handling old-style URLs.
    opldb: &State<ManagedOplDb>,
    languages: AcceptLanguage,
    device: Device,
    cookies: &CookieJar<'_>,
) -> Option<IndexReturn> {
    // Handle old-style URLs. Hopefully we can remove this code one day.
    if let Some(fedstr) = fed {
        let fed = fedstr.parse::<Federation>().ok()?;
        let target = format!("/rankings/{}", fed.to_string().to_ascii_lowercase());
        return Some(IndexReturn::Redirect(Redirect::permanent(target)));
    }

    // Otherwise, render the main rankings template.
    let defaults = opldb::query::direct::RankingsQuery::default();
    let locale = make_locale(lang, languages, cookies);
    let cx = pages::rankings::Context::new(opldb, &locale, &defaults, &defaults, false);

    Some(IndexReturn::Template(match device {
        Device::Desktop => Template::render("openpowerlifting/desktop/rankings", &cx),
        Device::Mobile => Template::render("openpowerlifting/mobile/rankings", &cx),
    }))
}

/// API endpoint for fetching a slice of rankings data as JSON.
#[get("/api/rankings/<selections..>?<query..>")]
fn rankings_api(
    selections: Option<PathBuf>,
    query: RankingsApiQuery,
    opldb: &State<ManagedOplDb>,
) -> Option<JsonString> {
    let defaults = opldb::query::direct::RankingsQuery::default();
    let selection = match selections {
        None => defaults,
        Some(path) => opldb::query::direct::RankingsQuery::from_url_path(&path, &defaults).ok()?,
    };

    let language = query.lang.parse::<Language>().ok()?;
    let units = query.units.parse::<WeightUnits>().ok()?;
    let locale = Locale::new(language, units);

    let slice = pages::api_rankings::query_slice(
        opldb,
        &locale,
        &selection,
        &defaults,
        query.start,
        query.end,
    );

    // TODO: Maybe we can use rocket_contrib::Json, but the lifetimes
    // of the values in `slice` outlive this function, which doesn't work.
    Some(JsonString(serde_json::to_string(&slice).ok()?))
}

#[get("/api/rankings?<query..>")]
fn default_rankings_api(
    query: RankingsApiQuery,
    opldb: &State<ManagedOplDb>,
) -> Option<JsonString> {
    rankings_api(None, query, opldb)
}

/// API endpoint for rankings search.
#[get("/api/search/rankings/<selections..>?<query..>")]
fn search_rankings_api(
    selections: Option<PathBuf>,
    query: SearchRankingsApiQuery,
    opldb: &State<ManagedOplDb>,
) -> Option<JsonString> {
    let default = opldb::query::direct::RankingsQuery::default();
    let selection = match selections {
        None => default,
        Some(path) => opldb::query::direct::RankingsQuery::from_url_path(&path, &default).ok()?,
    };

    let result = pages::api_search::search_rankings(opldb, &selection, query.start, &query.q);

    Some(JsonString(serde_json::to_string(&result).ok()?))
}

#[get("/api/search/rankings?<query..>")]
fn default_search_rankings_api(
    query: SearchRankingsApiQuery,
    opldb: &State<ManagedOplDb>,
) -> Option<JsonString> {
    search_rankings_api(None, query, opldb)
}

// Renders the development environment.
#[get("/")]
fn dev_main() -> Template {
    let dummy: HashMap<String, String> = HashMap::new();
    Template::render("dev/checker", dummy)
}

/// Handles POST requests for getting data checked.
#[post("/checker", data = "<input>")]
fn dev_checker_post(
    opldb: &State<ManagedOplDb>,
    input: Json<pages::checker::CheckerInput>,
) -> Option<JsonString> {
    let output = pages::checker::check(opldb, &input);
    Some(JsonString(serde_json::to_string(&output).ok()?))
}

#[get("/lifters.html?<q>")]
fn old_lifters(opldb: &State<ManagedOplDb>, q: &str) -> Option<Redirect> {
    let username = Username::from_name(q).ok()?;
    opldb.lifter_id(username.as_str())?; // Ensure username exists.
    Some(Redirect::permanent(format!("/u/{username}")))
}

#[get("/meetlist.html")]
fn old_meetlist() -> Redirect {
    Redirect::permanent("/mlist")
}

#[get("/meet.html?<m>")]
fn old_meet(opldb: &State<ManagedOplDb>, m: &str) -> Option<Redirect> {
    let id = opldb.meet_id(m)?;
    let pathstr = &opldb.meet(id).path;
    Some(Redirect::permanent(format!("/m/{pathstr}")))
}

#[get("/index.html")]
async fn old_index() -> Redirect {
    Redirect::permanent("/")
}

#[get("/faq.html")]
async fn old_faq() -> Redirect {
    Redirect::permanent("/faq")
}

#[get("/contact.html")]
async fn old_contact() -> Redirect {
    Redirect::permanent("/contact")
}

#[get("/robots.txt")]
async fn robots_txt() -> &'static str {
    // Allow robots full site access except for JSON endpoints.
    r#"User-agent: *
Disallow: /api/
Disallow: /dev/

# Disallow bots from marketing and SEO companies.
User-agent: AhrefsBot
Disallow: /

User-agent: DataForSeoBot
Disallow: /

User-agent: dotbot
Disallow: /

User-agent: MJ12bot
Disallow: /

User-agent: SemrushBot
Disallow: /

User-agent: SemrushBot-SA
Disallow: /

User-agent: BLEXBot
Disallow: /

# Disallow bots from LLM-training companies.
User-agent: GPTBot
Disallow: /

User-agent: Google-Extended
Disallow: /

User-agent: FacebookBot
Disallow: /

User-agent: cohere-ai
Disallow: /

User-agent: PerplexityBot
Disallow: /

User-agent: anthropic-ai
Disallow: /

User-agent: ClaudeBot
Disallow: /
"#
}

#[catch(404)]
async fn not_found() -> &'static str {
    "404"
}

#[catch(500)]
async fn internal_error() -> &'static str {
    "500"
}

fn rocket(opldb: ManagedOplDb) -> Rocket<Build> {
    rocket::build()
        .manage(opldb)
        .mount(
            "/",
            routes![
                readyz,
                index,
                rankings,
                rankings_redirect,
                records,
                records_default,
                lifter,
                lifter_csv,
                meetlist,
                meetlist_default,
                meet,
                meet_csv,
                statics,
                root_favicon,
                root_apple_touch_icon,
                status,
                faq,
                contact,
                robots_txt,
            ],
        )
        .mount(
            dist::openipf::LOCAL_PREFIX,
            routes![
                dist::openipf::index,
                dist::openipf::rankings,
                dist::openipf::rankings_api,
                dist::openipf::default_rankings_api,
                dist::openipf::search_rankings_api,
                dist::openipf::default_search_rankings_api,
                dist::openipf::records,
                dist::openipf::records_default,
                dist::openipf::lifter,
                dist::openipf::lifter_csv,
                dist::openipf::meetlist,
                dist::openipf::meetlist_default,
                dist::openipf::meet,
                dist::openipf::meet_csv,
                dist::openipf::status,
                dist::openipf::faq,
                dist::openipf::contact,
            ],
        )
        .mount("/dev/", routes![dev_main, dev_checker_post])
        .mount(
            "/",
            routes![
                rankings_api,
                default_rankings_api,
                search_rankings_api,
                default_search_rankings_api
            ],
        )
        .mount(
            "/",
            routes![
                old_lifters,
                old_meetlist,
                old_meet,
                old_index,
                old_faq,
                old_contact,
            ],
        )
        .attach(Template::fairing())
        .register("/", catchers![not_found, internal_error])
        .attach(rocket::fairing::AdHoc::on_response(
            "Delete Server Header",
            |_request, response| {
                Box::pin(async move {
                    response.remove_header("Server");
                })
            },
        ))
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Accept an optional "--set-cwd" argument to manually specify the
    // current working directory. This allows the binary and the data
    // to be separated on a production server.
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "--set-cwd" {
        let fileroot = Path::new(&args[2]);
        env::set_current_dir(fileroot).expect("Invalid --set-cwd argument");
    }

    // Populate std::env with the contents of any .env file.
    dotenv::from_filename("server.env").expect("Couldn't find server.env");

    // Ensure that "STATICDIR" is set.
    env::var("STATICDIR").expect("STATICDIR envvar not set");

    // Load the OplDb.
    let start = std::time::Instant::now();
    let lifters_csv = env::var("LIFTERS_CSV").expect("LIFTERS_CSV not set");
    let meets_csv = env::var("MEETS_CSV").expect("MEETS_CSV not set");
    let entries_csv = env::var("ENTRIES_CSV").expect("ENTRIES_CSV not set");

    let opldb = opldb::OplDb::from_csv(
        Path::new(&lifters_csv),
        Path::new(&meets_csv),
        Path::new(&entries_csv),
    )?;

    println!(
        "DB loaded in {}MB and {:#?} (not counting the GlobalTable).",
        opldb.size_bytes() / 1024 / 1024,
        start.elapsed()
    );

    #[cfg(not(test))]
    let _ = rocket(opldb).launch().await;

    Ok(())
}
