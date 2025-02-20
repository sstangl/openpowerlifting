//! Checks for entries.csv files.

use opltypes::states::*;
use opltypes::*;
use smartstring::alias::CompactString;
use strum::IntoEnumIterator;
use unicode_normalization::UnicodeNormalization;

use std::borrow::Cow;
use std::error::Error;
use std::io;
use std::path::PathBuf;

use crate::checklib::config::{Config, Exemption, WeightClassConfig};
use crate::checklib::lifterdata::LifterDataMap;
use crate::checklib::meet::Meet;
use crate::{EntryIndex, Report};

/// List of all plausible weightclasses, for non-configured federations.
const DEFAULT_WEIGHTCLASSES: [WeightClassKg; 51] = [
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
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(69)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(72)),
    WeightClassKg::UnderOrEqual(WeightKg::from_i32(76)),
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
];

/// Maps Header to index.
struct HeaderIndexMap(Vec<Option<usize>>);

impl HeaderIndexMap {
    pub fn get(&self, header: Header) -> Option<usize> {
        self.0[header as usize]
    }
    pub fn has(&self, header: Header) -> bool {
        self.0[header as usize].is_some()
    }
}

pub struct EntriesCheckResult {
    pub report: Report,
    // TODO: Memory could be improved further by using bump allocation in an arena.
    //  Have each worker thread allocate entries into its own pool, rather than a separate
    //  allocation per meet.
    pub entries: Option<Box<[Entry]>>,
}

/// Returns s as a string in Unicode NFKC form.
///
/// Unicode decompositions take 2 forms, canonical equivalence and
/// compatibility.
///
/// Canonical equivalence means that characters or sequences of characters
/// represent the same written character and should always be displayed the
/// same. For example Ω and Ω are canonically equivalent, as are Ç and C+◌̧.
///
/// Compatibility means that characters or sequences of characters represent the
/// same written character but may be displayed differently.
/// For example ｶ and カ are compatible, as are ℌ and H.
///
/// NFKC form decomposes characters by compatibility and then recomposes by
/// canonical equivalence. We want NFKC form as half-width characters should
/// display the same as full width characters on the site, as should font
/// variants.
fn canonicalize_name_utf8(s: &str) -> Cow<'_, str> {
    // Fast-path: the majority of names are ASCII.
    if s.is_ascii() {
        return Cow::Borrowed(s);
    }
    Cow::from(s.nfkc().collect::<String>())
}

/// Stores parsed data for a single row.
///
/// The intention is for each field to only be parsed once, after
/// which further processing can use the standard datatype.
#[derive(Default)]
pub struct Entry {
    /// A measurement shows that 99.27% of names benefit from `CompactString`.
    pub name: CompactString,
    pub username: Username,

    // These should not be made `CompactString`: that massively *increases* memory.
    pub chinesename: Option<String>,
    pub cyrillicname: Option<String>,
    pub greekname: Option<String>,
    pub japanesename: Option<String>,
    pub koreanname: Option<String>,

    pub sex: Sex,
    pub place: Place,
    pub event: Event,
    pub division: CompactString,
    pub equipment: Equipment,
    pub squat_equipment: Option<Equipment>,
    pub bench_equipment: Option<Equipment>,
    pub deadlift_equipment: Option<Equipment>,

    /// The recorded age of the lifter at the time of competition.
    pub age: Age,

    /// May be explicitly specified, or inferred by other age information.
    /// Division age ranges are carried here (possibly after further narrowing).
    pub agerange: AgeRange,
    pub birthyearrange: BirthYearRange,
    // The BirthYearClass is kept on the entry because the CSV exporting functions
    // do not have the ability to look up the meet date.
    pub birthyearclass: BirthYearClass,
    pub birthdate: Option<Date>,
    pub entrydate: Date,

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

    /// Whether the entry counts as tested. Does not imply testing actually occurred.
    pub tested: bool,

    pub country: Option<Country>,
    pub state: Option<State>,

    /// The index of this `Entry` in the `AllMeetData`.
    ///
    /// Because this refers to vector indices in the final `AllMeetData`,
    /// it can only be constructed after the checker is completely finished.
    pub index: Option<EntryIndex>,
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

    /// Uses `Age`, `BirthYear`, and `BirthDate` columns to calculate
    /// the lifter's `Age` on a given date.
    pub fn age_on(&self, date: Date) -> Age {
        // If the age is provided explicitly, just use that.
        if self.age != Age::None {
            return self.age;
        }

        // If the BirthDate is provided, calculate an exact age.
        if let Some(birthdate) = self.birthdate {
            if let Ok(age) = birthdate.age_on(date) {
                return age;
            }
        }

        // If the BirthYear is provided, calculate an approximate age.
        if let Some(birthyear) = self.birthyearrange.exact_birthyear() {
            if date.year() >= birthyear {
                return Age::from_birthyear_on_date(birthyear, date);
            }
        }

        Age::None
    }
}

#[derive(Copy, Clone, Debug, Display, EnumIter, Eq, PartialEq, EnumString)]
enum Header {
    Name,
    ChineseName,
    CyrillicName,
    JapaneseName,
    KoreanName,
    GreekName,
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
    BirthDate,
    Tested,
    AgeRange,
    Country,
    EntryDate,

    // Weights in kilograms.
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

    // Weights in pounds. These get converted to kilograms.
    WeightClassLbs,
    BodyweightLbs,
    TotalLbs,

    Best3SquatLbs,
    Squat1Lbs,
    Squat2Lbs,
    Squat3Lbs,
    Squat4Lbs,

    Best3BenchLbs,
    Bench1Lbs,
    Bench2Lbs,
    Bench3Lbs,
    Bench4Lbs,

    Best3DeadliftLbs,
    Deadlift1Lbs,
    Deadlift2Lbs,
    Deadlift3Lbs,
    Deadlift4Lbs,

    // Columns below this point are ignored.
    Team,
    State,
    #[strum(serialize = "College/University")]
    CollegeUniversity,
    School,
}

/// Checks that the headers are valid.
fn check_headers(
    headers: &csv::StringRecord,
    meet: &Meet,
    config: Option<&Config>,
    report: &mut Report,
) -> HeaderIndexMap {
    // Build a map of (Header -> index).
    let known_header_count = Header::iter().count();
    let mut header_index_vec: Vec<Option<usize>> = Vec::with_capacity(known_header_count);
    header_index_vec.resize(known_header_count, None);

    // There must be headers.
    if headers.is_empty() {
        report.error("Missing column headers");
        return HeaderIndexMap(header_index_vec);
    }

    let mut has_squat = false;
    let mut has_bench = false;
    let mut has_deadlift = false;

    for (i, header) in headers.iter().enumerate() {
        // Every header must be known. Build the header_index_vec.
        match header.parse::<Header>() {
            Ok(known) => {
                // Error on duplicate headers.
                if header_index_vec[known as usize].is_some() {
                    report.error(format!("Duplicate header '{header}'"));
                }
                header_index_vec[known as usize] = Some(i)
            }
            Err(_) => report.error(format!("Unknown header '{header}'")),
        }

        has_squat = has_squat || header.contains("Squat");
        has_bench = has_bench || header.contains("Bench");
        has_deadlift = has_deadlift || header.contains("Deadlift");
    }

    let header_map = HeaderIndexMap(header_index_vec);

    // If there is data for a particular lift, there must be a 'Best' column.
    if has_squat && !header_map.has(Header::Best3SquatKg) && !header_map.has(Header::Best3SquatLbs)
    {
        report.error("Squat data requires a 'Best3SquatKg' or 'Best3SquatLbs' column");
    }
    if has_bench && !header_map.has(Header::Best3BenchKg) && !header_map.has(Header::Best3BenchLbs)
    {
        report.error("Bench data requires a 'Best3BenchKg' or 'Best3BenchLbs' column");
    }
    if has_deadlift
        && !header_map.has(Header::Best3DeadliftKg)
        && !header_map.has(Header::Best3DeadliftLbs)
    {
        report.error("Deadlift data requires a 'Best3DeadliftKg' or 'Best3DeadliftLbs' column");
    }

    // Require mandatory columns.
    {
        use Header::*;
        const MANDATORY_COLUMNS: [Header; 5] = [Name, Sex, Equipment, Place, Event];
        for column in &MANDATORY_COLUMNS {
            if !header_map.has(*column) {
                report.error(format!("There must be a '{column}' column"));
            }
        }
        // There must be a total.
        if !header_map.has(Header::TotalKg) && !header_map.has(Header::TotalLbs) {
            report.error("There must be a 'TotalKg' or 'TotalLbs' column".to_string());
        }

        // Some weight-based information must be present.
        if !header_map.has(Header::WeightClassKg)
            && !header_map.has(Header::WeightClassLbs)
            && !header_map.has(Header::BodyweightKg)
            && !header_map.has(Header::BodyweightLbs)
        {
            report.error("There must be a 'BodyweightKg' or 'WeightClassKg' column (or in Lbs)");
        }
    }

    // Configured federations must have standardized divisions,
    // and therefore must have a "Division" column.
    if let Some(config) = config {
        // But only if the configuration file actually specifies divisions!
        if !header_map.has(Header::Division) && !config.divisions.is_empty() {
            report.error("Configured federations require a 'Division' column");
        }
    }

    // We commonly add lifter BirthDates when lifters ask us to fix their age data.
    // Unfortunately, since git is text-oriented, adding a column is significantly
    // more costly than changing just one row.
    //
    // To avoid excessive version-control churn, the BirthDate column is mandatory
    // (even if totally blank) for all meets since 2020.
    if meet.date.year() >= 2020 && !header_map.has(Header::BirthDate) {
        report.error("The BirthDate column is mandatory for all meets since 2020");
    }

    header_map
}

fn check_column_name(name: &str, line: u64, report: &mut Report) -> CompactString {
    // Allow discarding disambiguation (everything after optional '#').
    let mut s = name;

    // Lifters with the same name are disambiguated by tagging them
    // with an integer preceded by the '#' character.
    if let Some(i) = s.find('#') {
        // The '#' must be preceded by a space.
        if i > 0 && s.get(i - 1..i) != Some(" ") {
            report.error_on(line, format!("Name '{s}' must have a space before '#'"));
        }

        // Everything after the '#' must be an integer.
        if let Some(number) = s.get(i + 1..) {
            for c in number.chars() {
                if !c.is_ascii_digit() {
                    report.error_on(line, format!("Name '{s}' can only have numbers after '#'"));
                    break;
                }
            }
        } else {
            report.error_on(line, format!("Name '{s}' must have a number after '#'"));
        }

        // For the purposes of the checks below, ignore the disambiguation.
        s = name.get(..i).unwrap().trim_end();
    }

    // Standardize on suffices without periods. Also just in general.
    if s.ends_with('.') {
        report.error_on(line, format!("Name '{name}' cannot end with a period"));
    }

    // All characters must be alphabetical or one of some few exceptions.
    for c in s.chars() {
        if !c.is_alphabetic() && c != ' ' && c != '\'' && c != '.' && c != '-' {
            report.error_on(line, format!("Name '{name}' contains illegal characters"));
            break;
        }
    }

    // Look at each component in part.
    for (word_index, mut word) in s.split(' ').enumerate() {
        // Some words are known exceptions, assuming they're not the first.
        if word_index != 0 {
            match word {
                // Common short words that mostly translate to "the".
                "bin" | "da" | "de" | "do" | "del" | "den" | "der" | "des" | "di" | "dos"
                | "du" | "e" | "el" | "i" | "in" | "in 't" | "in't" | "la" | "le" | "los"
                | "op" | "of" | "'t" | "te" | "ten" | "ter" | "und" | "v" | "v." | "v.d."
                | "van" | "von" | "zur" | "y" | "zu" => {
                    continue;
                }

                // Standardize Dutch names on "v.d.".
                "vd" | "v.d" | "vd." | "V.D." => {
                    report.error_on(line, format!("Name '{name}' should use 'v.d.'"));
                    continue;
                }
                _ => (),
            }
        }

        // Some French names begin with "d'". Ignore that part.
        // Spanish names should be capitalized like "DeLeon".
        if word.starts_with("d'") {
            word = word.get(2..).unwrap();
        }

        // Disallow nicknames. They're usually written as "Tom 'Tommy' Thompson".
        // Allow 't, the abbreviation for the dutch word het.
        if word.starts_with('\'') {
            report.error_on(line, format!("Name '{name}' cannot contain nicknames"));
            continue;
        }

        // Punctuation should never be a separate word.
        if word == "-" || word == "." || word == "'" {
            report.error_on(line, format!("Name '{name}' has separable punctuation"));
            continue;
        }

        // Name components must usually start capitalized, with exceptions.
        for c in word.chars().take(1) {
            if !c.is_uppercase() {
                report.error_on(
                    line,
                    format!("Name '{name}' must have '{word}' capitalized"),
                );
            }
        }
    }

    // Complain about meet data that got left over.
    if s.ends_with("DT") || s.ends_with("SP") || s.ends_with("MP") {
        report.error_on(line, format!("Name '{name}' contains lifting information"));
    }

    // Complain about Junior/Senior at the start of the name. USAPL does this.
    if s.starts_with("Jr ") || s.starts_with("Sr ") {
        report.error_on(line, format!("Name '{name}' needs Jr/Sr moved to end"));
    }

    // Suffices that must be fully-capitalized.
    if s.ends_with("Ii") || s.ends_with("Iii") {
        report.error_on(
            line,
            format!("Name '{name}' must have suffix fully-capitalized"),
        );
    }

    canonicalize_name_utf8(name).into()
}

fn check_column_chinesename(s: &str, line: u64, report: &mut Report) -> Option<String> {
    if s.is_empty() {
        return None;
    }

    for c in s.chars() {
        if writing_system(c) != WritingSystem::CJK && c != ' ' && c != '·' {
            let msg = format!("ChineseName '{s}' contains non-CJK character '{c}'");
            report.error_on(line, msg);
            return None;
        }
    }
    Some(canonicalize_name_utf8(s).into())
}

fn check_column_cyrillicname(s: &str, line: u64, report: &mut Report) -> Option<String> {
    if s.is_empty() {
        return None;
    }

    for c in s.chars() {
        if writing_system(c) != WritingSystem::Cyrillic && !"-' .".contains(c) {
            let msg = format!("CyrillicName '{s}' contains non-Cyrillic character '{c}'");
            report.error_on(line, msg);
            return None;
        }
    }
    Some(canonicalize_name_utf8(s).into())
}

fn check_column_japanesename(s: &str, line: u64, report: &mut Report) -> Option<String> {
    if s.is_empty() {
        return None;
    }

    for c in s.chars() {
        if writing_system(c) != WritingSystem::Japanese
            && writing_system(c) != WritingSystem::CJK
            && c != ' '
        {
            let msg = format!("JapaneseName '{s}' contains non-Japanese character '{c}'");
            report.error_on(line, msg);
            return None;
        }
    }
    Some(canonicalize_name_utf8(s).into())
}

fn check_column_greekname(s: &str, line: u64, report: &mut Report) -> Option<String> {
    if s.is_empty() {
        return None;
    }

    for c in s.chars() {
        if writing_system(c) != WritingSystem::Greek && !"-' .".contains(c) {
            let msg = format!("GreekName '{s}' contains non-Greek character '{c}'");
            report.error_on(line, msg);
            return None;
        }
    }
    Some(canonicalize_name_utf8(s).into())
}

fn check_column_koreanname(s: &str, line: u64, report: &mut Report) -> Option<String> {
    if s.is_empty() {
        return None;
    }

    for c in s.chars() {
        if writing_system(c) != WritingSystem::Korean
            && writing_system(c) != WritingSystem::Japanese
            && !"-' .".contains(c)
        {
            let msg = format!("KoreanName '{s}' contains non-Korean character '{c}'");
            report.error_on(line, msg);
            return None;
        }
    }
    Some(canonicalize_name_utf8(s).into())
}

fn check_column_birthyear(s: &str, meet: &Meet, line: u64, report: &mut Report) -> Option<u32> {
    if s.is_empty() {
        return None;
    }

    let year = match s.parse::<u32>() {
        Ok(year) => year,
        Err(_) => {
            report.error_on(line, format!("BirthYear '{s}' must be a number"));
            return None;
        }
    };

    // A year must have four digits.
    if !(1000..=9999).contains(&year) {
        report.error_on(line, format!("BirthYear '{year}' must have 4 digits"));
        return None;
    }

    // Compare the BirthYear to the meet date for some basic sanity checks.
    if year > meet.date.year().saturating_sub(4) || meet.date.year().saturating_sub(year) > 98 {
        report.error_on(line, format!("BirthYear '{year}' looks implausible"));
        return None;
    }
    Some(year)
}

fn check_column_birthdate(s: &str, meet: &Meet, line: u64, report: &mut Report) -> Option<Date> {
    if s.is_empty() {
        return None;
    }

    match s.parse::<Date>() {
        Ok(bd) => {
            // Compare the BirthDate to the meet date for some basic sanity checks.
            if bd.year() >= meet.date.year() - 4 || meet.date.year() - bd.year() > 98 {
                report.error_on(line, format!("BirthDate '{s}' looks implausible"));
                return None;
            }

            if let Err(e) = bd.age_on(meet.date) {
                report.error_on(line, format!("BirthDate '{s}' error: {e}"));
                return None;
            }

            // Ensure that the BirthDate exists in the Gregorian calendar.
            if !bd.is_valid() {
                let msg = format!("BirthDate '{s}' does not exist in the Gregorian calendar");
                report.error_on(line, msg);
            }

            Some(bd)
        }
        Err(e) => {
            report.error_on(line, format!("Invalid BirthDate '{s}': '{e}'"));
            None
        }
    }
}

fn check_column_sex(s: &str, line: u64, report: &mut Report) -> Sex {
    match s.parse::<Sex>() {
        Ok(s) => s,
        Err(_) => {
            report.error_on(line, format!("Invalid Sex '{s}'"));
            Sex::default()
        }
    }
}

fn check_column_equipment(s: &str, line: u64, report: &mut Report) -> Equipment {
    match s.parse::<Equipment>() {
        Ok(eq) => eq,
        Err(_) => {
            report.error_on(line, format!("Invalid Equipment '{s}'"));
            Equipment::Multi
        }
    }
}

fn check_column_squatequipment(s: &str, line: u64, report: &mut Report) -> Option<Equipment> {
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
            report.error_on(line, format!("Invalid SquatEquipment '{s}'"));
            None
        }
    }
}

fn check_column_benchequipment(s: &str, line: u64, report: &mut Report) -> Option<Equipment> {
    if s.is_empty() {
        return None;
    }
    match s.parse::<Equipment>() {
        Ok(eq) => {
            if eq == Equipment::Wraps || eq == Equipment::Straps {
                report.error_on(line, "BenchEquipment can't be '{s}'");
            }
            Some(eq)
        }
        Err(_) => {
            report.error_on(line, format!("Invalid BenchEquipment '{s}'"));
            None
        }
    }
}

fn check_column_deadliftequipment(s: &str, line: u64, report: &mut Report) -> Option<Equipment> {
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
            report.error_on(line, format!("Invalid DeadliftEquipment '{s}'"));
            None
        }
    }
}

fn check_column_place(s: &str, line: u64, report: &mut Report) -> Place {
    match s.parse::<Place>() {
        Ok(p) => p,
        Err(_) => {
            if s.is_empty() {
                report.error_on(line, "Invalid Place '': should it be 'DQ'?");
            } else {
                report.error_on(line, format!("Invalid Place '{s}'"));
            }
            Place::default()
        }
    }
}

fn check_column_age(s: &str, exempt_age: bool, line: u64, report: &mut Report) -> Age {
    match s.parse::<Age>() {
        Ok(age) => {
            if !exempt_age {
                let num = match age {
                    Age::Exact(n) => n,
                    Age::Approximate(n) => n,
                    Age::None => 24,
                };

                if num < 5 {
                    report.error_on(line, format!("Age '{s}' unexpectedly low"));
                } else if num > 100 {
                    report.error_on(line, format!("Age '{s}' unexpectedly high"));
                }
            }
            age
        }
        Err(_) => {
            report.error_on(line, format!("Invalid Age '{s}'"));
            Age::default()
        }
    }
}

fn check_column_agerange(
    s: &str,
    inferred_agerange: AgeRange,
    line: u64,
    report: &mut Report,
) -> AgeRange {
    if s.is_empty() {
        return inferred_agerange;
    }

    // Ensure that there is a dash, or the split below can panic.
    if s.chars().filter(|c| *c == '-').count() != 1 {
        report.error_on(line, format!("AgeRange '{s}' must be a range of two Ages"));
        return inferred_agerange;
    }

    // Knowing that there is one dash, split into two parts.
    let (left, right_with_dash) = s.split_at(s.find('-').unwrap());
    let right = &right_with_dash[1..];

    // Parse the two Ages into an AgeRange.
    let lower: Age = left.parse::<Age>().unwrap_or(Age::None);
    let upper: Age = right.parse::<Age>().unwrap_or(Age::None);
    if lower.is_none() {
        report.error_on(line, format!("Lower bound of AgeRange '{s}' looks invalid"));
    }
    if upper.is_none() {
        report.error_on(line, format!("Upper bound of AgeRange '{s}' looks invalid"));
    }
    let explicit_agerange = AgeRange::from((lower, upper));

    // Return the most meaningful AgeRange possible.
    match (inferred_agerange.is_some(), explicit_agerange.is_some()) {
        (false, false) => AgeRange::default(),
        (true, false) => inferred_agerange,
        (false, true) => explicit_agerange,
        (true, true) => {
            // If both are defined, narrow to the intersection.
            let intersection = inferred_agerange.intersect(explicit_agerange);
            if intersection.is_none() {
                report.error_on(
                    line,
                    format!("AgeRange value '{explicit_agerange}' doesn't agree with AgeRange '{inferred_agerange}' inferred from other age data")
                );
            }
            intersection
        }
    }
}

fn check_column_event(s: &str, line: u64, headers: &HeaderIndexMap, report: &mut Report) -> Event {
    match s.parse::<Event>() {
        Ok(event) => {
            if event.has_squat()
                && !headers.has(Header::Best3SquatKg)
                && !headers.has(Header::Best3SquatLbs)
            {
                report.error_on(line, "Event has 'S', but no Best3SquatKg or Best3SquatLbs");
            }
            if event.has_bench()
                && !headers.has(Header::Best3BenchKg)
                && !headers.has(Header::Best3BenchLbs)
            {
                report.error_on(line, "Event has 'B', but no Best3BenchKg or Best3BenchLbs");
            }
            if event.has_deadlift()
                && !headers.has(Header::Best3DeadliftKg)
                && !headers.has(Header::Best3DeadliftLbs)
            {
                report.error_on(
                    line,
                    "Event has 'D', but no Best3DeadliftKg or Best3DeadliftLbs",
                );
            }
            event
        }
        Err(e) => {
            report.error_on(line, format!("Invalid Event '{s}': {e}"));
            Event::default()
        }
    }
}

/// Tests a column describing the amount of weight lifted.
fn check_weight_kg(s: &str, line: u64, header: Header, report: &mut Report) -> WeightKg {
    // Disallow zeros.
    if s == "0" {
        report.error_on(line, format!("{header} cannot be zero"));
    } else if s.starts_with('0') {
        report.error_on(line, format!("{header} cannot start with 0 in '{s}'"));
    }

    match s.parse::<WeightKg>() {
        Ok(kg) => {
            // Check for weights that are far beyond what's ever been lifted.
            const MAX_KG: i32 = 650;
            if header != Header::TotalKg
                && (kg > WeightKg::from_i32(MAX_KG) || kg < WeightKg::from_i32(-MAX_KG))
            {
                report.error_on(
                    line,
                    format!("{header} '{s}' exceeds maximum expected weight"),
                )
            }
            kg
        }
        Err(_) => {
            report.error_on(line, format!("Invalid {header} '{s}'"));
            WeightKg::default()
        }
    }
}

fn check_weight_lbs(s: &str, line: u64, header: Header, report: &mut Report) -> WeightKg {
    // Disallow zeros.
    if s == "0" {
        report.error_on(line, format!("{header} cannot be zero"));
    } else if s.starts_with('0') {
        report.error_on(line, format!("{header} cannot start with 0 in '{s}'"));
    }

    match s.parse::<WeightLbs>() {
        Ok(lbs) => {
            // Check for weights that are far beyond what's ever been lifted.
            const MAX_LBS: i32 = 1430;
            if header != Header::TotalLbs
                && (lbs > WeightLbs::from_i32(MAX_LBS) || lbs < WeightLbs::from_i32(-MAX_LBS))
            {
                report.error_on(
                    line,
                    format!("{header} '{s}' exceeds maximum expected weight"),
                )
            }
            WeightKg::from(lbs)
        }
        Err(_) => {
            report.error_on(line, format!("Invalid {header} '{s}'"));
            WeightKg::default()
        }
    }
}

fn check_nonnegative_weight_kg(
    s: &str,
    line: u64,
    header: Header,
    report: &mut Report,
) -> WeightKg {
    if s.starts_with('-') {
        report.error_on(line, format!("{header} '{s}' cannot be negative"))
    }
    check_weight_kg(s, line, header, report)
}

fn check_nonnegative_weight_lbs(
    s: &str,
    line: u64,
    header: Header,
    report: &mut Report,
) -> WeightKg {
    if s.starts_with('-') {
        report.error_on(line, format!("{header} '{s}' cannot be negative"))
    }
    check_weight_lbs(s, line, header, report)
}

fn check_column_bodyweightkg(s: &str, line: u64, report: &mut Report) -> WeightKg {
    let weight = check_nonnegative_weight_kg(s, line, Header::BodyweightKg, report);
    if weight != WeightKg::from_i32(0)
        && (weight < WeightKg::from_i32(15) || weight > WeightKg::from_i32(300))
    {
        report.error_on(line, format!("Implausible BodyweightKg '{s}'"));
    }
    weight
}
fn check_column_bodyweightlbs(s: &str, line: u64, report: &mut Report) -> WeightKg {
    let weight = check_nonnegative_weight_lbs(s, line, Header::BodyweightLbs, report);
    if weight != WeightKg::from_i32(0)
        && (weight < WeightKg::from_i32(15) || weight > WeightKg::from_i32(300))
    {
        report.error_on(line, format!("Implausible BodyweightLbs '{s}'"));
    }
    weight
}

fn check_column_weightclasskg(s: &str, line: u64, report: &mut Report) -> WeightClassKg {
    // Disallow zeros.
    if s == "0" {
        report.error_on(line, "WeightClassKg cannot be zero");
    } else if s.starts_with('0') && s != "0+" {
        report.error_on(line, format!("WeightClassKg cannot start with 0 in '{s}'"));
    }

    match s.parse::<WeightClassKg>() {
        Ok(w) => w,
        Err(e) => {
            report.error_on(line, format!("Invalid WeightClassKg '{s}': {e}"));
            WeightClassKg::default()
        }
    }
}
fn check_column_weightclasslbs(s: &str, line: u64, report: &mut Report) -> WeightClassKg {
    // Disallow zeros.
    if s == "0" {
        report.error_on(line, "WeightClassLbs cannot be zero");
    } else if s.starts_with('0') && s != "0+" {
        report.error_on(line, format!("WeightClassLbs cannot start with 0 in '{s}'"));
    }

    match s.parse::<WeightClassLbs>() {
        Ok(w) => w.into(),
        Err(e) => {
            report.error_on(line, format!("Invalid WeightClassLbs '{s}': {e}"));
            WeightClassLbs::default().into()
        }
    }
}

fn check_column_tested(s: &str, line: u64, report: &mut Report) -> Option<bool> {
    match s {
        "Yes" => Some(true),
        "No" => Some(false),
        "" => None,
        _ => {
            report.error_on(line, format!("Unknown Tested value '{s}'"));
            None
        }
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
        None => return,
    };

    // Configuration files covering directories with results from
    // several federations, such as meet-data/plusa, can omit
    // the list of divisions to effectively cause full exemption.
    if config.divisions.is_empty() {
        return;
    }

    // The division must appear in the configuration file.
    if !config.divisions.iter().any(|d| d.name == s) {
        report.error_on(line, format!("Unknown division '{s}'"));
    }
}

fn check_column_country(s: &str, line: u64, report: &mut Report) -> Option<Country> {
    if s.is_empty() {
        return None;
    }

    match s.parse::<Country>() {
        Ok(c) => Some(c),
        Err(_) => {
            report.error_on(line, format!("Unknown Country '{s}'"));
            None
        }
    }
}

fn check_column_entrydate(s: &str, line: u64, report: &mut Report) -> Option<Date> {
    if s.is_empty() {
        return None;
    }

    match s.parse::<Date>() {
        Ok(d) => Some(d),
        Err(_) => {
            report.error_on(line, format!("Invalid EntryDate '{s}'"));
            None
        }
    }
}

/// Checks the "State" column.
///
/// If the lifter's Country is explicitly specified, the State is checked
/// against that. Otherwise, the State is checked against the MeetCountry.
fn check_column_state(
    s: &str,
    lifter_country: Option<Country>,
    meet: &Meet,
    line: u64,
    report: &mut Report,
) -> Option<State> {
    if s.is_empty() {
        return None;
    }

    // Get the country either from the Country column or from the MeetCountry.
    let country = lifter_country.unwrap_or(meet.country);
    let state = State::from_str_and_country(s, country).ok();
    if state.is_none() {
        let c = country.to_string();
        let msg = format!("Unknown State '{s}' for Country '{c}'");
        report.error_on(line, msg);
    }
    state
}

fn check_event_and_total_consistency(entry: &Entry, line: u64, report: &mut Report) {
    let event = entry.event;
    let equipment = entry.equipment;
    let has_squat_data: bool = entry.has_squat_data();
    let has_bench_data: bool = entry.has_bench_data();
    let has_deadlift_data: bool = entry.has_deadlift_data();

    // Check that lift data isn't present outside of the specified Event.
    if has_squat_data && !event.has_squat() {
        report.error_on(line, format!("Event '{event}' cannot have squat data"));
    }
    if has_bench_data && !event.has_bench() {
        report.error_on(line, format!("Event '{event}' cannot have bench data"));
    }
    if has_deadlift_data && !event.has_deadlift() {
        report.error_on(line, format!("Event '{event}' cannot have deadlift data"));
    }

    // Check that the Equipment makes sense for the given Event.
    if equipment == Equipment::Wraps && !event.has_squat() {
        report.error_on(line, format!("Event '{event}' doesn't use Wraps"));
    }
    if equipment == Equipment::Straps && !event.has_deadlift() {
        report.error_on(line, format!("Event '{event}' doesn't use Straps"));
    }

    // Check that the SquatEquipment makes sense.
    if let Some(squat_eq) = entry.squat_equipment {
        if squat_eq > equipment {
            report.error_on(
                line,
                format!(
                    "SquatEquipment '{squat_eq}' can't be more supportive \
                     than the Equipment '{equipment}'"
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
                    "BenchEquipment '{bench_eq}' can't be more supportive \
                     than the Equipment '{equipment}'"
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
                    "DeadliftEquipment '{deadlift_eq}' can't be more supportive \
                     than the Equipment '{equipment}'"
                ),
            );
        }
    }

    // If the lifter wasn't DQ'd, they should have data from each lift.
    if !entry.place.is_dq() {
        // Allow entries that only have a Total but no lift data.
        if has_squat_data || has_bench_data || has_deadlift_data {
            if !has_squat_data && event.has_squat() {
                let s = format!("Non-DQ Event '{event}' requires squat data");
                report.error_on(line, s);
            }
            if !has_bench_data && event.has_bench() {
                let s = format!("Non-DQ Event '{event}' requires bench data");
                report.error_on(line, s);
            }
            if !has_deadlift_data && event.has_deadlift() {
                let s = format!("Non-DQ Event '{event}' requires deadlift data");
                report.error_on(line, s);
            }
        }
    }

    // Check that TotalKg matches the Place.
    let has_totalkg: bool = entry.totalkg != WeightKg::from_i32(0);
    if entry.place.is_dq() && has_totalkg {
        report.error_on(line, "DQ'd entries cannot have a TotalKg");
    } else if !entry.place.is_dq() && !has_totalkg {
        report.error_on(line, "Non-DQ entries must have a TotalKg");
    }

    // If any "Best" lift is failed, the lifter must be DQ'd.
    if !entry.place.is_dq() && entry.best3squatkg < WeightKg::from_i32(0) {
        report.error_on(line, "Non-DQ entries cannot have a negative Best3SquatKg");
    }
    if !entry.place.is_dq() && entry.best3benchkg < WeightKg::from_i32(0) {
        report.error_on(line, "Non-DQ entries cannot have a negative Best3BenchKg");
    }
    if !entry.place.is_dq() && entry.best3deadliftkg < WeightKg::from_i32(0) {
        report.error_on(
            line,
            "Non-DQ entries cannot have a negative Best3DeadliftKg",
        );
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
                "Calculated TotalKg '{calculated}', but meet recorded '{}'",
                entry.totalkg
            );
            report.error_on(line, s)
        }
    }

    // Check that the TotalKg isn't something completely nonsensical.
    // Usually this occurs when pounds were mislabeled as kilograms.
    // The current Multi-ply record is 1407.5.
    if entry.totalkg >= WeightKg::from_i32(1408) {
        report.error_on(
            line,
            format!(
                "TotalKg '{}' exceeds the world record. Are the weights actually in LBS?",
                entry.totalkg
            ),
        );
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
            format!("{lift}{attempt_num}Kg '{attempt}' lowered weight from '{maxweight}'"),
        );
    }

    // A successful attempt shouldn't have been repeated.
    // However, allow it if `exempt_lift_order` is set: this can happen due to misloads.
    if !maxweight.is_failed() && attempt.abs() == maxweight && !exempt_lift_order {
        report.error_on(
            line,
            format!("{lift}{attempt_num}Kg '{attempt}' repeated a successful attempt"),
        );
    }

    if attempt.abs() >= maxweight.abs() {
        attempt
    } else {
        maxweight
    }
}

#[allow(clippy::too_many_arguments)]
fn check_attempt_consistency_helper(
    lift: &str,
    attempt1: WeightKg,
    attempt2: WeightKg,
    attempt3: WeightKg,
    attempt4: WeightKg,
    best3lift: WeightKg,
    exempt_lift_order: bool,
    fourths_may_lower: bool,
    line: u64,
    report: &mut Report,
) {
    // Check that the bar weight is ascending over attempts.
    let mut maxweight =
        process_attempt_pair(lift, 2, attempt1, attempt2, exempt_lift_order, line, report);
    maxweight = process_attempt_pair(
        lift,
        3,
        maxweight,
        attempt3,
        exempt_lift_order,
        line,
        report,
    );
    if !fourths_may_lower {
        process_attempt_pair(
            lift,
            4,
            maxweight,
            attempt4,
            exempt_lift_order,
            line,
            report,
        );
    }

    // Check the Best3Lift validity.
    let best = attempt1.max(attempt2.max(attempt3));

    // If the best attempt was successful, it should be in the Best3Lift.
    if best > WeightKg::from_i32(0) && best != best3lift {
        report.error_on(
            line,
            format!("Best3{lift}Kg '{best3lift}' does not match best attempt '{best}'"),
        );
    }

    // If the best attempt was a failure, the least failure can be in the Best3Lift.
    if best < WeightKg::from_i32(0) && best3lift != WeightKg::from_i32(0) && best != best3lift {
        let s = format!("Best3{lift}Kg '{best3lift}' does not match least failed attempt '{best}'");
        report.error_on(line, s);
    }
}

fn check_attempt_consistency(
    entry: &Entry,
    exempt_lift_order: bool,
    fourths_may_lower: bool,
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
        fourths_may_lower,
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
        fourths_may_lower,
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
        fourths_may_lower,
        line,
        report,
    );
}

/// Checks that gear wasn't used prior to its date of invention.
fn check_equipment_year(entry: &Entry, meet: &Meet, line: u64, report: &mut Report) {
    // Helper function for checking equipped status.
    fn is_equipped(e: Option<Equipment>) -> bool {
        e.map_or(false, |eq| match eq {
            Equipment::Raw | Equipment::Wraps | Equipment::Straps => false,
            Equipment::Single | Equipment::Multi | Equipment::Unlimited => true,
        })
    }

    let event = entry.event;

    // Years of equipment invention.
    let squat_suit_invention_year = 1977;
    let bench_shirt_invention_year = 1985;

    // TODO: This is just a safe value.
    // Need to figure out when deadlift suits were invented.
    let deadlift_suit_invention_year = 1980;

    // Check that squat equipment isn't listed before its invention.
    if meet.date.year() < squat_suit_invention_year
        && (is_equipped(entry.squat_equipment)
            || (event.has_squat() && is_equipped(Some(entry.equipment))))
    {
        report.error_on(
            line,
            format!("Squat equipment wasn't invented until {squat_suit_invention_year}"),
        );
    }

    // Check that bench equipment isn't listed before its invention.
    // TODO: This avoids conflation with the squat equipment.
    if meet.date.year() < bench_shirt_invention_year
        && (is_equipped(entry.bench_equipment)
            || (event.has_bench() && !event.has_squat() && is_equipped(Some(entry.equipment))))
    {
        report.error_on(
            line,
            format!("Bench shirts weren't invented until {bench_shirt_invention_year}"),
        );
    }

    // Check that deadlift equipment isn't listed before its invention.
    // TODO: This avoids conflation with the squat equipment.
    if meet.date.year() < deadlift_suit_invention_year
        && (is_equipped(entry.deadlift_equipment)
            || (event.has_deadlift() && !event.has_squat() && is_equipped(Some(entry.equipment))))
    {
        report.error_on(
            line,
            format!("Deadlift suits weren't invented until {deadlift_suit_invention_year}"),
        );
    }
}

fn check_weightclass_consistency(
    entry: &Entry,
    meet: &Meet,
    config: Option<&Config>,
    exempt_weightclass_consistency: bool,
    line: u64,
    report: &mut Report,
) {
    // Any provided bodyweight should at least be plausible.
    if entry.bodyweightkg.is_non_zero()
        && entry.weightclasskg != WeightClassKg::None
        && !entry.weightclasskg.matches_bodyweight(entry.bodyweightkg)
    {
        report.error_on(
            line,
            format!(
                "BodyweightKg '{}' not in WeightClassKg '{}'",
                entry.bodyweightkg, entry.weightclasskg
            ),
        );
    }

    // If the configuration exempts consistency checking, stop here.
    if exempt_weightclass_consistency {
        return;
    }

    // Configuration files covering directories with results from
    // several federations, such as meet-data/plusa, can omit
    // the list of divisions to effectively cause full exemption.
    if config.map_or(false, |c| c.divisions.is_empty()) {
        return;
    }

    // If there's no weightclass data, there's nothing to check.
    if entry.weightclasskg == WeightClassKg::None {
        // Weightclass data may be omitted for lifters who never showed up.
        if entry.place == Place::NS {
            return;
        }

        // Configured federations should have weightclass data.
        if config.is_some() {
            report.error_on(line, "Configured federations cannot omit WeightClassKg");
        }
        return;
    }

    // If there's nothing configured, we can still do some basic checks.
    if config.is_none() {
        // Check that the weightclass appears in the list of known defaults.
        if !DEFAULT_WEIGHTCLASSES
            .iter()
            .any(|c| *c == entry.weightclasskg)
        {
            report.error_on(
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

    // Attempt to find out what weightclass group this row is a member of.
    //
    // Groups are specified in an arbitrary order with date, sex, and division data.
    // We want to find the one group that matches most closely.
    let mut matched_group: Option<&WeightClassConfig> = None;
    for group in &config.weightclasses {
        // Sex and date information are mandatory and must match.
        if meet.date < group.date_min || meet.date > group.date_max || entry.sex != group.sex {
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
            if best.divisions.is_some() && group.divisions.is_none() {
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
        // Federations don't define Mx weightclasses at this point.
        if entry.sex != Sex::Mx {
            report.error_on(
                line,
                "Could not match to any weightclass group in the CONFIG.toml",
            );
        }
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
        .map(|(i, _)| i);

    if index.is_none() {
        // Try to make a helpful suggestion about the most likely candidate.
        let suggestion: WeightClassKg = if entry.bodyweightkg.is_non_zero() {
            // Find a class that matches the bodyweight.
            matched_group
                .classes
                .iter()
                .find(|w| w.matches_bodyweight(entry.bodyweightkg))
                .map_or(WeightClassKg::None, |w| *w)
        } else if entry.weightclasskg.is_shw() {
            // Suggest any SHW weightclass.
            matched_group
                .classes
                .iter()
                .find(|w| w.is_shw())
                .map_or(WeightClassKg::None, |w| *w)
        } else {
            // No bodyweight information is provided: look for the first weightclass
            // that has a value above the invalid one.
            matched_group
                .classes
                .iter()
                .find(|w| *w > &entry.weightclasskg)
                .map_or(WeightClassKg::None, |w| *w)
        };

        report.error_on(
            line,
            format!(
                "WeightClassKg '{}' not found in [weightclasses.{}], suggest '{}'",
                entry.weightclasskg, matched_group.name, suggestion
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

            report.error_on(
                line,
                format!(
                    "BodyweightKg '{}' matches '{}', not '{}' in [weightclasses.{}]",
                    entry.bodyweightkg, first_match, entry.weightclasskg, matched_group.name
                ),
            );
        }
    }
}

/// Checks internal age consistency, and checks that the CONFIG-controlled Age
/// range is consistent with the known data.
///
/// Returns the (min_age, max_age) associated with the Division.
fn check_division_age_consistency(
    entry: &Entry,
    meet: &Meet,
    config: Option<&Config>,
    exempt_division: bool,
    line: u64,
    report: &mut Report,
) -> (Age, Age) {
    // Since it will be needed a bunch below, if there's a BirthDate,
    // figure out how old the lifter would be on the meet date.
    let age_from_birthdate: Option<Age> = entry.birthdate.map(|birthdate| {
        // Unwrapping is safe: the BirthDate column check already validated.
        birthdate.age_on(meet.date).unwrap()
    });

    let birthyear: Option<u32> = entry.birthyearrange.exact_birthyear();

    // Check that the Age, BirthYear, and BirthDate columns are internally
    // consistent.
    let age_from_birthyear: Option<Age> =
        birthyear.map(|birthyear| Age::from_birthyear_on_date(birthyear, meet.date));
    if let Some(birthyear) = birthyear {
        let approx_age = age_from_birthyear.unwrap();

        // Pairwise check BirthYear and Age.
        if approx_age.is_definitely_less_than(entry.age)
            || approx_age.is_definitely_greater_than(entry.age)
        {
            report.error_on(
                line,
                format!(
                    "Age '{}' doesn't match BirthYear '{}', expected '{}'",
                    entry.age, birthyear, approx_age
                ),
            );
        }

        // Pairwise check BirthYear and BirthDate.
        if let Some(birthdate) = entry.birthdate {
            if birthdate.year() != birthyear {
                report.error_on(
                    line,
                    format!("BirthDate '{birthdate}' doesn't match BirthYear '{birthyear}'"),
                );
            }
        }
    }

    // Pairwise check Age and BirthDate.
    match entry.age {
        Age::Exact(age) => {
            if let Some(Age::Exact(bd_age)) = age_from_birthdate {
                if age != bd_age {
                    let s = format!(
                        "Age '{}' doesn't match BirthDate '{}', expected '{}'",
                        entry.age,
                        entry.birthdate.unwrap(),
                        bd_age
                    );
                    report.error_on(line, s);
                }
            }
        }
        Age::Approximate(age) => {
            if let Some(Age::Exact(bd_age)) = age_from_birthdate {
                if age != bd_age && age != bd_age + 1 {
                    let s = format!(
                        "Age '{}' doesn't match BirthDate '{}, expected '{}'",
                        entry.age,
                        entry.birthdate.unwrap(),
                        bd_age
                    );
                    report.error_on(line, s);
                }
            }
        }
        Age::None => (),
    }

    // Allow exemptions from division-specific checks.
    if exempt_division || entry.division.is_empty() {
        return (Age::None, Age::None);
    }

    // If no divisions are configured, there's nothing left to do.
    let config = match config {
        Some(config) => config,
        None => return (Age::None, Age::None),
    };

    // Configuration files covering directories with results from
    // several federations, such as meet-data/plusa, can omit
    // the list of divisions to effectively cause full exemption.
    if config.divisions.is_empty() {
        return (Age::None, Age::None);
    }

    // Division string errors are already handled by check_column_division().
    let (min_age, max_age) = match config.divisions.iter().find(|d| d.name == entry.division) {
        Some(div) => (div.min, div.max),
        None => return (Age::None, Age::None),
    };

    // Use the various age-related columns to calculate a representative Age value.
    let age = entry.age_on(meet.date);

    if age.is_definitely_less_than(min_age) {
        report.error_on(
            line,
            format!(
                "Calculated Age {} too young for division '{}': min age {}",
                age, entry.division, min_age
            ),
        );
    }

    if age.is_definitely_greater_than(max_age) {
        report.error_on(
            line,
            format!(
                "Calculated Age {} too old for division '{}': max age {}",
                age, entry.division, max_age
            ),
        );
    }

    // Handle specially the case of BirthYear-based age divisions.
    // The issue is that if we define a division that is 19.5-22.5 (IPF Juniors),
    // the "is_definitely" checks above will permit ages between 19-23,
    // and therefore allow Age::Approximate(18), since that is not definitely
    // lower than 19.5. That allows lifters to be in *either* T3 or Juniors,
    // even though federation rules would only allow one.
    if let Age::Approximate(min) = min_age {
        if let Age::Approximate(max) = max_age {
            if let Some(Age::Approximate(age)) = age_from_birthyear {
                // Compare approximate age values exactly.
                if age < min {
                    report.error_on(
                        line,
                        format!(
                            "BirthYear Age {} too young for division '{}': min age {}",
                            age_from_birthyear.unwrap(),
                            entry.division,
                            min_age
                        ),
                    );
                }

                if age > max {
                    report.error_on(
                        line,
                        format!(
                            "BirthYear Age {} too old for division '{}': max age {}",
                            age_from_birthyear.unwrap(),
                            entry.division,
                            max_age
                        ),
                    );
                }
            }
        }
    }

    (min_age, max_age)
}

/// Checks that a configured division is consistent with any sex restrictions.
fn check_division_sex_consistency(
    entry: &Entry,
    config: Option<&Config>,
    line: u64,
    report: &mut Report,
) {
    if entry.division.is_empty() {
        return;
    }

    let config = match config {
        Some(c) => c,
        None => return,
    };

    // Get the configured sex for the division, or return if not specified.
    let sex = match config.divisions.iter().find(|d| d.name == entry.division) {
        Some(div) => match div.sex {
            Some(sex) => sex,
            None => return,
        },
        None => return,
    };

    if sex != entry.sex {
        report.error_on(
            line,
            format!(
                "Division '{}' requires Sex '{:?}', found '{:?}'",
                entry.division, sex, entry.sex
            ),
        );
    }
}

/// Checks that a configured division is consistent with any Place restrictions.
fn check_division_place_consistency(
    entry: &Entry,
    config: Option<&Config>,
    line: u64,
    report: &mut Report,
) {
    if entry.division.is_empty() {
        return;
    }

    let config = match config {
        Some(c) => c,
        None => return,
    };

    // Get the configured place for the division, or return if not specified.
    let place = match config.divisions.iter().find(|d| d.name == entry.division) {
        Some(div) => match div.place {
            Some(place) => place,
            None => return,
        },
        None => return,
    };

    // Only perform checks if the entry was non-DQ'd.
    if !entry.place.is_dq() && place != entry.place {
        report.error_on(
            line,
            format!(
                "Division '{}' requires Place '{:?}', found '{:?}'",
                entry.division, place, entry.place
            ),
        );
    }
}

/// Checks that a configured division is consistent with any equipment
/// restrictions.
fn check_division_equipment_consistency(
    entry: &Entry,
    config: Option<&Config>,
    line: u64,
    report: &mut Report,
) {
    // Allow exemptions from division-specific checks.
    if entry.division.is_empty() {
        return;
    }

    let equipment = entry.equipment;
    let config = match config {
        Some(c) => c,
        None => return,
    };

    // Get the configured sex for the division, or return if not specified.
    let eqlist = match config.divisions.iter().find(|d| d.name == entry.division) {
        Some(div) => match &div.equipment {
            Some(vec) => vec,
            None => return,
        },
        None => return,
    };

    if !eqlist.contains(&equipment) {
        report.error_on(
            line,
            format!(
                "Division '{}' does not allow equipment '{}'",
                entry.division, equipment
            ),
        );
    }
}

/// Returns Testedness based on division configuration.
fn tested_from_division_config(entry: &Entry, config: Option<&Config>) -> bool {
    let config = match config {
        Some(c) => c,
        None => {
            return entry.tested;
        }
    };

    match config.divisions.iter().find(|d| d.name == entry.division) {
        Some(div) => match div.tested {
            Some(value) => value,
            None => entry.tested,
        },
        None => entry.tested,
    }
}

/// Determines whether this meet falls in the valid range for a
/// partially-configured federation.
fn should_ignore_config(meet: &Meet, config: Option<&Config>) -> bool {
    if let Some(config) = config {
        if let Some(valid_since) = config.valid_since() {
            return meet.date < valid_since;
        }
    }
    false
}

/// Checks a single entries.csv file from an open `csv::Reader`.
///
/// Extracting this out into a `Reader`-specific function is useful
/// for creating tests that do not have a backing CSV file.
pub fn do_check<R: io::Read>(
    rdr: &mut csv::Reader<R>,
    meet: &Meet,
    config: Option<&Config>,
    lifterdata: Option<&LifterDataMap>,
    mut report: Report,
) -> Result<EntriesCheckResult, Box<dyn Error>> {
    // If the federation is only partially configured and this meet doesn't fall in
    // the valid range, ignore the config by reassigning it.
    let config = if should_ignore_config(meet, config) {
        None
    } else {
        config
    };

    // Should pending disambiguations be errors?
    let report_disambiguations = config.map_or(false, |c| c.does_require_manual_disambiguation());

    let fourths_may_lower: bool = meet.ruleset.contains(Rule::FourthAttemptsMayLower);

    // Scan for check exemptions.
    let exemptions = {
        let parent_folder = &report.parent_folder()?;
        config.and_then(|c| c.exemptions_for(parent_folder))
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
    let exempt_age: bool =
        exemptions.map_or(false, |el| el.iter().any(|&e| e == Exemption::ExemptAge));

    let headers: HeaderIndexMap = check_headers(rdr.headers()?, meet, config, &mut report);
    if !report.messages.is_empty() {
        return Ok(EntriesCheckResult {
            report,
            entries: None,
        });
    }

    let mut entries: Vec<Entry> = Vec::new();

    // This allocation can be re-used for each row.
    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        let line = record.position().map_or(0, csv::Position::line);

        // Check each field for whitespace errors.
        for field in &record {
            if field.contains("  ") || field.starts_with(' ') || field.ends_with(' ') {
                let msg = format!("Field '{field}' contains extraneous spacing");
                report.error_on(line, msg);
            }
        }

        let mut entry = Entry {
            entrydate: meet.date, // The EntryDate column can overwrite this later.
            ..Default::default()
        };

        // TODO: This code currently lets you mix Kg and Lbs, so you can define both TotalKg
        // and TotalLbs. That's incorrect -- we should force the units in a single direction.
        // Either the meet is in pounds, or in kilos. However, note that international meets
        // often do weigh-in in pounds, but lifting in kilos, so keep those separate.
        //
        // TODO: Rather than checking the existence of every column manually, walk over the columns.

        // Check mandatory fields.
        if let Some(idx) = headers.get(Header::Name) {
            entry.name = check_column_name(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Sex) {
            entry.sex = check_column_sex(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Equipment) {
            entry.equipment = check_column_equipment(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::SquatEquipment) {
            entry.squat_equipment = check_column_squatequipment(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::BenchEquipment) {
            entry.bench_equipment = check_column_benchequipment(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::DeadliftEquipment) {
            entry.deadlift_equipment =
                check_column_deadliftequipment(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Place) {
            entry.place = check_column_place(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Age) {
            entry.age = check_column_age(&record[idx], exempt_age, line, &mut report);
        }
        if let Some(idx) = headers.get(Header::Event) {
            entry.event = check_column_event(&record[idx], line, &headers, &mut report);
        }

        // Check all the weight fields: they must contain non-zero values.
        // Squat Kg.
        if let Some(idx) = headers.get(Header::Squat1Kg) {
            entry.squat1kg = check_weight_kg(&record[idx], line, Header::Squat1Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat2Kg) {
            entry.squat2kg = check_weight_kg(&record[idx], line, Header::Squat2Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat3Kg) {
            entry.squat3kg = check_weight_kg(&record[idx], line, Header::Squat3Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat4Kg) {
            entry.squat4kg = check_weight_kg(&record[idx], line, Header::Squat4Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3SquatKg) {
            entry.best3squatkg =
                check_weight_kg(&record[idx], line, Header::Best3SquatKg, &mut report);
        }
        // Squat Lbs.
        if let Some(idx) = headers.get(Header::Squat1Lbs) {
            entry.squat1kg = check_weight_lbs(&record[idx], line, Header::Squat1Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat2Lbs) {
            entry.squat2kg = check_weight_lbs(&record[idx], line, Header::Squat2Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat3Lbs) {
            entry.squat3kg = check_weight_lbs(&record[idx], line, Header::Squat3Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Squat4Lbs) {
            entry.squat4kg = check_weight_lbs(&record[idx], line, Header::Squat4Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3SquatLbs) {
            entry.best3squatkg =
                check_weight_lbs(&record[idx], line, Header::Best3SquatLbs, &mut report);
        }

        // Bench Kg.
        if let Some(idx) = headers.get(Header::Bench1Kg) {
            entry.bench1kg = check_weight_kg(&record[idx], line, Header::Bench1Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench2Kg) {
            entry.bench2kg = check_weight_kg(&record[idx], line, Header::Bench2Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench3Kg) {
            entry.bench3kg = check_weight_kg(&record[idx], line, Header::Bench3Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench4Kg) {
            entry.bench4kg = check_weight_kg(&record[idx], line, Header::Bench4Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3BenchKg) {
            entry.best3benchkg =
                check_weight_kg(&record[idx], line, Header::Best3BenchKg, &mut report);
        }
        // Bench Lbs.
        if let Some(idx) = headers.get(Header::Bench1Lbs) {
            entry.bench1kg = check_weight_lbs(&record[idx], line, Header::Bench1Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench2Lbs) {
            entry.bench2kg = check_weight_lbs(&record[idx], line, Header::Bench2Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench3Lbs) {
            entry.bench3kg = check_weight_lbs(&record[idx], line, Header::Bench3Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Bench4Lbs) {
            entry.bench4kg = check_weight_lbs(&record[idx], line, Header::Bench4Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3BenchLbs) {
            entry.best3benchkg =
                check_weight_lbs(&record[idx], line, Header::Best3BenchLbs, &mut report);
        }

        // Deadlift Kg.
        if let Some(idx) = headers.get(Header::Deadlift1Kg) {
            entry.deadlift1kg =
                check_weight_kg(&record[idx], line, Header::Deadlift1Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift2Kg) {
            entry.deadlift2kg =
                check_weight_kg(&record[idx], line, Header::Deadlift2Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift3Kg) {
            entry.deadlift3kg =
                check_weight_kg(&record[idx], line, Header::Deadlift3Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift4Kg) {
            entry.deadlift4kg =
                check_weight_kg(&record[idx], line, Header::Deadlift4Kg, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3DeadliftKg) {
            entry.best3deadliftkg =
                check_weight_kg(&record[idx], line, Header::Best3DeadliftKg, &mut report);
        }
        // Deadlift Lbs.
        if let Some(idx) = headers.get(Header::Deadlift1Lbs) {
            entry.deadlift1kg =
                check_weight_lbs(&record[idx], line, Header::Deadlift1Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift2Lbs) {
            entry.deadlift2kg =
                check_weight_lbs(&record[idx], line, Header::Deadlift2Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift3Lbs) {
            entry.deadlift3kg =
                check_weight_lbs(&record[idx], line, Header::Deadlift3Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Deadlift4Lbs) {
            entry.deadlift4kg =
                check_weight_lbs(&record[idx], line, Header::Deadlift4Lbs, &mut report);
        }
        if let Some(idx) = headers.get(Header::Best3DeadliftLbs) {
            entry.best3deadliftkg =
                check_weight_lbs(&record[idx], line, Header::Best3DeadliftLbs, &mut report);
        }

        // TotalKg is a positive weight if present or 0 if missing.
        if let Some(idx) = headers.get(Header::TotalKg) {
            entry.totalkg =
                check_nonnegative_weight_kg(&record[idx], line, Header::TotalKg, &mut report);
        }
        if let Some(idx) = headers.get(Header::TotalLbs) {
            entry.totalkg =
                check_nonnegative_weight_lbs(&record[idx], line, Header::TotalLbs, &mut report);
        }

        if let Some(idx) = headers.get(Header::BodyweightKg) {
            entry.bodyweightkg = check_column_bodyweightkg(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::BodyweightLbs) {
            entry.bodyweightkg = check_column_bodyweightlbs(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::WeightClassKg) {
            entry.weightclasskg = check_column_weightclasskg(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::WeightClassLbs) {
            entry.weightclasskg = check_column_weightclasslbs(&record[idx], line, &mut report);
        }

        // If no bodyweight is given but there is a bounded weightclass,
        // assume the pessimal case of the lifter at the top of the class.
        if entry.bodyweightkg.is_zero() {
            if let WeightClassKg::UnderOrEqual(w) = entry.weightclasskg {
                entry.bodyweightkg = w;
            }
        }

        // Set the Tested column early for federations that are fully-Tested.
        // This allows check_column_tested() to override it later if needed.
        entry.tested = meet.federation.is_fully_tested(meet.date);

        // Check optional fields.
        if let Some(idx) = headers.get(Header::Division) {
            check_column_division(&record[idx], config, exempt_division, line, &mut report);
            entry.division = CompactString::from(&record[idx]);
        }

        // Assign the Tested column if it's configured for the Division.
        // Note that this check has to occur after setting the division above.
        entry.tested = tested_from_division_config(&entry, config);

        // Check the Country and State information.
        if let Some(idx) = headers.get(Header::Country) {
            entry.country = check_column_country(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::EntryDate) {
            if let Some(date) = check_column_entrydate(&record[idx], line, &mut report) {
                entry.entrydate = date;
            }
        }
        if let Some(idx) = headers.get(Header::State) {
            let c = entry.country;
            entry.state = check_column_state(&record[idx], c, meet, line, &mut report);

            // If the Country was not explicitly specified, but the State was,
            // the lifter's Country is inferrable from the MeetCountry.
            if entry.country.is_none() {
                entry.country = entry.state.map(|s| s.to_country());
            }
        }

        if let Some(idx) = headers.get(Header::Tested) {
            // Blank "Tested" columns default to the federation configuration.
            if let Some(tested) = check_column_tested(&record[idx], line, &mut report) {
                entry.tested = tested;
            }
        }
        if let Some(idx) = headers.get(Header::ChineseName) {
            entry.chinesename = check_column_chinesename(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::CyrillicName) {
            entry.cyrillicname = check_column_cyrillicname(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::JapaneseName) {
            entry.japanesename = check_column_japanesename(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::GreekName) {
            entry.greekname = check_column_greekname(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::KoreanName) {
            entry.koreanname = check_column_koreanname(&record[idx], line, &mut report);
        }
        if let Some(idx) = headers.get(Header::BirthYear) {
            if let Some(y) = check_column_birthyear(&record[idx], meet, line, &mut report) {
                entry.birthyearrange = BirthYearRange::from_birthyear(y);
            }
        }
        if let Some(idx) = headers.get(Header::BirthDate) {
            entry.birthdate = check_column_birthdate(&record[idx], meet, line, &mut report);
        }

        // Check consistency across fields.
        check_event_and_total_consistency(&entry, line, &mut report);
        check_attempt_consistency(
            &entry,
            exempt_lift_order,
            fourths_may_lower,
            line,
            &mut report,
        );
        check_equipment_year(&entry, meet, line, &mut report);
        check_weightclass_consistency(
            &entry,
            meet,
            config,
            exempt_weightclass_consistency,
            line,
            &mut report,
        );

        let (division_age_min, division_age_max) = check_division_age_consistency(
            &entry,
            meet,
            config,
            exempt_division,
            line,
            &mut report,
        );

        if !exempt_division {
            check_division_sex_consistency(&entry, config, line, &mut report);
            check_division_place_consistency(&entry, config, line, &mut report);
            check_division_equipment_consistency(&entry, config, line, &mut report);
        }

        // If the Age wasn't assigned yet, infer it from any surrounding information.
        if entry.age == Age::None {
            if let Some(birthdate) = entry.birthdate {
                entry.age = birthdate.age_on(meet.date).unwrap_or(Age::None);
            }
        }
        if entry.age == Age::None {
            if let Some(birthyear) = entry.birthyearrange.exact_birthyear() {
                entry.age = Age::from_birthyear_on_date(birthyear, meet.date);
            }
        }

        // Infer the AgeRange based on Age or Division.
        let range_from_age = AgeRange::from(entry.age);
        let inferred_agerange = if range_from_age.is_some() {
            range_from_age
        } else {
            // Fall back to Division-based ranges if the exact Age isn't specified.
            AgeRange::from((division_age_min, division_age_max))
        };

        // The AgeRange can also be specified explicitly in an optional column.
        if let Some(idx) = headers.get(Header::AgeRange) {
            entry.agerange =
                check_column_agerange(&record[idx], inferred_agerange, line, &mut report);
        } else {
            entry.agerange = inferred_agerange;
        }

        // Try narrowing the BirthYearRange based on surrounding information.
        match entry.birthdate {
            Some(birthdate) => {
                entry.birthyearrange = BirthYearRange::from_birthyear(birthdate.year());
            }
            _ => {
                // Try using the AgeRange.
                entry.birthyearrange = entry.birthyearrange.intersect(BirthYearRange::from_range(
                    entry.agerange.min,
                    entry.agerange.max,
                    meet.date,
                ));

                // Try using division information.
                entry.birthyearrange = entry.birthyearrange.intersect(BirthYearRange::from_range(
                    division_age_min,
                    division_age_max,
                    meet.date,
                ));
            }
        }

        // Infer the BirthYearClass.
        entry.birthyearclass = BirthYearClass::from_range(entry.birthyearrange, meet.date.year());

        // If the Name isn't provided, but there is an international name,
        // just use the international name.
        if entry.name.is_empty() {
            if let Some(idx) = headers.get(Header::JapaneseName) {
                entry.name = record[idx].into();
            }
        }
        if entry.name.is_empty() {
            if let Some(idx) = headers.get(Header::ChineseName) {
                entry.name = record[idx].into();
            }
        }
        if entry.name.is_empty() {
            if let Some(idx) = headers.get(Header::KoreanName) {
                entry.name = record[idx].into();
            }
        }

        // Create the username if applicable.
        if !entry.name.is_empty() {
            match Username::from_name(&entry.name) {
                Ok(username) => entry.username = username,
                Err(msg) => report.error_on(line, format!("Username error: {msg}")),
            }
        }

        // If requested, report if the username requires disambiguation.
        if report_disambiguations && !entry.username.as_str().is_empty() {
            if let Some(data) = lifterdata.and_then(|map| map.get(&entry.username)) {
                if data.disambiguation_count > 0 {
                    let url = format!("https://www.openpowerlifting.org/u/{}", entry.username);
                    report.error_on(line, format!("Disambiguate {} ({})", entry.name, url));
                }
            }
        }

        if entry.name.is_empty() {
            report.error_on(line, "No Name was given or could be inferred");
        }

        entries.push(entry);
    }

    Ok(EntriesCheckResult {
        report,
        entries: Some(entries.into_boxed_slice()),
    })
}

/// Checks a single entries.csv string, used by the server.
pub fn check_entries_from_string(
    reader: &csv::ReaderBuilder,
    entries_csv: &str,
    meet: &Meet,
) -> Result<EntriesCheckResult, Box<dyn Error>> {
    let report = Report::new(PathBuf::from("uploaded/content"));
    let mut rdr = reader.from_reader(entries_csv.as_bytes());
    do_check(&mut rdr, meet, None, None, report)
}

/// Checks a single entries.csv file by path.
pub fn check_entries(
    reader: &csv::ReaderBuilder,
    entries_csv: PathBuf,
    meet: &Meet,
    config: Option<&Config>,
    lifterdata: Option<&LifterDataMap>,
) -> Result<EntriesCheckResult, Box<dyn Error>> {
    // Allow the pending Report to own the PathBuf.
    let mut report = Report::new(entries_csv);

    // The entries.csv file must exist.
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(EntriesCheckResult {
            report,
            entries: None,
        });
    }

    let mut rdr = reader.from_path(&report.path)?;
    do_check(&mut rdr, meet, config, lifterdata, report)
}
