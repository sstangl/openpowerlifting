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

    /// Either one of two ages, stored as the lower of the pair.
    ///
    /// For example, an Approximate Age of 19 means "19 or 20".
    ///
    /// Approximate Ages occur when derived from a BirthYear,
    /// where it's not known whether the birthday occurred yet.
    Approximate(u8),

    /// No age specified.
    None,
}

impl Default for Age {
    fn default() -> Age {
        Age::None
    }
}

impl Age {
    /// Convert from an i64. Used by the TOML deserializer.
    pub fn from_i64(n: i64) -> Result<Self, &'static str> {
        // Some of the CONFIG.toml files hardcode 999 to mean "max Age".
        if n == 999 {
            return Ok(Age::Exact(u8::max_value()));
        }

        if n < 0 {
            return Err("Age may not be negative");
        }
        if n > (i64::from(u8::max_value())) {
            return Err("Age can be at most 256");
        }

        Ok(Age::Exact(n as u8))
    }

    /// Convert from an f64. Used by the TOML deserializer.
    pub fn from_f64(f: f64) -> Result<Self, num::ParseIntError> {
        // Just use the from_str() implementation.
        // This function is not called often, so it's OK to be slow.
        let s = format!("{}", f);
        s.parse::<Age>()
    }

    /// Whether the given Age is definitely less than another.
    ///
    /// Because of Approximate Ages, this does not produce a deterministic ordering,
    /// but it is still useful to determine whether one Age is out of range
    /// of another, usually for purposes of error checking.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Age;
    /// let approx_18 = Age::Approximate(18);  // "18 or 19"
    /// let approx_19 = Age::Approximate(19);  // "19 or 20"
    /// let exact_20 = Age::Exact(20);
    ///
    /// assert_eq!(approx_18.is_definitely_less_than(exact_20), true);
    /// assert_eq!(approx_19.is_definitely_less_than(exact_20), false);
    /// ```
    pub fn is_definitely_less_than(self, other: Age) -> bool {
        match self {
            Age::Exact(age) => match other {
                Age::Exact(other) => age < other,
                Age::Approximate(other) => age < other,
                Age::None => false,
            },
            Age::Approximate(age) => match other {
                Age::Exact(other) => age + 1 < other,
                Age::Approximate(other) => age + 1 < other,
                Age::None => false,
            },
            Age::None => false,
        }
    }

    /// Whether the given Age is definitely greater than another.
    ///
    /// Because of Approximate Ages, this does not produce a deterministic ordering,
    /// but it is still useful to determine whether one Age is out of range
    /// of another, usually for purposes of error checking.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Age;
    /// let approx_17 = Age::Approximate(17);  // "17 or 18"
    /// let approx_19 = Age::Approximate(19);  // "19 or 20"
    /// let exact_18 = Age::Exact(18);
    /// let exact_19 = Age::Exact(19);
    ///
    /// assert_eq!(approx_17.is_definitely_greater_than(exact_18), false);
    /// assert_eq!(approx_19.is_definitely_greater_than(exact_18), true);
    /// assert_eq!(approx_19.is_definitely_greater_than(exact_19), false);
    /// ```
    pub fn is_definitely_greater_than(self, other: Age) -> bool {
        match self {
            Age::Exact(age) => match other {
                Age::Exact(other) => age > other,
                Age::Approximate(other) => age > other + 1,
                Age::None => false,
            },
            Age::Approximate(age) => match other {
                Age::Exact(other) => age > other,
                Age::Approximate(other) => age + 1 > other,
                Age::None => false,
            },
            Age::None => false,
        }
    }
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

        // Some of the CONFIG.toml files hardcode 999 to mean "max Age".
        if s == "999" {
            return Ok(Age::Exact(u8::max_value()));
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

    fn visit_f64<E>(self, value: f64) -> Result<Age, E>
    where
        E: de::Error,
    {
        Age::from_f64(value).map_err(E::custom)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Age, E>
    where
        E: de::Error,
    {
        Age::from_i64(value).map_err(E::custom)
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

#[cfg(test)]
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
