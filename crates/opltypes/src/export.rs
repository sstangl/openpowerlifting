//! Defines a common data export format shared between the checker and server.

use crate::*;

/// Common human-facing serialization format for a single row of exported data.
///
/// This is a shared structure used both for the openpowerlifting.csv on the Data
/// page and for the lifter CSV data export on the Lifter page.
#[derive(Serialize)]
pub struct ExportRow<'d> {
    #[serde(rename = "Name")]
    pub name: &'d str,
    #[serde(rename = "Sex")]
    pub sex: Sex,
    #[serde(rename = "Event")]
    pub event: Event,
    #[serde(rename = "Equipment")]
    pub equipment: Equipment,
    #[serde(rename = "Age")]
    pub age: Age,
    #[serde(rename = "AgeClass")]
    pub ageclass: AgeClass,
    #[serde(rename = "BirthYearClass")]
    pub birthyearclass: BirthYearClass,
    #[serde(rename = "Division")]
    pub division: &'d str,
    #[serde(rename = "BodyweightKg")]
    pub bodyweightkg: WeightKg,
    #[serde(rename = "WeightClassKg")]
    pub weightclasskg: WeightClassKg,
    #[serde(rename = "Squat1Kg")]
    pub squat1kg: WeightKg,
    #[serde(rename = "Squat2Kg")]
    pub squat2kg: WeightKg,
    #[serde(rename = "Squat3Kg")]
    pub squat3kg: WeightKg,
    #[serde(rename = "Squat4Kg")]
    pub squat4kg: WeightKg,
    #[serde(rename = "Best3SquatKg")]
    pub best3squatkg: WeightKg,
    #[serde(rename = "Bench1Kg")]
    pub bench1kg: WeightKg,
    #[serde(rename = "Bench2Kg")]
    pub bench2kg: WeightKg,
    #[serde(rename = "Bench3Kg")]
    pub bench3kg: WeightKg,
    #[serde(rename = "Bench4Kg")]
    pub bench4kg: WeightKg,
    #[serde(rename = "Best3BenchKg")]
    pub best3benchkg: WeightKg,
    #[serde(rename = "Deadlift1Kg")]
    pub deadlift1kg: WeightKg,
    #[serde(rename = "Deadlift2Kg")]
    pub deadlift2kg: WeightKg,
    #[serde(rename = "Deadlift3Kg")]
    pub deadlift3kg: WeightKg,
    #[serde(rename = "Deadlift4Kg")]
    pub deadlift4kg: WeightKg,
    #[serde(rename = "Best3DeadliftKg")]
    pub best3deadliftkg: WeightKg,
    #[serde(rename = "TotalKg")]
    pub totalkg: WeightKg,
    #[serde(rename = "Place")]
    pub place: Place,
    #[serde(rename = "Dots")]
    pub dots: Points,
    #[serde(rename = "Wilks")]
    pub wilks: Points,
    #[serde(rename = "Glossbrenner")]
    pub glossbrenner: Points,
    #[serde(rename = "Goodlift")]
    pub goodlift: Points,
    #[serde(rename = "Tested")]
    pub tested: &'static str,
    #[serde(rename = "Country")]
    pub country: Option<Country>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "Federation")]
    pub federation: Federation,
    #[serde(rename = "ParentFederation")]
    pub parent_federation: Option<Federation>,
    #[serde(rename = "Date")]
    pub date: Date,
    #[serde(rename = "MeetCountry")]
    pub meet_country: Country,
    #[serde(rename = "MeetState")]
    pub meet_state: Option<String>,
    #[serde(rename = "MeetTown")]
    pub meet_town: Option<&'d str>,
    #[serde(rename = "MeetName")]
    pub meet_name: &'d str,
    #[serde(rename = "Sanctioned")]
    pub sanctioned: &'static str,
}
