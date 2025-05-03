//! Defines the `Place` field for the `entries` table.

use serde::de::{self, Deserialize, Visitor};

use std::error::Error;
use std::fmt;
use std::num;
use std::str::FromStr;

/// The definition of the "Place" column.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Ord, Eq, Serialize)]
pub enum Place {
    /// The placing assigned to the entry.
    P(num::NonZeroU8),
    /// Guest Lifter.
    G,
    /// Disqualified.
    DQ,
    /// Doping Disqualification.
    DD,
    /// No-Show.
    #[default]
    NS,
}

impl Place {
    pub fn is_dq(self) -> bool {
        match self {
            Place::P(_) => false,
            Place::G => false,
            Place::DQ => true,
            Place::DD => true,
            Place::NS => true,
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Place::P(p) => write!(f, "{p}"),
            Place::G => write!(f, "G"),
            Place::DQ => write!(f, "DQ"),
            Place::DD => write!(f, "DD"),
            Place::NS => write!(f, "NS"),
        }
    }
}

impl FromStr for Place {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "G" => Ok(Place::G),
            "DQ" => Ok(Place::DQ),
            "DD" => Ok(Place::DD),
            "NS" => Ok(Place::NS),
            _ => {
                let num = num::NonZeroU8::new(s.parse::<u8>()?).ok_or("Place cannot be '0'")?;
                Ok(Place::P(num))
            }
        }
    }
}

struct PlaceVisitor;

impl Visitor<'_> for PlaceVisitor {
    type Value = Place;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or G, DQ, DD, NS")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Place, E> {
        Place::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Place {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Place, D::Error> {
        deserializer.deserialize_str(PlaceVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num_place(n: u8) -> Place {
        Place::P(num::NonZeroU8::new(n).unwrap())
    }

    #[test]
    fn basic() {
        assert_eq!("1".parse::<Place>().unwrap(), num_place(1));
        assert_eq!("2".parse::<Place>().unwrap(), num_place(2));
        assert_eq!("3".parse::<Place>().unwrap(), num_place(3));
        assert_eq!("27".parse::<Place>().unwrap(), num_place(27));

        assert_eq!("G".parse::<Place>().unwrap(), Place::G);
        assert_eq!("DQ".parse::<Place>().unwrap(), Place::DQ);
        assert_eq!("DD".parse::<Place>().unwrap(), Place::DD);
        assert_eq!("NS".parse::<Place>().unwrap(), Place::NS);
    }

    #[test]
    fn errors() {
        assert!("0".parse::<Place>().is_err());
        assert!("-1".parse::<Place>().is_err());
        assert!("-G".parse::<Place>().is_err());
        assert!("GG".parse::<Place>().is_err());
        assert!(" ".parse::<Place>().is_err());
        assert!("999999999999999999".parse::<Place>().is_err());
    }

    #[test]
    fn display() {
        let place = "5".parse::<Place>().unwrap();
        assert_eq!(format!("{place}"), "5");

        let place = "100".parse::<Place>().unwrap();
        assert_eq!(format!("{place}"), "100");

        let place = "G".parse::<Place>().unwrap();
        assert_eq!(format!("{place}"), "G");

        let place = "DQ".parse::<Place>().unwrap();
        assert_eq!(format!("{place}"), "DQ");

        let place = "DD".parse::<Place>().unwrap();
        assert_eq!(format!("{place}"), "DD");

        let place = "NS".parse::<Place>().unwrap();
        assert_eq!(format!("{place}"), "NS");
    }
}
