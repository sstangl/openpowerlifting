//! Logic for each meet's individual results page.

use langpack::{get_localized_name, Language, Locale, LocalizeNumber};
use opldb::{self, algorithms, Entry};
use opltypes::*;

use itertools::Itertools;

use std::cmp;
use std::str::FromStr;

/// The context object passed to `templates/meet.html.tera`
#[derive(Serialize)]
pub struct Context<'db> {
    pub urlprefix: &'static str,
    pub page_title: String,
    pub page_description: &'db str,
    pub meet: MeetInfo<'db>,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: WeightUnits,
    pub points_column_title: &'db str,

    /// Whether to use Rank instead of Place.
    pub use_rank_column: bool,

    // Instead of having the JS try to figure out how to access
    // other sorts, just tell it what the paths are.
    pub path_if_by_ah: String,
    pub path_if_by_division: String,
    pub path_if_by_dots: String,
    pub path_if_by_glossbrenner: String,
    pub path_if_by_goodlift: String,
    pub path_if_by_ipfpoints: String,
    pub path_if_by_mcculloch: String,
    pub path_if_by_nasa: String,
    pub path_if_by_reshel: String,
    pub path_if_by_schwartzmalone: String,
    pub path_if_by_total: String,
    pub path_if_by_wilks: String,
    pub path_if_by_wilks2020: String,

    /// True iff the meet reported any age data.
    pub has_age_data: bool,
    pub sortselection: MeetSortSelection,

    /// Year of competition, used in the heading.
    pub year: u32,

    /// List of tables, to be printed one after the other.
    pub tables: Vec<Table<'db>>,
}

/// A grouping of rows under a single category.
#[derive(Serialize)]
pub struct Table<'db> {
    pub title: Option<String>,
    pub rows: Vec<ResultsRow<'db>>,
}

/// A sort selection widget just for the meet page.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub enum MeetSortSelection {
    ByAH,
    ByDivision,
    ByDots,
    ByGlossbrenner,
    ByGoodlift,
    ByIPFPoints,
    ByMcCulloch,
    ByNASA,
    ByReshel,
    BySchwartzMalone,
    ByTotal,
    ByWilks,
    ByWilks2020,

    /// Special value that resolves to one of the others after lookup.
    ByFederationDefault,
}

impl FromStr for MeetSortSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "by-ah" => Ok(MeetSortSelection::ByAH),
            "by-division" => Ok(MeetSortSelection::ByDivision),
            "by-dots" => Ok(MeetSortSelection::ByDots),
            "by-glossbrenner" => Ok(MeetSortSelection::ByGlossbrenner),
            "by-goodlift" => Ok(MeetSortSelection::ByGoodlift),
            "by-ipf-points" => Ok(MeetSortSelection::ByIPFPoints),
            "by-mcculloch" => Ok(MeetSortSelection::ByMcCulloch),
            "by-nasa" => Ok(MeetSortSelection::ByNASA),
            "by-reshel" => Ok(MeetSortSelection::ByReshel),
            "by-schwartz-malone" => Ok(MeetSortSelection::BySchwartzMalone),
            "by-total" => Ok(MeetSortSelection::ByTotal),
            "by-wilks" => Ok(MeetSortSelection::ByWilks),
            "by-wilks2020" => Ok(MeetSortSelection::ByWilks2020),
            _ => Err(()),
        }
    }
}

impl From<PointsSystem> for MeetSortSelection {
    fn from(system: PointsSystem) -> Self {
        match system {
            PointsSystem::AH => MeetSortSelection::ByAH,
            PointsSystem::Dots => MeetSortSelection::ByDots,
            PointsSystem::Glossbrenner => MeetSortSelection::ByGlossbrenner,
            PointsSystem::Goodlift => MeetSortSelection::ByGoodlift,
            PointsSystem::IPFPoints => MeetSortSelection::ByIPFPoints,
            PointsSystem::McCulloch => MeetSortSelection::ByMcCulloch,
            PointsSystem::Reshel => MeetSortSelection::ByReshel,
            PointsSystem::SchwartzMalone => MeetSortSelection::BySchwartzMalone,
            PointsSystem::NASA => MeetSortSelection::ByNASA,
            PointsSystem::Total => MeetSortSelection::ByTotal,
            PointsSystem::Wilks => MeetSortSelection::ByWilks,
            PointsSystem::Wilks2020 => MeetSortSelection::ByWilks2020,
        }
    }
}

/// Gets the title of a column displaying a certain points system.
pub fn points_column_title<'db>(
    system: PointsSystem,
    locale: &'db Locale,
    default_points: PointsSystem,
) -> &'db str {
    match system {
        PointsSystem::AH => "AH",
        PointsSystem::Dots => &locale.strings.columns.dots,
        PointsSystem::Glossbrenner => &locale.strings.columns.glossbrenner,
        PointsSystem::Goodlift => &locale.strings.columns.goodlift,
        PointsSystem::IPFPoints => &locale.strings.columns.ipfpoints,
        PointsSystem::McCulloch => &locale.strings.columns.mcculloch,
        PointsSystem::NASA => "NASA",
        PointsSystem::Reshel => "Reshel",
        PointsSystem::SchwartzMalone => "S/Malone",
        PointsSystem::Wilks => &locale.strings.columns.wilks,
        PointsSystem::Wilks2020 => &locale.strings.columns.wilks2020,

        // This occurs if the federation default is ByTotal.
        PointsSystem::Total => {
            if default_points != PointsSystem::Total {
                points_column_title(default_points, locale, default_points)
            } else {
                &locale.strings.columns.total
            }
        }
    }
}

impl MeetSortSelection {
    /// Converts to a PointsSystem.
    pub fn as_points_system(self, default_points: PointsSystem) -> PointsSystem {
        match self {
            // Default sort.
            MeetSortSelection::ByFederationDefault => default_points,

            // Specifically-requested point sorts.
            MeetSortSelection::ByAH => PointsSystem::AH,
            MeetSortSelection::ByDots => PointsSystem::Dots,
            MeetSortSelection::ByGlossbrenner => PointsSystem::Glossbrenner,
            MeetSortSelection::ByGoodlift => PointsSystem::Goodlift,
            MeetSortSelection::ByIPFPoints => PointsSystem::IPFPoints,
            MeetSortSelection::ByMcCulloch => PointsSystem::McCulloch,
            MeetSortSelection::ByNASA => PointsSystem::NASA,
            MeetSortSelection::ByReshel => PointsSystem::Reshel,
            MeetSortSelection::BySchwartzMalone => PointsSystem::SchwartzMalone,
            MeetSortSelection::ByWilks => PointsSystem::Wilks,
            MeetSortSelection::ByWilks2020 => PointsSystem::Wilks2020,

            // Specifically-requested weight sorts.
            MeetSortSelection::ByDivision => default_points,
            MeetSortSelection::ByTotal => PointsSystem::Total,
        }
    }

    /// Gets the title of the column to show for the given selection.
    pub fn column_title<'db>(self, locale: &'db Locale, default_points: PointsSystem) -> &'db str {
        let system = self.as_points_system(default_points);
        points_column_title(system, locale, default_points)
    }

    /// Resolves ByFederationDefault to the actual default.
    ///
    /// This is used to get the "real" MeetSortSelection, to be passed to the
    /// templates.
    pub fn resolve_fed_default(self, default_points: PointsSystem) -> Self {
        if self == MeetSortSelection::ByFederationDefault {
            MeetSortSelection::from(default_points)
        } else {
            self
        }
    }
}

#[derive(Serialize)]
pub struct MeetInfo<'a> {
    pub path: &'a str,
    pub federation: Federation,
    pub date: String,
    pub country: &'a str,
    pub state: Option<&'a str>,
    pub town: Option<&'a str>,
    pub name: &'a str,
}

impl<'a> MeetInfo<'a> {
    pub fn from(meet: &'a opldb::Meet, strings: &'a langpack::Translations) -> MeetInfo<'a> {
        MeetInfo {
            path: &meet.path,
            federation: meet.federation,
            date: format!("{}", &meet.date),
            country: strings.translate_country(meet.country),
            state: meet.state.as_ref().map(|s| s as _),
            town: meet.town.as_ref().map(|t| t as _),
            name: &meet.name,
        }
    }
}

/// A row in the results table.
#[derive(Serialize)]
pub struct ResultsRow<'a> {
    /// The Place given by the federation.
    pub place: langpack::LocalizedPlace,
    /// The rank in the ranking-by-points view (by Wilks).
    pub rank: langpack::LocalizedOrdinal,
    pub localized_name: &'a str,
    pub lifter: &'a opldb::Lifter,
    pub sex: &'a str,
    pub age: PrettyAge,
    pub equipment: &'a str,
    pub weightclass: langpack::LocalizedWeightClassAny,
    pub bodyweight: langpack::LocalizedWeightAny,

    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,
    pub total: langpack::LocalizedWeightAny,
    pub points: langpack::LocalizedPoints,
}

impl<'a> ResultsRow<'a> {
    fn from(
        opldb: &'a opldb::OplDb,
        locale: &'a Locale,
        points_system: PointsSystem,
        entry: &'a opldb::Entry,
        rank: u32,
    ) -> ResultsRow<'a> {
        let lifter: &'a opldb::Lifter = opldb.get_lifter(entry.lifter_id);

        let strings = locale.strings;
        let number_format = locale.number_format;
        let units = locale.units;

        ResultsRow {
            place: locale.place(entry.place, entry.sex),
            rank: locale.ordinal(rank, entry.sex),
            localized_name: get_localized_name(lifter, locale.language),
            lifter,
            sex: strings.translate_sex(entry.sex),
            age: PrettyAge::from(entry.age),
            equipment: strings.translate_equipment(entry.equipment),
            weightclass: entry.weightclasskg.as_type(units).in_format(number_format),
            bodyweight: entry.bodyweightkg.as_type(units).in_format(number_format),

            squat: entry
                .highest_squatkg()
                .as_type(units)
                .in_format(number_format),
            bench: entry
                .highest_benchkg()
                .as_type(units)
                .in_format(number_format),
            deadlift: entry
                .highest_deadliftkg()
                .as_type(units)
                .in_format(number_format),
            total: entry.totalkg.as_type(units).in_format(number_format),
            points: entry.points(points_system, units).in_format(number_format),
        }
    }
}

/// Defines the order of events for the ByDivision display.
const EVENT_SORT_ORDER: [Event; 7] = [
    Event::sbd(),
    Event::bd(),
    Event::sb(),
    Event::sd(),
    Event::s(),
    Event::b(),
    Event::d(),
];

/// Defines the order of equipment for the ByDivision display.
#[inline]
fn order_by_equipment(a: Equipment) -> u32 {
    match a {
        Equipment::Raw => 0,
        Equipment::Wraps => 1,
        Equipment::Single => 2,
        Equipment::Multi => 3,
        Equipment::Unlimited => 4,
        Equipment::Straps => 5,
    }
}

/// Defines the order of sex for the ByDivision display.
#[inline]
fn order_by_sex(a: Sex) -> u32 {
    match a {
        Sex::F => 0,
        Sex::M => 1,
        Sex::Mx => 2,
    }
}

/// Compares two entries' Division columns.
fn cmp_by_division(a: Option<&str>, b: Option<&str>) -> cmp::Ordering {
    // Handle the case of blank divisions.
    if a.is_none() {
        return a.cmp(&b);
    }
    if b.is_none() {
        return cmp::Ordering::Less;
    }

    let a: &str = a.unwrap();
    let b: &str = b.unwrap();

    // "Professional" divisions precede "Amateur" divisions.
    let a_pro = a.contains("Pro");
    let b_pro = b.contains("Pro");
    if a_pro && !b_pro {
        return cmp::Ordering::Less;
    }
    if !a_pro && b_pro {
        return cmp::Ordering::Greater;
    }

    // Finally, just compare alphabetically.
    a.cmp(b)
}

/// Helper function for use in `cmp_by_group`.
#[inline]
fn cmp_by_equipment(ruleset: RuleSet, a: &Entry, b: &Entry) -> cmp::Ordering {
    // A rule may combine all equipment into one category.
    if ruleset.contains(Rule::CombineAllEquipment) {
        return cmp::Ordering::Equal;
    }

    let a_equipment = order_by_equipment(a.equipment);
    let b_equipment = order_by_equipment(b.equipment);

    if a_equipment != b_equipment {
        // A rule may combine Raw and Wraps into one equipment category.
        if ruleset.contains(Rule::CombineRawAndWraps) {
            let x = a.equipment == Equipment::Raw || a.equipment == Equipment::Wraps;
            let y = b.equipment == Equipment::Raw || b.equipment == Equipment::Wraps;
            if x && y {
                return cmp::Ordering::Equal;
            }
        }

        // A rule may combine Single-ply and Multi-ply into one equipment category.
        if ruleset.contains(Rule::CombineSingleAndMulti) {
            let x = a.equipment == Equipment::Single || a.equipment == Equipment::Multi;
            let y = b.equipment == Equipment::Single || b.equipment == Equipment::Multi;
            if x && y {
                return cmp::Ordering::Equal;
            }
        }

        return a_equipment.cmp(&b_equipment);
    }

    cmp::Ordering::Equal
}

/// Compares two entries for grouping into per-division tables.
fn cmp_by_group(ruleset: RuleSet, a: &Entry, b: &Entry) -> cmp::Ordering {
    // First, sort by Event.
    let a_event = EVENT_SORT_ORDER.iter().position(|&x| x == a.event).unwrap();
    let b_event = EVENT_SORT_ORDER.iter().position(|&x| x == b.event).unwrap();
    if a_event != b_event {
        return a_event.cmp(&b_event);
    }

    // Next, sort by Equipment.
    let by_equipment = cmp_by_equipment(ruleset, a, b);
    if by_equipment != cmp::Ordering::Equal {
        return by_equipment;
    }

    // Next, sort by Sex.
    let a_sex = order_by_sex(a.sex);
    let b_sex = order_by_sex(b.sex);
    if a_sex != b_sex {
        return a_sex.cmp(&b_sex);
    }

    // Next, sort by Division.
    let a_div = a.get_division();
    let b_div = b.get_division();
    if a_div != b_div {
        return cmp_by_division(a_div, b_div);
    }

    // Finally, sort by WeightClass.
    a.weightclasskg.cmp(&b.weightclasskg)
}

fn finish_table<'db>(
    opldb: &'db opldb::OplDb,
    locale: &'db Locale,
    points_system: PointsSystem,
    ruleset: RuleSet,
    entries: &mut Vec<&'db Entry>,
) -> Table<'db> {
    entries.sort_unstable_by(|a, b| a.place.cmp(&b.place));

    let units = locale.units;
    let format = locale.number_format;

    let sex: &str = match entries[0].sex {
        Sex::M => &locale.strings.selectors.sex.m,
        Sex::F => &locale.strings.selectors.sex.f,
        Sex::Mx => "Mx",
    };

    let equip: &str = if ruleset.contains(Rule::CombineAllEquipment) {
        "" // No equipment specifier.
    } else if ruleset.contains(Rule::CombineRawAndWraps)
        && (entries[0].equipment == Equipment::Raw || entries[0].equipment == Equipment::Wraps)
    {
        locale.strings.translate_equipment(Equipment::Wraps)
    } else if ruleset.contains(Rule::CombineSingleAndMulti)
        && (entries[0].equipment == Equipment::Single || entries[0].equipment == Equipment::Multi)
    {
        locale.strings.translate_equipment(Equipment::Multi)
    } else {
        locale.strings.translate_equipment(entries[0].equipment)
    };

    let class = entries[0].weightclasskg.as_type(units).in_format(format);
    let div: &str = entries[0].division.as_deref().unwrap_or("");

    // TODO: Internationalization.
    // TODO: Cover all the cases. Try to use some match arms.
    let event: &str = if entries[0].event.is_push_pull() {
        " Push-Pull"
    } else if entries[0].event.is_squat_only() {
        " Squat Only"
    } else if entries[0].event.is_bench_only() {
        " Bench Only"
    } else if entries[0].event.is_deadlift_only() {
        " Deadlift Only"
    } else {
        ""
    };

    let title = Some(format!("{} {} {} {}{}", sex, equip, class, div, event));

    let rows: Vec<ResultsRow> = entries
        .iter()
        .map(|e| ResultsRow::from(opldb, locale, points_system, e, 0))
        .collect();

    Table { title, rows }
}

fn make_tables_by_division<'db>(
    opldb: &'db opldb::OplDb,
    locale: &'db Locale,
    points_system: PointsSystem,
    meet_id: u32,
    ruleset: RuleSet,
) -> Vec<Table<'db>> {
    let mut entries = opldb.get_entries_for_meet(meet_id);
    if entries.is_empty() {
        return vec![Table {
            title: None,
            rows: vec![],
        }];
    }

    // Sort each entry so that entries that should be in the same table
    // appear next to each other in the vector.
    entries.sort_unstable_by(|a, b| cmp_by_group(ruleset, a, b));

    // Iterate over each entry, constructing a group.
    let mut key_entry = &entries[0];
    let mut group: Vec<&Entry> = Vec::new();
    let mut tables: Vec<Table> = Vec::new();

    for entry in &entries {
        // Keep batching entries that are in the same group.
        if cmp_by_group(ruleset, entry, key_entry) == cmp::Ordering::Equal {
            group.push(entry);
            continue;
        }

        // This entry isn't part of the old group.
        // Finish the old group.
        tables.push(finish_table(
            opldb,
            locale,
            points_system,
            ruleset,
            &mut group,
        ));

        // Start a new group.
        key_entry = entry;
        group.clear();
        group.push(key_entry);
    }

    // Wrap up the last batch.
    tables.push(finish_table(
        opldb,
        locale,
        points_system,
        ruleset,
        &mut group,
    ));
    tables
}

fn make_tables_by_points<'db>(
    opldb: &'db opldb::OplDb,
    locale: &'db Locale,
    points_system: PointsSystem,
    meet_id: u32,
) -> Vec<Table<'db>> {
    let meets = opldb.get_meets();

    // Display at most one entry for each lifter.
    let groups = opldb
        .get_entries_for_meet(meet_id)
        .into_iter()
        .group_by(|e| e.lifter_id);

    // FIXME: This should use the current PointsSystem, not Wilks.
    let mut entries: Vec<&opldb::Entry> = groups
        .into_iter()
        .map(|(_key, group)| group.max_by_key(|x| x.wilks).unwrap())
        .collect();

    // The points system to be used for display.
    // The "Total" logic below changes this to select the federation default.
    let mut display_points_system = points_system;

    match points_system {
        PointsSystem::AH => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_ah(meets, a, b));
        }
        PointsSystem::Dots => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_dots(meets, a, b));
        }
        PointsSystem::Glossbrenner => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_glossbrenner(meets, a, b));
        }
        PointsSystem::Goodlift => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_goodlift(meets, a, b));
        }
        PointsSystem::IPFPoints => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_ipfpoints(meets, a, b));
        }
        PointsSystem::McCulloch => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_mcculloch(meets, a, b));
        }
        PointsSystem::NASA => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_nasa(meets, a, b));
        }
        PointsSystem::Reshel => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_reshel(meets, a, b));
        }
        PointsSystem::SchwartzMalone => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_schwartzmalone(meets, a, b));
        }
        PointsSystem::Total => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_total(meets, a, b));
            let meet = opldb.get_meet(meet_id);
            display_points_system = meet.federation.default_points(meet.date);
        }
        PointsSystem::Wilks => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_wilks(meets, a, b));
        }
        PointsSystem::Wilks2020 => {
            entries.sort_unstable_by(|a, b| algorithms::cmp_wilks2020(meets, a, b));
        }
    };

    let rows: Vec<ResultsRow> = entries
        .into_iter()
        .zip(1..)
        .map(|(e, i)| ResultsRow::from(opldb, locale, display_points_system, e, i))
        .collect();

    vec![Table { title: None, rows }]
}

impl<'db> Context<'db> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        locale: &'db Locale,
        meet_id: u32,
        sort: MeetSortSelection,
    ) -> Context<'db> {
        let meet = opldb.get_meet(meet_id);
        let default_points: PointsSystem = meet.federation.default_points(meet.date);

        let tables: Vec<Table> = match sort {
            MeetSortSelection::ByDivision => {
                make_tables_by_division(opldb, locale, default_points, meet_id, meet.ruleset)
            }
            _ => {
                let system = sort.as_points_system(default_points);
                make_tables_by_points(opldb, locale, system, meet_id)
            }
        };

        // Paths do not include the urlprefix, which defaults to "/".
        let path_if_by_ah = match default_points {
            PointsSystem::AH => format!("m/{}", meet.path),
            _ => format!("m/{}/by-ah", meet.path),
        };
        let path_if_by_division = format!("m/{}/by-division", meet.path);
        let path_if_by_dots = match default_points {
            PointsSystem::Dots => format!("m/{}", meet.path),
            _ => format!("m/{}/by-dots", meet.path),
        };
        let path_if_by_glossbrenner = match default_points {
            PointsSystem::Glossbrenner => format!("m/{}", meet.path),
            _ => format!("m/{}/by-glossbrenner", meet.path),
        };
        let path_if_by_goodlift = match default_points {
            PointsSystem::Goodlift => format!("m/{}", meet.path),
            _ => format!("m/{}/by-goodlift", meet.path),
        };
        let path_if_by_ipfpoints = match default_points {
            PointsSystem::IPFPoints => format!("m/{}", meet.path),
            _ => format!("m/{}/by-ipf-points", meet.path),
        };
        let path_if_by_mcculloch = match default_points {
            PointsSystem::McCulloch => format!("m/{}", meet.path),
            _ => format!("m/{}/by-mcculloch", meet.path),
        };
        let path_if_by_nasa = match default_points {
            PointsSystem::NASA => format!("m/{}", meet.path),
            _ => format!("m/{}/by-nasa", meet.path),
        };
        let path_if_by_reshel = match default_points {
            PointsSystem::Reshel => format!("m/{}", meet.path),
            _ => format!("m/{}/by-reshel", meet.path),
        };
        let path_if_by_schwartzmalone = match default_points {
            PointsSystem::SchwartzMalone => format!("m/{}", meet.path),
            _ => format!("m/{}/by-schwartz-malone", meet.path),
        };
        let path_if_by_total = match default_points {
            PointsSystem::NASA => format!("m/{}", meet.path),
            _ => format!("m/{}/by-total", meet.path),
        };
        let path_if_by_wilks = match default_points {
            PointsSystem::Wilks => format!("m/{}", meet.path),
            _ => format!("m/{}/by-wilks", meet.path),
        };
        let path_if_by_wilks2020 = match default_points {
            PointsSystem::Wilks2020 => format!("m/{}", meet.path),
            _ => format!("m/{}/by-wilks2020", meet.path),
        };

        Context {
            urlprefix: "/",
            page_title: format!("{} {} {}", meet.date.year(), meet.federation, meet.name),
            page_description: &locale.strings.html_header.description,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            points_column_title: sort.column_title(locale, default_points),
            sortselection: sort.resolve_fed_default(default_points),
            meet: MeetInfo::from(meet, locale.strings),
            year: meet.date.year(),
            has_age_data: true, // TODO: Maybe use again?
            tables,
            use_rank_column: sort != MeetSortSelection::ByDivision,
            path_if_by_ah,
            path_if_by_division,
            path_if_by_dots,
            path_if_by_glossbrenner,
            path_if_by_goodlift,
            path_if_by_ipfpoints,
            path_if_by_mcculloch,
            path_if_by_nasa,
            path_if_by_reshel,
            path_if_by_schwartzmalone,
            path_if_by_total,
            path_if_by_wilks,
            path_if_by_wilks2020,
        }
    }
}
