//! Defines the `Date` field for the `meets` table.

use serde;
use serde::de::{self, Deserialize, Visitor};

use std::fmt;
use std::num;
use std::str::FromStr;

/// Our data uses imprecise dates in the "YYYY-MM-DD" format,
/// with no timezone or time data.
/// Dates in this format can be stored as a `u32` with value YYYYMMDD.
/// This format is compact and remains human-readable.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Serialize)]
pub struct Date(u32);

impl Date {
    #[inline]
    pub fn from_u32(u: u32) -> Date {
        Date(u)
    }

    #[inline]
    pub fn year(&self) -> u32 {
        self.0 / 10_000
    }

    #[inline]
    pub fn month(&self) -> u32 {
        (self.0 / 100) % 100
    }

    #[inline]
    pub fn day(&self) -> u32 {
        self.0 % 100
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}",
            self.year(),
            self.month(),
            self.day()
        )
    }
}

#[derive(Debug)]
pub enum ParseDateError {
    FormatError,
    MonthError,
    DayError,
    ParseIntError(num::ParseIntError),
}

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseDateError::FormatError => write!(f, "date not in the correct format"),
            ParseDateError::MonthError => write!(f, "invalid month"),
            ParseDateError::DayError => write!(f, "invalid day"),
            ParseDateError::ParseIntError(ref p) => p.fmt(f),
        }
    }
}

impl FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split('-').collect();
        if v.len() != 3 || v[0].len() != 4 || v[1].len() != 2 || v[2].len() != 2 {
            return Err(ParseDateError::FormatError);
        }

        let year: u32 = v[0].parse::<u32>().map_err(ParseDateError::ParseIntError)?;
        let month: u32 = v[1].parse::<u32>().map_err(ParseDateError::ParseIntError)?;
        let day: u32 = v[2].parse::<u32>().map_err(ParseDateError::ParseIntError)?;

        if month == 0 || month > 12 {
            return Err(ParseDateError::MonthError);
        }
        if day == 0 || day > 31 {
            return Err(ParseDateError::DayError);
        }

        let value = (year * 10_000) + (month * 100) + day;

        Ok(Date(value))
    }
}

struct DateVisitor;

impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the format YYYY-MM-DD")
    }

    fn visit_str<E>(self, value: &str) -> Result<Date, E>
    where
        E: de::Error,
    {
        Date::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DateVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_date_basic() {
        let date = "2017-03-04".parse::<Date>().unwrap();
        assert_eq!(date.year(), 2017);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 4);
    }

    #[test]
    fn test_date_errors() {
        // Malformed dates.
        assert!("2017-03-04-05".parse::<Date>().is_err());
        assert!("2017-03-004".parse::<Date>().is_err());
        assert!("2017-003-04".parse::<Date>().is_err());
        assert!("02017-03-04".parse::<Date>().is_err());
        assert!("2017-3-4".parse::<Date>().is_err());
        assert!("20170304".parse::<Date>().is_err());
        assert!("".parse::<Date>().is_err());
        assert!("nota-ni-nt".parse::<Date>().is_err());

        // Impossible dates.
        assert!("2017-13-04".parse::<Date>().is_err());
        assert!("2017-03-32".parse::<Date>().is_err());
        assert!("2017-00-04".parse::<Date>().is_err());
        assert!("2017-03-00".parse::<Date>().is_err());
    }

    #[test]
    fn test_date_ordering() {
        let d1 = "2017-01-12".parse::<Date>().unwrap();
        let d2 = "2016-01-12".parse::<Date>().unwrap();
        let d3 = "2017-01-13".parse::<Date>().unwrap();
        let d4 = "2017-02-11".parse::<Date>().unwrap();

        // True assertions.
        assert!(d1 > d2);
        assert!(d2 < d1);
        assert!(d3 > d1);
        assert!(d4 > d1);
        assert!(d3 < d4);

        // False assertions.
        assert_eq!(d1 < d2, false);
        assert_eq!(d2 > d1, false);
        assert_eq!(d3 < d1, false);
        assert_eq!(d4 < d1, false);
        assert_eq!(d3 > d4, false);

        let d5 = "2017-01-12".parse::<Date>().unwrap();
        assert_eq!(d1, d5);
        assert_ne!(d1, d4);
    }

    #[test]
    fn test_date_display() {
        let date = "2017-03-04".parse::<Date>().unwrap();
        assert_eq!(format!("{}", date), "2017-03-04");
    }
}
