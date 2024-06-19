//! Defines the `Age` field for the `entries` table.

use arrayvec::ArrayString;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::num;
use std::str::FromStr;

use crate::Date;

/// The reported age of the lifter at a given meet.
/// In the CSV file, approximate ages are reported with '.5' added.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
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
    #[default]
    None,
}

/// An opaque wrapper for `Age` that serializes to "23~" instead of "23.5".
#[derive(Copy, Clone, Debug)]
pub struct PrettyAge(Age);

impl From<Age> for PrettyAge {
    fn from(a: Age) -> PrettyAge {
        PrettyAge(a)
    }
}

impl From<Age> for f64 {
    fn from(a: Age) -> f64 {
        match a {
            Age::Exact(a) => a.into(),
            Age::Approximate(a) => f64::from(a) + 0.5,
            Age::None => 0.0,
        }
    }
}

impl Age {
    /// Convert from an i64. Used by the TOML deserializer.
    pub fn from_i64(n: i64) -> Result<Self, &'static str> {
        // Some of the CONFIG.toml files hardcode 999 to mean "max Age".
        if n == 999 {
            return Ok(Age::Exact(u8::MAX));
        }

        if n < 0 {
            return Err("Age may not be negative");
        }
        if n > (i64::from(u8::MAX)) {
            return Err("Age can be at most 256");
        }

        Ok(Age::Exact(n as u8))
    }

    /// Convert from an f64. Used by the TOML deserializer.
    pub fn from_f64(f: f64) -> Result<Self, num::ParseIntError> {
        // Just use the from_str() implementation.
        // This function is not called often, so it's OK to be slow.
        let s = format!("{f}");
        s.parse::<Age>()
    }

    /// Given a BirthYear, calculates the approximate age of the lifter
    /// on the given Date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::{Age, Date};
    /// let date = Date::from_parts(2019, 02, 16);
    /// assert_eq!(Age::from_birthyear_on_date(1988, date), Age::Approximate(30));
    /// ```
    pub fn from_birthyear_on_date(birthyear: u32, on_date: Date) -> Self {
        let on_year = on_date.year();

        match on_year.cmp(&birthyear) {
            Ordering::Less => Age::None,
            Ordering::Greater => Age::Approximate((on_year - birthyear - 1) as u8),
            Ordering::Equal => Age::Approximate(0),
        }
    }

    /// Converts to an `Option<u8>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Age;
    /// assert_eq!(Age::Exact(23).to_u8_option(), Some(23));
    /// assert_eq!(Age::Approximate(23).to_u8_option(), Some(23));
    /// assert_eq!(Age::None.to_u8_option(), None);
    /// ```
    pub fn to_u8_option(self) -> Option<u8> {
        match self {
            Age::Exact(age) | Age::Approximate(age) => Some(age),
            Age::None => None,
        }
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
                Age::Approximate(other) => age > other + 1,
                Age::None => false,
            },
            Age::None => false,
        }
    }

    /// Whether the given Age is an Age::Exact.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Age;
    /// assert!(Age::Exact(23).is_exact());
    /// assert!(!Age::Approximate(23).is_exact());
    /// assert!(!Age::None.is_exact());
    /// ```
    pub fn is_exact(self) -> bool {
        match self {
            Age::Exact(_) => true,
            Age::Approximate(_) | Age::None => false,
        }
    }

    /// Whether the given Age is an Age::None.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Age;
    /// assert!(!Age::Exact(23).is_none());
    /// assert!(!Age::Approximate(23).is_none());
    /// assert!(Age::None.is_none());
    /// ```
    pub fn is_none(self) -> bool {
        self == Age::None
    }

    /// Whether the given Age is not an Age::None.
    pub fn is_some(self) -> bool {
        self != Age::None
    }
}

impl PartialOrd for Age {
    fn partial_cmp(&self, other: &Age) -> Option<Ordering> {
        match self {
            Age::Exact(age) => match other {
                Age::Exact(other_age) => Some(age.cmp(other_age)),
                Age::Approximate(other_age) => {
                    if *other_age == u8::MAX {
                        if *age == u8::MAX {
                            Some(Ordering::Equal)
                        } else {
                            Some(Ordering::Less)
                        }
                    } else {
                        Some(age.cmp(&(other_age + 1)))
                    }
                }
                Age::None => Some(Ordering::Less),
            },
            Age::Approximate(age) => match other {
                Age::Exact(other_age) => {
                    if *age == u8::MAX {
                        if *other_age == u8::MAX {
                            Some(Ordering::Equal)
                        } else {
                            Some(Ordering::Greater)
                        }
                    } else {
                        Some((age + 1).cmp(other_age))
                    }
                }
                Age::Approximate(other_age) => Some(age.cmp(other_age)),
                Age::None => Some(Ordering::Less),
            },
            Age::None => match other {
                Age::Exact(_) | Age::Approximate(_) => Some(Ordering::Greater),
                Age::None => Some(Ordering::Equal),
            },
        }
    }
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Age::Exact(n) => write!(f, "{n}"),
            Age::Approximate(n) => write!(f, "{n}~"),
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
            return Ok(Age::Exact(u8::MAX));
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
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Largest possible string for a `u8` Age is "256.5", 5 characters.
        let mut buf = ArrayString::<5>::new();

        match *self {
            Age::Exact(n) => write!(buf, "{n}").expect("ArrayString overflow"),
            Age::Approximate(n) => write!(buf, "{n}.5").expect("ArrayString overflow"),
            Age::None => (),
        };
        serializer.serialize_str(&buf)
    }
}

struct AgeVisitor;

impl<'de> Visitor<'de> for AgeVisitor {
    type Value = Age;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an age (23) or approximate age (23.5)")
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Age, E> {
        Age::from_f64(value).map_err(E::custom)
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Age, E> {
        Age::from_i64(value).map_err(E::custom)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Age, E> {
        Age::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Age {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Age, D::Error> {
        deserializer.deserialize_str(AgeVisitor)
    }
}

impl Serialize for PrettyAge {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Largest possible string for a `u8` Age is "256~", 4 characters.
        let mut buf = ArrayString::<4>::new();

        match self.0 {
            Age::Exact(n) => write!(buf, "{n}").expect("ArrayString overflow"),
            Age::Approximate(n) => write!(buf, "{n}~").expect("ArrayString overflow"),
            Age::None => (),
        };
        serializer.serialize_str(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let a = "29".parse::<Age>().unwrap();
        assert_eq!(format!("{a}"), "29");

        let a = "29.5".parse::<Age>().unwrap();
        assert_eq!(format!("{a}"), "29~");

        let a = "".parse::<Age>().unwrap();
        assert_eq!(format!("{a}"), "");
    }

    #[test]
    fn is_definitely_less_than() {
        let approx_17 = Age::Approximate(17); // "17 or 18"
        let approx_18 = Age::Approximate(18); // "18 or 19"
        let approx_19 = Age::Approximate(19); // "19 or 20"
        let exact_17 = Age::Exact(17);
        let exact_18 = Age::Exact(18);
        let exact_19 = Age::Exact(19);

        // Lower approximates compared to higher approximates.
        assert_eq!(approx_17.is_definitely_less_than(approx_17), false);
        assert_eq!(approx_17.is_definitely_less_than(approx_18), false);
        assert_eq!(approx_17.is_definitely_less_than(approx_19), true);

        // Higher approximates compared to lower approximates.
        assert_eq!(approx_19.is_definitely_less_than(approx_17), false);
        assert_eq!(approx_19.is_definitely_less_than(approx_18), false);
        assert_eq!(approx_19.is_definitely_less_than(approx_19), false);

        // Lower exacts compared to higher approximates.
        assert_eq!(exact_17.is_definitely_less_than(approx_17), false);
        assert_eq!(exact_17.is_definitely_less_than(approx_18), true);
        assert_eq!(exact_17.is_definitely_less_than(approx_19), true);

        // Higher approximates compared to lower exacts.
        assert_eq!(approx_19.is_definitely_less_than(exact_17), false);
        assert_eq!(approx_19.is_definitely_less_than(exact_18), false);
        assert_eq!(approx_19.is_definitely_less_than(exact_19), false);

        // Lower approximates compared to higher exacts.
        assert_eq!(approx_17.is_definitely_less_than(exact_17), false);
        assert_eq!(approx_17.is_definitely_less_than(exact_18), false);
        assert_eq!(approx_17.is_definitely_less_than(exact_19), true);

        // Higher exacts compared to lower approximates.
        assert_eq!(exact_19.is_definitely_less_than(approx_17), false);
        assert_eq!(exact_19.is_definitely_less_than(approx_18), false);
        assert_eq!(exact_19.is_definitely_less_than(approx_19), false);
    }

    #[test]
    fn is_definitely_greater_than() {
        let approx_17 = Age::Approximate(17); // "17 or 18"
        let approx_18 = Age::Approximate(18); // "18 or 19"
        let approx_19 = Age::Approximate(19); // "19 or 20"
        let exact_17 = Age::Exact(17);
        let exact_18 = Age::Exact(18);
        let exact_19 = Age::Exact(19);

        // Lower approximates compared to higher approximates.
        assert_eq!(approx_17.is_definitely_greater_than(approx_17), false);
        assert_eq!(approx_17.is_definitely_greater_than(approx_18), false);
        assert_eq!(approx_17.is_definitely_greater_than(approx_19), false);

        // Higher approximates compared to lower approximates.
        assert_eq!(approx_19.is_definitely_greater_than(approx_17), true);
        assert_eq!(approx_19.is_definitely_greater_than(approx_18), false);
        assert_eq!(approx_19.is_definitely_greater_than(approx_19), false);

        // Lower exacts compared to higher approximates.
        assert_eq!(exact_17.is_definitely_greater_than(approx_17), false);
        assert_eq!(exact_17.is_definitely_greater_than(approx_18), false);
        assert_eq!(exact_17.is_definitely_greater_than(approx_19), false);

        // Higher approximates compared to lower exacts.
        assert_eq!(approx_19.is_definitely_greater_than(exact_17), true);
        assert_eq!(approx_19.is_definitely_greater_than(exact_18), true);
        assert_eq!(approx_19.is_definitely_greater_than(exact_19), false);

        // Lower approximates compared to higher exacts.
        assert_eq!(approx_17.is_definitely_greater_than(exact_17), false);
        assert_eq!(approx_17.is_definitely_greater_than(exact_18), false);
        assert_eq!(approx_17.is_definitely_greater_than(exact_19), false);

        // Higher exacts compared to lower approximates.
        assert_eq!(exact_19.is_definitely_greater_than(approx_17), true);
        assert_eq!(exact_19.is_definitely_greater_than(approx_18), false);
        assert_eq!(exact_19.is_definitely_greater_than(approx_19), false);
    }
}
