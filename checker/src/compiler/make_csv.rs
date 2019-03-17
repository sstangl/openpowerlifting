//! Transforms a `Vec<MeetData>` into the final CSV files.

use coefficients::mcculloch;
use csv::{QuoteStyle, WriterBuilder};
use hashbrown::HashMap;
use opltypes::*;

use std::path::Path;

use crate::checklib::{Entry, Meet};
use crate::MeetData;

/// Serialization source for the meets.csv.
/// The MeetData continues as the backing store.
#[derive(Serialize)]
struct MeetsRow<'d> {
    #[serde(rename = "MeetID")]
    pub id: u32,
    #[serde(rename = "MeetPath")]
    pub path: &'d str,
    #[serde(rename = "Federation")]
    pub federation: Federation,
    #[serde(rename = "Date")]
    pub date: Date,
    #[serde(rename = "MeetCountry")]
    pub country: Country,
    #[serde(rename = "MeetState")]
    pub state: Option<State>,
    #[serde(rename = "MeetTown")]
    pub town: Option<&'d str>,
    #[serde(rename = "MeetName")]
    pub name: &'d str,
}

impl<'d> MeetsRow<'d> {
    fn from(meet: &'d Meet, meet_id: u32) -> MeetsRow<'d> {
        MeetsRow {
            id: meet_id,
            path: &meet.path,
            federation: meet.federation,
            date: meet.date,
            country: meet.country,
            state: meet.state,
            town: meet.town.deref(),
            name: &meet.name,
        }
    }
}

/// Serialization source for the entries.csv.
#[derive(Serialize)]
struct EntriesRow<'d> {
    #[serde(rename = "MeetID")]
    meet_id: u32,
    #[serde(rename = "LifterID")]
    lifter_id: u32,
    #[serde(rename = "Sex")]
    sex: Sex,
    #[serde(rename = "Event")]
    event: Event,
    #[serde(rename = "Equipment")]
    equipment: Equipment,
    #[serde(rename = "Age")]
    age: Age,
    #[serde(rename = "AgeClass")]
    ageclass: AgeClass,
    #[serde(rename = "Division")]
    division: &'d str,
    #[serde(rename = "BodyweightKg")]
    bodyweightkg: WeightKg,
    #[serde(rename = "WeightClassKg")]
    weightclasskg: WeightClassKg,
    #[serde(rename = "Squat1Kg")]
    squat1kg: WeightKg,
    #[serde(rename = "Squat2Kg")]
    squat2kg: WeightKg,
    #[serde(rename = "Squat3Kg")]
    squat3kg: WeightKg,
    #[serde(rename = "Squat4Kg")]
    squat4kg: WeightKg,
    #[serde(rename = "Best3SquatKg")]
    best3squatkg: WeightKg,
    #[serde(rename = "Bench1Kg")]
    bench1kg: WeightKg,
    #[serde(rename = "Bench2Kg")]
    bench2kg: WeightKg,
    #[serde(rename = "Bench3Kg")]
    bench3kg: WeightKg,
    #[serde(rename = "Bench4Kg")]
    bench4kg: WeightKg,
    #[serde(rename = "Best3BenchKg")]
    best3benchkg: WeightKg,
    #[serde(rename = "Deadlift1Kg")]
    deadlift1kg: WeightKg,
    #[serde(rename = "Deadlift2Kg")]
    deadlift2kg: WeightKg,
    #[serde(rename = "Deadlift3Kg")]
    deadlift3kg: WeightKg,
    #[serde(rename = "Deadlift4Kg")]
    deadlift4kg: WeightKg,
    #[serde(rename = "Best3DeadliftKg")]
    best3deadliftkg: WeightKg,
    #[serde(rename = "TotalKg")]
    totalkg: WeightKg,
    #[serde(rename = "Place")]
    place: Place,
    #[serde(rename = "Wilks")]
    wilks: Points,
    #[serde(rename = "McCulloch")]
    mcculloch: Points,
    #[serde(rename = "Glossbrenner")]
    glossbrenner: Points,
    #[serde(rename = "IPFPoints")]
    ipfpoints: Points,
    #[serde(rename = "Tested")]
    tested: &'static str,
    #[serde(rename = "Country")]
    country: Option<Country>,
}

impl<'d> EntriesRow<'d> {
    fn from(entry: &'d Entry, meet_id: u32, lifter_id: u32) -> EntriesRow<'d> {
        // McCulloch points are calculated as late as possible because they are
        // Age-dependent, and the lifter's Age may be inferred by post-checker phases.
        let mcpts = mcculloch(entry.sex, entry.bodyweightkg, entry.totalkg, entry.age);

        EntriesRow {
            meet_id: meet_id,
            lifter_id: lifter_id,
            sex: entry.sex,
            event: entry.event,
            equipment: entry.equipment,
            age: entry.age,
            ageclass: AgeClass::None, // TODO
            division: &entry.division,
            bodyweightkg: entry.bodyweightkg,
            weightclasskg: entry.weightclasskg,
            squat1kg: entry.squat1kg,
            squat2kg: entry.squat2kg,
            squat3kg: entry.squat3kg,
            squat4kg: entry.squat4kg,
            best3squatkg: entry.best3squatkg,
            bench1kg: entry.bench1kg,
            bench2kg: entry.bench2kg,
            bench3kg: entry.bench3kg,
            bench4kg: entry.bench4kg,
            best3benchkg: entry.best3benchkg,
            deadlift1kg: entry.deadlift1kg,
            deadlift2kg: entry.deadlift2kg,
            deadlift3kg: entry.deadlift3kg,
            deadlift4kg: entry.deadlift4kg,
            best3deadliftkg: entry.best3deadliftkg,
            totalkg: entry.totalkg,
            place: entry.place,
            wilks: entry.wilks,
            mcculloch: mcpts,
            glossbrenner: entry.glossbrenner,
            ipfpoints: entry.ipfpoints,
            tested: if entry.tested { "Yes" } else { "" },
            country: entry.country,
        }
    }
}

pub fn make_csv(meetdata: &[MeetData], buildpath: &Path) -> Result<(), csv::Error> {
    // Generate paths to the individual output files.
    let entries_path = buildpath.join("entries.csv");
    let lifters_path = buildpath.join("lifters.csv");
    let meets_path = buildpath.join("meets.csv");

    // Create CSV writers.
    let mut meets_wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Never)
        .from_path(&meets_path)?;
    let mut entries_wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Never)
        .from_path(&entries_path)?;

    // Data structures for assigning globally-unique IDs.
    let mut next_meet_id: u32 = 0;
    let mut next_lifter_id: u32 = 1; // 0 is for "seanstangl", needed by server tests.
    let mut lifter_hash: HashMap<&str, u32> = HashMap::new();

    for MeetData { meet, entries } in meetdata {
        // Unique ID for this meet.
        let meet_id = next_meet_id;
        next_meet_id += 1;

        // Write out the line for this meet.
        meets_wtr.serialize(MeetsRow::from(&meet, meet_id))?;

        // Write a line for each entry.
        for entry in entries {
            // TODO: Use an actual lifter_id.
            entries_wtr.serialize(EntriesRow::from(&entry, meet_id, 0))?;
        }
    }

    Ok(())
}
