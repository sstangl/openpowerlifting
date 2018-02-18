//! Defines the `WeightClassKg` field for the `entries` table.

use serde;
use serde::de::{self, Deserialize, Visitor};

use std::num;
use std::fmt;
use std::str::FromStr;

use opldb::fields::WeightKg;

/// The definition of the "WeightClassKg" column.
#[derive(Debug, PartialEq)]
pub enum WeightClassKg {
    /// A class defined as being under or equal to a maximum weight.
    UnderOrEqual(WeightKg),
    /// A class defined as being over a minimum weight, for superheavies.
    Over(WeightKg),
    /// No weight class information supplied.
    None,
}

impl fmt::Display for WeightClassKg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt:: Result {
        match self {
            &WeightClassKg::UnderOrEqual(ref x) => { x.fmt(f) }
            &WeightClassKg::Over(ref x) => {
                x.fmt(f)?;
                write!(f, "+")
            }
            &WeightClassKg::None => Ok(())
        }
    }
}

impl FromStr for WeightClassKg {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(WeightClassKg::None);
        }

        if s.ends_with('+') {
            let v = &s[..s.len() - 1];
            v.parse::<WeightKg>().map(WeightClassKg::Over)
        } else {
            s.parse::<WeightKg>().map(WeightClassKg::UnderOrEqual)
        }
    }
}

struct WeightClassKgVisitor;

impl<'de> Visitor<'de> for WeightClassKgVisitor {
    type Value = WeightClassKg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A floating-point value optionally ending with '+'")
    }

    fn visit_str<E>(self, value: &str) -> Result<WeightClassKg, E>
    where
        E: de::Error,
    {
        WeightClassKg::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for WeightClassKg {
    fn deserialize<D>(deserializer: D) -> Result<WeightClassKg, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(WeightClassKgVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weightclasskg_display() {
        let w = "140+".parse::<WeightClassKg>().unwrap();
        assert_eq!(format!("{}", w), "140+");

        let w = "82.5".parse::<WeightClassKg>().unwrap();
        assert_eq!(format!("{}", w), "82.5");

        let w = "".parse::<WeightClassKg>().unwrap();
        assert_eq!(format!("{}", w), "");
    }
}
