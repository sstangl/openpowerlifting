//! Defines the in-memory data format.

use opltypes::states::*;
use opltypes::*;
use smartstring::alias::CompactString;
use symbol_table::GlobalSymbol;

use crate::yesno::deserialize_yes_no;

/// Row for a unique lifter.
///
/// Lifters are uniquely identified throughout the database by any of:
///  1. LifterID
///  2. Username
///  3. Name
///
/// Lifters are stored in a `Vec<Lifter>`.
/// The LifterID is the index of the struct in the vector.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Lifter {
    pub name: CompactString,
    pub username: Username,

    // These fields intentionally do not use CompactString.
    //
    // Because they are almost always None in the current dataset, it's more beneficial
    // to take the Option<String> None optimization than the CompactString optimization.
    pub chinese_name: Option<Box<str>>,
    pub cyrillic_name: Option<Box<str>>,
    pub greek_name: Option<Box<str>>,
    pub japanese_name: Option<Box<str>>,
    pub korean_name: Option<Box<str>>,

    pub instagram: Option<Box<str>>,
    pub color: Option<Box<str>>,
}

/// The definition of a Meet in the database.
///
/// Meets are stored in a `Vec<Meet>`.
/// The MeetID is the index of the struct in the vector.
#[derive(Debug, Serialize, Deserialize)]
pub struct Meet {
    #[serde(rename(deserialize = "MeetPath"))]
    pub path: CompactString,
    #[serde(rename(deserialize = "Federation"))]
    pub federation: Federation,
    #[serde(rename(deserialize = "Date"))]
    pub date: Date,
    #[serde(rename(deserialize = "MeetCountry"))]
    pub country: Country,
    #[serde(rename(deserialize = "MeetState"))]
    pub state: Option<CompactString>,
    #[serde(rename(deserialize = "MeetTown"))]
    pub town: Option<CompactString>,
    #[serde(rename(deserialize = "MeetName"))]
    pub name: Box<str>,
    #[serde(rename(deserialize = "RuleSet"))]
    pub ruleset: RuleSet,
    #[serde(rename(deserialize = "Sanctioned"))]
    #[serde(deserialize_with = "deserialize_yes_no")]
    pub sanctioned: bool,

    /// Number of unique competitors, by LifterID.
    /// Calculated at load-time.
    #[serde(default)]
    pub num_unique_lifters: u16,
}

/// The definition of an Entry in the database.
///
/// Entries are stored in a `Vec<Entry>` such that all entries for a given `lifter_id`
/// are contiguous. This allows for very quickly determining a lifter's best Entry.
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename(deserialize = "MeetID"))]
    pub meet_id: u32,
    #[serde(rename(deserialize = "LifterID"))]
    pub lifter_id: u32,
    #[serde(rename(deserialize = "Sex"))]
    pub sex: Sex,
    #[serde(rename(deserialize = "Event"))]
    pub event: Event,
    #[serde(rename(deserialize = "Equipment"))]
    pub equipment: Equipment,
    #[serde(rename(deserialize = "Age"))]
    pub age: Age,
    #[serde(rename(deserialize = "Division"))]
    pub division: Option<GlobalSymbol>,
    #[serde(rename(deserialize = "BodyweightKg"))]
    pub bodyweightkg: WeightKg,
    #[serde(rename(deserialize = "WeightClassKg"))]
    pub weightclasskg: WeightClassKg,
    #[serde(rename(deserialize = "Squat1Kg"))]
    pub squat1kg: WeightKg,
    #[serde(rename(deserialize = "Squat2Kg"))]
    pub squat2kg: WeightKg,
    #[serde(rename(deserialize = "Squat3Kg"))]
    pub squat3kg: WeightKg,
    #[serde(rename(deserialize = "Squat4Kg"))]
    pub squat4kg: WeightKg,
    #[serde(rename(deserialize = "Best3SquatKg"))]
    pub best3squatkg: WeightKg,
    #[serde(rename(deserialize = "Bench1Kg"))]
    pub bench1kg: WeightKg,
    #[serde(rename(deserialize = "Bench2Kg"))]
    pub bench2kg: WeightKg,
    #[serde(rename(deserialize = "Bench3Kg"))]
    pub bench3kg: WeightKg,
    #[serde(rename(deserialize = "Bench4Kg"))]
    pub bench4kg: WeightKg,
    #[serde(rename(deserialize = "Best3BenchKg"))]
    pub best3benchkg: WeightKg,
    #[serde(rename(deserialize = "Deadlift1Kg"))]
    pub deadlift1kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift2Kg"))]
    pub deadlift2kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift3Kg"))]
    pub deadlift3kg: WeightKg,
    #[serde(rename(deserialize = "Deadlift4Kg"))]
    pub deadlift4kg: WeightKg,
    #[serde(rename(deserialize = "Best3DeadliftKg"))]
    pub best3deadliftkg: WeightKg,
    #[serde(rename(deserialize = "TotalKg"))]
    pub totalkg: WeightKg,
    #[serde(rename(deserialize = "Place"))]
    pub place: Place,
    #[serde(rename(deserialize = "Wilks"))]
    pub wilks: Points,
    #[serde(rename(deserialize = "McCulloch"))]
    pub mcculloch: Points,
    #[serde(rename(deserialize = "Glossbrenner"))]
    pub glossbrenner: Points,
    #[serde(rename(deserialize = "Goodlift"))]
    pub goodlift: Points,
    #[serde(rename(deserialize = "Dots"))]
    pub dots: Points,
    #[serde(rename = "Tested")]
    #[serde(deserialize_with = "deserialize_yes_no")]
    pub tested: bool,
    #[serde(rename(deserialize = "AgeClass"))]
    pub ageclass: AgeClass,
    #[serde(rename(deserialize = "BirthYearClass"))]
    pub birthyearclass: BirthYearClass,
    #[serde(rename(deserialize = "Country"))]
    pub lifter_country: Option<Country>,
    #[serde(rename(deserialize = "State"))]
    pub lifter_state: Option<State>,
}

impl Entry {
    /// Returns `max(best3squatkg, squat4kg)`.
    #[inline]
    pub fn highest_squatkg(&self) -> WeightKg {
        self.best3squatkg.max(self.squat4kg)
    }

    /// Returns `max(best3benchkg, bench4kg)`.
    #[inline]
    pub fn highest_benchkg(&self) -> WeightKg {
        self.best3benchkg.max(self.bench4kg)
    }

    /// Returns `max(best3deadliftkg, deadlift4kg)`.
    #[inline]
    pub fn highest_deadliftkg(&self) -> WeightKg {
        self.best3deadliftkg.max(self.deadlift4kg)
    }

    /// Borrows the Division string.
    #[inline]
    pub fn division(&self) -> Option<&str> {
        self.division.map(|symbol| symbol.as_str())
    }

    /// Calculates the Entry's points.
    #[inline(always)]
    pub fn points(&self, system: PointsSystem, units: WeightUnits) -> Points {
        let sex = self.sex;
        let eqp = self.equipment;
        let evt = self.event;
        let bw = self.bodyweightkg;
        let total = self.totalkg;

        match system {
            PointsSystem::AH => coefficients::ah(sex, bw, total),
            PointsSystem::Dots => self.dots,
            PointsSystem::Glossbrenner => self.glossbrenner,
            PointsSystem::Goodlift => self.goodlift,
            PointsSystem::IPFPoints => coefficients::ipf(sex, eqp, evt, bw, total),
            PointsSystem::McCulloch => self.mcculloch,
            PointsSystem::NASA => coefficients::nasa(bw, total),
            PointsSystem::Reshel => coefficients::reshel(sex, bw, total),
            PointsSystem::SchwartzMalone => coefficients::schwartzmalone(sex, bw, total),
            PointsSystem::Total => self.totalkg.as_type(units).as_points(),
            PointsSystem::Wilks => self.wilks,
            PointsSystem::Wilks2020 => coefficients::wilks2020(sex, bw, total),
        }
    }
}
