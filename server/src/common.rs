//! Shared Rocket code between main.rs and dist/.

use rocket::http::{CookieJar, Status};
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::response::{self, content, Responder};

use langpack::{Language, Locale};
use opltypes::WeightUnits;

// Use a single static database when testing.
#[cfg(not(test))]
pub type ManagedOplDb = opldb::OplDb;
#[cfg(test)]
pub type ManagedOplDb = &'static opldb::OplDb;

/// Request guard for reading the "Host" HTTP header.
pub struct Host(pub Option<String>);

impl Host {
    pub fn served_from_openipf_org(&self) -> bool {
        match &self.0 {
            None => false,
            Some(s) => s.contains("openipf.org"),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Host {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Host").collect();
        match keys.len() {
            0 => Outcome::Success(Host(None)),
            1 => Outcome::Success(Host(Some(keys[0].to_string()))),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AcceptEncoding {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Accept-Encoding").collect();
        match keys.len() {
            0 => Outcome::Success(AcceptEncoding(None)),
            1 => Outcome::Success(AcceptEncoding(Some(keys[0].to_string()))),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

/// Request guard for determining whether the client is on a mobile device.
///
/// In order for "Request Desktop Site" to work, mobile detection is done
/// by User-Agent instead of by viewport size.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Device {
    Desktop,
    Mobile,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Device {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("User-Agent").collect();
        match keys.len() {
            1 => {
                if keys[0].contains("Mobile") {
                    Outcome::Success(Device::Mobile)
                } else {
                    Outcome::Success(Device::Desktop)
                }
            }
            _ => Outcome::Success(Device::Desktop),
        }
    }
}

/// Request guard for reading the "Accept-Language" HTTP header.
pub struct AcceptLanguage(pub Option<String>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AcceptLanguage {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Allow an "X-Default-Language" header to override Accept-Language.
        // This allows the "ru" subdomain to always begin serving in Russian.
        let keys: Vec<_> = request
            .headers()
            .get("X-Default-Language")
            .take(1)
            .collect();
        if keys.len() == 1 {
            return Outcome::Success(AcceptLanguage(Some(keys[0].to_string())));
        }

        let keys: Vec<_> = request.headers().get("Accept-Language").collect();
        match keys.len() {
            0 => Outcome::Success(AcceptLanguage(None)),
            1 => Outcome::Success(AcceptLanguage(Some(keys[0].to_string()))),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

pub fn select_display_language(languages: &AcceptLanguage, cookies: &CookieJar<'_>) -> Language {
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
    match &languages.0 {
        Some(s) => {
            // TODO: It would be better if this vector was static.
            let known_languages: Vec<String> = Language::string_list();
            let borrowed: Vec<&str> = known_languages.iter().map(|s| s.as_ref()).collect();
            let valid_languages = accept_language::intersection(s, borrowed);

            if valid_languages.is_empty() {
                default
            } else {
                valid_languages[0].parse::<Language>().unwrap_or(default)
            }
        }
        None => default,
    }
}

pub fn select_weight_units(
    languages: &AcceptLanguage,
    language: Language,
    cookies: &CookieJar<'_>,
) -> WeightUnits {
    // The user may explicitly override the weight unit choice by using
    // a cookie named "units".
    if let Some(cookie) = cookies.get("units") {
        if let Ok(units) = cookie.value().parse::<WeightUnits>() {
            return units;
        }
    }

    // Check the Accept-Language header for regional variants of English,
    // to decide whether to change from Kg to Lbs.
    if language == Language::en {
        if let Some(s) = &languages.0 {
            // This should handle the majority of pounds-preferring speakers.
            if s.starts_with("en-US") || s.starts_with("en-CA") {
                return WeightUnits::Lbs;
            }
        }
    }

    // Otherwise, infer based on the language.
    language.default_units()
}

pub fn make_locale(
    lang: Option<&str>,
    languages: AcceptLanguage,
    cookies: &CookieJar<'_>,
) -> Locale {
    let language = match lang.and_then(|s| s.parse::<Language>().ok()) {
        // Allow an explicit "lang" GET parameter.
        Some(lang) => lang,
        // Otherwise, consult the cookies or defaults.
        None => select_display_language(&languages, cookies),
    };

    let units = select_weight_units(&languages, language, cookies);
    Locale::new(language, units)
}

/// Return type for pre-rendered Json strings.
#[derive(Debug)]
pub struct JsonString(pub String);

impl<'r> Responder<'r, 'static> for JsonString {
    fn respond_to(self, req: &'r Request) -> response::Result<'static> {
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
