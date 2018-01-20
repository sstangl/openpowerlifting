//! An in-memory data store for OpenPowerlifting data.
//!
//! Because our data is read-only at runtime, we can lay out data structures
//! better than a "real" database like SQLite3 or PostreSQL. Additionally,
//! by storing all the data in formats native to Rust, we avoid copy overhead.

pub use opldb_enums::*;

/// The definition of a Lifter in the database.
/// The LifterID is implicit in the backing vector, as the index.
pub struct Lifter {
    pub name: String,
    pub username: String,
    pub instagram: Option<String>,
}

/// The definition of a Meet in the database.
/// The MeetID is implicit in the backing vector, as the index.
pub struct Meet {
    pub path: String,
    pub federation: Federation,
    pub date: String,
    pub country: String,
    pub state: Option<String>,
    pub town: Option<String>,
    pub name: String,
}

/// The definition of an Entry in the database.
/// The EntryID is implicit in the backing vector, as the index.
pub struct Entry {
    pub meet_id: u32,
    pub lifter_id: u32,
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

/// The collection of data stores that constitute the complete dataset.
pub struct OplDb {
    pub lifters: Vec<Lifter>,
    pub meets: Vec<Meet>,
    pub entries: Vec<Entry>,
}
