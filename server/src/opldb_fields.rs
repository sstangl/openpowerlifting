//! A collection of fields used by the database.
//!
//! This file exists to separate deserialization and internal
//! representation details out from database definition file,
//! to make it easier to see the design from a high level.

use serde;
use serde::de::{self, Visitor, Deserialize};

use std::error::Error;
use std::num;
use std::mem;
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
    GBPF,
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
