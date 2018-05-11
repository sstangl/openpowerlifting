//! Tests for the Rocket code in main.rs.

use super::rocket;
use server::langpack::LangInfo;
use server::opldb::OplDb;

use rocket::http::Status;
use rocket::local::Client;

use std::sync::{Once, ONCE_INIT};

static mut OPLDB_GLOBAL: Option<OplDb> = None;
static OPLDB_INIT: Once = ONCE_INIT;

fn db() -> &'static OplDb {
    const LIFTERS_CSV: &str = "../build/lifters.csv";
    const MEETS_CSV: &str = "../build/meets.csv";
    const ENTRIES_CSV: &str = "../build/openpowerlifting.csv";

    unsafe {
        OPLDB_INIT.call_once(|| {
            OPLDB_GLOBAL =
                Some(OplDb::from_csv(LIFTERS_CSV, MEETS_CSV, ENTRIES_CSV).unwrap());
        });

        OPLDB_GLOBAL.as_ref().unwrap()
    }
}

static mut LANGINFO_GLOBAL: Option<LangInfo> = None;
static LANGINFO_INIT: Once = ONCE_INIT;

fn langinfo() -> &'static LangInfo {
    unsafe {
        LANGINFO_INIT.call_once(|| {
            LANGINFO_GLOBAL = Some(super::load_langinfo());
        });
        LANGINFO_GLOBAL.as_ref().unwrap()
    }
}

/// Returns a client's view into the Rocket server, suitable for making
/// requests.
fn client() -> Client {
    Client::new(rocket(db(), langinfo())).expect("valid rocket instance")
}

#[test]
fn test_db_loads() {
    db();
    langinfo();
}

#[test]
fn test_pages_load() {
    let client = client();
    assert_eq!(client.get("/").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/rankings/uspa").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/u/seanstangl").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/m/uspa/0485").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/status").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/data").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/faq").dispatch().status(), Status::Ok);
    assert_eq!(client.get("/contact").dispatch().status(), Status::Ok);
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

#[test]
fn test_old_redirects() {
    // Test that URL patterns from the old web/ implementation are redirected
    // to their server/ equivalents.
    let client = client();

    let response = client.get("/lifters.html?q=Sean Stangl").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/u/seanstangl"
    );

    let response = client.get("/lifters.html?q=Sean%20Stangl").dispatch();
    assert_eq!(response.status(), Status::PermanentRedirect);
    assert_eq!(
        response.headers().get_one("location").unwrap(),
        "/u/seanstangl"
    );
}
