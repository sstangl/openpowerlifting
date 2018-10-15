//! Checks for entries.csv files.

use csv;
use opltypes::*;
use strum::IntoEnumIterator;

use std::error::Error;
use std::io;
use std::path::PathBuf;

use check_config::{Config, Exemption, WeightClassConfig};
use check_meet::Meet;
use Report;

/// List of all plausible weightclasses, for non-configured federations.
const DEFAULT_WEIGHTCLASSES: [WeightClassKg; 54] = [
    // IPF Men.
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(53)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(59)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(66)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(74)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(83)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(93)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(105)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(120)),
    WeightClassKg::Over(WeightKg::from_i32(120)),
    // IPF Women.
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(43)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(47)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(52)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(57)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(63)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(72)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(84)),
    WeightClassKg::Over(WeightKg::from_i32(84)),
    // Traditional and extra classes.
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(30)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(34)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(35)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(39)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(40)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(44)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(48)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(52)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(56)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(60)),
    WeightClassKg::Over(WeightKg::from_i32(60)),
    WeightClassKg::UnderOrEqual(WeightKg::from_raw(67_50)),
    WeightClassKg::Over(WeightKg::from_raw(67_50)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(75)),
    WeightClassKg::Over(WeightKg::from_i32(75)),
    WeightClassKg::UnderOrEqual(WeightKg::from_raw(82_50)),
    WeightClassKg::Over(WeightKg::from_i32(83)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(90)),
    WeightClassKg::Over(WeightKg::from_i32(90)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(100)),
    WeightClassKg::Over(WeightKg::from_i32(100)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(110)),
    WeightClassKg::Over(WeightKg::from_i32(110)),
    WeightClassKg::UnderOrEqual(WeightKg::from_raw(117_50)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(125)),
    WeightClassKg::Over(WeightKg::from_i32(125)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(140)),
    WeightClassKg::Over(WeightKg::from_i32(140)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(145)),
    WeightClassKg::Over(WeightKg::from_i32(145)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(155)),
    WeightClassKg::Over(WeightKg::from_i32(155)),
    // ProRaw classes.
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(70)),
    WeightClassKg::Over(WeightKg::from_i32(70)),
    WeightClassKg::Over(WeightKg::from_i32(70)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(80)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(95)),
];

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
    pub sex: Sex,
    pub age: Age,
    pub place: Place,
    pub event: Event,
    pub division: String,
    pub equipment: Option<Equipment>,
    pub squat_equipment: Option<Equipment>,
    pub bench_equipment: Option<Equipment>,
    pub deadlift_equipment: Option<Equipment>,

    pub weightclasskg: WeightClassKg,
    pub bodyweightkg: WeightKg,

    // Weights, defaulting to zero.
    pub totalkg: WeightKg,
    pub best3squatkg: WeightKg,
    pub squat1kg: WeightKg,
    pub squat2kg: WeightKg,
    pub squat3kg: WeightKg,
    pub squat4kg: WeightKg,
    pub best3benchkg: WeightKg,
    pub bench1kg: WeightKg,
    pub bench2kg: WeightKg,
    pub bench3kg: WeightKg,
    pub bench4kg: WeightKg,
    pub best3deadliftkg: WeightKg,
    pub deadlift1kg: WeightKg,
    pub deadlift2kg: WeightKg,
    pub deadlift3kg: WeightKg,
    pub deadlift4kg: WeightKg,
}

impl Entry {
    /// Whether the Entry contains any Squat data.
    #[inline]
    pub fn has_squat_data(&self) -> bool {
        self.best3squatkg.is_non_zero()
            || self.squat1kg.is_non_zero()
            || self.squat2kg.is_non_zero()
            || self.squat3kg.is_non_zero()
            || self.squat4kg.is_non_zero()
    }

    /// Whether the Entry contains any Bench data.
    #[inline]
    pub fn has_bench_data(&self) -> bool {
        self.best3benchkg.is_non_zero()
            || self.bench1kg.is_non_zero()
            || self.bench2kg.is_non_zero()
            || self.bench3kg.is_non_zero()
            || self.bench4kg.is_non_zero()
    }

    /// Whether the Entry contains any Deadlift data.
    #[inline]
    pub fn has_deadlift_data(&self) -> bool {
        self.best3deadliftkg.is_non_zero()
            || self.deadlift1kg.is_non_zero()
            || self.deadlift2kg.is_non_zero()
            || self.deadlift3kg.is_non_zero()
            || self.deadlift4kg.is_non_zero()
    }
}

#[derive(Copy, Clone, Debug, Display, EnumIter, Eq, PartialEq, EnumString)]
enum Header {
    Name,
    CyrillicName,
    JapaneseName,
    ChineseName,
    Sex,
    Age,
    Place,
    Event,
    Division,
    Equipment,
    SquatEquipment,
    BenchEquipment,
    DeadliftEquipment,
    BirthYear,
    BirthDay,
    Tested,
    AgeClass,
    Country,

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
    State,
    #[strum(serialize = "College/University")]
    CollegeUniversity,
    School,
    Category,
}

/// Checks that the headers are valid.
fn check_headers(
    headers: &csv::StringRecord,
    config: Option<&Config>,
    report: &mut Report,
) -> HeaderIndexMap {
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
        if headers.iter().skip(i + 1).any(|x| x == header) {
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
        report.error("Bench data requires a 'Best3BenchKg' column");
    }
    if has_deadlift && !headers.iter().any(|x| x == "Best3DeadliftKg") {
        report.error("Deadlift data requires a 'Best3DeadliftKg' column");
    }

    // Test for mandatory columns.
    if !headers.iter().any(|x| x == "Name") {
        report.error("There must be a 'Name' column");
    }
    if !headers
        .iter()
        .any(|x| x == "BodyweightKg" || x == "WeightClassKg")
    {
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

    // Configured federations must have standardized divisions,
    // and therefore must have a "Division" column.
    if config.is_some() && !headers.iter().any(|x| x == "Division") {
        report.error("Configured federations require a 'Division' column");
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
            let msg = format!(
                "CyrillicName '{}' contains non-Cyrillic character '{}'",
                s, c
            );
            report.error_on(line, msg);
            break;
        }
    }
}

fn check_column_birthyear(s: &str, meet: Option<&Meet>, line: u64, report: &mut Report) {
    if s.is_empty() {
        return;
    }

    match s.parse::<u32>() {
        Ok(year) => {
            if year < 1000 || year > 9999 {
                report.error_on(line, format!("BirthYear '{}' must have 4 digits", year));
            }

            // Compare the BirthYear to the meet date for some basic sanity checks.
            if let Some(m) = meet {
                if year > m.date.year() - 4 || m.date.year() - year > 98 {
                    report.error_on(
                        line,
                        format!("BirthYear '{}' looks implausible", year),
                    );
                }
            }
        }
        Err(_) => {
            report.error_on(line, format!("BirthYear '{}' must be a number", s));
        }
    }
}

fn check_column_birthday(s: &str, meet: Option<&Meet>, line: u64, report: &mut Report) {
    if s.is_empty() {
        return;
    }
    match s.parse::<Date>() {
        Ok(birthday) => {
            // Compare the BirthDay to the meet date for some basic sanity checks.
            match meet {
                Some(m) => {
                    if birthday.year() >= m.date.year() - 4
                        || m.date.year() - birthday.year() > 98
                    {
                        report.error_on(
                            line,
                            format!("BirthDay '{}' looks implausible", s),
                        );
                    }
                }
                None => {}
            }
        }
        Err(e) => {
            report.error_on(line, format!("Invalid BirthDay '{}': '{}'", s, e));
        }
    }
}

fn check_column_sex(s: &str, line: u64, report: &mut Report) -> Sex {
    match s.parse::<Sex>() {
        Ok(s) => s,
        Err(_) => {
            report.error_on(line, format!("Invalid Sex '{}'", s));
            Sex::default()
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

fn check_column_squatequipment(
    s: &str,
    line: u64,
    report: &mut Report,
) -> Option<Equipment> {
    if s.is_empty() {
        return None;
    }
    match s.parse::<Equipment>() {
        Ok(eq) => {
            if eq == Equipment::Straps {
                report.error_on(line, "SquatEquipment can't be 'Straps'");
            }
            Some(eq)
        }
        Err(_) => {
            report.error_on(line, format!("Invalid SquatEquipment '{}'", s));
            None
        }
    }
}

fn check_column_benchequipment(
    s: &str,
    line: u64,
    report: &mut Report,
) -> Option<Equipment> {
    if s.is_empty() {
        return None;
    }
    match s.parse::<Equipment>() {
        Ok(eq) => {
            if eq == Equipment::Wraps {
                report.error_on(line, "BenchEquipment can't be 'Wraps'");
            } else if eq == Equipment::Straps {
                report.error_on(line, "BenchEquipment can't be 'Straps'");
            }
            Some(eq)
        }
        Err(_) => {
            report.error_on(line, format!("Invalid BenchEquipment '{}'", s));
            None
        }
    }
}

fn check_column_deadliftequipment(
    s: &str,
    line: u64,
    report: &mut Report,
) -> Option<Equipment> {
    if s.is_empty() {
        return None;
    }

    match s.parse::<Equipment>() {
        Ok(eq) => {
            if eq == Equipment::Wraps {
                report.error_on(line, "DeadliftEquipment can't be 'Wraps'");
            }
            Some(eq)
        }
        Err(_) => {
            report.error_on(line, format!("Invalid DeadliftEquipment '{}'", s));
            None
        }
    }
}

fn check_column_place(s: &str, line: u64, report: &mut Report) -> Place {
    match s.parse::<Place>() {
        Ok(p) => p,
        Err(_) => {
            report.error_on(line, format!("Invalid Place '{}'", s));
            Place::default()
        }
    }
}

fn check_column_age(s: &str, line: u64, report: &mut Report) -> Age {
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

            age
        }
        Err(_) => {
            report.error_on(line, format!("Invalid Age '{}'", s));
            Age::default()
        }
    }
}

fn check_column_event(
    s: &str,
    line: u64,
    headers: &HeaderIndexMap,
    report: &mut Report,
) -> Event {
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
            event
        }
        Err(e) => {
            report.error_on(line, format!("Invalid Event '{}': {}", s, e.to_string()));
            Event::default()
        }
    }
}

/// Tests a column describing the amount of weight lifted.
fn check_weight(s: &str, line: u64, header: Header, report: &mut Report) -> WeightKg {
    // Disallow zeros.
    if s == "0" {
        report.error_on(line, format!("{} cannot be zero", header));
    } else if s.starts_with('0') {
        report.error_on(line, format!("{} cannot start with 0 in '{}'", header, s));
    }

    match s.parse::<WeightKg>() {
        Ok(w) => w,
        Err(_) => {
            report.error_on(line, format!("Invalid {} '{}'", header, s));
            WeightKg::default()
        }
    }
}

fn check_positive_weight(
    s: &str,
    line: u64,
    header: Header,
    report: &mut Report,
) -> WeightKg {
    if s.starts_with('-') {
        report.error_on(line, format!("{} '{}' cannot be negative", header, s))
    }
    check_weight(s, line, header, report)
}

fn check_column_bodyweightkg(s: &str, line: u64, report: &mut Report) -> WeightKg {
    let weight = check_positive_weight(s, line, Header::BodyweightKg, report);
    if weight != WeightKg::from_i32(0) {
        if weight < WeightKg::from_i32(15) || weight > WeightKg::from_i32(300) {
            report.error_on(line, format!("Implausible BodyweightKg '{}'", s));
        }
    }
    weight
}

fn check_column_weightclasskg(s: &str, line: u64, report: &mut Report) -> WeightClassKg {
    // Disallow zeros.
    if s == "0" {
        report.error_on(line, "WeightClassKg cannot be zero");
    } else if s.starts_with('0') {
        report.error_on(
            line,
            format!("WeightClassKg cannot start with 0 in '{}'", s),
        );
    }

    match s.parse::<WeightClassKg>() {
        Ok(w) => w,
        Err(e) => {
            report.error_on(line, format!("Invalid WeightClassKg '{}': {}", s, e));
            WeightClassKg::default()
        }
    }
}

fn check_column_tested(s: &str, line: u64, report: &mut Report) {
    match s {
        "" | "Yes" | "No" => (),
        _ => report.error_on(line, format!("Unknown Tested value '{}'", s)),
    }
}

fn check_column_division(
    s: &str,
    config: Option<&Config>,
    exempt_division: bool,
    line: u64,
    report: &mut Report,
) {
    if exempt_division {
        return;
    }

    let config = match config {
        Some(config) => config,
        None => {
            return;
        }
    };

    // The division must appear in the configuration file.
    if !config.divisions.iter().any(|d| d.name == s) {
        report.error_on(line, format!("Unknown division '{}'", s));
    }
}

fn check_column_country(s: &str, line: u64, report: &mut Report) {
    if !s.is_empty() && s.parse::<Country>().is_err() {
        report.error_on(line, format!("Unknown Country '{}'", s));
    }
}

fn check_column_state(s: &str, line: u64, report: &mut Report) {
    if !s.is_empty() && !s.is_ascii() {
        report.error_on(line, format!("State '{}' must be ASCII", s));
    }
}

fn check_event_and_total_consistency(entry: &Entry, line: u64, report: &mut Report) {
    let event = entry.event;
    let has_squat_data: bool = entry.has_squat_data();
    let has_bench_data: bool = entry.has_bench_data();
    let has_deadlift_data: bool = entry.has_deadlift_data();

    // Check that lift data isn't present outside of the specified Event.
    if has_squat_data && !event.has_squat() {
        report.error_on(line, format!("Event '{}' cannot have squat data", event));
    }
    if has_bench_data && !event.has_bench() {
        report.error_on(line, format!("Event '{}' cannot have bench data", event));
    }
    if has_deadlift_data && !event.has_deadlift() {
        report.error_on(line, format!("Event '{}' cannot have deadlift data", event));
    }

    // Check that the Equipment makes sense for the given Event.
    if let Some(equipment) = entry.equipment {
        if equipment == Equipment::Wraps && !event.has_squat() {
            report.error_on(line, format!("Event '{}' doesn't use Wraps", event));
        }
        if equipment == Equipment::Straps && !event.has_deadlift() {
            report.error_on(line, format!("Event '{}' doesn't use Straps", event));
        }

        // Check that the SquatEquipment makes sense.
        if let Some(squat_eq) = entry.squat_equipment {
            if squat_eq > equipment {
                report.error_on(
                    line,
                    format!(
                        "SquatEquipment '{}' can't be more supportive \
                         than the Equipment '{}'",
                        squat_eq, equipment
                    ),
                );
            }
        }

        // Check that the BenchEquipment makes sense.
        if let Some(bench_eq) = entry.bench_equipment {
            if bench_eq > equipment {
                report.error_on(
                    line,
                    format!(
                        "BenchEquipment '{}' can't be more supportive \
                         than the Equipment '{}'",
                        bench_eq, equipment
                    ),
                );
            }
        }

        // Check that the DeadliftEquipment makes sense.
        if let Some(deadlift_eq) = entry.deadlift_equipment {
            if deadlift_eq > equipment {
                report.error_on(
                    line,
                    format!(
                        "DeadliftEquipment '{}' can't be more supportive \
                         than the Equipment '{}'",
                        deadlift_eq, equipment
                    ),
                );
            }
        }
    }

    // If the lifter wasn't DQ'd, they should have data from each lift.
    if !entry.place.is_dq() {
        // Allow entries that only have a Total but no lift data.
        if has_squat_data || has_bench_data || has_deadlift_data {
            if !has_squat_data && event.has_squat() {
                let s = format!("Non-DQ Event '{}' requires squat data", event);
                report.error_on(line, s);
            }
            if !has_bench_data && event.has_bench() {
                let s = format!("Non-DQ Event '{}' requires bench data", event);
                report.error_on(line, s);
            }
            if !has_deadlift_data && event.has_deadlift() {
                let s = format!("Non-DQ Event '{}' requires deadlift data", event);
                report.error_on(line, s);
            }
        }
    }

    // Check that TotalKg matches the Place.
    let has_totalkg: bool = entry.totalkg != WeightKg::from_i32(0);
    if entry.place.is_dq() && has_totalkg {
        report.error_on(line, format!("DQ'd entries cannot have a TotalKg"));
    } else if !entry.place.is_dq() && !has_totalkg {
        report.error_on(line, format!("Non-DQ entries must have a TotalKg"));
    }

    // Check that a non-DQ lifter's total is the sum of their best attempts,
    // if their lifts have been recorded.
    if !entry.place.is_dq()
        && has_totalkg
        && (entry.best3squatkg.is_non_zero()
            || entry.best3benchkg.is_non_zero()
            || entry.best3deadliftkg.is_non_zero())
    {
        let calculated = entry.best3squatkg + entry.best3benchkg + entry.best3deadliftkg;

        if (calculated - entry.totalkg).abs() > WeightKg::from_f32(0.5) {
            let s = format!(
                "Calculated TotalKg '{}', but meet recorded '{}'",
                calculated, entry.totalkg
            );
            report.error_on(line, s)
        }
    }
}

// Compares an attempt versus the current ascending weight.
// Returns the new value for maxweight.
fn process_attempt_pair(
    lift: &str,
    attempt_num: u32,
    maxweight: WeightKg,
    attempt: WeightKg,
    exempt_lift_order: bool,
    line: u64,
    report: &mut Report,
) -> WeightKg {
    // Only check the attempt if it was actually attempted.
    if attempt == WeightKg::from_i32(0) {
        return maxweight;
    }

    // If nothing has been attempted thus far, this is the new highest attempt.
    if maxweight == WeightKg::from_i32(0) {
        return attempt;
    }

    // The bar weight shouldn't have lowered.
    if !exempt_lift_order && attempt.abs() < maxweight.abs() {
        report.error_on(
            line,
            format!(
                "{}{}Kg '{}' lowered weight from '{}'",
                lift, attempt_num, attempt, maxweight
            ),
        );
    }

    // A successful attempt shouldn't have been repeated.
    if !maxweight.is_failed() && attempt.abs() == maxweight {
        report.error_on(
            line,
            format!(
                "{}{}Kg '{}' repeated a successful attempt",
                lift, attempt_num, attempt
            ),
        );
    }

    if attempt.abs() >= maxweight.abs() {
        attempt
    } else {
        maxweight
    }
}

fn check_attempt_consistency_helper(
    lift: &str,
    attempt1: WeightKg,
    attempt2: WeightKg,
    attempt3: WeightKg,
    attempt4: WeightKg,
    best3lift: WeightKg,
    exempt_lift_order: bool,
    line: u64,
    report: &mut Report,
) {
    // Check that the bar weight is ascending over attempts.
    let mut maxweight = process_attempt_pair(
        lift,
        2,
        attempt1,
        attempt2,
        exempt_lift_order,
        line,
        report,
    );
    maxweight = process_attempt_pair(
        lift,
        3,
        maxweight,
        attempt3,
        exempt_lift_order,
        line,
        report,
    );
    process_attempt_pair(
        lift,
        4,
        maxweight,
        attempt4,
        exempt_lift_order,
        line,
        report,
    );

    // Check the Best3Lift validity.
    let best = attempt1.max(attempt2.max(attempt3));

    // If the best attempt was successful, it should be in the Best3Lift.
    if best > WeightKg::from_i32(0) && best != best3lift {
        report.error_on(
            line,
            format!(
                "Best3{}Kg '{}' does not match best attempt '{}'",
                lift, best3lift, best
            ),
        );
    }

    // If the best attempt was a failure, the least failure can be in the Best3Lift.
    if best < WeightKg::from_i32(0) && best3lift != WeightKg::from_i32(0) {
        if best != best3lift {
            let s = format!(
                "Best3{}Kg '{}' does not match least failed attempt '{}'",
                lift, best3lift, best
            );
            report.error_on(line, s);
        }
    }
}

fn check_attempt_consistency(
    entry: &Entry,
    exempt_lift_order: bool,
    line: u64,
    report: &mut Report,
) {
    // Squat attempts.
    check_attempt_consistency_helper(
        "Squat",
        entry.squat1kg,
        entry.squat2kg,
        entry.squat3kg,
        entry.squat4kg,
        entry.best3squatkg,
        exempt_lift_order,
        line,
        report,
    );

    // Bench attempts.
    check_attempt_consistency_helper(
        "Bench",
        entry.bench1kg,
        entry.bench2kg,
        entry.bench3kg,
        entry.bench4kg,
        entry.best3benchkg,
        exempt_lift_order,
        line,
        report,
    );

    // Deadlift attempts.
    check_attempt_consistency_helper(
        "Deadlift",
        entry.deadlift1kg,
        entry.deadlift2kg,
        entry.deadlift3kg,
        entry.deadlift4kg,
        entry.best3deadliftkg,
        exempt_lift_order,
        line,
        report,
    );
}

/// Checks that gear wasn't used prior to its date of invention.
fn check_equipment_year(
    entry: &Entry,
    meet: Option<&Meet>,
    line: u64,
    report: &mut Report,
) {
    // Helper function for checking equipped status.
    fn is_equipped(e: Option<Equipment>) -> bool {
        e.map_or(false, |eq| match eq {
            Equipment::Raw | Equipment::Wraps | Equipment::Straps => false,
            Equipment::Single | Equipment::Multi => true,
        })
    }

    // Inelegant unwrapping.
    let date = match meet.and_then(|m| Some(&m.date)) {
        Some(d) => d,
        None => {
            return;
        }
    };
    let event = entry.event;

    // Years of equipment invention.
    let squat_suit_invention_year = 1977;
    let bench_shirt_invention_year = 1985;

    // TODO: This is just a safe value.
    // Need to figure out when deadlift suits were invented.
    let deadlift_suit_invention_year = 1980;

    // Check that squat equipment isn't listed before its invention.
    if date.year() < squat_suit_invention_year
        && (is_equipped(entry.squat_equipment)
            || (event.has_squat() && is_equipped(entry.equipment)))
    {
        report.error_on(
            line,
            format!(
                "Squat equipment wasn't invented until {}",
                squat_suit_invention_year
            ),
        );
    }

    // Check that bench equipment isn't listed before its invention.
    // TODO: This avoids conflation with the squat equipment.
    if date.year() < bench_shirt_invention_year
        && (is_equipped(entry.bench_equipment)
            || (event.has_bench() && !event.has_squat() && is_equipped(entry.equipment)))
    {
        report.error_on(
            line,
            format!(
                "Bench shirts weren't invented until {}",
                bench_shirt_invention_year
            ),
        );
    }

    // Check that deadlift equipment isn't listed before its invention.
    // TODO: This avoids conflation with the squat equipment.
    if date.year() < deadlift_suit_invention_year
        && (is_equipped(entry.deadlift_equipment)
            || (event.has_deadlift()
                && !event.has_squat()
                && is_equipped(entry.equipment)))
    {
        report.error_on(
            line,
            format!(
                "Deadlift suits weren't invented until {}",
                deadlift_suit_invention_year
            ),
        );
    }
}

fn check_weightclass_consistency(
    entry: &Entry,
    meet: Option<&Meet>,
    config: Option<&Config>,
    exempt_weightclass_consistency: bool,
    line: u64,
    report: &mut Report,
) {
    // If the configuration exempts this check, do nothing.
    if exempt_weightclass_consistency {
        return;
    }

    // If there's no weightclass data, there's nothing to check.
    if entry.weightclasskg == WeightClassKg::None {
        // Configured federations should have weightclass data.
        if config.is_some() {
            report.warning_on(line, "Configured federations cannot omit WeightClassKg");
        }
        return;
    }

    // Any provided bodyweight should at least be plausible.
    if entry.bodyweightkg.is_non_zero()
        && !entry.weightclasskg.matches_bodyweight(entry.bodyweightkg)
    {
        report.warning_on(
            line,
            format!(
                "BodyweightKg '{}' not in WeightClassKg '{}'",
                entry.bodyweightkg, entry.weightclasskg
            ),
        );
    }

    // If there's nothing configured, we can still do some basic checks.
    if config.is_none() {
        // Check that the weightclass appears in the list of known defaults.
        if !DEFAULT_WEIGHTCLASSES
            .iter()
            .any(|c| *c == entry.weightclasskg)
        {
            report.warning_on(
                line,
                format!(
                    "Unknown unconfigured WeightClassKg '{}'",
                    entry.weightclasskg
                ),
            );
        }
        return;
    }

    // The no-config case was handled above, so the config can be known here.
    let config = config.unwrap();
    let date = meet.map_or(Date::from_u32(20160101), |m| m.date);

    // Attempt to find out what weightclass group this row is a member of.
    //
    // Groups are specified in an arbitrary order with date, sex, and division data.
    // We want to find the one group that matches most closely.
    let mut matched_group: Option<&WeightClassConfig> = None;
    for group in &config.weightclasses {
        // Sex and date information are mandatory and must match.
        if date < group.date_min || date > group.date_max || entry.sex != group.sex {
            continue;
        }

        // If there is a division qualifier, it must match.
        let division_matches = match group.divisions {
            Some(ref divs) => divs
                .iter()
                .any(|x| config.divisions[*x].name == entry.division),
            None => true,
        };

        if !division_matches {
            continue;
        }

        // Everything matches. Determine whether this group has a narrower scope
        // than the currently best-known group.
        if let Some(best) = matched_group {
            // Ignore this group if it drops division information.
            if best.divisions.is_some() && !group.divisions.is_some() {
                continue;
            }

            // Check if two things matched and we don't have reason to prefer
            // one over the other! That's an error in the configuration file,
            // so whine at the user and select an arbitrary group.
            if best.divisions.is_some() == group.divisions.is_some() {
                report.error_on(
                    line,
                    format!(
                        "Matched both [weightclasses.{}] and [weightclasses.{}]",
                        best.name, group.name
                    ),
                );
            }
        }

        matched_group = Some(group);
    }

    // If no group matched, the config is in trouble.
    if matched_group.is_none() {
        report.error_on(
            line,
            "Could not match to any weightclass group in the CONFIG.toml",
        );
        return;
    }

    // We've matched to a particular group in the config!
    let matched_group = matched_group.unwrap();

    // Find the index of the weightclass in the list.
    let index: Option<usize> = matched_group
        .classes
        .iter()
        .enumerate()
        .find(|&(_, w)| *w == entry.weightclasskg)
        .map_or(None, |(i, _)| Some(i));

    if index.is_none() {
        report.warning_on(
            line,
            format!(
                "WeightClassKg '{}' not found in [weightclasses.{}]",
                entry.weightclasskg, matched_group.name
            ),
        );
    } else if let Some(index) = index {
        // The bodyweight was already verified to be in the weightclass
        // by a check above, but now we additionally check that the bodyweight
        // isn't *also* in the weightclass that comes before it in the
        // ordered classes vector.
        //
        // But don't do this for SHW, because there can be multiple SHW
        // weightclasses and the decision of which to use is arbitrary.
        if entry.bodyweightkg.is_non_zero()
            && !entry.weightclasskg.is_shw()
            && index > 0
            && matched_group.classes[index - 1].matches_bodyweight(entry.bodyweightkg)
        {
            // This is an error state, but we can calculate a more helpful message.
            // Iterate over all of the classes in order and find the first one
            // that matches.
            let first_match = matched_group
                .classes
                .iter()
                .find(|c| c.matches_bodyweight(entry.bodyweightkg))
                .unwrap();

            report.warning_on(
                line,
                format!(
                    "BodyweightKg '{}' matches '{}', not '{}' in [weightclasses.{}]",
                    entry.bodyweightkg,
                    first_match,
                    entry.weightclasskg,
                    matched_group.name
                ),
            );
        }
    }
}

/// Checks a single entries.csv file from an open `csv::Reader`.
///
/// Extracting this out into a `Reader`-specific function is useful
/// for creating tests that do not have a backing CSV file.
pub fn do_check<R>(
    rdr: &mut csv::Reader<R>,
    meet: Option<&Meet>,
    config: Option<&Config>,
    mut report: Report,
) -> Result<Report, Box<Error>>
where
    R: io::Read,
{
    // Scan for check exemptions.
    let exemptions = {
        let parent_folder = &report.get_parent_folder()?;
        config.map_or(None, |c| c.exemptions_for(parent_folder))
    };
    let exempt_lift_order: bool = exemptions.map_or(false, |el| {
        el.iter().any(|&e| e == Exemption::ExemptLiftOrder)
    });
    let exempt_division: bool = exemptions.map_or(false, |el| {
        el.iter().any(|&e| e == Exemption::ExemptDivision)
    });
    let exempt_weightclass_consistency: bool = exemptions.map_or(false, |el| {
        el.iter()
            .any(|&e| e == Exemption::ExemptWeightClassConsistency)
    });

    let headers: HeaderIndexMap = check_headers(rdr.headers()?, config, &mut report);
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
                report.error_on(line, msg);
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
        if let Some(idx) = headers.get(Header::SquatEquipment) {
            entry.squat_equipment =
                check_column_squatequipment(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::BenchEquipment) {
            entry.bench_equipment =
                check_column_benchequipment(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::DeadliftEquipment) {
            entry.deadlift_equipment =
                check_column_deadliftequipment(&record[idx], line, &mut report);
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

        // Check all the weight fields: they must contain non-zero values.
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

        // TotalKg is a positive weight if present or 0 if missing.
        if let Some(idx) = headers.get(Header::TotalKg) {
            entry.totalkg =
                check_positive_weight(&record[idx], line, Header::TotalKg, &mut report);
        }

        if let Some(idx) = headers.get(Header::BodyweightKg) {
            entry.bodyweightkg =
                check_column_bodyweightkg(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::WeightClassKg) {
            entry.weightclasskg =
                check_column_weightclasskg(&record[idx], line, &mut report);
        }

        // Check optional fields.
        if let Some(idx) = headers.get(Header::Division) {
            check_column_division(
                &record[idx],
                config,
                exempt_division,
                line,
                &mut report,
            );
            entry.division = record[idx].to_string();
        }
        if let Some(idx) = headers.get(Header::Country) {
            check_column_country(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::State) {
            check_column_state(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Tested) {
            check_column_tested(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::CyrillicName) {
            check_column_cyrillicname(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::BirthYear) {
            check_column_birthyear(&record[idx], meet, line, &mut report);
        }
        if let Some(idx) = headers.get(Header::BirthDay) {
            check_column_birthday(&record[idx], meet, line, &mut report);
        }

        // Check consistency across fields.
        check_event_and_total_consistency(&entry, line, &mut report);
        check_attempt_consistency(&entry, exempt_lift_order, line, &mut report);
        check_equipment_year(&entry, meet, line, &mut report);
        check_weightclass_consistency(
            &entry,
            meet,
            config,
            exempt_weightclass_consistency,
            line,
            &mut report,
        );
    }

    Ok(report)
}

/// Checks a single entries.csv file by path.
pub fn check_entries(
    entries_csv: PathBuf,
    meet: Option<&Meet>,
    config: Option<&Config>,
) -> Result<Report, Box<Error>> {
    // Allow the pending Report to own the PathBuf.
    let mut report = Report::new(entries_csv);

    // The entries.csv file must exist.
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(report);
    }

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(&report.path)?;

    Ok(do_check(&mut rdr, meet, config, report)?)
}
