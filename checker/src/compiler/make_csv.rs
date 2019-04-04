//! Transforms `AllMeetData`` into the final CSV files.

use coefficients::mcculloch;
use csv::{QuoteStyle, Terminator, WriterBuilder};
use hashbrown::HashMap;
use opltypes::*;

use std::path::Path;

use crate::checklib::{Entry, LifterData, LifterDataMap, Meet};
use crate::{AllMeetData, SingleMeetData};

/// Serialization source for the meets.csv.
/// The AllMeetData continues as the backing store.
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

        EntriesRow {
            meet_id,
            lifter_id,
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
        }
    }
}

/// Serialization source for the lifters.csv.
///
/// The `'md` lifetime refers to the `AllMeetData` data owner, while
/// the `'ld` lifetime refers to the LifterDataMap data owner.
#[derive(Serialize)]
struct LiftersRow<'md, 'ld> {
    #[serde(rename = "LifterID")]
    id: u32,
    #[serde(rename = "Name")]
    name: &'md str,
    #[serde(rename = "CyrillicName")]
    cyrillicname: Option<&'md str>,
    #[serde(rename = "Username")]
    username: &'md str,
    #[serde(rename = "Instagram")]
    instagram: Option<&'ld str>,
    #[serde(rename = "VKontakte")]
    vkontakte: Option<&'ld str>,
    #[serde(rename = "Color")]
    color: Option<&'ld str>,
    #[serde(rename = "Flair")]
    flair: Option<&'ld str>,
}

impl<'md, 'ld> LiftersRow<'md, 'ld> {
    fn from(
        entrydata: &'md EntryLifterData,
        lifterdata: &'ld LifterData,
    ) -> LiftersRow<'md, 'ld> {
        LiftersRow {
            id: entrydata.id,
            name: entrydata.name,
            cyrillicname: entrydata.cyrillicname,
            username: entrydata.username,
            instagram: lifterdata.instagram.deref(),
            vkontakte: lifterdata.vkontakte.deref(),
            color: lifterdata.color.deref(),
            flair: lifterdata.flair.deref(),
        }
    }
}

/// A struct for collecting unique lifter data while iterating over the Entries.
struct EntryLifterData<'md> {
    id: u32,
    name: &'md str,
    username: &'md str, // Stored again for simplicity of iteration.
    cyrillicname: Option<&'md str>,
}

impl<'md> EntryLifterData<'md> {
    fn from(entry: &'md Entry, lifter_id: u32) -> EntryLifterData {
        EntryLifterData {
            id: lifter_id,
            name: &entry.name,
            username: &entry.username,
            cyrillicname: entry.cyrillicname.deref(),
        }
    }

    /// This is not solely vanity: server tests require 'seanstangl' with ID 0,
    /// since we needed something with a stable ID to test against.
    fn seanstangl() -> EntryLifterData<'md> {
        EntryLifterData {
            id: 0,
            name: "Sean Stangl",
            username: "seanstangl",
            cyrillicname: Some("Шон Стангл"),
        }
    }
}

/// Map from Username to EntryLifterData.
type EntryLifterDataMap<'md> = HashMap<&'md str, EntryLifterData<'md>>;

pub fn make_csv(
    meetdata: &AllMeetData,
    lifterdata: &LifterDataMap,
    buildpath: &Path,
) -> Result<(), csv::Error> {
    // Create CSV writers.
    let mut entries_wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Never)
        .terminator(Terminator::Any(b'\n'))
        .from_path(&buildpath.join("entries.csv"))?;
    let mut lifters_wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Never)
        .terminator(Terminator::Any(b'\n'))
        .from_path(&buildpath.join("lifters.csv"))?;
    let mut meets_wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Never)
        .terminator(Terminator::Any(b'\n'))
        .from_path(&buildpath.join("meets.csv"))?;

    // For remembering consistent lifter information across multiple Entries.
    let mut lifter_hash = EntryLifterDataMap::new();
    lifter_hash.insert("seanstangl", EntryLifterData::seanstangl());

    // Data structures for assigning globally-unique IDs.
    let mut next_meet_id: u32 = 0;
    let mut next_lifter_id: u32 = 1; // 0 is for "seanstangl", needed by server tests.

    for SingleMeetData { meet, entries } in meetdata.get_meets() {
        // Unique ID for this meet.
        let meet_id = next_meet_id;
        next_meet_id += 1;

        // Write out the line for this meet.
        meets_wtr.serialize(MeetsRow::from(&meet, meet_id))?;

        // Write a line for each entry.
        for entry in entries {
            // See whether this lifter already exists in the EntryLifterDataMap.
            // If it does not, then we haven't seen the lifter before,
            // so a new LifterID is generated.
            let lifter_id = match lifter_hash.get_mut(entry.username.as_str()) {
                Some(data) => {
                    // If there was already data present, maybe the new Entry
                    // has more information that could be attributed.
                    if data.cyrillicname.is_none() && entry.cyrillicname.is_some() {
                        data.cyrillicname = entry.cyrillicname.deref();
                    }
                    data.id
                }
                None => {
                    let lifter_id = next_lifter_id;
                    next_lifter_id += 1;
                    let data = EntryLifterData::from(&entry, lifter_id);
                    lifter_hash.insert(&entry.username, data);
                    lifter_id
                }
            };

            // Write out to entries.csv.
            entries_wtr.serialize(EntriesRow::from(&entry, meet_id, lifter_id))?;
        }
    }

    // With all LifterIDs now assigned, iterate over all lifters in sorted order.
    let mut lifters: Vec<&EntryLifterData> = lifter_hash.values().collect();
    lifters.sort_by_key(|x| x.id);

    for lifter in lifters {
        let default = LifterData::default();
        let data = lifterdata.get(lifter.username).unwrap_or(&default);
        lifters_wtr.serialize(LiftersRow::from(&lifter, &data))?;
    }

    Ok(())
}
