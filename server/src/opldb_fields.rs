//! A collection of fields used by the database.
//!
//! This file exists to separate deserialization and internal
//! representation details out from database definition file,
//! to make it easier to see the design from a high level.

use serde;
use serde::de::{self, Visitor, Deserialize};

use std::num;
use std::fmt;
use std::str::FromStr;

/// Deserializes a f32 field from the CSV source,
/// defaulting to 0.0 if the empty string is encountered.
pub fn deserialize_f32_with_default<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where D: serde::Deserializer<'de>
{
    struct F32StrVisitor;

    impl<'de> Visitor<'de> for F32StrVisitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("f32 or the empty string")
        }

        fn visit_str<E>(self, value: &str) -> Result<f32, E>
            where E: de::Error
        {
            if value.is_empty() {
                return Ok(0.0);
            }
            f32::from_str(value).map_err(E::custom)
        }
    }

    deserializer.deserialize_str(F32StrVisitor)
}

/// The definition of the "Event" column.
/// An Event is represented as a bitfield, with
/// one bit for each of S, B, and D.
#[derive(PartialEq)]
pub struct Event {
    bits: u8,
}

impl Event {
    const BITFLAG_SQUAT: u8 = 0b100;
    const BITFLAG_BENCH: u8 = 0b010;
    const BITFLAG_DEADLIFT: u8  = 0b001;

    #[inline]
    pub fn has_squat(&self) -> bool {
        self.bits & Self::BITFLAG_SQUAT == 0x1
    }

    #[inline]
    pub fn has_bench(&self) -> bool {
        self.bits & Self::BITFLAG_BENCH == 0x1
    }

    #[inline]
    pub fn has_deadlift(&self) -> bool {
        self.bits & Self::BITFLAG_DEADLIFT == 0x1
    }
}

impl FromStr for Event {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits: u8 = 0;
        for c in s.chars() {
            match c {
                'S' => bits = bits | Event::BITFLAG_SQUAT,
                'B' => bits = bits | Event::BITFLAG_BENCH,
                'D' => bits = bits | Event::BITFLAG_DEADLIFT,
                _ => return Err("Unexpected event character."),
            }
        }
        Ok(Event { bits })
    }
}

struct EventVisitor;

impl<'de> Visitor<'de> for EventVisitor {
    type Value = Event;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing only the characters S,B,D")
    }

    fn visit_str<E>(self, value: &str) -> Result<Event, E>
        where E: de::Error
    {
        // TODO: Make Event a required field.
        //if value.is_empty() {
        //    return Err(E::custom("unexpected empty Event"));
        //}
        Event::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Event, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(EventVisitor)
    }
}

#[derive(PartialEq)]
pub enum Place {
    P(u8),
    G,
    DQ,
    DD,
    NS,
    None, // TODO: Require every row to have a Place.
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
        where E: de::Error
    {
        Place::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Place {
    fn deserialize<D>(deserializer: D) -> Result<Place, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(PlaceVisitor)
    }
}

/// Our data uses imprecise dates in the "YYYY-MM-DD" format,
/// with no timezone or time data.
/// Dates in this format can be stored as a u32 with value YYYYMMDD.
/// This format is compact and remains human-readable.
#[derive(Debug,PartialEq,PartialOrd)]
pub struct Date {
    value: u32,
}

impl Date {
    pub fn year(&self) -> u32 {
        self.value / 10000
    }
    pub fn month(&self) -> u32 {
        (self.value / 100) % 100
    }
    pub fn day(&self) -> u32 {
        self.value % 100
    }
}

#[derive(Debug)]
pub enum ParseDateError {
    FormatError,
    ParseIntError(num::ParseIntError),
}

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseDateError::FormatError => write!(f, "date not in the correct format"),
            ParseDateError::ParseIntError(ref p) => p.fmt(f),
        }
    }
}

impl FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split("-").collect();
        if v.len() != 3 || v[0].len() != 4 || v[1].len() != 2 || v[2].len() != 2 {
            return Err(ParseDateError::FormatError);
        }

        let year: u32 = v[0].parse::<u32>().map_err(ParseDateError::ParseIntError)?;
        let month: u32 = v[1].parse::<u32>().map_err(ParseDateError::ParseIntError)?;
        let day: u32 = v[2].parse::<u32>().map_err(ParseDateError::ParseIntError)?;

        if year < 1000 {
            return Err(ParseDateError::FormatError);
        }
        if month == 0 || month > 12 {
            return Err(ParseDateError::FormatError);
        }
        if day == 0 || day > 31 {
            return Err(ParseDateError::FormatError);
        }

        let value = (year * 10000) + (month * 100) + day;

        Ok(Date { value })
    }
}

struct DateVisitor;

impl<'de> Visitor<'de> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the format YYYY-MM-DD")
    }

    fn visit_str<E>(self, value: &str) -> Result<Date, E>
        where E: de::Error
    {
        Date::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Date, D::Error>
        where D: serde::Deserializer<'de>
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
        assert!("0007-03-04".parse::<Date>().is_err());
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
        assert!("0000-03-04".parse::<Date>().is_err());
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
}

#[derive(Deserialize,PartialEq)]
pub enum Sex {
    M,
    F,
}

#[derive(Deserialize,PartialEq)]
pub enum Equipment {
    Raw,
    Wraps,
    #[serde(rename = "Single-ply")]
    Single,
    #[serde(rename = "Multi-ply")]
    Multi,
    Straps,
}

/// The reported age of the lifter at a given meet.
/// In the CSV file, approximate ages are reported with '.5' added.
pub enum Age {
    /// The exact age of the lifter.
    Exact(u8),
    /// The lower possible age of the lifter.
    Approximate(u8),
    /// No age specified.
    None,
}

impl FromStr for Age {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Age::None);
        }

        let v: Vec<&str> = s.split(".").collect();
        if v.len() == 1 {
            v[0].parse::<u8>().map(Age::Exact)
        } else {
            v[0].parse::<u8>().map(Age::Approximate)
        }
    }
}

struct AgeVisitor;

impl<'de> Visitor<'de> for AgeVisitor {
    type Value = Age;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an age (23) or approximate age (23.5)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Age, E>
        where E: de::Error
    {
        Age::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Age {
    fn deserialize<D>(deserializer: D) -> Result<Age, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(AgeVisitor)
    }
}

pub enum WeightClassKg {
    UnderOrEqual(f32),
    Over(f32),
    None,
}

impl FromStr for WeightClassKg {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(WeightClassKg::None);
        }

        if s.ends_with("+") {
            let v = &s[..s.len()-1];
            v.parse::<f32>().map(WeightClassKg::Over)
        } else {
            s.parse::<f32>().map(WeightClassKg::UnderOrEqual)
        }
    }
}

struct WeightClassKgVisitor;

impl<'de> Visitor<'de> for WeightClassKgVisitor {
    type Value = WeightClassKg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A floating-point value optionally ending with '+'")
    }

    fn visit_str<E>(self, value: &str) -> Result<WeightClassKg, E>
        where E: de::Error
    {
        WeightClassKg::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for WeightClassKg {
    fn deserialize<D>(deserializer: D) -> Result<WeightClassKg, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(WeightClassKgVisitor)
    }
}

#[derive(Deserialize,PartialEq)]
pub enum Federation {
    #[serde(rename = "365Strong")]
    _365Strong,
    AAPF,
    AAU,
    ADFPA,
    APA,
    APC,
    APF,
    AsianPF,
    BB,
    BPU,
    BP,
    CAPO,
    CommonwealthPF,
    CPF,
    CPL,
    CPU,
    EPA,
    EPF,
    FESUPO,
    FFForce,
    FPO,
    GPA,
    GPC,
    #[serde(rename = "GPC-GB")]
    GPCGB,
    #[serde(rename = "GPC-AUS")]
    GPCAUS,
    HERC,
    IPA,
    IPF,
    IPL,
    IrishPF,
    MHP,
    MM,
    NAPF,
    NASA,
    NIPF,
    NPA,
    NSF,
    NZPF,
    OceaniaPF,
    ProRaw,
    PA,
    RAW,
    RPS,
    RUPC,
    ScottishPL,
    SCT,
    SPF,
    THSPA,
    UPA,
    USAPL,
    USPF,
    USPA,
    WelshPA,
    WPC,
    WNPF,
    WRPF,
    #[serde(rename = "WRPF-AUS")]
    WRPFAUS,
    #[serde(rename = "WRPF-CAN")]
    WRPFCAN,
    WUAP,
    XPC,
}
