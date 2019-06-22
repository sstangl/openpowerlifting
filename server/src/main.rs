#![feature(proc_macro_hygiene, decl_macro)]

extern crate accept_language;
extern crate dotenv;
extern crate opltypes;
use opltypes::{Federation, WeightUnits};
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate strum;

// Distributions, like OpenIPF.
mod dist;

// Shared Rocket code between the main server and distributions.
mod common;
use common::*;

#[cfg(test)]
mod tests;

use rocket::fairing::AdHoc;
use rocket::http::{ContentType, Cookies, Status};
use rocket::request::{Form, Request};
use rocket::response::{NamedFile, Redirect, Responder, Response};
use rocket::State;
use rocket_contrib::templates::Template;

use strum::IntoEnumIterator;

use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

extern crate server;
use server::langpack::{self, LangInfo, Language, Locale};
use server::opldb;
use server::pages;

/// A file served from /static.
enum StaticFile {
    /// PathBuf is the path to the non-gz version of the file.
    Gzipped(PathBuf, File),
    Plain(NamedFile),
}

impl Responder<'static> for StaticFile {
    fn respond_to(self, req: &Request) -> Result<Response<'static>, Status> {
        let mut response = match self {
            StaticFile::Gzipped(p, f) => {
                let mut r = f.respond_to(req)?;
                r.set_raw_header("Content-Encoding", "gzip");
                if let Some(ext) = p.extension() {
                    if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy())
                    {
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
fn statics(file: PathBuf, encoding: AcceptEncoding) -> Option<StaticFile> {
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

    let namedfile = NamedFile::open(filepath).ok()?;
    Some(StaticFile::Plain(namedfile))
}

/// Actually store the favicon in static/images/,
/// but allow serving from the root.
#[get("/favicon.ico")]
fn root_favicon(encoding: AcceptEncoding) -> Option<StaticFile> {
    statics(PathBuf::from("images/favicon.ico"), encoding)
}

#[get("/apple-touch-icon.png")]
fn root_apple_touch_icon(encoding: AcceptEncoding) -> Option<StaticFile> {
    statics(PathBuf::from("images/apple-touch-icon.png"), encoding)
}

#[get("/rankings/<selections..>?<lang>")]
fn rankings(
    selections: PathBuf,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let default = pages::selection::Selection::default();
    let selection = pages::selection::Selection::from_path(&selections, &default).ok()?;
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::rankings::Context::new(&opldb, &locale, &selection)?;
    Some(Template::render("rankings", &context))
}

#[get("/rankings")]
fn rankings_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/records/<selections..>?<lang>")]
fn records(
    selections: Option<PathBuf>,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let default = pages::records::RecordsSelection::default();
    let selection = if let Some(sel) = selections {
        pages::records::RecordsSelection::from_path(&sel, &default).ok()?
    } else {
        default
    };
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::records::Context::new(
        &opldb,
        &locale,
        &selection,
        &pages::selection::Selection::default(),
    );
    Some(Template::render("records", &context))
}

#[get("/records?<lang>")]
fn records_default(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    records(None, lang, opldb, langinfo, languages, cookies)
}

#[get("/u/<username>?<lang>")]
fn lifter(
    username: String,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Result<Template, Redirect>> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);

    // Disambiguations end with a digit.
    // Some lifters may have failed to be merged with their disambiguated username.
    // Therefore, for usernames without a digit, it cannot be assumed that they are
    // *not* a disambiguation.
    let is_definitely_disambiguation: bool = username
        .chars()
        .last()
        .map_or(false, |c| c.is_ascii_digit());

    let lifter_ids: Vec<u32> = if is_definitely_disambiguation {
        if let Some(id) = opldb.get_lifter_id(&username) {
            vec![id]
        } else {
            vec![]
        }
    } else {
        opldb.get_lifters_under_username(&username)
    };

    match lifter_ids.len() {
        // If no LifterID was found, maybe the name just needs to be lowercased.
        0 => {
            let lowercase = username.to_ascii_lowercase();
            let _guard = opldb.get_lifter_id(&lowercase)?;
            Some(Err(Redirect::permanent(format!("/u/{}", lowercase))))
        }

        // If a specific lifter was referenced, return the lifter's unique page.
        1 => {
            let context =
                pages::lifter::Context::new(&opldb, &locale, lifter_ids[0], None);
            Some(Ok(Template::render("lifter", &context)))
        }

        // If multiple lifters were referenced, return a disambiguation page.
        _ => {
            let context = pages::disambiguation::Context::new(
                &opldb,
                &locale,
                &username,
                &lifter_ids,
            );
            Some(Ok(Template::render("disambiguation", &context)))
        }
    }
}

#[get("/mlist/<mselections..>?<lang>")]
fn meetlist(
    mselections: Option<PathBuf>,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let mselection = match mselections {
        None => pages::meetlist::MeetListSelection::default(),
        Some(p) => pages::meetlist::MeetListSelection::from_path(&p).ok()?,
    };
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::meetlist::Context::new(&opldb, &locale, &mselection);
    Some(Template::render("meetlist", &context))
}

#[get("/mlist?<lang>")]
fn meetlist_default(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    meetlist(None, lang, opldb, langinfo, languages, cookies)
}

#[get("/m/<meetpath..>?<lang>")]
fn meet(
    meetpath: PathBuf,
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let mut meetpath_str: &str = meetpath.to_str()?;
    let mut sort = pages::meet::MeetSortSelection::ByFederationDefault;

    // The meetpath may contain an optional sorting directive.
    // If present, detect and remove that component from the path.
    let component = meetpath.as_path().file_name()?.to_str()?;
    if let Ok(sortselection) = component.parse::<pages::meet::MeetSortSelection>() {
        sort = sortselection;
        meetpath_str = meetpath.as_path().parent()?.to_str()?;
    }

    let meet_id = opldb.get_meet_id(meetpath_str)?;
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::meet::Context::new(&opldb, &locale, meet_id, sort);
    Some(Template::render("meet", &context))
}

#[get("/status?<lang>")]
fn status(
    lang: Option<String>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::status::Context::new(&opldb, &locale);
    Some(Template::render("status", &context))
}

#[get("/data?<lang>")]
fn data(
    lang: Option<String>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::data::Context::new(&locale);
    Some(Template::render("data", &context))
}

#[get("/faq?<lang>")]
fn faq(
    lang: Option<String>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::faq::Context::new(&locale);
    Some(Template::render("faq", &context))
}

#[get("/contact?<lang>")]
fn contact(
    lang: Option<String>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::contact::Context::new(&locale);
    Some(Template::render("contact", &context))
}

#[derive(Responder)]
enum IndexReturn {
    Redirect(Redirect),
    Template(Template),
}

#[get("/?<lang>&<fed>")]
fn index(
    lang: Option<String>,
    fed: Option<String>, // For handling old-style URLs.
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<IndexReturn> {
    // Handle old-style URLs. Hopefully we can remove this code one day.
    if let Some(fedstr) = fed {
        let fed = fedstr.parse::<Federation>().ok()?;
        let target = format!("/rankings/{}", fed.to_string().to_ascii_lowercase());
        return Some(IndexReturn::Redirect(Redirect::permanent(target)));
    }

    // Otherwise, render the main rankings template.
    let selection = pages::selection::Selection::default();
    let locale = make_locale(&langinfo, lang, languages, &cookies);
    let context = pages::rankings::Context::new(&opldb, &locale, &selection);
    Some(IndexReturn::Template(Template::render(
        "rankings", &context,
    )))
}

/// API endpoint for fetching a slice of rankings data as JSON.
#[get("/api/rankings/<selections..>?<query..>")]
fn rankings_api(
    selections: Option<PathBuf>,
    query: Form<RankingsApiQuery>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
) -> Option<JsonString> {
    let default = pages::selection::Selection::default();
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
fn default_rankings_api<'db>(
    query: Form<RankingsApiQuery>,
    opldb: State<ManagedOplDb>,
    langinfo: State<ManagedLangInfo>,
) -> Option<JsonString> {
    rankings_api(None, query, opldb, langinfo)
}

/// API endpoint for rankings search.
#[get("/api/search/rankings/<selections..>?<query..>")]
fn search_rankings_api<'db>(
    selections: Option<PathBuf>,
    query: Form<SearchRankingsApiQuery>,
    opldb: State<ManagedOplDb>,
) -> Option<JsonString> {
    let default = pages::selection::Selection::default();
    let selection = match selections {
        None => default,
        Some(path) => pages::selection::Selection::from_path(&path, &default).ok()?,
    };

    let result =
        pages::api_search::search_rankings(&opldb, &selection, query.start, &query.q);

    Some(JsonString(serde_json::to_string(&result).ok()?))
}

#[get("/api/search/rankings?<query..>")]
fn default_search_rankings_api(
    query: Form<SearchRankingsApiQuery>,
    opldb: State<ManagedOplDb>,
) -> Option<JsonString> {
    search_rankings_api(None, query, opldb)
}

#[get("/lifters.html?<q>")]
fn old_lifters(opldb: State<ManagedOplDb>, q: String) -> Option<Redirect> {
    let name = &q;
    let id = opldb.get_lifter_id_by_name(name)?;
    let username = &opldb.get_lifter(id).username;
    Some(Redirect::permanent(format!("/u/{}", username)))
}

#[get("/meetlist.html")]
fn old_meetlist() -> Redirect {
    Redirect::permanent("/mlist")
}

#[get("/meet.html?<m>")]
fn old_meet(opldb: State<ManagedOplDb>, m: String) -> Option<Redirect> {
    let meetpath = &m;
    let id = opldb.get_meet_id(meetpath)?;
    let pathstr = &opldb.get_meet(id).path;
    Some(Redirect::permanent(format!("/m/{}", pathstr)))
}

#[get("/index.html")]
fn old_index() -> Redirect {
    Redirect::permanent("/")
}

#[get("/data.html")]
fn old_data() -> Redirect {
    Redirect::permanent("/data")
}

#[get("/faq.html")]
fn old_faq() -> Redirect {
    Redirect::permanent("/faq")
}

#[get("/contact.html")]
fn old_contact() -> Redirect {
    Redirect::permanent("/contact")
}

#[get("/robots.txt")]
fn robots_txt() -> &'static str {
    // Allow robots full site access except for JSON endpoints.
    "User-agent: *\nDisallow: /api/"
}

#[catch(404)]
fn not_found() -> &'static str {
    "404"
}

#[catch(500)]
fn internal_error() -> &'static str {
    "500"
}

// Tests want to load the data only once.
#[cfg(not(test))]
type ManagedOplDb = opldb::OplDb;
#[cfg(test)]
type ManagedOplDb = &'static opldb::OplDb;

#[cfg(not(test))]
type ManagedLangInfo = langpack::LangInfo;
#[cfg(test)]
type ManagedLangInfo = &'static langpack::LangInfo;

fn rocket(opldb: ManagedOplDb, langinfo: ManagedLangInfo) -> rocket::Rocket {
    // Initialize the server.
    rocket::ignite()
        .manage(opldb)
        .manage(langinfo)
        .mount(
            "/",
            routes![
                index,
                rankings,
                rankings_redirect,
                records,
                records_default,
                lifter,
                meetlist,
                meetlist_default,
                meet,
                statics,
                root_favicon,
                root_apple_touch_icon,
                status,
                data,
                faq,
                contact,
                robots_txt,
            ],
        )
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
                old_data,
                old_faq,
                old_contact,
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
                dist::openipf::meet,
                dist::openipf::faq,
            ],
        )
        .register(catchers![not_found, internal_error])
        .attach(Template::fairing())
        .attach(AdHoc::on_response(
            "Delete Server Header",
            |_request, response| {
                response.remove_header("Server");
            },
        ))
}

fn load_langinfo() -> Result<LangInfo, Box<dyn Error>> {
    let mut langinfo = langpack::LangInfo::default();
    for language in Language::iter() {
        let path = format!("translations/{}.json", language);
        langinfo.load_translations(language, &path)?;
    }
    Ok(langinfo)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Accept an optional "--set-cwd" argument to manually specify the
    // current working directory. This allows the binary and the data
    // to be separated on a production server.
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "--set-cwd" {
        let fileroot = Path::new(&args[2]);
        env::set_current_dir(&fileroot).expect("Invalid --set-cwd argument");
    }

    // Populate std::env with the contents of any .env file.
    dotenv::from_filename("server.env").expect("Couldn't find server.env");

    // Ensure that "STATICDIR" is set.
    env::var("STATICDIR").expect("STATICDIR envvar not set");

    // Load the OplDb.
    let lifters_csv = env::var("LIFTERS_CSV").expect("LIFTERS_CSV not set");
    let meets_csv = env::var("MEETS_CSV").expect("MEETS_CSV not set");
    let entries_csv = env::var("ENTRIES_CSV").expect("ENTRIES_CSV not set");
    let opldb = opldb::OplDb::from_csv(&lifters_csv, &meets_csv, &entries_csv)?;
    println!("OplDb loaded in {}MB.", opldb.size_bytes() / 1024 / 1024);

    #[allow(unused_variables)]
    let langinfo = load_langinfo()?;

    #[cfg(not(test))]
    rocket(opldb, langinfo).launch();
    Ok(())
}
