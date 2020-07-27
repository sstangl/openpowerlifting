//! Beta API definitions.

use langpack::*;
use opldb::query::direct::RankingsQuery;
use opldb::OplDb;
use opltypes::PointsSystem;
use serde::ser::{Serialize, SerializeStruct};

/// Pseudo-column types, used for selection.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Column {
    /// The zero-indexed, context-dependent row number.
    RowId,

    // Lifter information.
    Username,
    Name,
    Sex,
    Event,
    Equipment,
    Age,
    AgeClass,
    BirthYearClass,
    Division,
    Bodyweight,
    WeightClass,
    Place,
    Tested,
    Country,

    // Lifts.
    Squat1,
    Squat2,
    Squat3,
    Squat4,
    Best3Squat,
    Best4Squat,
    Bench1,
    Bench2,
    Bench3,
    Bench4,
    Best3Bench,
    Best4Bench,
    Deadlift1,
    Deadlift2,
    Deadlift3,
    Deadlift4,
    Best3Deadlift,
    Best4Deadlift,
    Total,

    // Meet information.
    MeetPath,
    Federation,
    ParentFederation,
    Date,
    MeetCountry,
    MeetState,
    MeetTown,
    MeetName,

    // Coefficients.
    AH,
    Dots,
    Glossbrenner,
    Goodlift,
    IPFPoints,
    McCulloch,
    NASA,
    Reshel,
    SchwartzMalone,
    Wilks,
    Wilks2020,
}

/// Options for a #[RankingsQuery].
///
/// The options are specified via JSON in the body of the HTTP POST request.
#[derive(Debug, Deserialize)]
pub struct RankingsOptions {
    /// Index of the first row from which to return rankings.
    ///
    /// Indices are zero-indexed, so the first row has index `0`.
    pub start: usize,

    /// Index of the last row from which to return rankings, inclusive.
    ///
    /// If the `end` is out of bounds, the server will clamp it to the
    /// last possible valid row.
    ///
    /// Although the API allows you to request any range of rows, in practice
    /// the number of rows returned per request is limited to a maximum,
    /// in order to enforce pagination.
    pub end: usize,

    /// List of columns that should be included in the output.
    pub columns: Vec<Column>,

    /// The units system in which weights should be reported.
    pub units: opltypes::WeightUnits,

    /// The language system used for rendering strings and numbers.
    pub language: langpack::Language,
}

/// A row in the returned JSON.
///
/// This struct holds all information necessary to complete serialize
/// the final JSON object representing the row.
pub struct RankingsReturnRow<'a> {
    // Environment information.
    pub opldb: &'a OplDb,
    pub locale: &'a Locale<'a>,
    pub columns: &'a [Column],

    // Row information.
    pub row_id: usize,
    pub entry_id: u32,
}

impl<'a> Serialize for RankingsReturnRow<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let num_keys = self.columns.len() + 1; // RowId is mandatory.
        let mut s = serializer.serialize_struct("unused", num_keys)?;

        let e: &opldb::Entry = self.opldb.get_entry(self.entry_id);
        let m: &opldb::Meet = self.opldb.get_meet(e.meet_id);
        let l: &opldb::Lifter = self.opldb.get_lifter(e.lifter_id);

        let t: &langpack::Translations = self.locale.strings;
        let units: opltypes::WeightUnits = self.locale.units;
        let nf = self.locale.language.number_format();

        s.serialize_field("RowId", &self.row_id)?;
        for &column in self.columns {
            use Column::*;
            match column {
                RowId => s.serialize_field("RowId", &self.row_id)?,
                Username => s.serialize_field("Username", &l.username)?,
                Name => {
                    let v = get_localized_name(l, self.locale.language);
                    s.serialize_field("Name", v)?;
                }
                Sex => s.serialize_field("Sex", t.translate_sex(e.sex))?,
                Event => s.serialize_field("Event", &e.event)?,
                Equipment => {
                    let v = t.translate_equipment(e.equipment);
                    s.serialize_field("Equipment", v)?;
                }
                Age => s.serialize_field("Age", &e.age)?,
                AgeClass => s.serialize_field("AgeClass", &e.ageclass)?,
                BirthYearClass => {
                    s.serialize_field("BirthYearClass", &e.birthyearclass)?;
                }
                Division => s.serialize_field("Division", &e.division)?,
                Bodyweight => {
                    let v = e.bodyweightkg.as_type(units).in_format(nf);
                    s.serialize_field("Bodyweight", &v)?;
                }
                WeightClass => {
                    let v = e.weightclasskg.as_type(units).in_format(nf);
                    s.serialize_field("WeightClass", &v)?;
                }
                Place => {
                    let v = LocalizedPlace::from(e.place, self.locale.language, e.sex);
                    s.serialize_field("Place", &v)?;
                }
                Tested => s.serialize_field("Tested", &e.tested)?,
                Country => {
                    if let Some(c) = e.lifter_country {
                        s.serialize_field("Country", t.translate_country(c))?;
                    } else {
                        let v: Option<opltypes::Country> = None;
                        s.serialize_field("Country", &v)?;
                    }
                }
                Squat1 => {
                    let v = e.squat1kg.as_type(units).in_format(nf);
                    s.serialize_field("Squat1", &v)?;
                }
                Squat2 => {
                    let v = e.squat2kg.as_type(units).in_format(nf);
                    s.serialize_field("Squat2", &v)?;
                }
                Squat3 => {
                    let v = e.squat3kg.as_type(units).in_format(nf);
                    s.serialize_field("Squat3", &v)?;
                }
                Squat4 => {
                    let v = e.squat4kg.as_type(units).in_format(nf);
                    s.serialize_field("Squat4", &v)?;
                }
                Best3Squat => {
                    let v = e.best3squatkg.as_type(units).in_format(nf);
                    s.serialize_field("Best3Squat", &v)?;
                }
                Best4Squat => {
                    let n = e.best3squatkg.max(e.squat4kg);
                    let v = n.as_type(units).in_format(nf);
                    s.serialize_field("Best4Squat", &v)?;
                }
                Bench1 => {
                    let v = e.bench1kg.as_type(units).in_format(nf);
                    s.serialize_field("Bench1", &v)?;
                }
                Bench2 => {
                    let v = e.bench2kg.as_type(units).in_format(nf);
                    s.serialize_field("Bench2", &v)?;
                }
                Bench3 => {
                    let v = e.bench3kg.as_type(units).in_format(nf);
                    s.serialize_field("Bench3", &v)?;
                }
                Bench4 => {
                    let v = e.bench4kg.as_type(units).in_format(nf);
                    s.serialize_field("Bench4", &v)?;
                }
                Best3Bench => {
                    let v = e.best3benchkg.as_type(units).in_format(nf);
                    s.serialize_field("Best3Bench", &v)?;
                }
                Best4Bench => {
                    let n = e.best3benchkg.max(e.squat4kg);
                    let v = n.as_type(units).in_format(nf);
                    s.serialize_field("Best4Bench", &v)?;
                }
                Deadlift1 => {
                    let v = e.deadlift1kg.as_type(units).in_format(nf);
                    s.serialize_field("Deadlift1", &v)?;
                }
                Deadlift2 => {
                    let v = e.deadlift2kg.as_type(units).in_format(nf);
                    s.serialize_field("Deadlift2", &v)?;
                }
                Deadlift3 => {
                    let v = e.deadlift3kg.as_type(units).in_format(nf);
                    s.serialize_field("Deadlift3", &v)?;
                }
                Deadlift4 => {
                    let v = e.deadlift4kg.as_type(units).in_format(nf);
                    s.serialize_field("Deadlift4", &v)?;
                }
                Best3Deadlift => {
                    let v = e.best3deadliftkg.as_type(units).in_format(nf);
                    s.serialize_field("Best3Deadlift", &v)?;
                }
                Best4Deadlift => {
                    let n = e.best3deadliftkg.max(e.squat4kg);
                    let v = n.as_type(units).in_format(nf);
                    s.serialize_field("Best4Deadlift", &v)?;
                }
                Total => {
                    let v = e.totalkg.as_type(units).in_format(nf);
                    s.serialize_field("Total", &v)?;
                }
                MeetPath => s.serialize_field("MeetPath", &m.path)?,
                Federation => s.serialize_field("Federation", &m.federation)?,
                ParentFederation => {
                    if let Some(v) = m.federation.sanctioning_body(m.date) {
                        s.serialize_field("ParentFederation", &v)?;
                    } else {
                        let v: Option<opltypes::Federation> = None;
                        s.serialize_field("ParentFederation", &v)?;
                    }
                }
                Date => s.serialize_field("Date", &m.date)?,
                MeetCountry => {
                    let v = t.translate_country(m.country);
                    s.serialize_field("MeetCountry", &v)?;
                }
                MeetState => s.serialize_field("MeetState", &m.state)?,
                MeetTown => s.serialize_field("MeetTown", &m.town)?,
                MeetName => s.serialize_field("MeetName", &m.name)?,
                AH => {
                    let v = e.points(PointsSystem::AH, units);
                    s.serialize_field("AH", &v.in_format(nf))?;
                }
                Dots => {
                    let v = e.points(PointsSystem::Dots, units);
                    s.serialize_field("Dots", &v.in_format(nf))?;
                }
                Glossbrenner => {
                    let v = e.points(PointsSystem::Glossbrenner, units);
                    s.serialize_field("Glossbrenner", &v.in_format(nf))?;
                }
                Goodlift => {
                    let v = e.points(PointsSystem::Goodlift, units);
                    s.serialize_field("Goodlift", &v.in_format(nf))?;
                }
                IPFPoints => {
                    let v = e.points(PointsSystem::IPFPoints, units);
                    s.serialize_field("IPFPoints", &v.in_format(nf))?;
                }
                McCulloch => {
                    let v = e.points(PointsSystem::McCulloch, units);
                    s.serialize_field("McCulloch", &v.in_format(nf))?;
                }
                NASA => {
                    let v = e.points(PointsSystem::NASA, units);
                    s.serialize_field("NASA", &v.in_format(nf))?;
                }
                Reshel => {
                    let v = e.points(PointsSystem::Reshel, units);
                    s.serialize_field("Reshel", &v.in_format(nf))?;
                }
                SchwartzMalone => {
                    let v = e.points(PointsSystem::SchwartzMalone, units);
                    s.serialize_field("SchwartzMalone", &v.in_format(nf))?;
                }
                Wilks => {
                    let v = e.points(PointsSystem::Wilks, units);
                    s.serialize_field("Wilks", &v.in_format(nf))?;
                }
                Wilks2020 => {
                    let v = e.points(PointsSystem::Wilks2020, units);
                    s.serialize_field("Wilks2020", &v.in_format(nf))?;
                }
            }
        }
        s.end()
    }
}

/// Return JSON from a rankings endpoint.
#[derive(Default, Serialize)]
pub struct RankingsReturn<'a> {
    /// Used for reporting errors.
    error: Option<&'static str>,

    /// Total number of rows in the rankings selection.
    total_length: usize,

    /// The returned rows, as an array of JSON objects.
    rows: Vec<RankingsReturnRow<'a>>,
}

impl<'a> RankingsReturn<'a> {
    pub fn from(
        opldb: &'a OplDb,
        locale: &'a Locale,
        query: &'a RankingsQuery,
        options: &'a RankingsOptions,
    ) -> Self {
        const ROW_LIMIT: usize = 100;

        if options.columns.len() == 0 {
            return Self {
                error: Some("No columns requested"),
                ..Self::default()
            };
        }

        let list = opldb::algorithms::get_full_sorted_uniqued(&query, &opldb);
        let total_length = list.0.len();

        let start = options.start;
        let mut end = options.end;

        // Limit the request size to something sane.
        // Arithmetic can't overflow since it's already been compared to total_length.
        if end - start + 1 > ROW_LIMIT {
            end = start + ROW_LIMIT
        }

        // The request [start..=end] must be in-bounds.
        if end >= total_length {
            if total_length > 0 {
                end = total_length - 1;
            } else {
                end = 0;
            }
        }
        if (start > end) || (start >= total_length) {
            return Self {
                total_length,
                error: Some("The 'start' parameter is invalid"),
                ..Self::default()
            };
        }

        let columns = &options.columns;
        let rows: Vec<RankingsReturnRow> = list.0[start..=end]
            .iter()
            .zip(start..)
            .map(|(&n, i)| RankingsReturnRow {
                opldb,
                locale,
                columns,
                row_id: i,
                entry_id: n,
            })
            .collect();

        Self {
            total_length,
            rows,
            ..Self::default()
        }
    }
}
