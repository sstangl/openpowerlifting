//! Logic for each meet's individual results page.

use itertools::Itertools;
use opltypes::*;

use std::cmp;
use std::str::FromStr;

use langpack::{self, get_localized_name, Language, Locale, LocalizeNumber};
use opldb::{self, Entry, algorithms};

/// The context object passed to `templates/meet.html.tera`
#[derive(Serialize)]
pub struct Context<'db> {
    pub page_title: String,
    pub meet: MeetInfo<'db>,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: WeightUnits,
    pub points_column_title: &'db str,

    /// Whether to use Rank instead of Place.
    pub use_rank_column: bool,

    // Instead of having the JS try to figure out how to access
    // other sorts, just tell it what the paths are.
    pub path_if_by_wilks: String,
    pub path_if_by_glossbrenner: String,
    pub path_if_by_division: String,

    /// True iff the meet reported any age data.
    pub has_age_data: bool,
    pub sortselection: MeetSortSelection,

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
    ByDivision,
    ByGlossbrenner,
    ByWilks,
}

impl FromStr for MeetSortSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "by-division" => Ok(MeetSortSelection::ByDivision),
            "by-glossbrenner" => Ok(MeetSortSelection::ByGlossbrenner),
            "by-wilks" => Ok(MeetSortSelection::ByWilks),
            _ => Err(()),
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
    pub fn from(
        meet: &'a opldb::Meet,
        strings: &'a langpack::Translations,
    ) -> MeetInfo<'a> {
        MeetInfo {
            path: &meet.path,
            federation: meet.federation,
            date: format!("{}", &meet.date),
            country: strings.translate_country(meet.country),
            state: match meet.state {
                None => None,
                Some(ref s) => Some(&s),
            },
            town: match meet.town {
                None => None,
                Some(ref s) => Some(&s),
            },
            name: &meet.name,
        }
    }
}

/// A row in the results table.
#[derive(Serialize)]
pub struct ResultsRow<'a> {
    /// The Place given by the federation.
    pub place: String,
    /// The rank in the ranking-by-points view (by Wilks).
    pub rank: u32,
    pub localized_name: &'a str,
    pub lifter: &'a opldb::Lifter,
    pub sex: &'a str,
    pub age: Age,
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
        sort: MeetSortSelection,
        entry: &'a opldb::Entry,
        rank: u32,
    ) -> ResultsRow<'a> {
        let lifter: &'a opldb::Lifter = opldb.get_lifter(entry.lifter_id);

        let strings = locale.strings;
        let number_format = locale.number_format;
        let units = locale.units;

        ResultsRow {
            place: format!("{}", &entry.place),
            rank,
            localized_name: get_localized_name(&lifter, locale.language),
            lifter,
            sex: strings.translate_sex(entry.sex),
            age: entry.age,
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
            points: match sort {
                MeetSortSelection::ByDivision
                | MeetSortSelection::ByWilks => entry.wilks.in_format(number_format),
                MeetSortSelection::ByGlossbrenner => {
                    entry.glossbrenner.in_format(number_format)
                }
            },
        }
    }
}

/// Defines the order of events for the ByDivision display.
const EVENT_SORT_ORDER: [Event; 7] = [
    Event::sbd(), Event::bd(), Event::sb(), Event::sd(),
    Event::s(), Event::b(), Event::d()
];

/// Defines the order of equipment for the ByDivision display.
#[inline]
fn order_by_equipment(a: Equipment) -> u32 {
    match a {
        Equipment::Raw => 0,
        Equipment::Wraps => 1,
        Equipment::Single => 2,
        Equipment::Multi => 3,
        Equipment::Straps => 4,
    }
}

/// Defines the order of sex for the ByDivision display.
#[inline]
fn order_by_sex(a: Sex) -> u32 {
    match a {
        Sex::F => 0,
        Sex::M => 1,
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
    a.cmp(&b)
}

/// Compares two entries for grouping into per-division tables.
fn cmp_by_group(a: &Entry, b: &Entry) -> cmp::Ordering {
    // First, sort by Event.
    let a_event = EVENT_SORT_ORDER.iter().position(|&x| x == a.event).unwrap();
    let b_event = EVENT_SORT_ORDER.iter().position(|&x| x == b.event).unwrap();
    if a_event != b_event {
        return a_event.cmp(&b_event);
    }

    // Next, sort by Equipment.
    let a_equipment = order_by_equipment(a.equipment);
    let b_equipment = order_by_equipment(b.equipment);
    if a_equipment != b_equipment {
        return a_equipment.cmp(&b_equipment);
    }

    // Next, sort by Sex.
    let a_sex = order_by_sex(a.sex);
    let b_sex = order_by_sex(b.sex);
    if a_sex != b_sex {
        return a_sex.cmp(&b_sex);
    }

    // Next, sort by WeightClass.
    a.weightclasskg.cmp(&b.weightclasskg)
        // Finally, sort by Division.
        .then(cmp_by_division(a.get_division(), b.get_division()))
}

fn finish_table<'db>(
    opldb: &'db opldb::OplDb,
    locale: &'db Locale,
    entries: &mut Vec<&'db Entry>
) -> Table<'db> {
    entries.sort_unstable_by(|a, b| a.place.cmp(&b.place));

    let units = locale.units;
    let format = locale.number_format;

    let sex: &str = match entries[0].sex {
        Sex::M => &locale.strings.selectors.sex.m,
        Sex::F => &locale.strings.selectors.sex.f,
    };
    let equip: &str = locale.strings.translate_equipment(entries[0].equipment);
    let class = entries[0].weightclasskg.as_type(units).in_format(format);
    let div: &str = match entries[0].division {
        Some(ref s) => s,
        None => "",
    };

    let title = Some(format!("{} {} {} {}", sex, equip, class, div));

    let rows: Vec<ResultsRow> = entries
        .iter()
        .map(|e| ResultsRow::from(opldb, locale, MeetSortSelection::ByWilks, e, 0))
        .collect();

    Table { title, rows }
}


fn make_tables_by_division<'db> (
    opldb: &'db opldb::OplDb,
    locale: &'db Locale,
    meet_id: u32,
) -> Vec<Table<'db>> {
    let mut entries = opldb.get_entries_for_meet(meet_id);
    if entries.is_empty() {
        return vec![Table { title: None, rows: vec![] }];
    }

    // Sort each entry so that entries that should be in the same table
    // appear next to each other in the vector.
    entries.sort_unstable_by(|a, b| {
        cmp_by_group(a, b)
    });

    // Iterate over each entry, constructing a group.
    let mut key_entry = &entries[0];
    let mut group: Vec<&Entry> = Vec::new();
    let mut tables: Vec<Table> = Vec::new();

    for entry in &entries {
        // Keep batching entries that are in the same group.
        if cmp_by_group(entry, key_entry) == cmp::Ordering::Equal {
            group.push(entry);
            continue;
        }

        // This entry isn't part of the old group.
        // Finish the old group.
        tables.push(finish_table(&opldb, &locale, &mut group));

        // Start a new group.
        key_entry = &entry;
        group.clear();
        group.push(key_entry);
    }

    // Wrap up the last batch.
    tables.push(finish_table(&opldb, &locale, &mut group));
    tables
}

fn make_tables_by_points<'db>(
    opldb: &'db opldb::OplDb,
    locale: &'db Locale,
    sort: MeetSortSelection,
    meet_id: u32,
) -> Vec<Table<'db>>
{
    let meets = opldb.get_meets();

    // Display at most one entry for each lifter.
    let groups = opldb
        .get_entries_for_meet(meet_id)
        .into_iter()
        .group_by(|e| e.lifter_id);

    let mut entries: Vec<&opldb::Entry> = groups
        .into_iter()
        .map(|(_key, group)| group.max_by_key(|x| x.wilks).unwrap())
        .collect();

    match sort {
        MeetSortSelection::ByDivision => panic!("Unexpected ByDivision"),
        MeetSortSelection::ByGlossbrenner => {
            entries.sort_unstable_by(|a, b| {
                algorithms::cmp_glossbrenner(&meets, a, b)
            });
        }
        MeetSortSelection::ByWilks => {
            entries.sort_unstable_by(|a, b| {
                algorithms::cmp_wilks(&meets, a, b)
            });
        }
    };

    let rows: Vec<ResultsRow> = entries
        .into_iter()
        .zip(1..)
        .map(|(e, i)| ResultsRow::from(opldb, locale, sort, e, i))
        .collect();

    vec!(Table { title: None, rows })
}

impl<'db> Context<'db> {
    pub fn new(
        opldb: &'db opldb::OplDb,
        locale: &'db Locale,
        meet_id: u32,
        sort: MeetSortSelection,
    ) -> Context<'db> {
        let meet = opldb.get_meet(meet_id);
        let tables: Vec<Table> = match sort {
            MeetSortSelection::ByDivision => {
                make_tables_by_division(&opldb, &locale, meet_id)
            }
            MeetSortSelection::ByWilks
            | MeetSortSelection::ByGlossbrenner => {
                make_tables_by_points(&opldb, &locale, sort, meet_id)
            }
        };

        let points_column_title = match sort {
            MeetSortSelection::ByDivision
            | MeetSortSelection::ByWilks => &locale.strings.columns.wilks,
            MeetSortSelection::ByGlossbrenner => &locale.strings.columns.glossbrenner,
        };

        Context {
            page_title: format!("{} {} {}", meet.date.year(), meet.federation, meet.name),
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            points_column_title,
            sortselection: sort,
            meet: MeetInfo::from(&meet, locale.strings),
            has_age_data: true, // TODO: Maybe use again?
            tables,
            use_rank_column: sort != MeetSortSelection::ByDivision,
            path_if_by_wilks: format!("/m/{}", meet.path.to_string()),
            path_if_by_glossbrenner: format!("/m/{}/by-glossbrenner", meet.path),
            path_if_by_division: format!("/m/{}/by-division", meet.path),
        }
    }
}
