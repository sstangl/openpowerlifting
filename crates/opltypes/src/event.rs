//! Defines the `Event` field for the `entries` table.

use arrayvec::ArrayString;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::fmt::{self, Write};
use std::str::FromStr;

/// The definition of the "Event" column.
/// An `Event` is represented as a bitfield, with
/// one bit for each of S, B, and D.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Event(u8);

impl Event {
    const BITFLAG_SQUAT: u8 = 0b100;
    const BITFLAG_BENCH: u8 = 0b010;
    const BITFLAG_DEADLIFT: u8 = 0b001;
    const BITFLAG_PUSHPULL: u8 = 0b011;
    const BITFLAG_FULLPOWER: u8 = 0b111;

    /// Constructs a new Event with value "SBD".
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Event;
    /// assert!(Event::sbd().is_full_power());
    /// ```
    #[inline(always)]
    pub const fn sbd() -> Event {
        Event(Self::BITFLAG_FULLPOWER)
    }

    /// Constructs a new Event with value "BD".
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Event;
    /// assert!(Event::bd().is_push_pull());
    /// ```
    #[inline(always)]
    pub const fn bd() -> Event {
        Event(Self::BITFLAG_BENCH | Self::BITFLAG_DEADLIFT)
    }

    /// Constructs a new Event with value "SB".
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Event;
    /// assert_eq!(Event::sb().has_squat(), true);
    /// assert_eq!(Event::sb().has_bench(), true);
    /// assert_eq!(Event::sb().has_deadlift(), false);
    /// ```
    #[inline(always)]
    pub const fn sb() -> Event {
        Event(Self::BITFLAG_SQUAT | Self::BITFLAG_BENCH)
    }

    /// Constructs a new Event with value "SD".
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Event;
    /// assert_eq!(Event::sd().has_squat(), true);
    /// assert_eq!(Event::sd().has_bench(), false);
    /// assert_eq!(Event::sd().has_deadlift(), true);
    /// ```
    #[inline(always)]
    pub const fn sd() -> Event {
        Event(Self::BITFLAG_SQUAT | Self::BITFLAG_DEADLIFT)
    }

    /// Constructs a new Event with value "S".
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Event;
    /// assert!(Event::s().is_squat_only());
    /// ```
    #[inline(always)]
    pub const fn s() -> Event {
        Event(Self::BITFLAG_SQUAT)
    }

    /// Constructs a new Event with value "B".
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Event;
    /// assert!(Event::b().is_bench_only());
    /// ```
    #[inline(always)]
    pub const fn b() -> Event {
        Event(Self::BITFLAG_BENCH)
    }

    /// Constructs a new Event with value "D".
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Event;
    /// assert!(Event::d().is_deadlift_only());
    /// ```
    #[inline(always)]
    pub const fn d() -> Event {
        Event(Self::BITFLAG_DEADLIFT)
    }

    /// True iff the Event contains a Bench and a Deadlift.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Event;
    /// assert_eq!(Event::sbd().has_push_pull(), true);
    /// assert_eq!(Event::bd().has_push_pull(), true);
    /// assert_eq!(Event::d().has_push_pull(), false);
    /// ```
    #[inline]
    pub fn has_push_pull(self) -> bool {
        self.0 & Self::BITFLAG_PUSHPULL == Self::BITFLAG_PUSHPULL
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
        self.0 == Self::BITFLAG_FULLPOWER
    }

    /// True iff the Event is BD.
    #[inline]
    pub fn is_push_pull(self) -> bool {
        self.0 == Self::BITFLAG_PUSHPULL
    }

    /// True iff the Event is S.
    #[inline]
    pub fn is_squat_only(self) -> bool {
        self.0 == Self::BITFLAG_SQUAT
    }

    /// True iff the Event is B.
    #[inline]
    pub fn is_bench_only(self) -> bool {
        self.0 == Self::BITFLAG_BENCH
    }

    /// True iff the Event is D.
    #[inline]
    pub fn is_deadlift_only(self) -> bool {
        self.0 == Self::BITFLAG_DEADLIFT
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

impl Serialize for Event {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // The greatest possible string is "SBD", 3 characters.
        let mut buf = ArrayString::<3>::new();
        write!(buf, "{self}").expect("ArrayString overflow");

        serializer.serialize_str(&buf)
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
                'S' => {
                    if bits & Event::BITFLAG_SQUAT != 0 {
                        return Err("Duplicate S character");
                    }
                    bits |= Event::BITFLAG_SQUAT;
                }
                'B' => {
                    if bits & Event::BITFLAG_BENCH != 0 {
                        return Err("Duplicate B character");
                    }
                    bits |= Event::BITFLAG_BENCH;
                }
                'D' => {
                    if bits & Event::BITFLAG_DEADLIFT != 0 {
                        return Err("Duplicate D character");
                    }
                    bits |= Event::BITFLAG_DEADLIFT;
                }
                _ => return Err("Unexpected Event character."),
            }
        }

        Ok(Event(bits))
    }
}

struct EventVisitor;

impl Visitor<'_> for EventVisitor {
    type Value = Event;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing only the characters S,B,D")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Event, E> {
        Event::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Event, D::Error> {
        deserializer.deserialize_str(EventVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let event = "SBD".parse::<Event>().unwrap();
        assert!(event.has_push_pull());
        assert!(event.has_squat());
        assert!(event.has_bench());
        assert!(event.has_deadlift());
        assert!(event.is_full_power());
        assert!(!event.is_push_pull());
        assert!(!event.is_squat_only());
        assert!(!event.is_bench_only());
        assert!(!event.is_deadlift_only());

        let event = "BD".parse::<Event>().unwrap();
        assert!(event.has_push_pull());
        assert!(!event.has_squat());
        assert!(event.has_bench());
        assert!(event.has_deadlift());
        assert!(!event.is_full_power());
        assert!(event.is_push_pull());
        assert!(!event.is_bench_only());
        assert!(!event.is_deadlift_only());

        let event = "S".parse::<Event>().unwrap();
        assert!(!event.has_push_pull());
        assert!(event.has_squat());
        assert!(!event.has_bench());
        assert!(!event.has_deadlift());
        assert!(!event.is_full_power());
        assert!(!event.is_push_pull());
        assert!(event.is_squat_only());
        assert!(!event.is_bench_only());
        assert!(!event.is_deadlift_only());

        let event = "B".parse::<Event>().unwrap();
        assert!(!event.has_push_pull());
        assert!(!event.has_squat());
        assert!(event.has_bench());
        assert!(!event.has_deadlift());
        assert!(!event.is_full_power());
        assert!(!event.is_push_pull());
        assert!(!event.is_squat_only());
        assert!(event.is_bench_only());
        assert!(!event.is_deadlift_only());

        let event = "D".parse::<Event>().unwrap();
        assert!(!event.has_push_pull());
        assert!(!event.has_squat());
        assert!(!event.has_bench());
        assert!(event.has_deadlift());
        assert!(!event.is_full_power());
        assert!(!event.is_push_pull());
        assert!(!event.is_squat_only());
        assert!(!event.is_bench_only());
        assert!(event.is_deadlift_only());
    }

    #[test]
    fn errors() {
        assert!("".parse::<Event>().is_err());
        assert!(" ".parse::<Event>().is_err());
        assert!("ABC".parse::<Event>().is_err());
        assert!("Jerry".parse::<Event>().is_err());
    }

    #[test]
    fn repeats() {
        assert!("BBBBBBBB".parse::<Event>().is_err());
        assert!("BSS".parse::<Event>().is_err());
        assert!("SSSBBBDDDDDD".parse::<Event>().is_err());
    }

    #[test]
    fn display() {
        let event = "SBD".parse::<Event>().unwrap();
        assert_eq!(format!("{event}"), "SBD");

        let event = "B".parse::<Event>().unwrap();
        assert_eq!(format!("{event}"), "B");
    }
}
