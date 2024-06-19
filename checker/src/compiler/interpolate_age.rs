//! Infers a lifter's Age given surrounding Age-related information.

use colored::*;
use opltypes::*;

use crate::{AllMeetData, EntryIndex, LifterMap};

use std::cmp;
use std::fmt;

/// Holds a minimum and maximum possible BirthDate.
///
/// For purposes of simplicity, the internal Date logic here is not concerned
/// with whether or not a given Date actually exists, and assumes that every
/// month has exactly 31 days. This is valid because we are only concerned with
/// whether a given MeetDate is less than or greater than a (possibly
/// nonexistent) Date.
#[derive(Debug, PartialEq, Eq)]
pub struct BirthDateRange {
    pub min: Date,
    pub max: Date,
}

/// An unrealistically low Date for use as a default minimum.
pub const BDR_DEFAULT_MIN: Date = Date::from_parts(1100, 01, 01);
/// An unrealistically high Date for use as a default maximum.
pub const BDR_DEFAULT_MAX: Date = Date::from_parts(9997, 06, 15);

impl Default for BirthDateRange {
    fn default() -> Self {
        BirthDateRange {
            min: BDR_DEFAULT_MIN,
            max: BDR_DEFAULT_MAX,
        }
    }
}

impl BirthDateRange {
    pub fn new(min: Date, max: Date) -> BirthDateRange {
        assert!(min <= max);
        BirthDateRange { min, max }
    }
}

impl fmt::Display for BirthDateRange {
    /// Used for --debug-age output.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

/// Named return enum from the BirthDateRange narrow functions, for clarity.
#[derive(Debug, PartialEq, Eq)]
pub enum NarrowResult {
    /// Returned if the new range information was successfully integrated.
    Integrated,
    /// Returned if the new data conflicted with the known range.
    Conflict,
}

/// Helper function: increments a Date by a single day.
///
/// For simplicity, because it doesn't matter in this context, every month
/// is assumed to have exactly 31 days.
fn next_day(date: Date) -> Date {
    let (mut year, mut month, mut day) = (date.year(), date.month(), date.day());
    day += 1;
    if day > 31 {
        day = 1;
        month += 1;
    }
    if month > 12 {
        month = 1;
        year += 1;
    }
    Date::from_parts(year, month, day)
}

impl BirthDateRange {
    /// Shorthand constructor for use in test code.
    #[cfg(test)]
    pub fn at(min: Option<(u32, u32, u32)>, max: Option<(u32, u32, u32)>) -> BirthDateRange {
        let default = BirthDateRange::default();
        BirthDateRange::new(
            min.map(|(y, m, d)| Date::from_parts(y, m, d))
                .unwrap_or(default.min),
            max.map(|(y, m, d)| Date::from_parts(y, m, d))
                .unwrap_or(default.max),
        )
    }

    /// Returns the Age on a given Date given the known range.
    pub fn age_on(&self, date: Date) -> Age {
        // Get exact ages with respect to the bounds.
        // The minimum Age comes from the maximum BirthDate.
        let min_inferred = self.max.age_on(date).unwrap_or(Age::None);
        // The maximum Age comes from the minimum BirthDate.
        let max_inferred = self.min.age_on(date).unwrap_or(Age::None);

        // If they match, return that Age::Exact.
        if min_inferred == max_inferred {
            return min_inferred;
        }

        // If they are off-by-one, return an Age::Approximate.
        let min_num = min_inferred.to_u8_option().unwrap_or(u8::MIN);
        let max_num = max_inferred.to_u8_option().unwrap_or(u8::MAX);
        if u32::from(min_num) + 1 == u32::from(max_num) {
            return Age::Approximate(min_num);
        }

        // The range was too wide to infer a specific Age.
        Age::None
    }

    /// Interprets the BirthDateRange as a BirthYearRange.
    pub fn as_birthyearrange(&self) -> BirthYearRange {
        BirthYearRange {
            min_year: self.min.year() as _,
            max_year: self.max.year() as _,
        }
    }

    /// Intersects this BirthDateRange with another.
    pub fn intersect(&mut self, other: &BirthDateRange) -> NarrowResult {
        if self.min > other.max || other.min > self.max {
            NarrowResult::Conflict
        } else {
            self.min = cmp::max(self.min, other.min);
            self.max = cmp::min(self.max, other.max);
            NarrowResult::Integrated
        }
    }

    /// Narrows the range by a known BirthDate.
    pub fn narrow_by_birthdate(&mut self, birthdate: Date) -> NarrowResult {
        if birthdate < self.min || birthdate > self.max {
            return NarrowResult::Conflict;
        }
        self.min = birthdate;
        self.max = birthdate;
        NarrowResult::Integrated
    }

    /// Narrows the range by a known BirthYear.
    pub fn narrow_by_birthyear_range(&mut self, range: BirthYearRange) -> NarrowResult {
        let min_yeardate = Date::from_parts(range.min_year as u32, 01, 01); // Jan 1.
        let max_yeardate = Date::from_parts(range.max_year as u32, 12, 31); // Dec 31.

        let birthyear_range = BirthDateRange::new(min_yeardate, max_yeardate);
        self.intersect(&birthyear_range)
    }

    /// Narrows the range by a known Age on a specific Date.
    pub fn narrow_by_age(&mut self, age: Age, on_date: Date) -> NarrowResult {
        let (year, month, day) = (on_date.year(), on_date.month(), on_date.day());
        match age {
            Age::Exact(age) => {
                let age = u32::from(age);

                // The greatest possible BirthDate is if their birthday is that day.
                let max = Date::from_parts(year - age, month, day);

                // The least possible BirthDate is if their birthday is the next day.
                let min = next_day(Date::from_parts(year - age - 1, month, day));

                self.intersect(&BirthDateRange::new(min, max))
            }
            Age::Approximate(age) => {
                let age = u32::from(age);

                // The greatest possible BirthDate is if the lifter is younger,
                // and that day is their birthday.
                let max = Date::from_parts(year - age, month, day);

                // The least possible BirthDate is if the lifter is older,
                // and their birthday is the next day.
                let min = next_day(Date::from_parts(year - age - 2, month, day));

                self.intersect(&BirthDateRange::new(min, max))
            }
            Age::None => NarrowResult::Integrated,
        }
    }

    /// Narrows the range by a known AgeRange on a specific Date.
    pub fn narrow_by_range(&mut self, min: Age, max: Age, on_date: Date) -> NarrowResult {
        let (year, month, day) = (on_date.year(), on_date.month(), on_date.day());

        // Determine the maximum BirthDate from the lower Age (they are younger).
        let birthdate_max = match min {
            Age::Exact(age) | Age::Approximate(age) => {
                // The greatest possible BirthDate is if their birthday is that day.
                // In the case of an Approximate, the lifter is the younger option.
                Date::from_parts(year - u32::from(age), month, day)
            }
            Age::None => BDR_DEFAULT_MAX,
        };

        // Determine the minimum BirthDate from the greater Age (they are older).
        let birthdate_min = match max {
            Age::Exact(age) => {
                let age = u32::from(age);
                // The least possible BirthDate is if their birthday is the next day.
                next_day(Date::from_parts(year - age - 1, month, day))
            }
            Age::Approximate(age) => {
                let age = u32::from(age);
                // The least possible BirthDate is if their birthday is the next day,
                // assuming that they are as old as allowed.
                next_day(Date::from_parts(year - age - 2, month, day))
            }
            Age::None => BDR_DEFAULT_MIN,
        };

        let range = BirthDateRange::new(birthdate_min, birthdate_max);
        self.intersect(&range)
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
pub fn trace_integrated<T>(
    debug: bool,
    range: &BirthDateRange,
    fieldname: &str,
    field: &T,
    path: &Option<String>,
) where
    T: fmt::Display,
{
    if debug {
        println!(
            "{} {} {} {} {} {} {}",
            "Narrowed to".green(),
            range,
            "by".green(),
            fieldname,
            field,
            "in".green(),
            path.as_ref().unwrap()
        );
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
pub fn trace_conflict<T>(
    debug: bool,
    range: &BirthDateRange,
    meetdate: Date,
    fieldname: &str,
    field: &T,
    path: &Option<String>,
) where
    T: fmt::Display,
{
    if debug {
        let age_on: Option<String> = match range.age_on(meetdate) {
            Age::None => None,
            age => Some(format!("{age:?}")),
        };

        println!(
            "{} {} {} {} {}{}{}{}",
            "Conflict with".bold().red(),
            fieldname,
            field,
            "in".bold().red(),
            path.as_ref().unwrap(),
            if age_on.is_some() {
                " -- expected ".bold().red()
            } else {
                "".red()
            },
            if age_on.is_some() { "Age " } else { "" },
            age_on.unwrap_or_default()
        );
    }
}

/// Determines a minimal BirthDateRange consistent with all given Entries.
///
/// If no consistent BirthDateRange could be determined,
/// `BirthDateRange::default()` is returned.
///
/// Executes in `O(n)` over the indices list.
fn birthdate_range(
    meetdata: &mut AllMeetData,
    indices: &[EntryIndex],
    debug: bool,
) -> BirthDateRange {
    let unknown = BirthDateRange::default();
    let mut range = BirthDateRange::default();
    for &index in indices {
        // Extract the MeetDate first. Because of the borrow checker, the Meet and Entry
        // structs cannot be referenced simultaneously.
        let mdate: Date = meetdata.meet(index).date;

        // Get the MeetPath for more helpful debugging output.
        // Cloning is OK since this is only for a few entries for one lifter.
        let path: Option<String> = if debug {
            Some(meetdata.meet(index).path.clone())
        } else {
            None
        };

        let entry = meetdata.entry(index);

        // Narrow by BirthDate.
        if let Some(birthdate) = entry.birthdate {
            if range.narrow_by_birthdate(birthdate) == NarrowResult::Conflict {
                trace_conflict(debug, &range, mdate, "BirthDate", &birthdate, &path);
                return unknown;
            }
            trace_integrated(debug, &range, "BirthDate", &birthdate, &path);
        }

        // Narrow by BirthYearRange.
        if !entry.birthyearrange.is_default() {
            let byr = entry.birthyearrange;
            if range.narrow_by_birthyear_range(byr) == NarrowResult::Conflict {
                trace_conflict(debug, &range, mdate, "BirthYearRange", &byr, &path);
                return unknown;
            }
            trace_integrated(debug, &range, "BirthYearRange", &byr, &path);
        }

        // Narrow by Age.
        if entry.age != Age::None {
            if range.narrow_by_age(entry.age, mdate) == NarrowResult::Conflict {
                trace_conflict(debug, &range, mdate, "Age", &entry.age, &path);
                return unknown;
            }
            trace_integrated(debug, &range, "Age", &entry.age, &path);
        }

        // Narrow by AgeRange.
        if entry.agerange.min.is_some() || entry.agerange.max.is_some() {
            if range.narrow_by_range(entry.agerange.min, entry.agerange.max, mdate)
                == NarrowResult::Conflict
            {
                trace_conflict(debug, &range, mdate, "AgeRange", &entry.agerange, &path);
                return unknown;
            }
            trace_integrated(debug, &range, "AgeRange", &entry.agerange, &path);
        }
    }

    if debug {
        println!("{} {}", "Final range".bold().green(), range);
    }
    range
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_inference<T>(debug: bool, fieldname: &str, field: &T, date: Date)
where
    T: fmt::Debug,
{
    if debug {
        println!(
            "{} {} {:?} {} {}",
            "Inferred".bold().purple(),
            fieldname,
            field,
            "on".bold().purple(),
            date
        );
    }
}

/// Given a known BirthDateRange, calculate the lifter's `Age` in each Entry.
///
/// The BirthDateRange was already validated by `birthdate_range()`,
/// so it is guaranteed to be consistent over all the Entries.
///
/// Executes in `O(n)` over the indices list.
fn infer_from_range(
    meetdata: &mut AllMeetData,
    indices: &[EntryIndex],
    range: BirthDateRange,
    debug: bool,
) {
    for &index in indices {
        let mdate: Date = meetdata.meet(index).date;
        let entry = meetdata.entry_mut(index);

        let entry_had_exact_age = entry.age.is_exact();
        let age_on_date = range.age_on(mdate);

        // Update the lifter's Age.
        match age_on_date {
            Age::Exact(_) | Age::Approximate(_) => {
                // Only overwrite Approximate Ages.
                if !entry.age.is_exact() {
                    trace_inference(debug, "Age", &age_on_date, mdate);
                    entry.age = age_on_date;
                }
            }
            Age::None => (),
        };

        // Update the AgeRange to match the Age, if applicable.
        //
        // If the entry initially had an Age::Approximate, the AgeRange matched
        // by previous information (and set by the checker) may be different
        // than the current best match.
        if entry.agerange.is_none() || !entry_had_exact_age {
            entry.agerange = AgeRange::from(age_on_date);
            if !entry.agerange.is_none() {
                trace_inference(debug, "AgeRange (via Age)", &entry.agerange, mdate);
            }
        }

        // If no specific Age is known, maybe Division information
        // can be used to at least find a range.
        if entry.agerange.is_none() {
            // The minimum age is from the maximum BirthDate.
            let age_min = range.max.age_on(mdate).unwrap_or(Age::None);
            let age_max = range.min.age_on(mdate).unwrap_or(Age::None);
            entry.agerange = AgeRange::from((age_min, age_max));
            if !entry.agerange.is_none() {
                trace_inference(debug, "AgeRange (via Range)", &entry.agerange, mdate);
            }
        }

        // Update the BirthYearRange.
        entry.birthyearrange = entry.birthyearrange.intersect(range.as_birthyearrange());

        // Update the BirthYearClass from the BirthYearRange, if possible.
        if entry.birthyearclass == BirthYearClass::None {
            let byr = range.as_birthyearrange();
            entry.birthyearclass = BirthYearClass::from_range(byr, mdate.year());
            if entry.birthyearclass != BirthYearClass::None {
                let message = "BirthYearClass (via BirthYearRange)";
                trace_inference(debug, message, &entry.birthyearclass, mdate);
            }
        }
    }
}

/// Age interpolation for a single lifter's entries.
fn interpolate_age_single_lifter(meetdata: &mut AllMeetData, indices: &[EntryIndex], debug: bool) {
    // Attempt to determine bounds for a BirthDate. O(indices).
    let range = birthdate_range(meetdata, indices, debug);

    // If found, attempt to apply those bounds. O(indices).
    if range != BirthDateRange::default() {
        infer_from_range(meetdata, indices, range, debug);
    }
}

/// Public-facing entry point for debugging a single lifter's interpolation.
pub fn interpolate_age_debug_for(
    meetdata: &mut AllMeetData,
    liftermap: &LifterMap,
    username: &Username,
) {
    match liftermap.get(username) {
        Some(indices) => interpolate_age_single_lifter(meetdata, indices, true),
        None => println!("Username '{username}' not found"),
    }
}

/// Attempts to infer BirthDate range for each lifter, used to assign Age
/// values.
pub fn interpolate_age(meetdata: &mut AllMeetData, liftermap: &LifterMap) {
    for indices in liftermap.values() {
        // Interpolation requires multiple entries.
        if indices.len() >= 2 {
            interpolate_age_single_lifter(meetdata, indices, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use NarrowResult::{Conflict, Integrated};

    #[test]
    fn range_narrow_by_birthdate() {
        let birthdate = Date::from_parts(1967, 02, 03);

        // Test a BirthDate against unknown bounds.
        let mut bdr = BirthDateRange::default();
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Integrated);
        assert_eq!(bdr.min, birthdate);
        assert_eq!(bdr.max, birthdate);

        // Test a BirthDate that narrows an upper bound.
        let mut bdr = BirthDateRange::at(None, Some((2019, 04, 24)));
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Integrated);
        assert_eq!(bdr.min, birthdate);
        assert_eq!(bdr.max, birthdate);

        // Test a BirthDate that conflicts with an upper bound.
        let mut bdr = BirthDateRange::at(None, Some((1967, 02, 02)));
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Conflict);

        // Test a BirthDate that narrows a lower bound.
        let mut bdr = BirthDateRange::at(Some((1955, 02, 03)), None);
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Integrated);
        assert_eq!(bdr.min, birthdate);
        assert_eq!(bdr.max, birthdate);

        // Test a BirthDate that conflicts with a lower bound.
        let mut bdr = BirthDateRange::at(Some((1967, 02, 04)), None);
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Conflict);

        // Test a BirthDate that provides no additional new information.
        let mut bdr = BirthDateRange::at(Some((1967, 02, 03)), Some((1967, 02, 03)));
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Integrated);
        assert_eq!(bdr.min, birthdate);
        assert_eq!(bdr.max, birthdate);
    }

    #[test]
    fn range_narrow_by_age() {
        // Test an Age::Exact against unknown bounds.
        let mut bdr = BirthDateRange::default();
        let date = Date::from_parts(2019, 01, 04);
        assert_eq!(bdr.narrow_by_age(Age::Exact(30), date), Integrated);
        assert_eq!(bdr.min, Date::from_parts(1988, 01, 05));
        assert_eq!(bdr.max, Date::from_parts(1989, 01, 04));

        // Test an Age::Approximate against unknown bounds.
        let mut bdr = BirthDateRange::default();
        let date = Date::from_parts(2019, 01, 04);
        assert_eq!(bdr.narrow_by_age(Age::Approximate(30), date), Integrated);
        assert_eq!(bdr.min, Date::from_parts(1987, 01, 05));
        assert_eq!(bdr.max, Date::from_parts(1989, 01, 04));

        // Test December 31st roll-over.
        let mut bdr = BirthDateRange::default();
        let date = Date::from_parts(2018, 12, 31);
        assert_eq!(bdr.narrow_by_age(Age::Exact(30), date), Integrated);
        assert_eq!(bdr.min, Date::from_parts(1988, 01, 01));
        assert_eq!(bdr.max, Date::from_parts(1988, 12, 31));
    }

    #[test]
    fn range_narrow_by_range() {
        // Basic sanity test.
        let mut bdr = BirthDateRange::default();
        let date = Date::from_parts(2019, 01, 04);
        let (min, max) = (Age::Exact(30), Age::Exact(34));
        assert_eq!(bdr.narrow_by_range(min, max, date), Integrated);
        assert_eq!(bdr.min, Date::from_parts(1984, 01, 05));
        assert_eq!(bdr.max, Date::from_parts(1989, 01, 04));

        // Regression test from Andrey Malanichev.
        // The Division is 0-17~, and Andrey was 18.
        let mut bdr = BirthDateRange::at(Some((1983, 03, 16)), Some((1983, 03, 16)));
        let date = Date::from_parts(2001, 07, 26);
        let (min, max) = (Age::Exact(0), Age::Approximate(17));
        assert_eq!(bdr.narrow_by_range(min, max, date), Integrated);
    }
}
