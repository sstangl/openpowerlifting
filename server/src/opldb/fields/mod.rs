//! A collection of fields used by the database.
//!
//! This file exists to separate deserialization and internal
//! representation details out from database definition file,
//! to make it easier to see the design from a high level.

mod age;
pub use self::age::*;
mod date;
pub use self::date::*;
mod event;
pub use self::event::*;
mod federation;
pub use self::federation::*;
mod place;
pub use self::place::*;
mod points;
pub use self::points::*;
mod weightkg;
pub use self::weightkg::*;
mod weightclasskg;
pub use self::weightclasskg::*;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Sex {
    M,
    F,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Equipment {
    Raw,
    Wraps,
    #[serde(rename(deserialize = "Single-ply"))]
    Single,
    #[serde(rename(deserialize = "Multi-ply"))]
    Multi,
    Straps,
}
