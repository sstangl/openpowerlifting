//! Defines the `Date` field for the `meets` table.

use serde;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::fmt;
use std::num;
use std::str::FromStr;

use crate::Age;

/// Our data uses imprecise dates in the "YYYY-MM-DD" format,
/// with no timezone or time data.
///
/// Dates are stored as a packed `u32` with 23 bits in use:
///  (YYYY << YEAR_SHIFT) | (MM << MONTH_SHIFT) | (DD << DAY_SHIFT).
///
/// YEAR_SHIFT > MONTH_SHIFT > DAY_SHIFT, so that dates are properly ordered.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct Date(u32);

impl Date {
    // The day occupies the rightmost 5 bits: ceil(log2(31)) = 5.
    const DAY_SHIFT: usize = 0;
    const DAY_MASK: u32 = 0x1f;

    // The month occupies the next 4 bits: ceil(log2(12)) = 4.
    const MONTH_SHIFT: usize = 5;
    const MONTH_MASK: u32 = 0xf;

    // The year occupies the next 14 bits: ceil(log2(9999)) = 14.
    const YEAR_SHIFT: usize = 5 + 4;
    const YEAR_MASK: u32 = 0x3fff;

    /// Creates a Date object from parts.
    ///
    /// FIXME: Using this constructor bypasses error checks.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Date;
    /// let date = Date::from_parts(1988, 02, 16);
    /// assert_eq!(date.year(), 1988);
    /// assert_eq!(date.month(), 2);
    /// assert_eq!(date.day(), 16);
    /// ```
    #[inline]
    pub const fn from_parts(year: u32, month: u32, day: u32) -> Date {
        Date(year << Self::YEAR_SHIFT | month << Self::MONTH_SHIFT | day << Self::DAY_SHIFT)
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
    pub const fn year(self) -> u32 {
        (self.0 >> Self::YEAR_SHIFT) & Self::YEAR_MASK
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
    pub const fn month(self) -> u32 {
        (self.0 >> Self::MONTH_SHIFT) & Self::MONTH_MASK
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
    pub const fn day(self) -> u32 {
        (self.0 >> Self::DAY_SHIFT) & Self::DAY_MASK
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
    pub const fn monthday(self) -> u32 {
        let month = self.month();
        let day = self.day();
        month * 100 + day
    }

    /// Determines whether a date exists in the Gregorian calendar.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Date;
    /// let date = "2000-02-29".parse::<Date>().unwrap();
    /// assert_eq!(date.is_valid(), true);
    ///
    /// let date = "2018-04-31".parse::<Date>().unwrap();
    /// assert_eq!(date.is_valid(), false);
    /// ```
    pub fn is_valid(self) -> bool {
        // The array has 13 elements so the month (starting from 1) can be an index.
        let days_in_month: [u8; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        // Ensure that the month is usable as an index into days_in_month (1-indexed).
        let month = self.month();
        if month > 12 {
            return false;
        }

        let mut max_days = u32::from(days_in_month[month as usize]);

        // February is a special case based on leap year logic.
        if month == 2 {
            let year = self.year();

            // Quoth Wikipedia:
            //  Every year that is exactly divisible by four is a leap year,
            //  except for years that are exactly divisible by 100,
            //  but these centurial years are leap years if they are exactly
            //  divisible by 400.
            let is_leap = (year % 400 == 0) || ((year % 4 == 0) && (year % 100 != 0));

            if is_leap {
                max_days += 1;
            }
        }

        let day = self.day();
        day > 0 && day <= max_days
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
        let (y, m, d) = (self.year(), self.month(), self.day());
        write!(f, "{:04}-{:02}-{:02}", y, m, d)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (y, m, d) = (self.year(), self.month(), self.day());
        // TODO: Write into a stack-allocated fixed-size buffer.
        serializer.serialize_str(&format!("{:04}-{:02}-{:02}", y, m, d))
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

        Ok(Date::from_parts(year, month, day))
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
