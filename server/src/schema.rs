// Definitions for accessing the database.

extern crate r2d2;

use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

// An alias to the type for a pool of Diesel SQLite connections.
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

// Attempts to retrieve a single connection from the managed database pool. If
// no pool is currently managed, fails with an `InternalServerError` status. If
// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


// Path to the database.
// FIXME: Should use an environment variable instead of being hardcoded.
static DATABASE_URL: &'static str = "../build/openpowerlifting.sqlite3";


pub fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    r2d2::Pool::new(config, manager).expect("Failed DB Pool creation.")
}


infer_schema!("../build/openpowerlifting.sqlite3");

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub meetid: i32,
    pub name: String,
    pub sex: String,
    pub event: Option<String>,
    pub equipment: Option<String>,
    pub age: Option<f32>,
    pub division: Option<String>,
    pub bodyweightkg: Option<f32>,
    pub weightclasskg: Option<f32>,
    pub squat1kg: Option<f32>,
    pub squat2kg: Option<f32>,
    pub squat3kg: Option<f32>,
    pub squat4kg: Option<f32>,
    pub bestsquatkg: Option<f32>,
    pub bench1kg: Option<f32>,
    pub bench2kg: Option<f32>,
    pub bench3kg: Option<f32>,
    pub bench4kg: Option<f32>,
    pub bestbenchkg: Option<f32>,
    pub deadlift1kg: Option<f32>,
    pub deadlift2kg: Option<f32>,
    pub deadlift3kg: Option<f32>,
    pub deadlift4kg: Option<f32>,
    pub bestdeadliftkg: Option<f32>,
    pub totalkg: Option<f32>,
    pub place: Option<String>,
    pub wilks: Option<f32>,
    pub mcculloch: Option<f32>,
}

#[derive(Queryable)]
pub struct Meet {
    pub id: i32,
    pub path: String,
    pub federation: String,
    pub date: String,
    pub country: String,
    pub state: Option<String>,
    pub town: Option<String>,
    pub name: String,
}

#[derive(Queryable)]
pub struct Social {
    pub id: i32,
    pub name: String,
    pub instagram: String,
}
