//! A collection of fields used by the database.
//!
//! This file exists to separate deserialization and internal
//! representation details out from database definition file,
//! to make it easier to see the design from a high level.

use serde;
use serde::de::{self, Visitor};

use std::fmt;
use std::str::FromStr;

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

/// Deserializes a `f32` field from the CSV source,
/// defaulting to `0.0` if the empty string is encountered.
pub fn deserialize_f32_with_default<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct F32StrVisitor;

    impl<'de> Visitor<'de> for F32StrVisitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("f32 or the empty string")
        }

        fn visit_str<E>(self, value: &str) -> Result<f32, E>
        where
            E: de::Error,
        {
            if value.is_empty() {
                return Ok(0.0);
            }
            f32::from_str(value).map_err(E::custom)
        }
    }

    deserializer.deserialize_str(F32StrVisitor)
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Sex {
    M,
    F,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Equipment {
    Raw,
    Wraps,
    #[serde(rename(deserialize = "Single-ply"))]
    Single,
    #[serde(rename(deserialize = "Multi-ply"))]
    Multi,
    Straps,
}
