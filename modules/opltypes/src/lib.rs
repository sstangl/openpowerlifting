//! A collection of datatypes used in the OpenPowerlifting database.

// External dependencies.
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate strum;
#[macro_use]
extern crate strum_macros;

// Public exports.
mod age;
pub use self::age::Age;
mod country;
pub use self::country::Country;
mod date;
pub use self::date::Date;
mod equipment;
pub use self::equipment::Equipment;
mod event;
pub use self::event::Event;
mod federation;
pub use self::federation::Federation;
mod place;
pub use self::place::Place;
mod points;
pub use self::points::Points;
mod sex;
pub use self::sex::Sex;
mod weightkg;
pub use self::weightkg::{WeightKg, WeightAny};
mod weightclasskg;
pub use self::weightclasskg::{WeightClassKg, WeightClassAny};
mod ageclass;
pub use self::ageclass::AgeClass;
mod yesno;
pub use self::yesno::deserialize_yes_no;

/// Units of weight.
#[derive(Copy, Clone, Debug, EnumString, Serialize)]
pub enum WeightUnits {
    #[strum(serialize = "kg")]
    Kg,
    #[strum(serialize = "lbs")]
    Lbs,
}
