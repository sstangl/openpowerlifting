//! Transforms `AllMeetData` into the single, humongous `openpowerlifting.csv`
//! offered on the website's Data page. Unlike the other CSV files, which are
//! intended for use by the server, this variant is intended for use by humans.

use coefficients::mcculloch;
use csv::{QuoteStyle, Terminator, WriterBuilder};
use opltypes::*;

use std::path::Path;

use crate::checklib::{Entry, Meet};
use crate::{AllMeetData, SingleMeetData};

fn make_export_row<'a>(entry: &'a Entry, meet: &'a Meet) -> ExportRow<'a> {
    // McCulloch points are calculated as late as possible because they are
    // Age-dependent, and the lifter's Age may be inferred by post-checker phases.
    let est_age = if !entry.age.is_none() {
        entry.age
    } else {
        // Round toward Senior (~30).
        let (min, max) = (entry.agerange.min, entry.agerange.max);
        if max.is_some() && max < Age::Exact(30) {
            max
        } else if min.is_some() && min > Age::Exact(30) {
            min
        } else {
            Age::None
        }
    };

    let mcculloch = mcculloch(entry.sex, entry.bodyweightkg, entry.totalkg, est_age);

    ExportRow {
        name: &entry.name,
        sex: entry.sex,
        event: entry.event,
        equipment: entry.equipment,
        age: entry.age,
        ageclass: AgeClass::from(entry.agerange),
        birthyearclass: entry.birthyearclass,
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
        meet_state: meet.state.and_then(|s| Some(s.to_state_string())),
        meet_name: &meet.name,
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
            csv.serialize(make_export_row(&entry, &meet))?;
        }
    }

    Ok(())
}
