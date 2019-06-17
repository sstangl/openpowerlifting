//! Shared Rocket code between main.rs and dist/.

use rocket::http::{Cookies, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::response::{self, content, Responder};
use rocket::Outcome;

use opltypes::WeightUnits;

use server::langpack::{self, LangInfo, Language, Locale};
use server::opldb;

// Use a single static database when testing.
#[cfg(not(test))]
pub type ManagedOplDb = opldb::OplDb;
#[cfg(test)]
pub type ManagedOplDb = &'static opldb::OplDb;

// Use a single static langpack when testing.
#[cfg(not(test))]
pub type ManagedLangInfo = langpack::LangInfo;
#[cfg(test)]
pub type ManagedLangInfo = &'static langpack::LangInfo;

/// Request guard for reading the "Accept-Encoding" HTTP header.
pub struct AcceptEncoding(pub Option<String>);

impl AcceptEncoding {
    pub fn supports_gzip(&self) -> bool {
        match &self.0 {
            None => false,
            Some(s) => s.contains("gzip"),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptEncoding {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AcceptEncoding, ()> {
        let keys: Vec<_> = request.headers().get("Accept-Encoding").collect();
        match keys.len() {
            0 => Outcome::Success(AcceptEncoding(None)),
            1 => Outcome::Success(AcceptEncoding(Some(keys[0].to_string()))),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

/// Request guard for reading the "Accept-Language" HTTP header.
pub struct AcceptLanguage(pub Option<String>);

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AcceptLanguage, ()> {
        let keys: Vec<_> = request.headers().get("Accept-Language").collect();
        match keys.len() {
            0 => Outcome::Success(AcceptLanguage(None)),
            1 => Outcome::Success(AcceptLanguage(Some(keys[0].to_string()))),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

pub fn select_display_language(languages: AcceptLanguage, cookies: &Cookies) -> Language {
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

            if valid_languages.is_empty() {
                default
            } else {
                valid_languages[0].parse::<Language>().unwrap_or(default)
            }
        }
        None => default,
    }
}

pub fn select_weight_units(language: Language, cookies: &Cookies) -> WeightUnits {
    // The user may explicitly override the weight unit choice by using
    // a cookie named "units".
    if let Some(cookie) = cookies.get("units") {
        if let Ok(units) = cookie.value().parse::<WeightUnits>() {
            return units;
        }
    }

    // TODO: Check Accept-Language header for regional variants of English,
    // for example Australia, to select Kg.

    // Otherwise, infer based on the language.
    language.default_units()
}

pub fn make_locale<'db>(
    langinfo: &'db LangInfo,
    lang: Option<String>,
    languages: AcceptLanguage,
    cookies: &Cookies,
) -> Locale<'db> {
    let language = match lang.and_then(|s| s.parse::<Language>().ok()) {
        // Allow an explicit "lang" GET parameter the "lang" cookie.
        Some(lang) => lang,
        // Otherwise, consult the cookies or defaults.
        None => select_display_language(languages, &cookies),
    };

    let units = select_weight_units(language, &cookies);
    Locale::new(&langinfo, language, units)
}

/// Return type for pre-rendered Json strings.
#[derive(Debug)]
pub struct JsonString(pub String);

impl Responder<'static> for JsonString {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        content::Json(self.0).respond_to(req)
    }
}

#[derive(FromForm)]
pub struct RankingsApiQuery {
    pub start: usize,
    pub end: usize,
    pub lang: String,
    pub units: String,
}

// TODO: Version / magicValue / etc.
#[derive(FromForm)]
pub struct SearchRankingsApiQuery {
    pub q: String,
    pub start: usize,
}
