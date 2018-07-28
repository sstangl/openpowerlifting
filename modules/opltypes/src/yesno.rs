//! Implementation of the YesNo deserializer.

use serde;
use serde::de::{self, Visitor};

use std::fmt;

/// Empty type for deserializing Yes/No fields to `bool`.
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
            _ => Err(E::custom("not Yes/No")),
        }
    }
}

pub fn deserialize_yes_no<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    de.deserialize_str(YesNo)
}
