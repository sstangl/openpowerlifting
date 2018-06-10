//! A collection of fields used by the database.
//!
//! This file exists to separate deserialization and internal
//! representation details out from database definition file,
//! to make it easier to see the design from a high level.

use serde;
use serde::de::{self, Visitor};

use std::fmt;

mod age;
pub use self::age::*;
mod country;
pub use self::country::*;
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

/// Deserializer to bool for fields that take Yes/No values.
struct YesNo;

impl<'de> Visitor<'de> for YesNo {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("'Yes' or 'No'")
    }

    fn visit_str<E>(self, value: &str) -> Result<bool, E>
    where
        E: de::Error,
    {
        match value {
            "Yes" => Ok(true),
            "No" => Ok(false),
            _ => Err(E::custom("not yes/no")),
        }
    }
}

pub fn deserialize_yes_no<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    de.deserialize_str(YesNo)
}
