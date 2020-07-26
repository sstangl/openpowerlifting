//! Beta API definitions.

use langpack::{get_localized_name, Locale};
use opldb::query::direct::RankingsQuery;
use opldb::OplDb;
use serde_json::Value;

use std::collections::HashMap;

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
    /*
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
    Hoffman,
    IPF,
    McCulloch,
    NASA,
    Reshel,
    SchwartzMalone,
    Wilks2020,
    Wilks,
    */
}

pub fn make_value(
    opldb: &OplDb,
    locale: &Locale,
    entry: &opldb::Entry,
    column: Column,
) -> serde_json::Value {
    let lifter = opldb.get_lifter(entry.lifter_id);
    let tr = locale.strings;

    use Column::*;
    match column {
        RowId => Value::Number((-1i32).into()), // Added manually elsewhere.

        Username => Value::String(lifter.username.as_str().to_string()),
        Name => Value::String(get_localized_name(lifter, locale.language).to_string()),
        Sex => Value::String(tr.translate_sex(entry.sex).to_string()),
        Event => Value::String(format!("{}", entry.event)),
        Equipment => Value::String(tr.translate_equipment(entry.equipment).to_string()),
        Age => Value::String(format!("{}", entry.age)),
    }
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

/// Return JSON from a rankings endpoint.
#[derive(Debug, Default, Serialize)]
pub struct RankingsReturn {
    /// Used for reporting errors.
    error: Option<&'static str>,

    /// Total number of rows in the rankings selection.
    total_length: usize,

    /// The returned rows, as an array of JSON objects.
    rows: Vec<HashMap<Column, serde_json::Value>>,
}

impl RankingsReturn {
    pub fn from(
        opldb: &OplDb,
        locale: &Locale,
        query: &RankingsQuery,
        options: &RankingsOptions,
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

        let rows: Vec<HashMap<Column, serde_json::Value>> = list.0[start..=end]
            .iter()
            .zip(start..)
            .map(|(&n, i)| {
                let entry = opldb.get_entry(n);
                let mut hash = HashMap::with_capacity(options.columns.len() + 1);

                for &column in &options.columns {
                    let value = make_value(&opldb, &locale, &entry, column);
                    hash.insert(column, value);
                }
                hash.insert(Column::RowId, Value::Number(i.into()));

                hash
            })
            .collect();

        Self {
            total_length,
            rows,
            ..Self::default()
        }
    }
}
