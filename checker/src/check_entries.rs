//! Checks for entries.csv files.

use csv;
use strum::IntoEnumIterator;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use Report;

/// Maps KnownHeader to index.
struct HeaderIndexMap(Vec<Option<usize>>);

#[derive(Copy, Clone, Debug, EnumIter, Eq, PartialEq, EnumString)]
enum KnownHeader {
    Name,
    CyrillicName,
    JapaneseName,
    Sex,
    Age,
    Place,
    Event,
    Division,
    Equipment,
    BirthYear,
    BirthDay,
    Tested,
    AgeClass,

    WeightClassKg,
    BodyweightKg,
    TotalKg,

    Best3SquatKg,
    Squat1Kg,
    Squat2Kg,
    Squat3Kg,
    Squat4Kg,

    Best3BenchKg,
    Bench1Kg,
    Bench2Kg,
    Bench3Kg,
    Bench4Kg,

    Best3DeadliftKg,
    Deadlift1Kg,
    Deadlift2Kg,
    Deadlift3Kg,
    Deadlift4Kg,

    // Columns below this point are ignored.
    Team,
    #[strum(serialize = "Country-State")]
    CountryState,
    Country,
    State,
    #[strum(serialize = "College/University")]
    CollegeUniversity,
    School,
    Category,
}

/// Checks that the headers are valid.
fn check_headers(headers: &csv::StringRecord, report: &mut Report) -> HeaderIndexMap {
    // Build a map of (KnownHeader -> index).
    let known_header_count = KnownHeader::iter().count();
    let mut header_index_map: Vec<Option<usize>> = Vec::with_capacity(known_header_count);
    for _ in 0..known_header_count {
        header_index_map.push(None);
    }

    // There must be headers.
    if headers.is_empty() {
        report.error("Missing column headers");
        return HeaderIndexMap(header_index_map);
    }

    let mut has_squat = false;
    let mut has_bench = false;
    let mut has_deadlift = false;

    for (i, header) in headers.iter().enumerate() {
        // Every header must be known. Build the header_index_map.
        match header.parse::<KnownHeader>() {
            Ok(known) => header_index_map[known as usize] = Some(i),
            Err(_) => report.error(format!("Unknown header '{}'", header)),
        }

        // Test for duplicate headers.
        if headers.iter().skip(i+1).any(|x| x == header) {
            report.error(format!("Duplicate header '{}'", header));
        }

        has_squat = has_squat || header.contains("Squat");
        has_bench = has_bench || header.contains("Bench");
        has_deadlift = has_deadlift || header.contains("Deadlift");
    }

    // If there is data for a particular lift, there must be a 'Best' column.
    if has_squat && !headers.iter().any(|x| x == "Best3SquatKg") {
        report.error("Squat data requires a 'Best3SquatKg' column");
    }
    if has_bench && !headers.iter().any(|x| x == "Best3BenchKg") {
        report.error("Squat data requires a 'Best3BenchKg' column");
    }
    if has_deadlift && !headers.iter().any(|x| x == "Best3DeadliftKg") {
        report.error("Squat data requires a 'Best3DeadliftKg' column");
    }

    // Test for mandatory columns.
    if !headers.iter().any(|x| x == "Name") {
        report.error("There must be a 'Name' column");
    }
    if !headers.iter().any(|x| { x == "BodyweightKg" || x == "WeightClassKg" }) {
        report.error("There must be a 'BodyweightKg' or 'WeightClassKg' column");
    }
    if !headers.iter().any(|x| x == "Sex") {
        report.error("There must be a 'Sex' column");
    }
    if !headers.iter().any(|x| x == "Equipment") {
        report.error("There must be an 'Equipment' column");
    }
    if !headers.iter().any(|x| x == "TotalKg") {
        report.error("There must be a 'TotalKg' column");
    }
    if !headers.iter().any(|x| x == "Place") {
        report.error("There must be a 'Place' column");
    }
    if !headers.iter().any(|x| x == "Event") {
        report.error("There must be an 'Event' column");
    }

    HeaderIndexMap(header_index_map)
}

/// Checks a single entries.csv file from an open `csv::Reader`.
///
/// Extracting this out into a `Reader`-specific function is useful
/// for creating tests that do not have a backing CSV file.
pub fn do_check<R>(
    rdr: &mut csv::Reader<R>,
    mut report: Report,
) -> Result<Report, Box<Error>>
where
    R: io::Read,
{
    check_headers(rdr.headers()?, &mut report);
    if !report.messages.is_empty() {
        return Ok(report);
    }
    
    // This allocation can be re-used for each row.
    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        let line = record.position().map_or(0, |p| p.line());
    }

    Ok(report)
}

/// Checks a single entries.csv file by path.
pub fn check_entries(entries_csv: PathBuf) -> Result<Report, Box<Error>> {
    // Allow the pending Report to own the PathBuf.
    let mut report = Report::new(entries_csv);

    // The entries.csv file must exist.
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(report);
    }

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .from_path(&report.path)?;

    Ok(do_check(&mut rdr, report)?)
}
