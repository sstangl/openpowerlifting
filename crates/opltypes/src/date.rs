//! Defines the `Date` field for the `meets` table.

use arrayvec::ArrayString;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::fmt::{self, Write};
use std::num;
use std::ops;
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

impl Default for Date {
    fn default() -> Self {
        Date::from_parts(1900, 01, 01)
    }
}

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

    // The array has 13 elements so the month (starting from 1) can be an index.
    const DAYS_IN_MONTH: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
    #[inline(always)]
    pub const fn from_parts(year: u32, month: u32, day: u32) -> Date {
        Date((year << Self::YEAR_SHIFT) | (month << Self::MONTH_SHIFT) | (day << Self::DAY_SHIFT))
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
    #[inline(always)]
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
    #[inline(always)]
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
    #[inline(always)]
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
        // Ensure that the month is usable as an index into days_in_month (1-indexed).
        let month = self.month();
        if month > 12 {
            return false;
        }

        let mut max_days = Self::DAYS_IN_MONTH[month as usize];

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
        if years_u32 > u8::MAX.into() {
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

    /// Counts the number of days for the Date in the Common Era (`0001-01-01`).
    ///
    /// This is used to provide a reference point to facilitate date math.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Date;
    /// let first_day = "0001-01-01".parse::<Date>().unwrap();
    /// assert_eq!(first_day.count_days(), 1); // The count is inclusive.
    ///
    /// let special_day = "1982-06-11".parse::<Date>().unwrap();
    /// assert_eq!(special_day.count_days(), 723_707);
    /// ```
    ///
    /// The absolute day count is also used to enable [Date] subtraction:
    /// ```
    /// # use opltypes::Date;
    /// let sixth = "2019-04-06".parse::<Date>().unwrap();
    /// let fifth = "2019-04-05".parse::<Date>().unwrap();
    /// assert_eq!(sixth - fifth, 1);
    /// ```
    pub fn count_days(self) -> u32 {
        // Count previous years, not yet worrying about leap days.
        let whole_previous_year_days = 365 * (self.year() - 1);

        // Get the last year that matters for leap year math.
        let last_maybe_leap_year = if self.month() <= 2 {
            self.year() - 1
        } else {
            self.year()
        };

        // How many leap years have there been?
        let leap_4_years = last_maybe_leap_year / 4;
        let leap_100_years = last_maybe_leap_year / 100;
        let leap_400_years = last_maybe_leap_year / 400;

        // Add one leap day for every 4 leap years;
        // subtract one leap day for every 100 leap years;
        // add one back for every 400 leap years.
        let leap_days = leap_4_years - leap_100_years + leap_400_years;

        // Count the days in the current year.
        // Any leap days in previous months have been counted above.
        let previous_months_iter = Self::DAYS_IN_MONTH[1..(self.month() as usize)].iter();
        let this_year_days: u32 = previous_months_iter.sum::<u32>() + self.day();

        whole_previous_year_days + leap_days + this_year_days
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (y, m, d) = (self.year(), self.month(), self.day());
        write!(f, "{y:04}-{m:02}-{d:02}")
    }
}

impl ops::Sub for Date {
    type Output = i32;

    fn sub(self, other: Date) -> i32 {
        self.count_days() as i32 - other.count_days() as i32
    }
}

impl Serialize for Date {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut buf = ArrayString::<10>::new();
        let (y, m, d) = (self.year(), self.month(), self.day());
        write!(buf, "{y:04}-{m:02}-{d:02}").expect("ArrayString overflow");
        serializer.serialize_str(&buf)
    }
}

#[derive(Debug)]
pub enum ParseDateError {
    FormatError,
    InvalidMonth,
    InvalidDay,
    ParseIntError(num::ParseIntError),
}

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseDateError::FormatError => write!(f, "date not in the correct format"),
            ParseDateError::InvalidMonth => write!(f, "invalid month"),
            ParseDateError::InvalidDay => write!(f, "invalid day"),
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
            return Err(ParseDateError::InvalidMonth);
        }
        if day == 0 || day > 31 {
            return Err(ParseDateError::InvalidDay);
        }

        Ok(Date::from_parts(year, month, day))
    }
}

struct DateVisitor;

impl Visitor<'_> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the format YYYY-MM-DD")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Date, E> {
        Date::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Date, D::Error> {
        deserializer.deserialize_str(DateVisitor)
    }
}

/// Shorthand for constructing a [Date].
///
/// # Example
/// ```
/// # use opltypes::{date, Date};
/// let date: Date = date!(2017-03-04);
/// assert_eq!(date.year(), 2017);
/// assert_eq!(date.month(), 3);
/// assert_eq!(date.day(), 4);
/// ```
///
/// # Safety
/// The date is not checked for validity at construction time.
///
/// # Formatting
/// To prevent rustfmt from reformatting the date to look like subtraction,
/// use `#[rustfmt::skip::macros(date)]`.
#[macro_export]
macro_rules! date {
    ($year:literal - $month:literal - $day:literal) => {
        $crate::Date::from_parts($year, $month, $day)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let date = "2017-03-04".parse::<Date>().unwrap();
        assert_eq!(date.year(), 2017);
        assert_eq!(date.month(), 3);
        assert_eq!(date.day(), 4);
    }

    #[test]
    fn errors() {
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
    fn ordering() {
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
    fn display() {
        let date = "2017-03-04".parse::<Date>().unwrap();
        assert_eq!(format!("{date}"), "2017-03-04");
    }

    #[test]
    fn age_on() {
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

    #[test]
    fn count_days() {
        // 1 leap and 3 non-leap years: 366+(3*365) days.
        let date = "0004-12-31".parse::<Date>().unwrap();
        assert_eq!(date.count_days(), 366 + (3 * 365));

        // 24 leap years ((100 / 4) - (100 / 100)) and 76 non-leap years.
        let date = "0100-12-31".parse::<Date>().unwrap();
        assert_eq!(date.count_days(), (24 * 366) + (76 * 365));

        // 97 leap years ((400 / 4) - (400 / 100) + (400 / 400))
        // and 303 non-leap years.
        let date = "0400-12-31".parse::<Date>().unwrap();
        assert_eq!(date.count_days(), (97 * 366) + (303 * 365));

        // 3 non-leap years and a leap year, but without passing Feb 29.
        let date = "0004-02-28".parse::<Date>().unwrap();
        assert_eq!(date.count_days(), (3 * 365) + 31 + 28);

        // Make sure leap days are not double-counted.
        let before_leap_day = "0004-02-28".parse::<Date>().unwrap();
        let on_leap_day = "0004-02-29".parse::<Date>().unwrap();
        let after_leap_day = "0004-03-01".parse::<Date>().unwrap();
        assert_eq!(on_leap_day - before_leap_day, 1);
        assert_eq!(after_leap_day - before_leap_day, 2);
    }
}
