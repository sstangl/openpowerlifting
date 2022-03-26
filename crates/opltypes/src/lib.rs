//! A collection of data types used in the OpenPowerlifting database.

// Suppress clippy warnings for date literals.
#![allow(clippy::inconsistent_digit_grouping)]
#![allow(clippy::zero_prefixed_literal)]

// External dependencies.
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;

// External dependencies used only in testing code.
#[cfg(test)]
#[macro_use]
extern crate serde_json;

// Public exports.
mod age;
pub use self::age::{Age, PrettyAge};

mod ageclass;
pub use self::ageclass::AgeClass;

mod agerange;
pub use self::agerange::AgeRange;

mod birthyearclass;
pub use self::birthyearclass::BirthYearClass;

mod birthyearrange;
pub use self::birthyearrange::BirthYearRange;

mod country;
pub use self::country::Country;

#[macro_use]
mod date;
pub use self::date::Date;

mod equipment;
pub use self::equipment::Equipment;

mod event;
pub use self::event::Event;

mod export;
pub use self::export::ExportRow;

mod federation;
pub use self::federation::Federation;

mod meetpath;
pub use self::meetpath::{dir_to_meetpath, file_to_meetpath, MeetPathError};

mod place;
pub use self::place::Place;

mod points;
pub use self::points::{Points, PointsSystem};

mod ruleset;
pub use self::ruleset::{Rule, RuleSet};

mod sex;
pub use self::sex::Sex;

pub mod states;

mod username;
pub use self::username::Username;

mod weightkg;
pub use self::weightkg::{WeightAny, WeightKg};

mod weightclasskg;
pub use self::weightclasskg::{WeightClassAny, WeightClassKg};

mod writing_system;
pub use self::writing_system::{infer_writing_system, writing_system, WritingSystem};

/// Units of weight.
#[derive(Copy, Clone, Debug, EnumString, Serialize, Deserialize)]
pub enum WeightUnits {
    /// Kilograms.
    #[serde(rename = "kg")]
    #[strum(serialize = "kg")]
    Kg,

    /// Pounds.
    #[serde(rename = "lbs")]
    #[strum(serialize = "lbs")]
    Lbs,
}
