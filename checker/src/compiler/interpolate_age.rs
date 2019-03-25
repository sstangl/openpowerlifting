//! Infers a lifter's Age given surrounding Age-related information.

use chrono::{Datelike, Duration, NaiveDate};
use opltypes::*;

use crate::{AllMeetData, EntryIndex, LifterMap};

use std::cmp::{self, Ordering};
use std::fmt;

/// Holds a minimum and maximum possible BirthDate.
///
/// For purposes of simplicity, the internal Date logic here is not concerned
/// with whether or not a given Date actually exists, and assume that every
/// month has exactly 31 days. This is valid because we are only concerned with
/// whether a given MeetDate is less than or greater than a (possibly
/// nonexistent) Date.
#[derive(Debug, PartialEq)]
struct BirthDateRange {
    pub min: Date,
    pub max: Date,
}

/// An unrealistically low Date for use as a default minimum.
const BDR_DEFAULT_MIN: Date = Date::from_u32(1100_01_01);
/// An unrealistically high Date for use as a default maximum.
const BDR_DEFAULT_MAX: Date = Date::from_u32(9997_06_15);

impl Default for BirthDateRange {
    fn default() -> Self {
        BirthDateRange {
            min: BDR_DEFAULT_MIN,
            max: BDR_DEFAULT_MAX,
        }
    }
}

impl fmt::Display for BirthDateRange {
    /// Used for --debug-age output.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

/// Named return enum from the BirthDateRange narrow functions, for clarity.
#[derive(Debug, PartialEq)]
enum NarrowResult {
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
    Date::from_u32(year * 1_00_00 + month * 1_00 + day)
}

impl BirthDateRange {
    /// Shorthand constructor for use in test code.
    #[cfg(test)]
    pub fn at(min: Option<u32>, max: Option<u32>) -> BirthDateRange {
        let default = BirthDateRange::default();
        BirthDateRange {
            min: min.map(|x| Date::from_u32(x)).unwrap_or(default.min),
            max: max.map(|x| Date::from_u32(x)).unwrap_or(default.max),
        }
    }

    /// Returns the Age on a given Date given the known range.
    pub fn age_on(&self, date: Date) -> Age {
        // Get exact ages with respect to the bounds.
        let min_inferred = self.min.age_on(date).unwrap_or(Age::None);
        let max_inferred = self.max.age_on(date).unwrap_or(Age::None);

        // If they match, return that Age::Exact.
        if min_inferred == max_inferred {
            return min_inferred;
        }

        // If they are off-by-one, return an Age::Approximate.
        let min_num = min_inferred.to_u8_option().unwrap_or(std::u8::MIN) as u32;
        let max_num = max_inferred.to_u8_option().unwrap_or(std::u8::MAX) as u32;
        if min_num == max_num + 1 {
            return Age::Approximate(min_num as u8);
        }

        // The range was too wide to infer a specific Age.
        Age::None
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
    pub fn narrow_by_birthyear(&mut self, birthyear: u32) -> NarrowResult {
        let year_in_date: u32 = birthyear * 1_00_00;
        let min_yeardate = Date::from_u32(year_in_date + 01_01); // Jan 1.
        let max_yeardate = Date::from_u32(year_in_date + 12_31); // Dec 31.

        let birthyear_range = BirthDateRange {
            min: min_yeardate,
            max: max_yeardate,
        };
        self.intersect(&birthyear_range)
    }

    /// Narrows the range by a known Age on a specific Date.
    pub fn narrow_by_age(&mut self, age: Age, on_date: Date) -> NarrowResult {
        let (year, monthday) = (on_date.year(), on_date.monthday());
        match age {
            Age::Exact(age) => {
                let age = age as u32;

                // The greatest possible BirthDate is if their birthday is that day.
                let max = Date::from_u32((year - age) * 1_00_00 + monthday);

                // The least possible BirthDate is if their birthday is the next day.
                let min = next_day(Date::from_u32((year - age - 1) * 1_00_00 + monthday));

                self.intersect(&BirthDateRange { min, max })
            }
            Age::Approximate(age) => {
                let age = age as u32;

                // The greatest possible BirthDate is if the approximate age was
                // an under-estimate (the higher value is correct) and that day
                // is their birthday.
                let max = Date::from_u32((year - age + 1) * 1_00_00 + monthday);

                // The least possible BirthDate is if the lower bound of the age
                // was correct and their birthday is the next day.
                let min = next_day(Date::from_u32((year - age - 1) * 1_00_00 + monthday));

                self.intersect(&BirthDateRange { min, max })
            }
            Age::None => NarrowResult::Integrated,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AgeData {
    pub age: Age,
    pub minage: Age,
    pub maxage: Age,
    pub birthyear: Option<u32>,
    pub birthdate: Option<Date>,
    pub date: Date,
}

impl PartialOrd for AgeData {
    fn partial_cmp(&self, other: &AgeData) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AgeData {
    fn cmp(&self, other: &AgeData) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl Default for AgeData {
    fn default() -> AgeData {
        AgeData {
            age: Age::None,
            minage: Age::None,
            maxage: Age::None,
            date: Date::from_u32(0000_01_01),
            birthyear: None,
            birthdate: None,
        }
    }
}

// Get the difference in years between 2 dates
fn year_diff(date1: Date, date2: Date) -> u8 {
    if date1.year() > date2.year() {
        if date1.monthday() >= date2.monthday() {
            (date1.year() - date2.year()) as u8
        } else {
            (date1.year() - date2.year() - 1) as u8
        }
    } else if date1.year() < date2.year() {
        if date2.monthday() >= date1.monthday() {
            (date2.year() - date1.year()) as u8
        } else {
            (date2.year() - date1.year() - 1) as u8
        }
    } else {
        0
    }
}

//  Translate age information to be from a given year
fn shift_age(age: u8, reference_year: u32, entry_year: u32) -> u8 {
    (age as i32 + (reference_year as i32 - entry_year as i32)) as u8
}

//  Offset a date by a given number of years
fn offset_date(initial_date: Date, offset: i32) -> Date {
    Date::from_u32(
        ((initial_date.year() as i32 + offset) as u32) * 100_00 + initial_date.monthday(),
    )
}

// Estimates the range that a lifters birthday lies within
// Uses a variable bd_range with the variables as follows:
// bdrange.0: Date at which age is <= bd_range.2
// bdrange.1: Date at which age is >= bd_range.3
fn estimate_birthdate(entries: &[AgeData]) -> (Date, Date) {
    // Ranges used to bound the birthdate
    // bd_range = [Min Date,Max Date,Age at min,Age at max]
    let mut bd_range: (Date, Date, u8, u8) = (
        Date::from_u32(0000_01_01),
        Date::from_u32(9999_12_31),
        255,
        0,
    );

    // So we can compare ages as if they were from the same year
    let reference_year: u32;

    if !entries.is_empty() {
        reference_year = entries[0].date.year();
    } else {
        return (bd_range.0, bd_range.1);
    }

    for entry in entries {
        let entry_age: Age;

        // If the lifter has a recorded birthdate use that
        if entry.birthdate.is_some() {
            return (entry.birthdate.unwrap(), entry.birthdate.unwrap());
        }

        // If the lifter has a birthday create an approximate age so we can reuse the
        // logic
        if entry.birthyear.is_some() && entry.age != Age::None {
            entry_age = Age::Approximate(
                (entry.date.year() - entry.birthyear.unwrap() - 1) as u8,
            );
        } else {
            entry_age = entry.age;
        }

        // Use age to tighten our birthdate bound
        match entry_age {
            Age::Exact(age) => {
                let shifted_age: u8;
                let shifted_date: Date;

                // Shift the age and date to be from reference_year
                shifted_age = shift_age(age, reference_year, entry.date.year());
                shifted_date =
                    Date::from_u32(reference_year * 100_00 + entry.date.monthday());

                // Ages & dates shifted relative to the existing bd_range
                let mut shifted_date_min: Date = shifted_date;
                let mut shifted_date_max: Date = shifted_date;

                let mut shifted_age_min = shifted_age;
                let mut shifted_age_max = shifted_age;

                // if we're greater than the lower bound or equal to the upper bound, try
                // subtracting a year
                if shifted_age_min > bd_range.2 || shifted_age_min == bd_range.3 {
                    shifted_date_min = offset_date(shifted_date_min, -1);
                    shifted_age_min = shifted_age_min - 1;
                }

                if (shifted_date_min > bd_range.0 && shifted_age_min == bd_range.2)
                    || (shifted_age_min < bd_range.2 && shifted_age_min != bd_range.3)
                {
                    bd_range =
                        (shifted_date_min, bd_range.1, shifted_age_min, bd_range.3);
                }

                if shifted_age_max < bd_range.3 || shifted_age_max == bd_range.2 {
                    // if we're smaller than the upper bound, try adding a year
                    shifted_date_max = offset_date(shifted_date_max, 1);
                    shifted_age_max = shifted_age_max + 1;
                }

                if (shifted_date_max < bd_range.1 && shifted_age_max == bd_range.3)
                    || (shifted_age_max > bd_range.3 && shifted_age_max != bd_range.2)
                {
                    bd_range =
                        (bd_range.0, shifted_date_max, bd_range.2, shifted_age_max);
                }
            }
            Age::Approximate(age) => {
                let bd_min = Date::from_u32((reference_year - 1) * 100_00 + 1231);
                let bd_max = Date::from_u32(reference_year * 100_00 + 1231);

                // Ages & dates shifted relative to the existing bd_range
                let mut shifted_date_min: Date = bd_min;
                let mut shifted_date_max: Date = bd_max;

                let mut shifted_age_min =
                    shift_age(age, reference_year, entry.date.year());
                let mut shifted_age_max = shifted_age_min + 1;

                if shifted_age_max < bd_range.3 {
                    // if we're smaller than the upper bound, try adding a year
                    shifted_date_max = offset_date(shifted_date_max, 1);
                    shifted_age_max = shifted_age_max + 1;
                }

                if (shifted_date_max < bd_range.1 && shifted_age_max == bd_range.3)
                    || (shifted_age_max > bd_range.3 && shifted_age_max != bd_range.2)
                {
                    bd_range =
                        (bd_range.0, shifted_date_max, bd_range.2, shifted_age_max);
                }

                if shifted_age_min > bd_range.2 {
                    // if we're greater than the lower bound, try subtracting a year
                    shifted_date_min = offset_date(shifted_date_min, -1);
                    shifted_age_min = shifted_age_min - 1;
                }

                if (shifted_date_min > bd_range.0 && shifted_age_min == bd_range.2)
                    || (shifted_age_min < bd_range.2 && shifted_age_min != bd_range.3)
                {
                    bd_range =
                        (shifted_date_min, bd_range.1, shifted_age_min, bd_range.3);
                }
            }
            Age::None => (),
        }

        // Use minage to tighten our birthdate bound slightly
        match entry.minage {
            Age::Exact(minage) => {
                // Ages & dates shifted relative to the existing bd_range
                let mut shifted_date_max: Date =
                    Date::from_u32(reference_year * 100_00 + entry.date.monthday());
                let mut shifted_age_max =
                    shift_age(minage, reference_year, entry.date.year());

                if shifted_age_max < bd_range.3 || shifted_age_max == bd_range.2 {
                    // if we're smaller than the upper bound, try adding a year
                    shifted_date_max = offset_date(shifted_date_max, 1);
                    shifted_age_max = shifted_age_max + 1;
                }

                if (shifted_date_max < bd_range.1 && shifted_age_max == bd_range.3)
                    || (shifted_age_max > bd_range.3 && shifted_age_max != bd_range.2)
                {
                    bd_range =
                        (bd_range.0, shifted_date_max, bd_range.2, shifted_age_max);
                }
            }
            Age::Approximate(minage) => {
                // Shift the age and date to be from reference_year
                let shifted_minage = shift_age(minage, reference_year, entry.date.year());
                let shifted_date = Date::from_u32(reference_year * 100_00 + 1231);

                // Then we have a tighter upper bound on their birthday
                if shifted_minage > bd_range.3 {
                    bd_range = (bd_range.0, shifted_date, bd_range.2, shifted_minage + 1);
                }
            }
            Age::None => (),
        }

        // Use maxage to tighten our birthdate bound slightly
        match entry.maxage {
            Age::Exact(maxage) => {
                // Ages & dates shifted relative to the existing bd_range
                let mut shifted_date_min: Date =
                    Date::from_u32(reference_year * 100_00 + entry.date.monthday());
                let mut shifted_age_min =
                    shift_age(maxage, reference_year, entry.date.year());

                // if we're greater than the lower bound or equal to the upper bound, try
                // subtracting a year
                if shifted_age_min > bd_range.2 || shifted_age_min == bd_range.3 {
                    shifted_date_min = offset_date(shifted_date_min, -1);
                    shifted_age_min = shifted_age_min - 1;
                }

                if (shifted_date_min > bd_range.0 && shifted_age_min == bd_range.2)
                    || (shifted_age_min < bd_range.2 && shifted_age_min != bd_range.3)
                {
                    bd_range =
                        (shifted_date_min, bd_range.1, shifted_age_min, bd_range.3);
                }
            }
            Age::Approximate(maxage) => {
                // Shift the age and date to be from reference_year
                let shifted_maxage = shift_age(maxage, reference_year, entry.date.year());
                let shifted_date = Date::from_u32((reference_year - 1) * 100_00 + 1231);

                // Then we have a tighter upper bound on their birthday
                if shifted_maxage < bd_range.2 {
                    bd_range = (shifted_date, bd_range.1, shifted_maxage, bd_range.3);
                }
            }
            Age::None => (),
        }
    }

    let start_range: Date;
    let end_range: Date;

    if bd_range.0.year() != 0000 {
        // Add a day to the lower bound so this is actually a birthdate range
        let d = NaiveDate::from_ymd(
            bd_range.0.year() as i32 - bd_range.2 as i32 - 1,
            bd_range.0.month(),
            bd_range.0.day(),
        ) + Duration::days(1);
        start_range =
            Date::from_u32((d.year() as u32) * 100_00 + d.month() * 100 + d.day());
    } else {
        // if we only have an upper bound then return 00000101 for the minimum birthdate
        start_range = Date::from_u32(0000_01_01);
    }
    if bd_range.1.year() != 9999 {
        end_range = Date::from_u32(
            (bd_range.1.year() - bd_range.3 as u32) * 100_00 + bd_range.1.monthday(),
        );
    } else {
        // if we only have a lower bound then return 99991231 for the minimum birthdate
        end_range = Date::from_u32(99991231);
    }

    return (start_range, end_range);
}

// Check if two AgeData are consistent with one another
fn are_entries_consistent(entry1: &AgeData, entry2: &AgeData) -> bool {
    let yd = year_diff(entry1.date, entry2.date);

    // Check that entry1.age is consistent with the data in entry2
    match entry1.age {
        Age::Exact(age1) => {
            match entry2.age {
                Age::Exact(age2) => {
                    if (age1 as i8 - age2 as i8).abs() as u8 != yd {
                        return false;
                    }
                }
                Age::Approximate(age2) => {
                    if (age1 as i8 - age2 as i8).abs() as u8 != yd {
                        return false;
                    }
                }
                Age::None => (),
            }
            match entry2.minage {
                Age::Exact(minage2) => {
                    if ((age1 as i8 - minage2 as i8).abs() as u8) > yd {
                        return false;
                    }
                }
                Age::Approximate(minage2) => {
                    if ((age1 as i8 - minage2 as i8).abs() as u8) > yd {
                        return false;
                    }
                }
                Age::None => (),
            }
            match entry2.maxage {
                Age::Exact(maxage2) => {
                    if ((age1 as i8 - maxage2 as i8).abs() as u8) < yd {
                        return false;
                    }
                }
                Age::Approximate(maxage2) => {
                    if ((age1 as i8 - maxage2 as i8).abs() as u8) < yd {
                        return false;
                    }
                }
                Age::None => (),
            }
            if entry2.birthdate.is_some()
                && entry1.age != entry2.birthdate.unwrap().age_on(entry1.date).unwrap()
            {
                return false;
            }
            if entry2.birthyear.is_some()
                && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1
                && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1 + 1
            {
                return false;
            }
        }
        Age::Approximate(age1) => {
            match entry2.age {
                Age::Exact(age2) => {
                    if (age1 as i8 - age2 as i8).abs() as u8 != yd {
                        return false;
                    }
                }
                Age::Approximate(age2) => {
                    if ((age1 as i8 - age2 as i8).abs() as u8) != yd {
                        return false;
                    }
                }
                Age::None => (),
            }
            match entry2.minage {
                Age::Exact(minage2) => {
                    if ((age1 as i8 - minage2 as i8).abs() as u8) > yd {
                        return false;
                    }
                }
                Age::Approximate(minage2) => {
                    if ((age1 as i8 - minage2 as i8).abs() as u8) > yd {
                        return false;
                    }
                }
                Age::None => (),
            }
            match entry2.maxage {
                Age::Exact(maxage2) => {
                    if ((age1 as i8 - maxage2 as i8).abs() as u8) < yd + 1 {
                        return false;
                    }
                }
                Age::Approximate(maxage2) => {
                    if ((age1 as i8 - maxage2 as i8).abs() as u8) < yd {
                        return false;
                    }
                }
                Age::None => (),
            }
            if entry2.birthdate.is_some() {
                let age_on = entry2
                    .birthdate
                    .unwrap()
                    .age_on(entry1.date)
                    .unwrap()
                    .to_u8_option()
                    .unwrap();
                if age_on != age1 && age_on != age1 + 1 {
                    return false;
                }
            }
            if entry2.birthyear.is_some()
                && (entry1.date.year() - entry2.birthyear.unwrap()) as u8 != age1 + 1
            {
                return false;
            }
        }
        Age::None => (),
    }

    // Check that entry1.minage is consistent with the data in entry2
    match entry1.minage {
        Age::Exact(minage1) => {
            match entry2.age {
                Age::Exact(age2) => {
                    if ((age2 as i8 - minage1 as i8).abs() as u8) > yd {
                        return false;
                    }
                }
                Age::Approximate(age2) => {
                    if ((age2 as i8 - minage1 as i8).abs() as u8) > yd {
                        return false;
                    }
                }
                Age::None => (),
            }
            match entry2.maxage {
                Age::Exact(maxage2) => {
                    if entry1.date < entry2.date && minage1 + yd > maxage2 {
                        return false;
                    }
                }
                Age::Approximate(maxage2) => {
                    if entry1.date < entry2.date && minage1 + yd > maxage2 {
                        return false;
                    }
                }
                Age::None => (),
            }
            if entry2.birthyear.is_some()
                && ((entry1.date.year() - entry2.birthyear.unwrap()) as u8) < minage1
            {
                return false;
            }
            if entry2.birthdate.is_some()
                && entry1.minage > entry2.birthdate.unwrap().age_on(entry1.date).unwrap()
            {
                return false;
            }
        }
        Age::Approximate(minage1) => {
            match entry2.age {
                Age::Exact(age2) => {
                    if ((age2 as i8 - minage1 as i8).abs() as u8) > yd {
                        return false;
                    }
                }
                Age::Approximate(age2) => {
                    if ((age2 as i8 - minage1 as i8).abs() as u8) > yd {
                        return false;
                    }
                }
                Age::None => (),
            }

            match entry2.maxage {
                Age::Exact(maxage2) => {
                    if entry1.date < entry2.date && minage1 + yd - 1 > maxage2 {
                        return false;
                    }
                }
                Age::Approximate(maxage2) => {
                    if entry1.date < entry2.date && minage1 + yd > maxage2 {
                        return false;
                    }
                }
                Age::None => (),
            }
            if entry2.birthyear.is_some()
                && ((entry1.date.year() - entry2.birthyear.unwrap()) as u8) < minage1
            {
                return false;
            }
            if entry2.birthdate.is_some()
                && entry1.minage > entry2.birthdate.unwrap().age_on(entry1.date).unwrap()
            {
                return false;
            }
        }
        Age::None => (),
    }

    // Check that entry1.maxage is consistent with the data in entry2
    match entry1.maxage {
        Age::Exact(maxage1) => {
            match entry2.age {
                Age::Exact(age2) => {
                    if ((age2 as i8 - maxage1 as i8).abs() as u8) < yd {
                        return false;
                    }
                }
                Age::Approximate(age2) => {
                    if ((age2 as i8 - maxage1 as i8).abs() as u8) < yd + 1 {
                        return false;
                    }
                }
                Age::None => (),
            }
            match entry2.minage {
                Age::Exact(minage2) => {
                    if entry2.date < entry1.date && minage2 + yd > maxage1 {
                        return false;
                    }
                }
                Age::Approximate(minage2) => {
                    if entry2.date < entry1.date && minage2 + yd - 1 > maxage1 {
                        return false;
                    }
                }
                Age::None => (),
            }

            if entry2.birthyear.is_some()
                && (entry1.date.year() - entry2.birthyear.unwrap() - 1) as u8 > maxage1
            {
                return false;
            }
            if entry2.birthdate.is_some()
                && entry1.maxage < entry2.birthdate.unwrap().age_on(entry1.date).unwrap()
            {
                return false;
            }
        }
        Age::Approximate(maxage1) => {
            match entry2.age {
                Age::Exact(age2) => {
                    if ((age2 as i8 - maxage1 as i8).abs() as u8) < yd {
                        return false;
                    }
                }
                Age::Approximate(age2) => {
                    if ((age2 as i8 - maxage1 as i8).abs() as u8) < yd {
                        return false;
                    }
                }
                Age::None => (),
            }
            match entry2.minage {
                Age::Exact(minage2) => {
                    if entry2.date < entry1.date && minage2 + yd > maxage1 {
                        return false;
                    }
                }
                Age::Approximate(minage2) => {
                    if entry2.date < entry1.date && minage2 + yd > maxage1 {
                        return false;
                    }
                }
                Age::None => (),
            }

            if entry2.birthyear.is_some()
                && (entry1.date.year() - entry2.birthyear.unwrap() - 1) as u8 > maxage1
            {
                return false;
            }
            if entry2.birthdate.is_some()
                && entry1.maxage < entry2.birthdate.unwrap().age_on(entry1.date).unwrap()
            {
                return false;
            }
        }
        Age::None => (),
    }

    // Check that entry1.birthyear is consistent with the data in entry2
    if entry1.birthyear.is_some() {
        match entry2.age {
            Age::Exact(age2) => {
                if (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2
                    && (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2 + 1
                {
                    return false;
                }
            }
            Age::Approximate(age2) => {
                if (entry2.date.year() - entry1.birthyear.unwrap()) as u8 != age2 + 1 {
                    return false;
                }
            }
            Age::None => (),
        }
        match entry2.minage {
            Age::Exact(minage2) => {
                if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) < minage2 {
                    return false;
                }
            }
            Age::Approximate(minage2) => {
                if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) < minage2 {
                    return false;
                }
            }
            Age::None => (),
        }
        match entry2.maxage {
            Age::Exact(maxage2) => {
                if ((entry2.date.year() - entry1.birthyear.unwrap()) as u8) > maxage2 {
                    return false;
                }
            }
            Age::Approximate(maxage2) => {
                if ((entry2.date.year() - entry1.birthyear.unwrap() - 1) as u8) > maxage2
                {
                    return false;
                }
            }
            Age::None => (),
        }
        if entry2.birthyear.is_some()
            && entry1.birthyear.unwrap() != entry2.birthyear.unwrap()
        {
            return false;
        }
        if entry2.birthdate.is_some()
            && entry1.birthyear.unwrap() != entry2.birthdate.unwrap().year()
        {
            return false;
        }
    }

    // Check that entry1.birthdate is consistent with the data in entry2
    if entry1.birthdate.is_some() {
        match entry2.age {
            Age::Exact(_age2) => {
                if entry1.birthdate.unwrap().age_on(entry2.date).unwrap() != entry2.age {
                    return false;
                }
            }
            Age::Approximate(age2) => {
                let age_on = entry1
                    .birthdate
                    .unwrap()
                    .age_on(entry2.date)
                    .unwrap()
                    .to_u8_option()
                    .unwrap();
                if age_on != age2 && age_on != age2 + 1 {
                    return false;
                }
            }
            Age::None => (),
        }

        if entry2.minage != Age::None
            && entry1.birthdate.unwrap().age_on(entry2.date).unwrap() < entry2.minage
        {
            return false;
        }
        if entry2.maxage != Age::None
            && entry1.birthdate.unwrap().age_on(entry2.date).unwrap() > entry2.maxage
        {
            return false;
        }
        if entry2.birthyear.is_some()
            && entry1.birthdate.unwrap().year() != entry2.birthyear.unwrap()
        {
            return false;
        }
        if entry2.birthdate.is_some()
            && entry1.birthdate.unwrap() != entry2.birthdate.unwrap()
        {
            return false;
        }
    }

    true
}

// Check that lifter age data is consistent across several entries
fn is_agedata_consistent(entries: &[AgeData]) -> bool {
    if entries.is_empty() {
        return true;
    }

    // This is O(N^2), there is probably a more efficient way if doing this...
    for ii in 0..entries.len() {
        for jj in ii..entries.len() {
            if !are_entries_consistent(&entries[ii], &entries[jj]) {
                return false;
            }
        }
    }

    true
}

// Interpolate a lifters age information
#[allow(dead_code)]
fn interpolate(entries: &mut [AgeData]) {
    let bd_constraint = estimate_birthdate(entries);

    if is_agedata_consistent(entries) {
        for entry in entries {
            if bd_constraint.0.year() == bd_constraint.1.year() {
                entry.birthyear = Some(bd_constraint.0.year());
            }

            // Then we know the lifters birthdate exactly
            if bd_constraint.0.monthday() == bd_constraint.1.monthday() {
                entry.birthyear = Some(bd_constraint.0.year());
                entry.age = bd_constraint.0.age_on(entry.date).unwrap();
                entry.minage = entry.age;
                entry.maxage = entry.age;
            } else {
                if bd_constraint.0.year() != 0000 {
                    entry.maxage = bd_constraint.0.age_on(entry.date).unwrap();
                } else {
                    entry.maxage = Age::Exact(0);
                }
                if bd_constraint.1.year() != 0000 {
                    entry.minage = bd_constraint.1.age_on(entry.date).unwrap();
                } else {
                    entry.minage = Age::Exact(0);
                }

                if entry.minage == entry.maxage {
                    // Then we have succesfully determined their age
                    entry.age = entry.minage;
                } else if entry.minage.to_u8_option().unwrap() + 1
                    == entry.maxage.to_u8_option().unwrap()
                {
                    // Then we know an approximate age
                    entry.age = Age::Approximate(entry.minage.to_u8_option().unwrap());
                }
            }
        }
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_integrated<T>(
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
            "Narrowed to {} by {} '{}' in '{}'",
            range,
            fieldname,
            field,
            path.as_ref().unwrap()
        );
    }
}

/// Helper function for debug-mode printing to keep the code legible.
#[inline]
fn trace_conflict<T>(debug: bool, fieldname: &str, field: &T, path: &Option<String>)
where
    T: fmt::Display,
{
    if debug {
        println!(
            "Conflict with {} '{}' in '{}'",
            fieldname,
            field,
            path.as_ref().unwrap()
        );
    }
}

/// Determines a minimal BirthDateRange consistent with all given Entries.
///
/// If no consistent BirthDateRange could be determined,
/// `BirthDateRange::default()` is returned.
///
/// Executes in `O(n)` over the indices list.
fn get_birthdate_range(
    meetdata: &mut AllMeetData,
    indices: &[EntryIndex],
    debug: bool,
) -> BirthDateRange {
    let unknown = BirthDateRange::default();
    let mut range = BirthDateRange::default();
    for &index in indices {
        // Extract the MeetDate first. Because of the borrow checker, the Meet and Entry
        // structs cannot be referenced simultaneously.
        let meetdate: Date = meetdata.get_meet(index).date;

        // Get the MeetPath for more helpful debugging output.
        // Cloning is OK since this is only for a few entries for one lifter.
        let path: Option<String> = if debug {
            Some(meetdata.get_meet(index).path.clone())
        } else {
            None
        };

        let entry = meetdata.get_entry(index);

        // Narrow by BirthDate.
        if let Some(birthdate) = entry.birthdate {
            if range.narrow_by_birthdate(birthdate) == NarrowResult::Conflict {
                trace_conflict(debug, "BirthDate", &birthdate, &path);
                return unknown;
            }
            trace_integrated(debug, &range, "BirthDate", &birthdate, &path);
        }

        // Narrow by BirthYear.
        if let Some(birthyear) = entry.birthyear {
            if range.narrow_by_birthyear(birthyear) == NarrowResult::Conflict {
                trace_conflict(debug, "BirthYear", &birthyear, &path);
                return unknown;
            }
            trace_integrated(debug, &range, "BirthYear", &birthyear, &path);
        }

        // Narrow by Age.
        if entry.age != Age::None {
            if range.narrow_by_age(entry.age, meetdate) == NarrowResult::Conflict {
                trace_conflict(debug, "Age", &entry.age, &path);
                return unknown;
            }
            trace_integrated(debug, &range, "Age", &entry.age, &path);
        }
    }

    if debug {
        println!("Final range {}", range);
    }
    range
}

/// Given a known BirthDateRange, calculate the lifter's `Age` in each Entry.
///
/// The BirthDateRange was already validated by `get_birthdate_range()`,
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
        let meetdate: Date = meetdata.get_meet(index).date;
        let entry = meetdata.get_entry_mut(index);

        let age_on_date = range.age_on(meetdate);
        if debug {
            println!("Inferred Age {:?} on {}", age_on_date, meetdate);
        }

        match age_on_date {
            Age::Exact(_) => entry.age = age_on_date,
            Age::Approximate(_) => {
                // Don't overwrite an exact Age with an approximate Age.
                if !entry.age.is_exact() {
                    entry.age = age_on_date;
                }
            }
            Age::None => (),
        };
    }
}

/// Age interpolation for a single lifter's entries.
fn interpolate_age_single_lifter(
    meetdata: &mut AllMeetData,
    indices: &[EntryIndex],
    debug: bool,
) {
    // Attempt to determine bounds for a BirthDate. O(indices).
    let range = get_birthdate_range(meetdata, indices, debug);

    // If found, attempt to apply those bounds. O(indices).
    if range != BirthDateRange::default() {
        infer_from_range(meetdata, indices, range, debug);
    }
}

/// Public-facing entry point for debugging a single lifter's interpolation.
pub fn interpolate_age_debug_for(
    meetdata: &mut AllMeetData,
    liftermap: &LifterMap,
    username: &str,
) {
    match liftermap.get(username) {
        Some(indices) => interpolate_age_single_lifter(meetdata, indices, true),
        None => println!("Username '{}' not found", username),
    }
}

/// Attempts to infer BirthDate range for each lifter, used to assign Age
/// values.
pub fn interpolate_age(meetdata: &mut AllMeetData, liftermap: &LifterMap) {
    for (_username, indices) in liftermap {
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
        let birthdate = Date::from_u32(1967_02_03);

        // Test a BirthDate against unknown bounds.
        let mut bdr = BirthDateRange::default();
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Integrated);
        assert_eq!(bdr.min, birthdate);
        assert_eq!(bdr.max, birthdate);

        // Test a BirthDate that narrows an upper bound.
        let mut bdr = BirthDateRange::at(None, Some(2019_04_24));
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Integrated);
        assert_eq!(bdr.min, birthdate);
        assert_eq!(bdr.max, birthdate);

        // Test a BirthDate that conflicts with an upper bound.
        let mut bdr = BirthDateRange::at(None, Some(1967_02_02));
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Conflict);

        // Test a BirthDate that narrows a lower bound.
        let mut bdr = BirthDateRange::at(Some(1955_02_03), None);
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Integrated);
        assert_eq!(bdr.min, birthdate);
        assert_eq!(bdr.max, birthdate);

        // Test a BirthDate that conflicts with a lower bound.
        let mut bdr = BirthDateRange::at(Some(1967_02_04), None);
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Conflict);

        // Test a BirthDate that provides no additional new information.
        let mut bdr = BirthDateRange::at(Some(1967_02_03), Some(1967_02_03));
        assert_eq!(bdr.narrow_by_birthdate(birthdate), Integrated);
        assert_eq!(bdr.min, birthdate);
        assert_eq!(bdr.max, birthdate);
    }

    #[test]
    fn range_narrow_by_birthyear() {
        // Test a BirthYear against unknown bounds.
        let mut bdr = BirthDateRange::default();
        assert_eq!(bdr.narrow_by_birthyear(1982), Integrated);
        assert_eq!(bdr.min, Date::from_u32(1982_01_01));
        assert_eq!(bdr.max, Date::from_u32(1982_12_31));

        // Test a BirthYear that narrows an upper bound.
        let mut bdr = BirthDateRange::at(None, Some(1983_04_24));
        assert_eq!(bdr.narrow_by_birthyear(1982), Integrated);
        assert_eq!(bdr.min, Date::from_u32(1982_01_01));
        assert_eq!(bdr.max, Date::from_u32(1982_12_31));

        // Test a BirthYear that conflicts with an upper bound.
        let mut bdr = BirthDateRange::at(None, Some(1981_01_01));
        assert_eq!(bdr.narrow_by_birthyear(1982), Conflict);

        // Test a BirthYear that narrows a lower bound.
        let mut bdr = BirthDateRange::at(Some(1981_01_01), None);
        assert_eq!(bdr.narrow_by_birthyear(1982), Integrated);
        assert_eq!(bdr.min, Date::from_u32(1982_01_01));
        assert_eq!(bdr.max, Date::from_u32(1982_12_31));

        // Test a BirthYear that conflicts with a lower bound.
        let mut bdr = BirthDateRange::at(Some(1983_01_01), None);
        assert_eq!(bdr.narrow_by_birthyear(1982), Conflict);

        // Test a BirthYear that entirely contains the known range.
        let mut bdr = BirthDateRange::at(Some(1982_03_04), Some(1982_05_06));
        assert_eq!(bdr.narrow_by_birthyear(1982), Integrated);
        assert_eq!(bdr.min, Date::from_u32(1982_03_04));
        assert_eq!(bdr.max, Date::from_u32(1982_05_06));
    }

    #[test]
    fn range_narrow_by_age() {
        // Test an Age::Exact against unknown bounds.
        let mut bdr = BirthDateRange::default();
        let date = Date::from_u32(2019_01_04);
        assert_eq!(bdr.narrow_by_age(Age::Exact(30), date), Integrated);
        assert_eq!(bdr.min, Date::from_u32(1988_01_05));
        assert_eq!(bdr.max, Date::from_u32(1989_01_04));

        // Test an Age::Approximate against unknown bounds.
        let mut bdr = BirthDateRange::default();
        let date = Date::from_u32(2019_01_04);
        assert_eq!(bdr.narrow_by_age(Age::Approximate(30), date), Integrated);
        assert_eq!(bdr.min, Date::from_u32(1988_01_05));
        assert_eq!(bdr.max, Date::from_u32(1990_01_04));

        // Test December 31st roll-over.
        let mut bdr = BirthDateRange::default();
        let date = Date::from_u32(2018_12_31);
        assert_eq!(bdr.narrow_by_age(Age::Exact(30), date), Integrated);
        assert_eq!(bdr.min, Date::from_u32(1988_01_01));
        assert_eq!(bdr.max, Date::from_u32(1988_12_31));
    }

    ///////////////////////////////////////////////////////////////////////////////
    // Matt's tests below (need to rephrase in terms of new data structures)
    ///////////////////////////////////////////////////////////////////////////////

    // asserts that all permutation of an array give the same birthdate constraint
    // This is a super crappy way of doing this, write something better :P
    fn all_permutation_bd_equal(entries: &[AgeData], bound: (Date, Date)) -> bool {
        let mut entries_copy = entries.to_vec().clone();
        let mut hasfailed = false;

        permute_bd_equal(&mut entries_copy, bound, entries.len(), &mut hasfailed);
        if hasfailed {
            return false;
        }
        true
    }

    // Generating permutation using Heap Algorithm
    fn permute_bd_equal(
        entries: &mut [AgeData],
        bound: (Date, Date),
        n: usize,
        hasfailed: &mut bool,
    ) {
        // if we are at the bottom of the permutation, check if this gives the correct
        // bound
        if n == 1 {
            if estimate_birthdate(entries) != bound {
                *hasfailed = true;
            }
        } else {
            for ii in 0..n {
                permute_bd_equal(entries, bound, n - 1, hasfailed);

                // if n is odd, swap first and last
                // element
                if n % 2 == 1 {
                    let temp: AgeData = entries[n - 1];

                    entries[n - 1] = entries[0];
                    entries[0] = temp;
                } else {
                    // If n is even, swap ith and last element
                    let temp: AgeData = entries[n - 1];

                    entries[n - 1] = entries[ii];
                    entries[ii] = temp;
                }
            }
        }
    }

    // Helper function for generating test data
    fn DateEntry(date: Date) -> AgeData {
        AgeData {
            date,
            ..AgeData::default()
        }
    }

    // Helper function for generating test data
    fn AgeEntry(age: Age, date: Date) -> AgeData {
        AgeData {
            age,
            date,
            ..AgeData::default()
        }
    }

    // Helper function for generating test data
    fn MinAgeEntry(minage: Age, date: Date) -> AgeData {
        AgeData {
            minage,
            date,
            ..AgeData::default()
        }
    }

    // Helper function for generating test data
    fn MaxAgeEntry(maxage: Age, date: Date) -> AgeData {
        AgeData {
            maxage,
            date,
            ..AgeData::default()
        }
    }

    // Helper function for generating test data
    fn BirthYearEntry(birthyear_: u32, date: Date) -> AgeData {
        AgeData {
            birthyear: Some(birthyear_),
            date,
            ..AgeData::default()
        }
    }

    // Helper function for generating test data
    fn BirthDateEntry(birthdate_: Date, date: Date) -> AgeData {
        AgeData {
            birthdate: Some(birthdate_),
            date,
            ..AgeData::default()
        }
    }

    #[test]
    fn test_invalid_exact_age() {
        // Age <-> Age
        let a = AgeEntry(Age::Exact(17), Date::from_u32(19800703));
        let b = AgeEntry(Age::Exact(41), Date::from_u32(20040605));
        let interp_arr = [a, b];
        let interp_arr2 = [b, a];

        // Age <-> Approx Age
        let c = AgeEntry(Age::Exact(17), Date::from_u32(19800703));
        let d = AgeEntry(Age::Approximate(41), Date::from_u32(20040605));
        let interp_arr3 = [c, d];
        let interp_arr4 = [d, c];

        // Age <-> Approx Minage
        let e = AgeEntry(Age::Exact(17), Date::from_u32(19800703));
        let f = MinAgeEntry(Age::Approximate(41), Date::from_u32(20040605));
        let interp_arr5 = [e, f];
        let interp_arr6 = [f, e];

        // Age <-> Exact Minage
        let g = AgeEntry(Age::Exact(17), Date::from_u32(19800703));
        let h = MinAgeEntry(Age::Exact(41), Date::from_u32(20040605));
        let interp_arr7 = [g, h];
        let interp_arr8 = [h, g];

        // Age <-> Approx Maxage
        let i = AgeEntry(Age::Exact(18), Date::from_u32(19800703));
        let j = MaxAgeEntry(Age::Approximate(40), Date::from_u32(20040605));
        let interp_arr9 = [i, j];
        let interp_arr10 = [j, i];

        // Age <-> Exact Maxage
        let k = AgeEntry(Age::Exact(17), Date::from_u32(19800703));
        let l = MaxAgeEntry(Age::Exact(40), Date::from_u32(20040705));
        let interp_arr11 = [k, l];
        let interp_arr12 = [l, k];

        // Age <-> BirthYear
        let m = AgeEntry(Age::Exact(17), Date::from_u32(19800703));
        let n = BirthYearEntry(1964, Date::from_u32(20040605));
        let interp_arr13 = [m, n];
        let interp_arr14 = [n, m];

        // Age <-> BirthDate
        let o = AgeEntry(Age::Exact(17), Date::from_u32(19800703));
        let p = BirthDateEntry(Date::from_u32(19630705), Date::from_u32(20040605));
        let interp_arr15 = [o, p];
        let interp_arr16 = [p, o];

        assert!(!is_agedata_consistent(&interp_arr));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
        assert!(!is_agedata_consistent(&interp_arr5));
        assert!(!is_agedata_consistent(&interp_arr6));
        assert!(!is_agedata_consistent(&interp_arr7));
        assert!(!is_agedata_consistent(&interp_arr8));
        assert!(!is_agedata_consistent(&interp_arr9));
        assert!(!is_agedata_consistent(&interp_arr10));
        assert!(!is_agedata_consistent(&interp_arr11));
        assert!(!is_agedata_consistent(&interp_arr12));
        assert!(!is_agedata_consistent(&interp_arr13));
        assert!(!is_agedata_consistent(&interp_arr14));
        assert!(!is_agedata_consistent(&interp_arr15));
        assert!(!is_agedata_consistent(&interp_arr16));
    }

    #[test]
    fn test_invalid_approx_age() {
        // Age <-> Approx Age
        let a = AgeEntry(Age::Approximate(17), Date::from_u32(19800703));
        let b = AgeEntry(Age::Approximate(41), Date::from_u32(20040605));
        let interp_arr1 = [a, b];
        let interp_arr2 = [b, a];

        // Age <-> Approx Minage
        let c = AgeEntry(Age::Approximate(17), Date::from_u32(19800703));
        let d = MinAgeEntry(Age::Approximate(41), Date::from_u32(20040605));
        let interp_arr3 = [c, d];
        let interp_arr4 = [d, c];

        // Age <-> Exact Minage
        let e = AgeEntry(Age::Approximate(17), Date::from_u32(19800703));
        let f = MinAgeEntry(Age::Exact(42), Date::from_u32(20040605));
        let interp_arr5 = [e, f];
        let interp_arr6 = [f, e];

        // Age <-> Approx Maxage
        let g = AgeEntry(Age::Approximate(18), Date::from_u32(19800703));
        let h = MaxAgeEntry(Age::Approximate(40), Date::from_u32(20040605));
        let interp_arr7 = [g, h];
        let interp_arr8 = [h, g];

        // Age <-> Exact Maxage
        let i = AgeEntry(Age::Approximate(17), Date::from_u32(19800703));
        let j = MaxAgeEntry(Age::Exact(40), Date::from_u32(20040705));
        let interp_arr9 = [i, j];
        let interp_arr10 = [j, i];

        // Age <-> BirthYear
        let k = AgeEntry(Age::Approximate(17), Date::from_u32(19800703));
        let l = BirthYearEntry(1963, Date::from_u32(20040605));
        let interp_arr11 = [k, l];
        let interp_arr12 = [l, k];

        // Age <-> BirthDate
        let m = AgeEntry(Age::Approximate(17), Date::from_u32(19800703));
        let n = BirthDateEntry(Date::from_u32(19630705), Date::from_u32(20040605));
        let interp_arr13 = [m, n];
        let interp_arr14 = [n, m];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
        assert!(!is_agedata_consistent(&interp_arr5));
        assert!(!is_agedata_consistent(&interp_arr6));
        assert!(!is_agedata_consistent(&interp_arr7));
        assert!(!is_agedata_consistent(&interp_arr8));
        assert!(!is_agedata_consistent(&interp_arr9));
        assert!(!is_agedata_consistent(&interp_arr10));
        assert!(!is_agedata_consistent(&interp_arr11));
        assert!(!is_agedata_consistent(&interp_arr12));
        assert!(!is_agedata_consistent(&interp_arr13));
        assert!(!is_agedata_consistent(&interp_arr14));
    }

    #[test]
    fn test_invalid_exact_minage() {
        // Exact Minage <-> Exact Maxage
        let a = MinAgeEntry(Age::Exact(40), Date::from_u32(19800703));
        let b = MaxAgeEntry(Age::Exact(53), Date::from_u32(20040705));

        let interp_arr1 = [a, b];
        let interp_arr2 = [b, a];

        // Exact Minage <-> Approx Maxage
        let c = MinAgeEntry(Age::Exact(40), Date::from_u32(19800703));
        let d = MaxAgeEntry(Age::Approximate(52), Date::from_u32(20040705));

        let interp_arr3 = [c, d];
        let interp_arr4 = [d, c];

        // Exact Minage <-> BirthYear
        let e = MinAgeEntry(Age::Exact(40), Date::from_u32(19800703));
        let f = BirthYearEntry(1941, Date::from_u32(20040705));

        let interp_arr5 = [e, f];
        let interp_arr6 = [f, e];

        // Exact Minage <-> BirthDate
        let g = MinAgeEntry(Age::Exact(40), Date::from_u32(19800703));
        let h = BirthDateEntry(Date::from_u32(19400705), Date::from_u32(20040705));

        let interp_arr7 = [g, h];
        let interp_arr8 = [h, g];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
        assert!(!is_agedata_consistent(&interp_arr5));
        assert!(!is_agedata_consistent(&interp_arr6));
        assert!(!is_agedata_consistent(&interp_arr7));
        assert!(!is_agedata_consistent(&interp_arr8));
    }

    #[test]
    fn test_invalid_approx_minage() {
        // Exact Minage <-> Exact Maxage
        let a = MinAgeEntry(Age::Approximate(40), Date::from_u32(1980_07_03));
        let b = MaxAgeEntry(Age::Exact(53), Date::from_u32(2004_07_05));

        let interp_arr1 = [a, b];
        let interp_arr2 = [b, a];

        // Exact Minage <-> Approx Maxage
        let c = MinAgeEntry(Age::Approximate(40), Date::from_u32(1980_07_03));
        let d = MaxAgeEntry(Age::Approximate(53), Date::from_u32(2004_07_05));

        let interp_arr3 = [c, d];
        let interp_arr4 = [d, c];

        // Exact Minage <-> BirthYear
        let e = MinAgeEntry(Age::Approximate(40), Date::from_u32(1980_07_03));
        let f = BirthYearEntry(1941, Date::from_u32(2004_07_05));

        let interp_arr5 = [e, f];
        let interp_arr6 = [f, e];

        // Exact Minage <-> BirthDate
        let g = MinAgeEntry(Age::Approximate(40), Date::from_u32(1980_07_03));
        let h = BirthDateEntry(Date::from_u32(1940_07_05), Date::from_u32(2004_07_05));

        let interp_arr7 = [g, h];
        let interp_arr8 = [h, g];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
        assert!(!is_agedata_consistent(&interp_arr5));
        assert!(!is_agedata_consistent(&interp_arr6));
        assert!(!is_agedata_consistent(&interp_arr7));
        assert!(!is_agedata_consistent(&interp_arr8));
    }

    #[test]
    fn test_invalid_exact_maxage() {
        // Exact Maxage <-> BirthYear
        let a = MaxAgeEntry(Age::Exact(18), Date::from_u32(1980_07_03));
        let b = BirthYearEntry(1960, Date::from_u32(2004_07_05));

        let interp_arr1 = [a, b];
        let interp_arr2 = [b, a];

        // Exact Maxage <-> BirthDate
        let c = MaxAgeEntry(Age::Exact(18), Date::from_u32(1980_07_05));
        let d = BirthDateEntry(Date::from_u32(1961_07_03), Date::from_u32(2004_07_05));

        let interp_arr3 = [c, d];
        let interp_arr4 = [d, c];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
    }

    #[test]
    fn test_invalid_approx_maxage() {
        // Approx Maxage <-> BirthYear
        let a = MaxAgeEntry(Age::Approximate(18), Date::from_u32(1980_07_03));
        let b = BirthYearEntry(1960, Date::from_u32(2004_07_05));

        let interp_arr1 = [a, b];
        let interp_arr2 = [b, a];

        // Approx Maxage <-> BirthDate
        let c = MaxAgeEntry(Age::Approximate(18), Date::from_u32(1980_07_05));
        let d = BirthDateEntry(Date::from_u32(1960_07_03), Date::from_u32(2004_07_05));

        let interp_arr3 = [c, d];
        let interp_arr4 = [d, c];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
    }

    #[test]
    fn test_invalid_birthyear() {
        // BirthYear <-> BirthYear
        let a = BirthYearEntry(1961, Date::from_u32(1980_07_03));
        let b = BirthYearEntry(1960, Date::from_u32(2004_07_05));

        let interp_arr1 = [a, b];
        let interp_arr2 = [b, a];

        // BirthYear <-> BirthDate
        let c = BirthYearEntry(1961, Date::from_u32(1980_07_05));
        let d = BirthDateEntry(Date::from_u32(1960_07_03), Date::from_u32(2004_07_05));

        let interp_arr3 = [c, d];
        let interp_arr4 = [d, c];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
        assert!(!is_agedata_consistent(&interp_arr3));
        assert!(!is_agedata_consistent(&interp_arr4));
    }

    #[test]
    fn test_invalid_birthdate() {
        // BirthDate <-> BirthDate
        let a = BirthDateEntry(Date::from_u32(1960_07_05), Date::from_u32(1960_07_04));
        let b = BirthDateEntry(Date::from_u32(1960_07_03), Date::from_u32(2004_07_05));

        let interp_arr1 = [a, b];
        let interp_arr2 = [b, a];

        assert!(!is_agedata_consistent(&interp_arr1));
        assert!(!is_agedata_consistent(&interp_arr2));
    }

    #[test]
    fn test_bound_no_data() {
        // Make sure no age data works
        let a1 = DateEntry(Date::from_u32(2000_08_05));
        let a2 = DateEntry(Date::from_u32(2001_10_12));
        let a3 = DateEntry(Date::from_u32(2001_07_04));
        let a4 = DateEntry(Date::from_u32(2007_03_05));

        let interp_arr1 = [a1, a2, a3, a4];

        assert_eq!(
            estimate_birthdate(&interp_arr1),
            (Date::from_u32(0000_01_01), Date::from_u32(9999_12_31))
        );
    }

    #[test]
    fn test_bound_age_range() {
        // See one instance of two different ages in one year
        let a1 = AgeEntry(Age::Exact(20), Date::from_u32(2000_08_05));
        let a2 = AgeEntry(Age::Exact(21), Date::from_u32(2000_10_12));
        let a3 = AgeEntry(Age::Exact(21), Date::from_u32(2001_07_04));
        let a4 = AgeEntry(Age::Exact(27), Date::from_u32(2007_03_05));

        let mut interp_arr1 = [a1, a2, a3, a4];
        let bound1 = (Date::from_u32(1979_08_06), Date::from_u32(1979_10_12));

        // See two instances of different ages in a year, bound should be tighter
        let b1 = AgeEntry(Age::Exact(20), Date::from_u32(2000_08_05));
        let b2 = AgeEntry(Age::Exact(21), Date::from_u32(2000_10_12));
        let b3 = AgeEntry(Age::Exact(21), Date::from_u32(2001_07_04));
        let b4 = AgeEntry(Age::Exact(27), Date::from_u32(2007_03_05));
        let b5 = AgeEntry(Age::Exact(28), Date::from_u32(2007_09_15));

        let mut interp_arr2 = [b1, b2, b3, b4, b5];
        let bound2 = (Date::from_u32(1979_08_06), Date::from_u32(1979_09_15));

        // See an age change, but split between two years
        let c1 = AgeEntry(Age::Exact(20), Date::from_u32(2000_08_05));
        let c2 = AgeEntry(Age::Exact(21), Date::from_u32(2001_06_12));
        let c3 = AgeEntry(Age::Exact(25), Date::from_u32(2004_10_12));
        let c4 = AgeEntry(Age::Exact(26), Date::from_u32(2006_03_05));

        let mut interp_arr3 = [c1, c2, c3, c4];
        let bound3 = (Date::from_u32(1979_08_06), Date::from_u32(1979_10_12));

        // See two age changes, split between years
        let d1 = AgeEntry(Age::Exact(20), Date::from_u32(2000_08_05));
        let d2 = AgeEntry(Age::Exact(21), Date::from_u32(2001_06_12));
        let d3 = AgeEntry(Age::Exact(25), Date::from_u32(2004_10_12));
        let d4 = AgeEntry(Age::Exact(26), Date::from_u32(2006_03_05));
        let d5 = AgeEntry(Age::Exact(29), Date::from_u32(2008_09_15));

        let mut interp_arr4 = [d1, d2, d3, d4, d5];
        let bound4 = (Date::from_u32(1979_08_06), Date::from_u32(1979_09_15));

        assert_eq!(estimate_birthdate(&interp_arr1), bound1);

        assert!(all_permutation_bd_equal(&mut interp_arr1, bound1));
        assert!(all_permutation_bd_equal(&mut interp_arr2, bound2));
        assert!(all_permutation_bd_equal(&mut interp_arr3, bound3));
        assert!(all_permutation_bd_equal(&mut interp_arr4, bound4));
    }

    #[test]
    fn test_known_age_range() {
        // All ages from one year, no age change
        let a1 = AgeEntry(Age::Exact(20), Date::from_u32(2000_08_05));
        let a2 = AgeEntry(Age::Exact(20), Date::from_u32(2000_10_12));
        let a3 = AgeEntry(Age::Exact(20), Date::from_u32(2000_07_04));
        let a4 = AgeEntry(Age::Exact(20), Date::from_u32(2000_03_05));

        let mut interp_arr1 = [a1, a2, a3, a4];
        let known1 = (Date::from_u32(1979_10_13), Date::from_u32(1980_03_05));

        // Ages from different years, no age change
        let b1 = AgeEntry(Age::Exact(20), Date::from_u32(2000_08_05));
        let b2 = AgeEntry(Age::Exact(21), Date::from_u32(2001_10_12));
        let b3 = AgeEntry(Age::Exact(24), Date::from_u32(2004_07_04));
        let b4 = AgeEntry(Age::Exact(26), Date::from_u32(2006_03_05));

        let mut interp_arr2 = [b1, b2, b3, b4];

        assert!(all_permutation_bd_equal(&mut interp_arr1, known1));
        assert!(all_permutation_bd_equal(&mut interp_arr2, known1));
    }

    #[test]
    fn test_approx_age() {
        // Only an approximate age
        let a1 = DateEntry(Date::from_u32(2000_08_05));
        let a2 = AgeEntry(Age::Approximate(20), Date::from_u32(2000_10_12));
        let a3 = DateEntry(Date::from_u32(2001_07_04));
        let interp_arr1 = [a1, a2, a3];
        let bound1 = (Date::from_u32(1979_01_01), Date::from_u32(1979_12_31));

        // Update a known age range to a birthdate range using an approximate age
        let b1 = AgeEntry(Age::Exact(20), Date::from_u32(2000_08_05));
        let b2 = AgeEntry(Age::Approximate(20), Date::from_u32(2000_10_12));
        let b3 = AgeEntry(Age::Exact(21), Date::from_u32(2001_07_04));
        let b4 = AgeEntry(Age::Exact(27), Date::from_u32(2007_03_05));
        let interp_arr2 = [b1, b2, b3, b4];
        let bound2 = (Date::from_u32(1979_08_06), Date::from_u32(1979_12_31));

        assert!(all_permutation_bd_equal(&interp_arr1, bound1));
        assert!(all_permutation_bd_equal(&interp_arr2, bound2));
    }

    #[test]
    fn test_minage() {
        // Just exact minage
        let a1 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_11_13));
        let interp_arr1 = [a1];
        let bound1 = (Date::from_u32(0000_01_01), Date::from_u32(1960_11_13));

        // Update minage bound based on new information
        let b1 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_11_13));
        let b2 = MinAgeEntry(Age::Exact(50), Date::from_u32(2005_08_05));
        let interp_arr2 = [b1, b2];
        let bound2 = (Date::from_u32(0000_01_01), Date::from_u32(1955_08_05));

        // Just approx minage
        let c1 = MinAgeEntry(Age::Approximate(39), Date::from_u32(2000_11_13));
        let interp_arr3 = [c1];
        let bound3 = (Date::from_u32(0000_01_01), Date::from_u32(1960_12_31));

        // Update minage bound based on new information
        let d1 = MinAgeEntry(Age::Approximate(39), Date::from_u32(2000_11_13));
        let d2 = MinAgeEntry(Age::Approximate(49), Date::from_u32(2005_08_05));
        let interp_arr4 = [d1, d2];
        let bound4 = (Date::from_u32(0000_01_01), Date::from_u32(1955_12_31));

        // Update approx minage bound based on exact minage
        let e1 = MinAgeEntry(Age::Approximate(39), Date::from_u32(2000_11_13));
        let e2 = MinAgeEntry(Age::Exact(55), Date::from_u32(2005_08_05));
        let interp_arr5 = [e1, e2];
        let bound5 = (Date::from_u32(0000_01_01), Date::from_u32(1950_08_05));

        assert!(all_permutation_bd_equal(&interp_arr1, bound1));
        assert!(all_permutation_bd_equal(&interp_arr2, bound2));
        assert!(all_permutation_bd_equal(&interp_arr3, bound3));
        assert!(all_permutation_bd_equal(&interp_arr4, bound4));
        assert!(all_permutation_bd_equal(&interp_arr5, bound5));
    }

    #[test]
    fn test_maxage() {
        // Just exact maxage
        let a1 = MaxAgeEntry(Age::Exact(23), Date::from_u32(2000_11_13));
        let interp_arr1 = [a1];
        let bound1 = (Date::from_u32(1976_11_14), Date::from_u32(9999_12_31));

        // Update maxage bound based on new information
        let b1 = MaxAgeEntry(Age::Exact(23), Date::from_u32(2000_11_13));
        let b2 = MaxAgeEntry(Age::Exact(20), Date::from_u32(2005_08_05));
        let interp_arr2 = [b1, b2];
        let bound2 = (Date::from_u32(1984_08_06), Date::from_u32(9999_12_31));

        // Just approx maxage
        let c1 = MaxAgeEntry(Age::Approximate(22), Date::from_u32(2000_11_13));
        let interp_arr3 = [c1];
        let bound3 = (Date::from_u32(1977_01_01), Date::from_u32(9999_12_31));

        // Update maxage bound based on new information
        let d1 = MaxAgeEntry(Age::Approximate(22), Date::from_u32(2000_11_13));
        let d2 = MaxAgeEntry(Age::Approximate(19), Date::from_u32(2005_08_05));
        let interp_arr4 = [d1, d2];
        let bound4 = (Date::from_u32(1985_01_01), Date::from_u32(9999_12_31));

        // Update approx maxage bound based on exact maxage
        let e1 = MaxAgeEntry(Age::Approximate(22), Date::from_u32(2000_11_13));
        let e2 = MaxAgeEntry(Age::Exact(20), Date::from_u32(2005_08_05));
        let interp_arr5 = [e1, e2];
        let bound5 = (Date::from_u32(1984_08_06), Date::from_u32(9999_12_31));

        assert!(all_permutation_bd_equal(&interp_arr1, bound1));
        assert!(all_permutation_bd_equal(&interp_arr2, bound2));
        assert!(all_permutation_bd_equal(&interp_arr3, bound3));
        assert!(all_permutation_bd_equal(&interp_arr4, bound4));
        assert!(all_permutation_bd_equal(&interp_arr5, bound5));
    }

    #[test]
    fn test_minage_maxage() {
        // Exact minage <-> Exact maxage
        let a1 = MinAgeEntry(Age::Exact(16), Date::from_u32(2000_11_13));
        let a2 = MaxAgeEntry(Age::Exact(23), Date::from_u32(2000_08_05));

        let interp_arr1 = [a1, a2];
        let bound1 = (Date::from_u32(1976_08_06), Date::from_u32(1984_11_13));

        // Exact minage <-> Approx maxage
        let b1 = MinAgeEntry(Age::Exact(16), Date::from_u32(2000_11_13));
        let b2 = MaxAgeEntry(Age::Approximate(22), Date::from_u32(2000_08_05));

        let interp_arr2 = [b1, b2];
        let bound2 = (Date::from_u32(1977_01_01), Date::from_u32(1984_11_13));

        // Approx minage <-> Exact maxage
        let c1 = MinAgeEntry(Age::Approximate(16), Date::from_u32(2000_11_13));
        let c2 = MaxAgeEntry(Age::Exact(23), Date::from_u32(2000_08_05));

        let interp_arr3 = [c1, c2];
        let bound3 = (Date::from_u32(1976_08_06), Date::from_u32(1983_12_31));

        // Approx minage <-> Approx maxage
        let d1 = MinAgeEntry(Age::Approximate(16), Date::from_u32(2000_11_13));
        let d2 = MaxAgeEntry(Age::Approximate(22), Date::from_u32(2000_08_05));

        let interp_arr4 = [d1, d2];
        let bound4 = (Date::from_u32(1977_01_01), Date::from_u32(1983_12_31));

        assert!(all_permutation_bd_equal(&interp_arr1, bound1));
        assert!(all_permutation_bd_equal(&interp_arr2, bound2));
        assert!(all_permutation_bd_equal(&interp_arr3, bound3));
        assert!(all_permutation_bd_equal(&interp_arr4, bound4));
    }

    #[test]
    fn test_age_minage() {
        // Age <-> Minage
        let a1 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_11_13));
        let a2 = AgeEntry(Age::Exact(39), Date::from_u32(2000_08_05));

        let interp_arr1 = [a1, a2];
        let bound1 = (Date::from_u32(1960_08_06), Date::from_u32(1960_11_13));

        // Update a minage bound using an exact age
        let b1 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_11_13));
        let b2 = AgeEntry(Age::Exact(39), Date::from_u32(2000_08_05));
        let b3 = AgeEntry(Age::Exact(40), Date::from_u32(2000_09_05));

        let interp_arr2 = [b1, b2, b3];
        let bound2 = (Date::from_u32(1960_08_06), Date::from_u32(1960_09_05));

        // Update a minage bound using an approximate age
        let c1 = MinAgeEntry(Age::Exact(36), Date::from_u32(2000_11_13));
        let c2 = AgeEntry(Age::Exact(39), Date::from_u32(2000_08_05));
        let c3 = AgeEntry(Age::Approximate(39), Date::from_u32(2000_09_05));

        let interp_arr3 = [c1, c2, c3];
        let bound3 = (Date::from_u32(1960_08_06), Date::from_u32(1960_12_31));

        // Update an age bound using a min age
        let d1 = AgeEntry(Age::Exact(40), Date::from_u32(2000_11_13));
        let d2 = AgeEntry(Age::Exact(39), Date::from_u32(2000_08_05));
        let d3 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_09_05));

        let interp_arr4 = [d1, d2, d3];
        let bound4 = (Date::from_u32(1960_08_06), Date::from_u32(1960_09_05));

        // Update a known_region using a min age
        let e1 = AgeEntry(Age::Exact(40), Date::from_u32(2000_11_13));
        let e2 = AgeEntry(Age::Exact(40), Date::from_u32(2000_08_05));
        let e3 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_06_05));

        let interp_arr5 = [e1, e2, e3];
        let bound5 = (Date::from_u32(1959_11_14), Date::from_u32(1960_06_05));

        assert!(all_permutation_bd_equal(&interp_arr1, bound1));
        assert!(all_permutation_bd_equal(&interp_arr2, bound2));
        assert!(all_permutation_bd_equal(&interp_arr3, bound3));
        assert!(all_permutation_bd_equal(&interp_arr4, bound4));
        assert!(all_permutation_bd_equal(&interp_arr5, bound5));
    }

    #[test]
    fn test_age_maxage() {
        // Age <-> Maxage
        let a1 = MaxAgeEntry(Age::Exact(39), Date::from_u32(2000_06_13));
        let a2 = AgeEntry(Age::Exact(40), Date::from_u32(2000_08_05));

        let interp_arr1 = [a1, a2];
        let bound1 = (Date::from_u32(1960_06_14), Date::from_u32(1960_08_05));

        // Update a maxage bound using an exact age
        let b1 = MaxAgeEntry(Age::Exact(39), Date::from_u32(2000_06_13));
        let b2 = AgeEntry(Age::Exact(39), Date::from_u32(2000_08_05));
        let b3 = AgeEntry(Age::Exact(40), Date::from_u32(2000_09_05));

        let interp_arr2 = [b1, b2, b3];
        let bound2 = (Date::from_u32(1960_08_06), Date::from_u32(1960_09_05));

        // Update a maxage bound using an approximate age
        let c1 = MaxAgeEntry(Age::Exact(45), Date::from_u32(2000_11_13));
        let c2 = AgeEntry(Age::Exact(39), Date::from_u32(2000_08_05));
        let c3 = AgeEntry(Age::Approximate(39), Date::from_u32(2000_09_05));

        let interp_arr3 = [c1, c2, c3];
        let bound3 = (Date::from_u32(1960_08_06), Date::from_u32(1960_12_31));

        // Update an age bound using a maxage
        let d1 = AgeEntry(Age::Exact(40), Date::from_u32(2000_11_13));
        let d2 = AgeEntry(Age::Exact(39), Date::from_u32(2000_08_05));
        let d3 = MaxAgeEntry(Age::Exact(39), Date::from_u32(2000_09_05));

        let interp_arr4 = [d1, d2, d3];
        let bound4 = (Date::from_u32(1960_09_06), Date::from_u32(1960_11_13));

        // Update a bound using a max age, all with the same age
        let e1 = AgeEntry(Age::Exact(40), Date::from_u32(2000_11_13));
        let e2 = AgeEntry(Age::Exact(40), Date::from_u32(2000_08_05));
        let e3 = MaxAgeEntry(Age::Exact(40), Date::from_u32(2001_07_05));

        let interp_arr5 = [e1, e2, e3];
        let bound5 = (Date::from_u32(1960_07_06), Date::from_u32(1960_08_05));

        assert!(all_permutation_bd_equal(&interp_arr1, bound1));
        assert!(all_permutation_bd_equal(&interp_arr2, bound2));
        assert!(all_permutation_bd_equal(&interp_arr3, bound3));
        assert!(all_permutation_bd_equal(&interp_arr4, bound4));
        assert!(all_permutation_bd_equal(&interp_arr5, bound5));
    }

    #[test]
    fn test_age_minage_maxage() {
        // Replace a bound from minage,maxage with one based off just an exact age
        let a1 = MaxAgeEntry(Age::Exact(50), Date::from_u32(2000_06_13));
        let a2 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_08_05));
        let a3 = AgeEntry(Age::Exact(45), Date::from_u32(2000_07_12));

        let interp_arr1 = [a1, a2, a3];
        let bound1 = (Date::from_u32(1954_07_13), Date::from_u32(1955_07_12));

        // Replace a bound from minage,maxage with one based off just an approximate age
        let b1 = MaxAgeEntry(Age::Exact(50), Date::from_u32(2000_06_13));
        let b2 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_08_05));
        let b3 = AgeEntry(Age::Approximate(40), Date::from_u32(2000_09_10));

        let interp_arr2 = [b1, b2, b3];
        let bound2 = (Date::from_u32(1959_01_01), Date::from_u32(1959_12_31));

        // Replace the upper bound from minage,maxage with one based off an exact age
        let c1 = MaxAgeEntry(Age::Exact(50), Date::from_u32(2000_06_13));
        let c2 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_08_05));
        let c3 = AgeEntry(Age::Exact(50), Date::from_u32(2000_06_10));

        let interp_arr3 = [c1, c2, c3];
        let bound3 = (Date::from_u32(1949_06_14), Date::from_u32(1950_06_10));

        // Replace the upper bound from minage,maxage with one based off an approximate
        // age
        let d1 = MaxAgeEntry(Age::Exact(50), Date::from_u32(2000_06_13));
        let d2 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_08_05));
        let d3 = AgeEntry(Age::Approximate(50), Date::from_u32(2000_06_10));

        let interp_arr4 = [d1, d2, d3];
        let bound4 = (Date::from_u32(1949_06_14), Date::from_u32(1949_12_31));

        // Replace the lower bound from minage,maxage with one based off an exact age
        let e1 = MaxAgeEntry(Age::Exact(50), Date::from_u32(2000_06_13));
        let e2 = MinAgeEntry(Age::Exact(40), Date::from_u32(2000_08_05));
        let e3 = AgeEntry(Age::Exact(40), Date::from_u32(2000_09_10));

        let interp_arr5 = [e1, e2, e3];
        let bound5 = (Date::from_u32(1959_09_11), Date::from_u32(1960_08_05));

        // Replace the lower bound from minage,maxage with one based off an exact age
        let f1 = MaxAgeEntry(Age::Exact(50), Date::from_u32(2000_06_13));
        let f2 = MinAgeEntry(Age::Exact(40), Date::from_u32(1999_08_05));
        let f3 = AgeEntry(Age::Approximate(40), Date::from_u32(2000_09_10));

        let interp_arr6 = [f1, f2, f3];
        let bound6 = (Date::from_u32(1959_01_01), Date::from_u32(1959_08_05));

        assert!(all_permutation_bd_equal(&interp_arr1, bound1));
        assert!(all_permutation_bd_equal(&interp_arr2, bound2));
        assert!(all_permutation_bd_equal(&interp_arr3, bound3));
        assert!(all_permutation_bd_equal(&interp_arr4, bound4));
        assert!(all_permutation_bd_equal(&interp_arr5, bound5));
        assert!(all_permutation_bd_equal(&interp_arr6, bound6));
    }

}
