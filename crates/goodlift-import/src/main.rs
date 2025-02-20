use std::io::Cursor;
use std::path::Path;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use serde::Serialize;

use crate::types::Attempt;

mod args;
mod goodlift;
mod openpowerlifting;
mod types;

use crate::args::Args;

const BASE_URL: &str = "https://goodlift.info/get-competitions-report-csv.php?cid=";
const ORIGINAL_FILENAME: &str = "original.csv";
const MEET_FILENAME: &str = "meet.csv";
const ENTRIES_FILENAME: &str = "entries.csv";

#[derive(Default, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Meet {
    federation: Option<String>,
    date: Option<String>,
    meet_country: Option<String>,
    meet_state: Option<String>,
    meet_town: Option<String>,
    meet_name: Option<String>,

    #[serde(skip)]
    is_populated: bool,
}

impl Meet {
    pub fn populate(
        &mut self,
        federation: String,
        date: String,
        meet_country: String,
        meet_state: String,
        meet_town: String,
        meet_name: String,
    ) -> Result<()> {
        if self.is_populated {
            return Ok(());
        }

        let resolved_federation = match federation.as_str() {
            "Commonwealth Powerlifting Federation" => "CommonwealthPF",
            "European Powerlifting Federation" => "EPF",
            "International Powerlifting Federation" => "IPF",
            "Asian Powerlifting Federation" => "APF",
            _ => {
                return Err(eyre!(
                    "Invalid/unknown federation {federation} found in file"
                ))
            }
        };

        self.federation = Some(resolved_federation.to_owned());
        self.date = Some(date);
        self.meet_country = Some(meet_country);
        self.meet_state = Some(meet_state);
        self.meet_town = Some(meet_town);
        self.meet_name = Some(meet_name);

        self.is_populated = true;

        Ok(())
    }
}

type Entries = Vec<openpowerlifting::Row>;

fn parse_content(content: &str) -> Result<(Meet, Entries)> {
    let cursor = Cursor::new(content);
    let mut reader = csv::Reader::from_reader(cursor);

    let mut meet = Meet::default();
    let mut entries = Vec::new();

    for record in reader.deserialize() {
        let row: goodlift::Row = record?;

        meet.populate(
            row.event_federation.clone(),
            row.event_date_begin.clone(),
            map_country(row.event_country.clone()),
            String::new(),
            row.event_city.clone(),
            row.event_title.clone(),
        )?;

        let parsed = openpowerlifting::Row::from(row);

        entries.push(parsed);
    }

    Ok((meet, entries))
}

fn write_csv_file<P: AsRef<Path>, T: Serialize>(filepath: P, data: &T) -> Result<()> {
    let mut writer = csv::Writer::from_path(filepath.as_ref())?;

    writer.serialize(data)?;
    writer.flush()?;

    Ok(())
}

fn decide_best_attempt(first: Attempt, second: Attempt, third: Attempt) -> Attempt {
    if third.was_successful() {
        return third;
    }

    if second.was_successful() {
        return second;
    }

    first
}

fn map_country(country: String) -> String {
    match country.as_str() {
        "Chinese Taipei" => "Taiwan".to_owned(),
        "North Ireland" => "N.Ireland".to_owned(),
        "Great Britain" => "UK".to_owned(),
        "Turkiye" => "Turkey".to_owned(),
        "U.S.America" => "USA".to_owned(),
        "United States" => "USA".to_owned(),
        _ => country,
    }
}

// Function to extract birth year from DOB field
fn extract_birth_year(dob: &str) -> String {
    // If DOB is just a year, return it
    if dob.len() == 4 && dob.parse::<u16>().is_ok() {
        return dob.to_string();
    }

    // If DOB is a full date, extract the year
    if dob.len() >= 10 {
        return dob[0..4].to_string();
    }

    // If DOB is in an unexpected format, return an empty string
    String::new()
}

impl From<goodlift::Row> for openpowerlifting::Row {
    fn from(row: goodlift::Row) -> Self {
        openpowerlifting::Row {
            name: format!("{} {}", row.firstname, row.surname),
            country: map_country(row.country),
            sex: row.gender.to_openpowerlifting(),
            birth_date: None,
            birth_year: extract_birth_year(&row.dob),
            age: row.age,
            division: row.division,
            weight_class_kg: row.weight_class.to_openpowerlifting(),
            bodyweight_kg: row.bodyweight,
            squat_1_kg: row.squat1,
            squat_2_kg: row.squat2,
            squat_3_kg: row.squat3,
            best_3_squat_kg: if row.best_squat.was_successful() {
                // Checking if `best_squat` is successful confirms whether we have a valid best squat from Goodlift
                row.best_squat
            } else {
                decide_best_attempt(row.squat1, row.squat2, row.squat3) // Fallback to calculated best attempt if we don't have a valid Goodlift best attempt
            },

            bench_1_kg: row.bench1,
            bench_2_kg: row.bench2,
            bench_3_kg: row.bench3,
            best_3_bench_kg: if row.best_bench.was_successful() {
                row.best_bench
            } else {
                decide_best_attempt(row.bench1, row.bench2, row.bench3)
            },

            deadlift_1_kg: row.deadlift1,
            deadlift_2_kg: row.deadlift2,
            deadlift_3_kg: row.deadlift3,
            best_3_deadlift_kg: if row.best_deadlift.was_successful() {
                row.best_deadlift
            } else {
                decide_best_attempt(row.deadlift1, row.deadlift2, row.deadlift3)
            },
            total_kg: row.total_kg,
            place: row.total_rank,
            event: row.event.to_openpowerlifting(),
            equipment: row.equipment.to_openpowerlifting(),
        }
    }
}

fn main() -> Result<()> {
    let Args { cid } = Args::parse()?;

    let url = format!("{BASE_URL}{cid}");
    println!("Downloading content from {url}");

    let content = ureq::get(&url).call()?.into_string()?;

    println!("Writing original data to '{ORIGINAL_FILENAME}'");
    std::fs::write(ORIGINAL_FILENAME, &content)?;

    // Sanitise the file, since it may contain null chars
    let content = content.replace(char::from(0), "");

    let (meet, entries) = parse_content(&content)?;

    println!("Writing meet data to '{MEET_FILENAME}'");
    write_csv_file(MEET_FILENAME, &meet)?;

    println!("Writing entries data to '{ENTRIES_FILENAME}'");
    let mut writer = csv::Writer::from_path(ENTRIES_FILENAME)?;

    for entry in entries {
        writer.serialize(entry)?;
    }

    writer.flush()?;

    Ok(())
}
