//! Defines the `WeightClassKg` field for the `entries` table.

use arrayvec::ArrayString;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::num;
use std::str::FromStr;

use crate::{WeightAny, WeightKg, WeightLbs, WeightUnits};

/// The definition of the "WeightClassKg" column.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum WeightClassKg {
    /// A class defined as being under or equal to a maximum weight.
    UnderOrEqual(WeightKg),
    /// A class defined as being over a minimum weight, for superheavies.
    Over(WeightKg),
    /// No weight class information supplied.
    #[default]
    None,
}

/// The definition of the "WeightClassLbs" column.
///
/// This exists to be immediately converted into [`WeightClassKg`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum WeightClassLbs {
    /// A class defined as being under or equal to a maximum weight.
    UnderOrEqual(WeightLbs),
    /// A class defined as being over a minimum weight, for superheavies.
    Over(WeightLbs),
    /// No weight class information supplied.
    #[default]
    None,
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

impl Serialize for WeightClassKg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.is_none() {
            return serializer.serialize_str("");
        }

        // Maximum length of a serialized WeightKg, plus one for a SHW "+".
        let mut buf = ArrayString::<14>::new();
        write!(buf, "{self}").expect("ArrayString overflow");

        serializer.serialize_str(&buf)
    }
}

impl Serialize for WeightClassAny {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.is_none() {
            return serializer.serialize_str("");
        }

        // Maximum length of a serialized WeightKg, plus one for a SHW "+".
        let mut buf = ArrayString::<14>::new();
        write!(buf, "{self}").expect("ArrayString overflow");

        serializer.serialize_str(&buf)
    }
}

impl WeightClassKg {
    pub fn as_kg(self) -> WeightClassAny {
        match self {
            WeightClassKg::UnderOrEqual(x) => WeightClassAny::UnderOrEqual(x.as_any()),
            WeightClassKg::Over(x) => WeightClassAny::Over(x.as_any()),
            WeightClassKg::None => WeightClassAny::None,
        }
    }

    pub fn as_lbs(self) -> WeightClassAny {
        match self {
            WeightClassKg::UnderOrEqual(x) => WeightClassAny::UnderOrEqual(x.as_lbs().as_class()),
            WeightClassKg::Over(x) => WeightClassAny::Over(x.as_lbs().as_class()),
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

    /// Whether this represents a Super HeavyWeight class.
    #[inline]
    pub fn is_shw(self) -> bool {
        match self {
            WeightClassKg::UnderOrEqual(_) => false,
            WeightClassKg::Over(_) => true,
            WeightClassKg::None => false,
        }
    }

    /// Whether this is the None kind.
    #[inline]
    pub fn is_none(self) -> bool {
        match self {
            WeightClassKg::UnderOrEqual(_) => false,
            WeightClassKg::Over(_) => false,
            WeightClassKg::None => true,
        }
    }
}

impl From<WeightClassLbs> for WeightClassKg {
    fn from(value: WeightClassLbs) -> Self {
        // Convert to kilograms. The problem is now that after the conversion, the weightclass
        // values are far off their correct values. For example, instead of 90kg, this will have 89.8kg,
        // instead of 82.5kg it will have 82.1kg. It's best to think of WeightClassLbs labels as
        // containing names instead of actual values.
        //
        // In general, the values are always an under-estimate, and should be rounded to the nearest
        // 0.5kg.
        fn convert(lbs: WeightLbs) -> WeightKg {
            // Get the raw representation and cast to i64, expanding the range.
            let kg = f32::from(WeightKg::from(lbs));
            let rounded = (kg * 2.0).ceil() / 2.0;

            // TODO: This probably should round to the fed's CONFIG, not hardcoded.
            if rounded == 117.5 {
                WeightKg::from_f32(118.0)
            } else {
                WeightKg::from_f32(rounded)
            }
        }

        match value {
            WeightClassLbs::UnderOrEqual(lbs) => WeightClassKg::UnderOrEqual(convert(lbs)),
            WeightClassLbs::Over(lbs) => WeightClassKg::Over(convert(lbs)),
            WeightClassLbs::None => WeightClassKg::None,
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
            WeightClassKg::UnderOrEqual(kg) => match other {
                WeightClassKg::UnderOrEqual(other_kg) => kg.cmp(other_kg),
                WeightClassKg::Over(_) => Ordering::Less,
                WeightClassKg::None => Ordering::Less,
            },
            WeightClassKg::Over(kg) => match other {
                WeightClassKg::UnderOrEqual(_) => Ordering::Greater,
                WeightClassKg::Over(other_kg) => kg.cmp(other_kg),
                WeightClassKg::None => Ordering::Less,
            },
            WeightClassKg::None => match other {
                WeightClassKg::UnderOrEqual(_) => Ordering::Greater,
                WeightClassKg::Over(_) => Ordering::Greater,
                WeightClassKg::None => Ordering::Equal,
            },
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

    /// Whether this is the None kind.
    #[inline]
    pub fn is_none(self) -> bool {
        match self {
            WeightClassAny::UnderOrEqual(_) => false,
            WeightClassAny::Over(_) => false,
            WeightClassAny::None => true,
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

        if let Some(v) = s.strip_suffix('+') {
            v.parse::<WeightKg>().map(WeightClassKg::Over)
        } else {
            s.parse::<WeightKg>().map(WeightClassKg::UnderOrEqual)
        }
    }
}

impl FromStr for WeightClassLbs {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(WeightClassLbs::None);
        }

        if let Some(v) = s.strip_suffix('+') {
            v.parse::<WeightLbs>().map(WeightClassLbs::Over)
        } else {
            s.parse::<WeightLbs>().map(WeightClassLbs::UnderOrEqual)
        }
    }
}

struct WeightClassKgVisitor;

impl Visitor<'_> for WeightClassKgVisitor {
    type Value = WeightClassKg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A floating-point value optionally ending with '+'")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<WeightClassKg, E> {
        WeightClassKg::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for WeightClassKg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(WeightClassKgVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let w = "140+".parse::<WeightClassKg>().unwrap();
        assert_eq!(format!("{w}"), "140+");

        let w = "82.5".parse::<WeightClassKg>().unwrap();
        assert_eq!(format!("{w}"), "82.5");

        let w = "".parse::<WeightClassKg>().unwrap();
        assert_eq!(format!("{w}"), "");
    }

    fn assert_kg_to_lbs(kg: &str, lbs: &str) {
        let w = kg.parse::<WeightClassKg>().unwrap();
        assert_eq!(format!("{}", w.as_lbs()), lbs);
    }

    #[test]
    fn as_lbs_rounding() {
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
