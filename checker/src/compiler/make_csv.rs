//! Transforms a `Vec<MeetData>` into the final CSV files.

use csv::{QuoteStyle, WriterBuilder};
use opltypes::*;

use std::path::Path;

use crate::checklib::Meet;
use crate::MeetData;

/// Serialization source for the meets.csv.
/// The MeetData continues as the backing store.
#[derive(Serialize)]
struct MeetRow<'d> {
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

impl<'d> MeetRow<'d> {
    fn from(meet: &'d Meet, meet_id: u32) -> MeetRow<'d> {
        MeetRow {
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

pub fn make_csv(meetdata: &[MeetData], buildpath: &Path) -> Result<(), csv::Error> {
    // Generate paths to the individual output files.
    let entries_path = buildpath.join("entries.csv");
    let lifters_path = buildpath.join("lifters.csv");
    let meet_path = buildpath.join("meets.csv");

    // Create CSV writers.
    let mut meet_wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Never)
        .from_path(&meet_path)?;

    let mut next_meet_id: u32 = 0;

    for MeetData { meet, entries } in meetdata {
        // Unique ID for this meet.
        let meet_id = next_meet_id;
        next_meet_id += 1;

        meet_wtr.serialize(MeetRow::from(&meet, meet_id))?;
    }

    Ok(())
}
