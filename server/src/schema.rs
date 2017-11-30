// Definitions for accessing the database.

extern crate diesel;
extern crate r2d2;

use diesel::prelude::*;
use diesel::row::Row;
use diesel::types::FromSqlRow;
use r2d2_diesel::ConnectionManager;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use std::fmt;
use std::error::Error;
use std::ops::Deref;


/// Database type, to avoid hardcoding Sqlite in multiple places.
type DB = diesel::sqlite::Sqlite;


/// An alias to the type for a pool of Diesel SQLite connections.
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


/// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
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


/// Path to the database.
// FIXME: Should use an environment variable instead of being hardcoded.
static DATABASE_URL: &'static str = "../build/openpowerlifting.sqlite3";


pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    r2d2::Pool::new(manager).expect("Failed DB Pool creation.")
}


/// A Value from the mandatory "Sex" column of the entries table.
pub enum Sex {
    Male,
    Female,
}

impl Queryable<diesel::types::Bool, DB> for Sex
{
    type Row = Self;

    fn build(row: Self::Row) -> Self {
        row
    }
}

impl FromSqlRow<diesel::types::Bool, DB> for Sex
{
    fn build_from_row<T: Row<DB>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
        match row.take() {
            Some(v) => match v.read_integer() {
                0 => Ok(Sex::Male),
                1 => Ok(Sex::Female),
                _ => Err("Unrecognized sex".into()),
            },
            None => Err("Unexpected null for sex column".into()),
        }
    }
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sex::Male => write!(f, "M"),
            Sex::Female => write!(f, "F"),
        }
    }
}



/// A value from the mandatory "Equipment" column of the entries table.
pub enum Equipment {
    Raw,
    Wraps,
    Single,
    Multi,
    Straps,
}

impl Queryable<diesel::types::Integer, DB> for Equipment
{
    type Row = Self;

    fn build(row: Self::Row) -> Self {
        row
    }
}

impl FromSqlRow<diesel::types::Integer, DB> for Equipment
{
    fn build_from_row<T: Row<DB>>(row: &mut T) -> Result<Self, Box<Error + Send + Sync>> {
        match row.take() {
            Some(v) => match v.read_integer() {
                0 => Ok(Equipment::Raw),
                1 => Ok(Equipment::Wraps),
                2 => Ok(Equipment::Single),
                3 => Ok(Equipment::Multi),
                4 => Ok(Equipment::Straps),
                _ => Err("Unrecognized equipment".into()),
            },
            None => Err("Unexpected null for equipment column".into())
        }
    }
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Equipment::Raw => write!(f, "Raw"),
            Equipment::Wraps => write!(f, "Wraps"),
            Equipment::Single => write!(f, "Single"),
            Equipment::Multi => write!(f, "Multi"),
            Equipment::Straps => write!(f, "Straps"),
        }
    }
}



#[derive(Identifiable, Queryable)]
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

#[derive(Identifiable, Queryable)]
pub struct Lifter {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub instagram: Option<String>,
}

#[derive(Identifiable, Queryable, Associations)]
#[table_name = "entries"]
#[belongs_to(Meet, foreign_key="MeetID")]
#[belongs_to(Lifter, foreign_key="LifterID")]
pub struct Entry {
    pub id: i32,
    pub meet_id: i32,
    pub lifter_id: i32,
    pub sex: Sex,
    pub event: Option<String>,
    pub equipment: Equipment,
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

// Reads in the database and generates type information for all tables.
// The types are hardcoded upon DB creation, in `scripts/compile-sqlite`.
infer_schema!("../build/openpowerlifting.sqlite3");
