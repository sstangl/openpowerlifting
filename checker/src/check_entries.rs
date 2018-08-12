//! Checks for entries.csv files.

use csv;
use opltypes::*;
use strum::IntoEnumIterator;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use Report;

/// Maps Header to index.
struct HeaderIndexMap(Vec<Option<usize>>);

impl HeaderIndexMap {
    pub fn get(&self, header: Header) -> Option<usize> {
        self.0[header as usize]
    }
}

/// Stores parsed data for a single row.
///
/// The intention is for each field to only be parsed once, after
/// which further processing can use the standard datatype.
#[derive(Default)]
struct Entry {
    pub sex: Option<Sex>,
    pub age: Option<Age>,
    pub place: Option<Place>,
    pub event: Option<Event>,
    pub equipment: Option<Equipment>,
    pub weightclasskg: Option<WeightClassKg>,
    pub bodyweightkg: Option<WeightKg>,
    pub totalkg: Option<WeightKg>,
    pub best3squatkg: Option<WeightKg>,
    pub squat1kg: Option<WeightKg>,
    pub squat2kg: Option<WeightKg>,
    pub squat3kg: Option<WeightKg>,
    pub squat4kg: Option<WeightKg>,
    pub best3benchkg: Option<WeightKg>,
    pub bench1kg: Option<WeightKg>,
    pub bench2kg: Option<WeightKg>,
    pub bench3kg: Option<WeightKg>,
    pub bench4kg: Option<WeightKg>,
    pub best3deadliftkg: Option<WeightKg>,
    pub deadlift1kg: Option<WeightKg>,
    pub deadlift2kg: Option<WeightKg>,
    pub deadlift3kg: Option<WeightKg>,
    pub deadlift4kg: Option<WeightKg>,
}

#[derive(Copy, Clone, Debug, Display, EnumIter, Eq, PartialEq, EnumString)]
enum Header {
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
    // Build a map of (Header -> index).
    let known_header_count = Header::iter().count();
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
        match header.parse::<Header>() {
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

const CYRILLIC_CHARACTERS: &str =
    "абвгдеёжзийклмнопрстуфхцчшщъыьэюя\
     АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ\
     ҐґЄєЖжІіЇї\
     -' ";

fn check_column_cyrillicname(s: &str, line: u64, report: &mut Report) {
    for c in s.chars() {
        if !CYRILLIC_CHARACTERS.contains(c) {
            let msg =
                format!("CyrillicName '{}' contains non-Cyrillic character '{}'", s, c);
            report.error_on(line, msg);
            break;
        }
    }
}

fn check_column_sex(s: &str, line: u64, report: &mut Report) -> Option<Sex> {
    match s.parse::<Sex>() {
        Ok(s) => Some(s),
        Err(_) => {
            report.error_on(line, format!("Invalid Sex '{}'", s));
            None
        }
    }
}

fn check_column_equipment(s: &str, line: u64, report: &mut Report) -> Option<Equipment> {
    match s.parse::<Equipment>() {
        Ok(eq) => Some(eq),
        Err(_) => {
            report.error_on(line, format!("Invalid Equipment '{}'", s));
            None
        }
    }
}

fn check_column_place(s: &str, line: u64, report: &mut Report) -> Option<Place> {
    match s.parse::<Place>() {
        Ok(p) => Some(p),
        Err(_) => {
            report.error_on(line, format!("Invalid Place '{}'", s));
            None
        }
    }
}

fn check_column_age(s: &str, line: u64, report: &mut Report) -> Option<Age> {
    match s.parse::<Age>() {
        Ok(age) => {
            let num = match age {
                Age::Exact(n) => n,
                Age::Approximate(n) => n,
                Age::None => 24,
            };

            if num < 5 {
                report.warning_on(line, format!("Age '{}' unexpectedly low", s));
            } else if num > 100 {
                report.warning_on(line, format!("Age '{}' unexpectedly high", s));
            }

            Some(age)
        }
        Err(_) => {
            report.error_on(line, format!("Invalid Age '{}'", s));
            None
        }
    }
}

fn check_column_event(s: &str, line: u64, headers: &HeaderIndexMap, report: &mut Report) -> Option<Event> {
    match s.parse::<Event>() {
        Ok(event) => {
            if event.has_squat() && headers.get(Header::Best3SquatKg).is_none() {
                report.error_on(line, "Event has 'S', but no Best3SquatKg");
            }
            if event.has_bench() && headers.get(Header::Best3BenchKg).is_none() {
                report.error_on(line, "Event has 'B', but no Best3BenchKg");
            }
            if event.has_deadlift() && headers.get(Header::Best3DeadliftKg).is_none() {
                report.error_on(line, "Event has 'D', but no Best3DeadliftKg");
            }
            Some(event)
        }
        Err(e) => {
            report.error_on(line, format!("Invalid Event '{}': {}", s, e.to_string()));
            None
        }
    }
}

/// Tests a column describing the amount of weight lifted.
fn check_weight(s: &str, line: u64, header: Header, report: &mut Report) -> Option<WeightKg> {
    // Disallow zeros.
    if s == "0" {
        report.error_on(line, format!("{} cannot be zero", header));
    } else if s.starts_with('0') {
        report.error_on(line, format!("{} cannot start with 0 in '{}'", header, s));
    }

    match s.parse::<WeightKg>() {
        Ok(w) => Some(w),
        Err(_) => {
            report.error_on(line, format!("Invalid {} '{}'", header, s));
            None
        }
    }
}

fn check_positive_weight(s: &str, line: u64, header: Header, report: &mut Report) -> Option<WeightKg> {
    if s.starts_with('-') {
        report.error_on(line, format!("{} '{}' cannot be negative", header, s))
    }
    check_weight(s, line, header, report)
}

fn check_column_weightclasskg(s: &str, line: u64, report: &mut Report) -> Option<WeightClassKg> {
    // Disallow zeros.
    if s == "0" {
        report.error_on(line, "WeightClassKg cannot be zero");
    } else if s.starts_with('0') {
        report.error_on(line, format!("WeightClassKg cannot start with 0 in '{}'", s));
    }

    match s.parse::<WeightClassKg>() {
        Ok(w) => Some(w),
        Err(e) => {
            report.error_on(line, format!("Invalid WeightClassKg '{}': {}", s, e));
            None
        }
    }
}

fn check_column_tested(s: &str, line: u64, report: &mut Report) {
    match s {
        "" | "Yes" | "No" => (),
        _ => report.error_on(line, format!("Unknown Tested value '{}'", s)),
    }
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
    let headers: HeaderIndexMap = check_headers(rdr.headers()?, &mut report);
    if !report.messages.is_empty() {
        return Ok(report);
    }

    // This allocation can be re-used for each row.
    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        let line = record.position().map_or(0, |p| p.line());

        // Check each field for whitespace errors.
        for field in &record {
            if field.contains("  ") || field.starts_with(' ') || field.ends_with(' ') {
                let msg = format!("Field '{}' contains extraneous spacing", field);
                report.warning_on(line, msg);
            }
        }

        let mut entry = Entry::default();

        // Check mandatory fields.
        if let Some(idx) = headers.get(Header::Sex) {
            entry.sex = check_column_sex(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Equipment) {
            entry.equipment = check_column_equipment(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Place) {
            entry.place = check_column_place(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Age) {
            entry.age = check_column_age(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Event) {
            entry.event = check_column_event(&record[idx], line, &headers, &mut report);
        }

        // Check all the weight fields.
        // Squat.
        if let Some(idx) = headers.get(Header::Squat1Kg) {
            entry.squat1kg =
                check_weight(&record[idx], line, Header::Squat1Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat2Kg) {
            entry.squat2kg =
                check_weight(&record[idx], line, Header::Squat2Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat3Kg) {
            entry.squat3kg =
                check_weight(&record[idx], line, Header::Squat3Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat4Kg) {
            entry.squat4kg =
                check_weight(&record[idx], line, Header::Squat4Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3SquatKg) {
            entry.best3squatkg =
                check_weight(&record[idx], line, Header::Best3SquatKg, &mut report);
        }

        // Bench.
        if let Some(idx) = headers.get(Header::Bench1Kg) {
            entry.bench1kg =
                check_weight(&record[idx], line, Header::Bench1Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench2Kg) {
            entry.bench2kg =
                check_weight(&record[idx], line, Header::Bench2Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench3Kg) {
            entry.bench3kg =
                check_weight(&record[idx], line, Header::Bench3Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench4Kg) {
            entry.bench4kg =
                check_weight(&record[idx], line, Header::Bench4Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3BenchKg) {
            entry.best3benchkg =
                check_weight(&record[idx], line, Header::Best3BenchKg, &mut report);
        }

        // Deadlift.
        if let Some(idx) = headers.get(Header::Deadlift1Kg) {
            entry.deadlift1kg =
                check_weight(&record[idx], line, Header::Deadlift1Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift2Kg) {
            entry.deadlift2kg =
                check_weight(&record[idx], line, Header::Deadlift2Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift3Kg) {
            entry.deadlift3kg =
                check_weight(&record[idx], line, Header::Deadlift3Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift4Kg) {
            entry.deadlift4kg =
                check_weight(&record[idx], line, Header::Deadlift4Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3DeadliftKg) {
            entry.best3deadliftkg =
                check_weight(&record[idx], line, Header::Best3DeadliftKg, &mut report);
        }

        // Total.
        if let Some(idx) = headers.get(Header::TotalKg) {
            entry.totalkg =
                check_positive_weight(&record[idx], line, Header::TotalKg, &mut report);
        }

        if let Some(idx) = headers.get(Header::BodyweightKg) {
            entry.bodyweightkg =
                check_positive_weight(&record[idx], line, Header::BodyweightKg, &mut report);
        }
        if let Some(idx) = headers.get(Header::WeightClassKg) {
            entry.weightclasskg =
                check_column_weightclasskg(&record[idx], line, &mut report);
        }

        // Check optional fields.
        if let Some(idx) = headers.get(Header::Tested) {
            check_column_tested(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::CyrillicName) {
            check_column_cyrillicname(&record[idx], line, &mut report);
        }
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
