//! Defines the `Event` field for the `entries` table.

use serde;
use serde::de::{self, Deserialize, Visitor};

use std::fmt;
use std::str::FromStr;

/// The definition of the "Event" column.
/// An `Event` is represented as a bitfield, with
/// one bit for each of S, B, and D.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Event(u8);

impl Event {
    const BITFLAG_SQUAT: u8 = 0b100;
    const BITFLAG_BENCH: u8 = 0b010;
    const BITFLAG_DEADLIFT: u8 = 0b001;
    const BITFLAG_FULLPOWER: u8 = 0b111;

    #[inline(always)]
    pub const fn sbd() -> Event {
        Event(Self::BITFLAG_FULLPOWER)
    }

    #[inline(always)]
    pub const fn bd() -> Event {
        Event(Self::BITFLAG_BENCH | Self::BITFLAG_DEADLIFT)
    }

    #[inline(always)]
    pub const fn sb() -> Event {
        Event(Self::BITFLAG_SQUAT | Self::BITFLAG_BENCH)
    }

    #[inline(always)]
    pub const fn sd() -> Event {
        Event(Self::BITFLAG_SQUAT | Self::BITFLAG_DEADLIFT)
    }

    #[inline(always)]
    pub const fn s() -> Event {
        Event(Self::BITFLAG_SQUAT)
    }

    #[inline(always)]
    pub const fn b() -> Event {
        Event(Self::BITFLAG_BENCH)
    }

    #[inline(always)]
    pub const fn d() -> Event {
        Event(Self::BITFLAG_DEADLIFT)
    }

    /// True iff the Event contains a Squat.
    #[inline]
    pub fn has_squat(self) -> bool {
        self.0 & Self::BITFLAG_SQUAT != 0x0
    }

    /// True iff the Event contains a Bench.
    #[inline]
    pub fn has_bench(self) -> bool {
        self.0 & Self::BITFLAG_BENCH != 0x0
    }

    /// True iff the Event contains a Deadlift.
    #[inline]
    pub fn has_deadlift(self) -> bool {
        self.0 & Self::BITFLAG_DEADLIFT != 0x0
    }

    /// True iff the Event is SBD.
    #[inline]
    pub fn is_full_power(self) -> bool {
        self.0 & Self::BITFLAG_FULLPOWER == Self::BITFLAG_FULLPOWER
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_squat() {
            write!(f, "S")?
        }
        if self.has_bench() {
            write!(f, "B")?
        }
        if self.has_deadlift() {
            write!(f, "D")?
        }
        Ok(())
    }
}

impl FromStr for Event {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Unexpected empty Event");
        }

        let mut bits: u8 = 0;
        for c in s.chars() {
            match c {
                'S' => bits |= Event::BITFLAG_SQUAT,
                'B' => bits |= Event::BITFLAG_BENCH,
                'D' => bits |= Event::BITFLAG_DEADLIFT,
                _ => return Err("Unexpected Event character."),
            }
        }

        Ok(Event(bits))
    }
}

struct EventVisitor;

impl<'de> Visitor<'de> for EventVisitor {
    type Value = Event;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing only the characters S,B,D")
    }

    fn visit_str<E>(self, value: &str) -> Result<Event, E>
    where
        E: de::Error,
    {
        Event::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Event, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EventVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_basic() {
        let event = "SBD".parse::<Event>().unwrap();
        assert!(event.has_squat());
        assert!(event.has_bench());
        assert!(event.has_deadlift());
        assert!(event.is_full_power());

        let event = "BD".parse::<Event>().unwrap();
        assert!(!event.has_squat());
        assert!(event.has_bench());
        assert!(event.has_deadlift());
        assert!(!event.is_full_power());

        let event = "S".parse::<Event>().unwrap();
        assert!(event.has_squat());
        assert!(!event.has_bench());
        assert!(!event.has_deadlift());
        assert!(!event.is_full_power());

        let event = "B".parse::<Event>().unwrap();
        assert!(!event.has_squat());
        assert!(event.has_bench());
        assert!(!event.has_deadlift());
        assert!(!event.is_full_power());

        let event = "D".parse::<Event>().unwrap();
        assert!(!event.has_squat());
        assert!(!event.has_bench());
        assert!(event.has_deadlift());
        assert!(!event.is_full_power());
    }

    #[test]
    fn test_event_errors() {
        assert!("".parse::<Event>().is_err());
        assert!(" ".parse::<Event>().is_err());
        assert!("ABC".parse::<Event>().is_err());
        assert!("Jerry".parse::<Event>().is_err());
    }

    #[test]
    fn test_event_repeats() {
        let event = "BBBBBBBB".parse::<Event>().unwrap();
        assert!(!event.has_squat());
        assert!(event.has_bench());
        assert!(!event.has_deadlift());

        let event = "BSS".parse::<Event>().unwrap();
        assert!(event.has_squat());
        assert!(event.has_bench());
        assert!(!event.has_deadlift());
    }

    #[test]
    fn test_event_display() {
        let event = "SSSBBBDDDDDD".parse::<Event>().unwrap();
        assert_eq!(format!("{}", event), "SBD");

        let event = "B".parse::<Event>().unwrap();
        assert_eq!(format!("{}", event), "B");
    }
}
