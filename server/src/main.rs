#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate accept_language;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::http::{Cookies, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::response::{NamedFile, Redirect};
use rocket::{Outcome, State};
use rocket_contrib::Template;

use std::env;
use std::path::{Path, PathBuf};
use std::process;

extern crate server;
use server::langpack::{self, LangInfo, Language, Locale};
use server::opldb;
use server::pages;

/// Request guard for reading the "Accept-Language" HTTP header.
struct AcceptLanguage(pub Option<String>);

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AcceptLanguage, ()> {
        let keys: Vec<_> = request.headers().get("Accept-Language").collect();
        match keys.len() {
            0 => Outcome::Success(AcceptLanguage(None)),
            1 => Outcome::Success(AcceptLanguage(Some(keys[0].to_string()))),
            _ => return Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

fn select_display_language(languages: AcceptLanguage, cookies: &Cookies) -> Language {
    let default = Language::en;

    // The user may explicitly override the language choice by using
    // a cookie named "lang".
    if let Some(cookie) = cookies.get("lang") {
        if let Ok(lang) = cookie.value().parse::<Language>() {
            return lang;
        }
    }

    // If a language was not explicitly selected, the Accept-Language HTTP
    // header is consulted, defaulting to English.
    match languages.0 {
        Some(s) => {
            // TODO: It would be better if this vector was static.
            let known_languages: Vec<String> = Language::string_list();
            let borrowed: Vec<&str> =
                known_languages.iter().map(|s| s.as_ref()).collect();
            let valid_languages = accept_language::intersection(&s, borrowed);

            if valid_languages.len() == 0 {
                default
            } else {
                valid_languages[0]
                    .parse::<Language>()
                    .unwrap_or(default)
            }
        }
        None => default,
    }
}

fn select_weight_units(language: Language, cookies: &Cookies) -> opldb::WeightUnits {
    // The user may explicitly override the weight unit choice by using
    // a cookie named "units".
    if let Some(cookie) = cookies.get("units") {
        if let Ok(units) = cookie.value().parse::<opldb::WeightUnits>() {
            return units;
        }
    }

    // TODO: Check Accept-Language header for regional variants of English,
    // for example Australia, to select Kg.

    // Otherwise, infer based on the language.
    language.default_units()
}

fn make_locale<'db>(
    langinfo: &'db LangInfo,
    languages: AcceptLanguage,
    cookies: &Cookies,
) -> Locale<'db> {
    let lang = select_display_language(languages, &cookies);
    let units = select_weight_units(lang, &cookies);
    Locale::new(&langinfo, lang, units)
}

#[get("/static/<file..>")]
fn statics(file: PathBuf) -> Option<NamedFile> {
    let staticdir = env::var("STATICDIR").unwrap();
    NamedFile::open(Path::new(&staticdir).join(file)).ok()
}

#[get("/rankings/<selections..>")]
fn rankings(
    selections: PathBuf,
    opldb: State<opldb::OplDb>,
    langinfo: State<langpack::LangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);

    let selection = match pages::rankings::Selection::from_path(&selections) {
        Ok(s) => s,
        Err(_) => return None,
    };

    let context = pages::rankings::Context::new(&opldb, &locale, &selection);
    Some(Template::render("rankings", &context))
}

#[get("/u/<username>")]
fn lifter(
    username: String,
    opldb: State<opldb::OplDb>,
    langinfo: State<langpack::LangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);

    let lifter_id = match opldb.get_lifter_id(&username) {
        None => return None,
        Some(id) => id,
    };

    let context = pages::lifter::Context::new(&opldb, &locale, lifter_id);
    Some(Template::render("lifter", &context))
}

#[get("/m/<meetpath..>")]
fn meet(
    meetpath: PathBuf,
    opldb: State<opldb::OplDb>,
    langinfo: State<langpack::LangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);

    let meetpath_str: &str = match meetpath.to_str() {
        None => return None,
        Some(s) => s,
    };
    let meet_id = match opldb.get_meet_id(meetpath_str) {
        None => return None,
        Some(id) => id,
    };

    let context = pages::meet::Context::new(&opldb, &locale, meet_id);
    Some(Template::render("meet", &context))
}

#[get("/status")]
fn status(
    opldb: State<opldb::OplDb>,
    langinfo: State<langpack::LangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::status::Context::new(&opldb, &locale);
    Some(Template::render("status", &context))
}

#[get("/data")]
fn data(
    langinfo: State<langpack::LangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::data::Context::new(&locale);
    Some(Template::render("data", &context))
}

#[get("/faq")]
fn faq(
    langinfo: State<langpack::LangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::faq::Context::new(&locale);
    Some(Template::render("faq", &context))
}

#[get("/contact")]
fn contact(
    langinfo: State<langpack::LangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let context = pages::contact::Context::new(&locale);
    Some(Template::render("contact", &context))
}

#[get("/")]
fn index(
    opldb: State<opldb::OplDb>,
    langinfo: State<langpack::LangInfo>,
    languages: AcceptLanguage,
    cookies: Cookies,
) -> Option<Template> {
    let locale = make_locale(&langinfo, languages, &cookies);
    let selection = pages::rankings::Selection::new_default();
    let context = pages::rankings::Context::new(&opldb, &locale, &selection);
    Some(Template::render("rankings", &context))
}

#[error(404)]
fn not_found() -> &'static str {
    "404"
}

#[error(500)]
fn internal_error() -> &'static str {
    "500"
}

fn rocket(opldb: opldb::OplDb, langinfo: langpack::LangInfo) -> rocket::Rocket {
    // Initialize the server.
    rocket::ignite()
        .manage(opldb)
        .manage(langinfo)
        .mount(
            "/",
            routes![index, rankings, lifter, meet, statics, status, data, faq, contact],
        )
        .catch(errors![not_found, internal_error])
        .attach(Template::fairing())
}

fn load_translations_or_exit(
    langinfo: &mut langpack::LangInfo,
    language: Language,
    file: &str,
) {
    langinfo
        .load_translations(language, file)
        .map_err(|e| {
            eprintln!("Error loading translations: {}", e);
            process::exit(1);
        })
        .ok();
}

fn get_envvar_or_exit(key: &str) -> String {
    env::var(key)
        .map_err(|_| {
            eprintln!("Environment variable '{}' not set.", key);
            process::exit(1);
        })
        .unwrap()
}

fn main() {
    // Populate std::env with the contents of any .env file.
    dotenv::from_filename("server.env").ok();

    // Ensure that "STATICDIR" is set.
    get_envvar_or_exit("STATICDIR");

    // Load the OplDb.
    let lifters_csv = get_envvar_or_exit("LIFTERS_CSV");
    let meets_csv = get_envvar_or_exit("MEETS_CSV");
    let entries_csv = get_envvar_or_exit("ENTRIES_CSV");

    let opldb = match opldb::OplDb::from_csv(&lifters_csv, &meets_csv, &entries_csv) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error loading OplDb: {}", e);
            process::exit(1);
        }
    };

    println!(
        "OplDb loaded in {}MB.",
        opldb.size_bytes() / 1024 / 1024
    );

    // Load translations.
    let mut langinfo = langpack::LangInfo::new();
    load_translations_or_exit(&mut langinfo, Language::de, "translations/de.json");
    load_translations_or_exit(&mut langinfo, Language::en, "translations/en.json");
    load_translations_or_exit(&mut langinfo, Language::eo, "translations/eo.json");
    load_translations_or_exit(&mut langinfo, Language::es, "translations/es.json");
    load_translations_or_exit(&mut langinfo, Language::fi, "translations/fi.json");
    load_translations_or_exit(&mut langinfo, Language::fr, "translations/fr.json");
    load_translations_or_exit(&mut langinfo, Language::it, "translations/it.json");
    load_translations_or_exit(&mut langinfo, Language::sl, "translations/sl.json");
    load_translations_or_exit(&mut langinfo, Language::ru, "translations/ru.json");

    // Run the server loop.
    rocket(opldb, langinfo).launch();
}
