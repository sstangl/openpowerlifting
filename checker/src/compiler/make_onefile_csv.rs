//! Transforms `AllMeetData` into the single, humongous `openpowerlifting.csv`
//! offered on the website's Data page. Unlike the other CSV files, which are
//! intended for use by the server, this variant is intended for use by humans.

use coefficients::mcculloch;
use csv::{QuoteStyle, Terminator, WriterBuilder};
use opltypes::*;

use std::path::Path;

use crate::checklib::{Entry, Meet};
use crate::{AllMeetData, SingleMeetData};

/// Serialization source for the openpowerlifting.csv.
/// The backing store is the AllMeetData.
#[derive(Serialize)]
struct Row<'d> {
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
    #[serde(rename = "Wilks")]
    pub wilks: Points,
    #[serde(rename = "McCulloch")]
    pub mcculloch: Points,
    #[serde(rename = "Glossbrenner")]
    pub glossbrenner: Points,
    #[serde(rename = "IPFPoints")]
    pub ipfpoints: Points,
    #[serde(rename = "Tested")]
    pub tested: &'static str,
    #[serde(rename = "Country")]
    pub country: Option<Country>,
    #[serde(rename = "Federation")]
    pub federation: Federation,
    #[serde(rename = "Date")]
    pub date: Date,
    #[serde(rename = "MeetCountry")]
    pub meet_country: Country,
    #[serde(rename = "MeetState")]
    pub meet_state: Option<State>,
    #[serde(rename = "MeetName")]
    pub meet_name: &'d str,
}

impl<'d> Row<'d> {
    fn from(entry: &'d Entry, meet: &'d Meet) -> Row<'d> {
        // McCulloch points are calculated as late as possible because they are
        // Age-dependent, and the lifter's Age may be inferred by post-checker phases.
        // TODO: Share this code with make_csv.rs.
        let est_age = if !entry.age.is_none() {
            entry.age
        } else {
            // From known bounds, choose the one that's closest to Senior (~30).
            entry.ageclass.to_range().map_or(Age::None, |(min, max)| {
                if max < Age::Exact(30) {
                    max
                } else {
                    min
                }
            })
        };

        let mcculloch = mcculloch(entry.sex, entry.bodyweightkg, entry.totalkg, est_age);

        Row {
            name: &entry.name,
            sex: entry.sex,
            event: entry.event,
            equipment: entry.equipment,
            age: entry.age,
            ageclass: entry.ageclass,
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
            mcculloch,
            glossbrenner: entry.glossbrenner,
            ipfpoints: entry.ipfpoints,
            tested: if entry.tested { "Yes" } else { "" },
            country: entry.country,
            federation: meet.federation,
            date: meet.date,
            meet_country: meet.country,
            meet_state: meet.state,
            meet_name: &meet.name,
        }
    }
}

pub fn make_onefile_csv(
    meetdata: &AllMeetData,
    buildpath: &Path,
) -> Result<(), csv::Error> {
    let mut csv = WriterBuilder::new()
        .quote_style(QuoteStyle::Never)
        .terminator(Terminator::Any(b'\n'))
        .from_path(&buildpath.join("openpowerlifting.csv"))?;

    for SingleMeetData { meet, entries } in meetdata.get_meets() {
        for entry in entries {
            csv.serialize(Row::from(&entry, &meet))?;
        }
    }

    Ok(())
}
