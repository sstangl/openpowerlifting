//! Defines the `WeightClassKg` field for the `entries` table.

use serde;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::cmp::Ordering;
use std::fmt;
use std::num;
use std::str::FromStr;

use {WeightAny, WeightKg, WeightUnits};

/// The definition of the "WeightClassKg" column.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WeightClassKg {
    /// A class defined as being under or equal to a maximum weight.
    UnderOrEqual(WeightKg),
    /// A class defined as being over a minimum weight, for superheavies.
    Over(WeightKg),
    /// No weight class information supplied.
    None,
}

impl Default for WeightClassKg {
    fn default() -> WeightClassKg {
        WeightClassKg::None
    }
}

/// Displayable, unit-less variant of WeightClassKg.
///
/// Becasue the type of the weight is forgotten, these weights
/// are incomparable with each other.
#[derive(Copy, Clone, Debug)]
pub enum WeightClassAny {
    UnderOrEqual(WeightAny),
    Over(WeightAny),
    None,
}

impl Serialize for WeightClassAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Write into a stack-allocated fixed-size buffer.
        // TODO: Short-circuit "None" case.
        serializer.serialize_str(&format!("{}", self))
    }
}

impl WeightClassKg {
    pub fn as_kg(self) -> WeightClassAny {
        match self {
            WeightClassKg::UnderOrEqual(x) => WeightClassAny::UnderOrEqual(x.as_kg()),
            WeightClassKg::Over(x) => WeightClassAny::Over(x.as_kg()),
            WeightClassKg::None => WeightClassAny::None,
        }
    }

    pub fn as_lbs(self) -> WeightClassAny {
        match self {
            WeightClassKg::UnderOrEqual(x) => WeightClassAny::UnderOrEqual(x.as_lbs_class()),
            WeightClassKg::Over(x) => WeightClassAny::Over(x.as_lbs_class()),
            WeightClassKg::None => WeightClassAny::None,
        }
    }

    pub fn as_type(self, unit: WeightUnits) -> WeightClassAny {
        match unit {
            WeightUnits::Kg => self.as_kg(),
            WeightUnits::Lbs => self.as_lbs(),
        }
    }

    pub fn matches_bodyweight(self, bw: WeightKg) -> bool {
        match self {
            WeightClassKg::UnderOrEqual(cls) => bw <= cls,
            WeightClassKg::Over(cls) => bw > cls,
            WeightClassKg::None => false,
        }
    }
}

impl fmt::Display for WeightClassKg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_kg().fmt(f)
    }
}

impl PartialOrd for WeightClassKg {
    fn partial_cmp(&self, other: &WeightClassKg) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeightClassKg {
    fn cmp(&self, other: &WeightClassKg) -> Ordering {
        match self {
            WeightClassKg::UnderOrEqual(kg) => {
                match other {
                    WeightClassKg::UnderOrEqual(other_kg) => kg.cmp(&other_kg),
                    WeightClassKg::Over(_) => Ordering::Less,
                    WeightClassKg::None => Ordering::Less,
                }
            }
            WeightClassKg::Over(kg) => {
                match other {
                    WeightClassKg::UnderOrEqual(_) => Ordering::Greater,
                    WeightClassKg::Over(other_kg) => kg.cmp(&other_kg),
                    WeightClassKg::None => Ordering::Less,
                }
            }
            WeightClassKg::None => {
                match other {
                    WeightClassKg::UnderOrEqual(_) => Ordering::Greater,
                    WeightClassKg::Over(_) => Ordering::Greater,
                    WeightClassKg::None => Ordering::Equal,
                }
            }
        }
    }
}

impl WeightClassAny {
    // TODO: Reduce duplication.
    pub fn format_comma(self) -> String {
        match self {
            WeightClassAny::UnderOrEqual(x) => x.format_comma(),
            WeightClassAny::Over(x) => {
                let mut s = x.format_comma();
                s.push('+');
                s
            }
            WeightClassAny::None => String::new(),
        }
    }
}

impl fmt::Display for WeightClassAny {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeightClassAny::UnderOrEqual(ref x) => x.fmt(f),
            WeightClassAny::Over(ref x) => {
                x.fmt(f)?;
                write!(f, "+")
            }
            WeightClassAny::None => Ok(()),
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

    fn assert_kg_to_lbs(kg: &str, lbs: &str) {
        let w = kg.parse::<WeightClassKg>().unwrap();
        assert_eq!(format!("{}", w.as_lbs()), lbs);
    }

    #[test]
    fn test_lbs_rounding() {
        // Traditional classes.
        assert_kg_to_lbs("44", "97");
        assert_kg_to_lbs("48", "105");
        assert_kg_to_lbs("52", "114");
        assert_kg_to_lbs("56", "123");
        assert_kg_to_lbs("60", "132");
        assert_kg_to_lbs("67.5", "148");
        assert_kg_to_lbs("75", "165");
        assert_kg_to_lbs("82.5", "181");
        assert_kg_to_lbs("90", "198");
        assert_kg_to_lbs("90+", "198+");
        assert_kg_to_lbs("100", "220");
        assert_kg_to_lbs("110", "242");
        assert_kg_to_lbs("125", "275");
        assert_kg_to_lbs("140", "308");
        assert_kg_to_lbs("140+", "308+");

        // IPF Men.
        assert_kg_to_lbs("53", "116");
        assert_kg_to_lbs("59", "130");
        assert_kg_to_lbs("66", "145");
        assert_kg_to_lbs("74", "163");
        assert_kg_to_lbs("83", "183");
        assert_kg_to_lbs("93", "205");
        assert_kg_to_lbs("105", "231");
        assert_kg_to_lbs("120", "264");
        assert_kg_to_lbs("120+", "264+");

        // IPF Women.
        assert_kg_to_lbs("43", "94");
        assert_kg_to_lbs("47", "103");
        assert_kg_to_lbs("52", "114");
        assert_kg_to_lbs("57", "125");
        assert_kg_to_lbs("63", "138");
        assert_kg_to_lbs("72", "158");
        assert_kg_to_lbs("84", "185");
        assert_kg_to_lbs("84+", "185+");
    }
}
