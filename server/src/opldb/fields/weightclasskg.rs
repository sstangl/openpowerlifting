//! Defines the `WeightClassKg` field for the `entries` table.

use serde;
use serde::de::{self, Visitor, Deserialize};

use std::num;
use std::fmt;
use std::str::FromStr;

/// The definition of the "WeightClassKg" column.
#[derive(PartialEq)]
pub enum WeightClassKg {
    /// A class defined as being under or equal to a maximum weight.
    UnderOrEqual(f32),
    /// A class defined as being over a minimum weight, for superheavies.
    Over(f32),
    /// No weight class information supplied.
    None,
}

impl FromStr for WeightClassKg {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(WeightClassKg::None);
        }

        if s.ends_with('+') {
            let v = &s[..s.len()-1];
            v.parse::<f32>().map(WeightClassKg::Over)
        } else {
            s.parse::<f32>().map(WeightClassKg::UnderOrEqual)
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
        where E: de::Error
    {
        WeightClassKg::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for WeightClassKg {
    fn deserialize<D>(deserializer: D) -> Result<WeightClassKg, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(WeightClassKgVisitor)
    }
}
