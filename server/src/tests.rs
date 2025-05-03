//! Tests for the Rocket code in main.rs.

use langpack::Language;
use opldb::OplDb;
use rocket::http::{Cookie, Header, Status};
use rocket::local::blocking::Client;

use std::path::Path;
use std::sync::LazyLock;

use super::rocket;
use super::Device;

static OPLDB_GLOBAL: LazyLock<OplDb> = LazyLock::new(|| {
    // This isn't really the place for it, but preload the environment.
    dotenvy::from_filename("server.env").unwrap();

    OplDb::from_csv(
        Path::new("../build/lifters.csv"),
        Path::new("../build/meets.csv"),
        Path::new("../build/entries.csv"),
    )
    .unwrap()
});

pub fn db() -> &'static OplDb {
    &*OPLDB_GLOBAL
}

/// Returns a client's view into the Rocket server, suitable for making
/// requests.
fn client() -> Client {
    // Untracked here means that cookie changes from the server need not be remembered.
    Client::untracked(rocket(db())).expect("valid rocket instance")
}

/// Simulates a GET request to a url from a specific device.
fn get(client: &Client, device: Device, url: &str) -> Status {
    match device {
        Device::Desktop => client.get(url).dispatch().status(),
        Device::Mobile => {
            let mut req = client.get(url);
            req.add_header(Header::new("User-Agent", "Mobile"));
            req.dispatch().status()
        }
    }
}

#[test]
fn test_pages_load() {
    let client = client();

    // Ensure that pages load on every kind of supported device.
    // Internally, these share contexts, but have different templates.
    for &device in &[Device::Desktop, Device::Mobile] {
        assert_eq!(get(&client, device, "/"), Status::Ok);
        assert_eq!(get(&client, device, "/rankings/uspa"), Status::Ok);
        assert_eq!(get(&client, device, "/records"), Status::Ok);
        assert_eq!(get(&client, device, "/records/uspa"), Status::Ok);
        assert_eq!(get(&client, device, "/u/seanstangl"), Status::Ok);
        assert_eq!(get(&client, device, "/mlist"), Status::Ok);
        assert_eq!(get(&client, device, "/m/uspa/0485"), Status::Ok);
        assert_eq!(get(&client, device, "/m/gpc-aus/1827"), Status::Ok);
        assert_eq!(get(&client, device, "/status"), Status::Ok);
        assert_eq!(get(&client, device, "/faq"), Status::Ok);
        assert_eq!(get(&client, device, "/contact"), Status::Ok);

        // Test a disambiguation page.
        assert_eq!(get(&client, device, "/u/joshsmith"), Status::Ok);

        // Test statics.
        assert_eq!(get(&client, device, "/robots.txt"), Status::Ok);
    }
}

#[test]
fn test_pages_load_for_openipf() {
    let client = client();

    // Ensure that pages load on every kind of supported device.
    // Internally, these share contexts, but have different templates.
    for &device in &[Device::Desktop, Device::Mobile] {
        assert_eq!(get(&client, device, "/dist/openipf/"), Status::Ok);
        assert_eq!(
            get(&client, device, "/dist/openipf/rankings/uspa"),
            Status::Ok
        );
        assert_eq!(get(&client, device, "/dist/openipf/records"), Status::Ok);
        assert_eq!(
            get(&client, device, "/dist/openipf/records/uspa"),
            Status::Ok
        );
        assert_eq!(
            get(&client, device, "/dist/openipf/u/seanstangl"),
            Status::Ok
        );
        assert_eq!(get(&client, device, "/dist/openipf/mlist"), Status::Ok);
        assert_eq!(
            get(&client, device, "/dist/openipf/m/usapl/CA-2019-04"),
            Status::Ok
        );
        assert_eq!(get(&client, device, "/dist/openipf/status"), Status::Ok);
        assert_eq!(get(&client, device, "/dist/openipf/faq"), Status::Ok);
        assert_eq!(get(&client, device, "/dist/openipf/contact"), Status::Ok);

        // Test a disambiguation page.
        assert_eq!(
            get(&client, device, "/dist/openipf/u/joshsmith"),
            Status::Ok
        );
    }
}

/// Some rankings pages that contain only a few entries have
/// historically produced crashes, when the context-generating
/// code assumes a minimum entry count.
#[test]
fn test_small_rankings_pages() {
    let client = client();
    // The BB federation is small and defunct, therefore good for testing.
    assert_eq!(get(&client, Device::Desktop, "/rankings/44/bb"), Status::Ok);
}

/// Test that meet pages load with different sorts.
#[test]
fn test_meet_pages_with_explicit_sorts() {
    let client = client();
    assert_eq!(
        get(&client, Device::Desktop, "/m/wrpf-usa/bob4"),
        Status::Ok
    );
    assert_eq!(
        get(&client, Device::Desktop, "/m/wrpf-usa/bob4/by-glossbrenner"),
        Status::Ok
    );
    assert_eq!(
        get(&client, Device::Desktop, "/m/wrpf-usa/bob4/by-ipf-points"),
        Status::Ok
    );
    assert_eq!(
        get(&client, Device::Desktop, "/m/wrpf-usa/bob4/by-division"),
        Status::Ok
    );
    assert_eq!(
        get(&client, Device::Desktop, "/m/wrpf-usa/bob4/by-total"),
        Status::Ok
    );
}

#[test]
fn test_username_redirects() {
    let client = client();
    let response = client.get("/u/TrystanOakley").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert!(response.headers().contains("location"));
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/u/trystanoakley"
    );
}

/// Test that URL patterns from the old web/ implementation are redirected
/// to their proper server/ equivalents.
#[test]
fn test_old_redirects() {
    let client = client();

    let response = client.get("/lifters.html?q=Sean%20Stangl").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/u/seanstangl"
    );

    let response = client.get("/meet.html?m=rps/1617").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/m/rps/1617"
    );

    // 365Strong is the last federation using this link style.
    let response = client.get("/?fed=365Strong").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/rankings/365strong"
    );

    let response = client.get("/index.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/");

    let response = client.get("/meetlist.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/mlist");

    let response = client.get("/faq.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/faq");

    let response = client.get("/contact.html").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(response.headers().get_one("location").unwrap(), "/contact");
}

#[test]
fn test_no_server_header() {
    // By default, the Rocket server serves a response header
    // "Server: Rocket". But it's unnecessary and an information leak.
    let client = client();
    let response = client.get("/").dispatch();
    assert!(!response.headers().contains("Server"));
}

/// Files served from "/static" should be served with the "Cache-Control"
/// header, to prevent them from being constantly reloaded.
#[test]
fn test_static_cache_control() {
    let client = client();
    let response = client.get("/static/images/favicon.ico").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.headers().contains("Cache-Control"));
    let cache_control = response.headers().get_one("Cache-Control").unwrap();
    assert!(cache_control.contains("max-age="));
}

/// Tests that the Accept-Language HTTP header can determine the language.
#[test]
fn test_accept_language_header() {
    // Iterate through all languages and ensure they are handled.
    for language in Language::string_list() {
        let content = format!("<html lang=\"{}\"", &language);
        let client = client();
        let res = client
            .get("/")
            .header(Header::new("Accept-Language", language))
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        assert!(res.into_string().unwrap().contains(&content));
    }

    // The "lang" cookie should override Accept-Language.
    let client = client();
    let res = client
        .get("/")
        .header(Header::new("Accept-Language", "ru"))
        .cookie(Cookie::new("lang", "eo"))
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.into_string().unwrap().contains("<html lang=\"eo\""));
}

/// Setting the "lang" cookie should change the text language,
/// via the HTML5 html "lang" tag.
#[test]
fn test_language_cookie() {
    let client = client();
    let lang_cookie = Cookie::new("lang", "ru");
    let res = client.get("/").cookie(lang_cookie).dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.into_string().unwrap().contains("<html lang=\"ru\""));
}

/// A nonsense "lang" cookie value should still render OK (with the English
/// default).
#[test]
fn test_language_cookie_nonsense() {
    let client = client();
    let lang_cookie = Cookie::new("lang", "fgsfds");
    let res = client.get("/").cookie(lang_cookie).dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.into_string().unwrap().contains("<html lang=\"en\""));
}

/// Passing the "?lang=" GET parameter should override the "lang" cookie.
#[test]
fn test_language_getparam_override() {
    let client = client();
    let lang_cookie = Cookie::new("lang", "ru");
    let res = client.get("/?lang=de").cookie(lang_cookie).dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.into_string().unwrap().contains("<html lang=\"de\""));
}

/// Passing nonsense to the "?lang=" GET parameter should still use the cookie.
#[test]
fn test_language_getparam_nonsense() {
    let client = client();
    let lang_cookie = Cookie::new("lang", "ru");
    let res = client.get("/?lang=fgsfds").cookie(lang_cookie).dispatch();
    assert_eq!(res.status(), Status::Ok);
    assert!(res.into_string().unwrap().contains("<html lang=\"ru\""));
}

/// Test that some nonsensical rankings options don't crash the server.
#[test]
fn test_rankings_nonsense() {
    let client = client();
    assert_eq!(
        get(&client, Device::Desktop, "/rankings/push-pull/by-squat"),
        Status::Ok
    );
}
