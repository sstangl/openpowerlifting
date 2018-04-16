//! Defines the `Age` field for the `entries` table.

use serde;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::fmt;
use std::num;
use std::str::FromStr;

/// The reported age of the lifter at a given meet.
/// In the CSV file, approximate ages are reported with '.5' added.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Age {
    /// The exact age of the lifter.
    Exact(u8),
    /// The lower possible age of the lifter.
    Approximate(u8),
    /// No age specified.
    None,
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Age::Exact(n) => write!(f, "{}", n),
            Age::Approximate(n) => write!(f, "{}~", n),
            Age::None => Ok(()),
        }
    }
}

impl FromStr for Age {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Age::None);
        }

        let v: Vec<&str> = s.split('.').collect();
        if v.len() == 1 {
            v[0].parse::<u8>().map(Age::Exact)
        } else {
            v[0].parse::<u8>().map(Age::Approximate)
        }
    }
}

impl Serialize for Age {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Write into a stack-allocated fixed-size buffer.
        serializer.serialize_str(&format!("{}", self))
    }
}

struct AgeVisitor;

impl<'de> Visitor<'de> for AgeVisitor {
    type Value = Age;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an age (23) or approximate age (23.5)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Age, E>
    where
        E: de::Error,
    {
        Age::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Age {
    fn deserialize<D>(deserializer: D) -> Result<Age, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(AgeVisitor)
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_age_display() {
        let a = "29".parse::<Age>().unwrap();
        assert_eq!(format!("{}", a), "29");

        let a = "29.5".parse::<Age>().unwrap();
        assert_eq!(format!("{}", a), "29~");

        let a = "".parse::<Age>().unwrap();
        assert_eq!(format!("{}", a), "");
    }
}
