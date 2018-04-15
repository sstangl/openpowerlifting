//! Defines the `Place` field for the `entries` table.

use serde;
use serde::de::{self, Deserialize, Visitor};

use std::fmt;
use std::num;
use std::str::FromStr;

/// The definition of the "Place" column.
#[derive(Debug, PartialEq, Serialize)]
pub enum Place {
    /// The placing assigned to the entry.
    P(u8),
    /// Guest Lifter.
    G,
    /// Disqualified.
    DQ,
    /// Doping Disqualification.
    DD,
    /// No-Show.
    NS,
    /// No place specified.
    None, // TODO: Require every row to have a Place.
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Place::P(p) => write!(f, "{}", p),
            Place::G => write!(f, "G"),
            Place::DQ => write!(f, "DQ"),
            Place::DD => write!(f, "DD"),
            Place::NS => write!(f, "NS"),
            Place::None => Ok(()),
        }
    }
}

impl FromStr for Place {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Place::None),
            "G" => Ok(Place::G),
            "DQ" => Ok(Place::DQ),
            "DD" => Ok(Place::DD),
            "NS" => Ok(Place::NS),
            _ => {
                let num = s.parse::<u8>()?;
                Ok(Place::P(num))
            }
        }
    }
}

struct PlaceVisitor;

impl<'de> Visitor<'de> for PlaceVisitor {
    type Value = Place;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or G, DQ, DD, NS")
    }

    fn visit_str<E>(self, value: &str) -> Result<Place, E>
    where
        E: de::Error,
    {
        Place::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Place {
    fn deserialize<D>(deserializer: D) -> Result<Place, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PlaceVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_basic() {
        assert_eq!("1".parse::<Place>().unwrap(), Place::P(1));
        assert_eq!("2".parse::<Place>().unwrap(), Place::P(2));
        assert_eq!("3".parse::<Place>().unwrap(), Place::P(3));
        assert_eq!("27".parse::<Place>().unwrap(), Place::P(27));

        assert_eq!("G".parse::<Place>().unwrap(), Place::G);
        assert_eq!("DQ".parse::<Place>().unwrap(), Place::DQ);
        assert_eq!("DD".parse::<Place>().unwrap(), Place::DD);
        assert_eq!("NS".parse::<Place>().unwrap(), Place::NS);
        assert_eq!("".parse::<Place>().unwrap(), Place::None);
    }

    #[test]
    fn test_place_errors() {
        assert!("-1".parse::<Place>().is_err());
        assert!("-G".parse::<Place>().is_err());
        assert!("GG".parse::<Place>().is_err());
        assert!(" ".parse::<Place>().is_err());
        assert!("999999999999999999".parse::<Place>().is_err());
    }

    #[test]
    fn test_place_display() {
        let place = "5".parse::<Place>().unwrap();
        assert_eq!(format!("{}", place), "5");

        let place = "100".parse::<Place>().unwrap();
        assert_eq!(format!("{}", place), "100");

        let place = "G".parse::<Place>().unwrap();
        assert_eq!(format!("{}", place), "G");

        let place = "DQ".parse::<Place>().unwrap();
        assert_eq!(format!("{}", place), "DQ");

        let place = "DD".parse::<Place>().unwrap();
        assert_eq!(format!("{}", place), "DD");

        let place = "NS".parse::<Place>().unwrap();
        assert_eq!(format!("{}", place), "NS");

        let place = "".parse::<Place>().unwrap();
        assert_eq!(format!("{}", place), "");
    }
}
