//! Implementation of the YesNo deserializer.

use serde::de::{self, Visitor};

use std::fmt;

/// Empty type for deserializing Yes/No fields to `bool`.
struct YesNo;

impl Visitor<'_> for YesNo {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("'Yes' or 'No'")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<bool, E> {
        match value {
            "Yes" => Ok(true),
            "No" | "" => Ok(false),
            _ => Err(E::custom("not Yes/No")),
        }
    }
}

/// Deserialization helper, converting "Yes" and "No" to a boolean.
pub fn deserialize_yes_no<'de, D: serde::Deserializer<'de>>(de: D) -> Result<bool, D::Error> {
    de.deserialize_str(YesNo)
}
