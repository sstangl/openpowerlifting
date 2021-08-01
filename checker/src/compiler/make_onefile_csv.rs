//! Transforms `AllMeetData` into the single, humongous `openpowerlifting.csv`
//! offered on the website's Data page. Unlike the other CSV files, which are
//! intended for use by the server, this variant is intended for use by humans.

use coefficients::{dots, goodlift};
use csv::{QuoteStyle, Terminator, WriterBuilder};
use opltypes::*;

use std::path::Path;

use crate::checklib::{Entry, Meet};
use crate::{AllMeetData, SingleMeetData};

fn make_export_row<'a>(entry: &'a Entry, meet: &'a Meet) -> ExportRow<'a> {
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
        dots: dots(entry.sex, entry.bodyweightkg, entry.totalkg),
        wilks: entry.wilks,
        glossbrenner: entry.glossbrenner,
        goodlift: goodlift(
            entry.sex,
            entry.equipment,
            entry.event,
            entry.bodyweightkg,
            entry.totalkg,
        ),
        tested: if entry.tested { "Yes" } else { "" },
        country: entry.country,
        state: entry.state.map(opltypes::states::State::to_state_string),
        federation: meet.federation,
        parent_federation: meet.federation.sanctioning_body(meet.date),
        date: meet.date,
        meet_country: meet.country,
        meet_state: meet.state.map(|s| s.to_state_string()),
        meet_town: meet.town.as_deref(),
        meet_name: &meet.name,
    }
}

pub fn make_onefile_csv(meetdata: &AllMeetData, buildpath: &Path) -> Result<(), csv::Error> {
    let mut csv = WriterBuilder::new()
        .quote_style(QuoteStyle::Never)
        .terminator(Terminator::Any(b'\n'))
        .from_path(&buildpath.join("openpowerlifting.csv"))?;

    for SingleMeetData { meet, entries } in meetdata.get_meets() {
        for entry in entries {
            csv.serialize(make_export_row(entry, meet))?;
        }
    }

    Ok(())
}
