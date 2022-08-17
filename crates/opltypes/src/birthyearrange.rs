//! Defines the `BirthYearRange` type.

use crate::{Age, Date};

use std::fmt;

/// The BirthYearRange used by the checker for interpreting BirthYear data.
///
/// Because AgeRange uses Age::Exact(), BirthYear-based divisions lose information
/// when translated to exact age ranges.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BirthYearRange {
    pub min_year: u16,
    pub max_year: u16,
}

impl Default for BirthYearRange {
    fn default() -> BirthYearRange {
        BirthYearRange {
            min_year: 0,
            max_year: u16::MAX,
        }
    }
}

impl BirthYearRange {
    /// Create a BirthYearRange from an exact BirthYear.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::BirthYearRange;
    /// let range = BirthYearRange::from_birthyear(1957);
    /// assert_eq!(range.min_year, 1957);
    /// assert_eq!(range.max_year, 1957);
    /// ```
    pub fn from_birthyear(birthyear: u32) -> Self {
        Self {
            min_year: birthyear as u16,
            max_year: birthyear as u16,
        }
    }

    /// Create a BirthYearRange from age range information.
    ///
    /// This function assumes that Approximate values come from Division configurations,
    /// and therefore interprets them as specifying a BirthYear-based class,
    /// not literally an approximate age.
    pub fn from_range(min_age: Age, max_age: Age, on_date: Date) -> Self {
        let on_year = on_date.year() as u16;

        // The lower year is derived from the older Age.
        let min_year: u16 = match max_age {
            Age::Exact(0) => 0,
            Age::Exact(n) => on_year - (n as u16) - 1,
            Age::Approximate(n) => on_year - (n as u16) - 1,
            Age::None => 0,
        };

        // The upper year is derived from the younger Age.
        let max_year: u16 = match min_age {
            Age::Exact(255) => u16::MAX,
            Age::Exact(n) => on_year - (n as u16),
            Age::Approximate(n) => on_year - (n as u16) - 1,
            Age::None => u16::MAX,
        };

        Self { min_year, max_year }
    }

    /// Whether the BirthYearRange is the default, maximal range.
    pub fn is_default(self) -> bool {
        self == BirthYearRange::default()
    }

    /// Gets the exact year of birth if known.
    pub fn exact_birthyear(self) -> Option<u32> {
        if self.min_year == self.max_year {
            Some(self.min_year as u32)
        } else {
            None
        }
    }

    /// Intersects this BirthYearRange with another.
    pub fn intersect(self, other: BirthYearRange) -> BirthYearRange {
        let min_year = self.min_year.max(other.min_year);
        let max_year = self.max_year.min(other.max_year);

        if min_year > max_year {
            Self::default()
        } else {
            Self { min_year, max_year }
        }
    }
}

impl fmt::Display for BirthYearRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.min_year, self.max_year)
    }
}
