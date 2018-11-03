//! Defines the `Date` field for the `meets` table.

use serde;
use serde::de::{self, Deserialize, Visitor};

use std::fmt;
use std::num;
use std::str::FromStr;

use Age;

/// Our data uses imprecise dates in the "YYYY-MM-DD" format,
/// with no timezone or time data.
/// Dates in this format can be stored as a `u32` with value YYYYMMDD.
/// This format is compact and remains human-readable.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Serialize)]
pub struct Date(u32);

impl Date {
    /// Creates a Date object from its exact internal representation.
    ///
    /// FIXME: Using this constructor bypasses error checks.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Date;
    /// let date = Date::from_u32(1988_02_16);
    /// assert_eq!(date.year(), 1988);
    /// assert_eq!(date.month(), 2);
    /// assert_eq!(date.day(), 16);
    /// ```
    #[inline]
    pub fn from_u32(u: u32) -> Date {
        Date(u)
    }

    /// Returns the year as an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Date;
    /// let date = "1988-02-16".parse::<Date>().unwrap();
    /// assert_eq!(date.year(), 1988);
    /// ```
    #[inline]
    pub fn year(self) -> u32 {
        self.0 / 10_000
    }

    /// Returns the month as an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Date;
    /// let date = "1988-02-16".parse::<Date>().unwrap();
    /// assert_eq!(date.month(), 2);
    /// ```
    #[inline]
    pub fn month(self) -> u32 {
        (self.0 / 100) % 100
    }

    /// Returns the day as an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Date;
    /// let date = "1988-02-16".parse::<Date>().unwrap();
    /// assert_eq!(date.day(), 16);
    /// ```
    #[inline]
    pub fn day(self) -> u32 {
        self.0 % 100
    }

    /// Returns the month and day as a combined integer.
    ///
    /// This is useful mostly for age calculations, where the `monthday()`
    /// corresponds to an exact day in the given year.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Date;
    /// let date = "1988-02-16".parse::<Date>().unwrap();
    /// assert_eq!(date.monthday(), 0216);
    /// ```
    #[inline]
    pub fn monthday(self) -> u32 {
        (self.0 % 10000)
    }

    /// Calculates the Age of a lifter on a given date,
    /// where `self` is the lifter's BirthDate.
    ///
    /// # Failures
    ///
    /// Fails if the lifter was not yet born by the given date.
    ///
    /// Fails if the lifter would be more than 256 years old.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::{Age,Date};
    /// let birthdate = "1991-12-16".parse::<Date>().unwrap();
    /// let meetdate  = "2018-11-03".parse::<Date>().unwrap();
    /// assert_eq!(birthdate.age_on(meetdate), Ok(Age::Exact(26)));
    /// ```
    pub fn age_on(self, date: Date) -> Result<Age, &'static str> {
        // The date of comparison must be after the lifter was born.
        if date.0 < self.0 {
            return Err("Lifter was not born yet");
        }

        // The diff of years must be able to fit into the limited range of an Age.
        let years_u32: u32 = date.year() - self.year();
        if years_u32 > u8::max_value().into() {
            return Err("Calculated Age greater than 256");
        }
        let years = years_u32 as u8;

        // If their birthday occured in the most recent year, just diff years.
        if date.monthday() >= self.monthday() {
            Ok(Age::Exact(years))
        } else {
            // This subtraction cannot underflow: the case for the lifter
            // not being born yet was handled above; since the lifter was born,
            // if `years == 0`, then `date.monthday() >= self.monthday()`.
            Ok(Age::Exact(years - 1))
        }
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

    #[test]
    fn test_age_on() {
        // The reference birthdate used in all the tests below.
        let birthdate = "1988-02-16".parse::<Date>().unwrap();

        // Not born yet, obvious by year.
        let date = "1987-01-01".parse::<Date>().unwrap();
        assert!(birthdate.age_on(date).is_err());

        // Not born yet, but in the same year.
        let date = "1988-02-15".parse::<Date>().unwrap();
        assert!(birthdate.age_on(date).is_err());

        // Exact date of birth.
        let date = "1988-02-16".parse::<Date>().unwrap();
        assert_eq!(birthdate.age_on(date).unwrap(), Age::Exact(0));

        // The next day.
        let date = "1988-02-16".parse::<Date>().unwrap();
        assert_eq!(birthdate.age_on(date).unwrap(), Age::Exact(0));

        // The next year, but not yet to 1 years old.
        let date = "1989-02-15".parse::<Date>().unwrap();
        assert_eq!(birthdate.age_on(date).unwrap(), Age::Exact(0));

        // One years old on the day.
        let date = "1989-02-16".parse::<Date>().unwrap();
        assert_eq!(birthdate.age_on(date).unwrap(), Age::Exact(1));

        // A date in the future, before the monthday of birth.
        let date = "2018-01-04".parse::<Date>().unwrap();
        assert_eq!(birthdate.age_on(date).unwrap(), Age::Exact(29));

        // A date in the future, after the monthday of birth.
        let date = "2018-11-03".parse::<Date>().unwrap();
        assert_eq!(birthdate.age_on(date).unwrap(), Age::Exact(30));

        // A date so far in the future that Age would be >256.
        let date = "3018-11-03".parse::<Date>().unwrap();
        assert!(birthdate.age_on(date).is_err());
    }
}
