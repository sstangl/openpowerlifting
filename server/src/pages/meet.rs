//! Logic for each meet's individual results page.

use itertools::Itertools;
use langpack::{self, Language};
use opldb;
use opldb::fields;

/// The context object passed to `templates/meet.html.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: String,
    pub meet: MeetInfo<'a>,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: opldb::WeightUnits,

    /// True iff the meet reported any age data.
    pub has_age_data: bool,
    pub rows: Vec<ResultsRow<'a>>,
}

#[derive(Serialize)]
pub struct MeetInfo<'a> {
    pub path: &'a str,
    pub federation: &'a fields::Federation,
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
            federation: &meet.federation,
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
    pub name: &'a str,
    pub username: &'a str,
    pub instagram: Option<&'a str>,
    pub sex: &'a str,
    pub age: fields::Age,
    pub equipment: &'a str,
    pub weightclass: langpack::LocalizedWeightClassAny,
    pub bodyweight: langpack::LocalizedWeightAny,

    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,
    pub total: langpack::LocalizedWeightAny,
    pub wilks: langpack::LocalizedPoints,
}

impl<'a> ResultsRow<'a> {
    fn from(
        opldb: &'a opldb::OplDb,
        strings: &'a langpack::Translations,
        number_format: langpack::NumberFormat,
        units: opldb::WeightUnits,
        entry: &'a opldb::Entry,
        rank: u32,
    ) -> ResultsRow<'a> {
        let lifter: &'a opldb::Lifter = opldb.get_lifter(entry.lifter_id);

        ResultsRow {
            place: format!("{}", &entry.place),
            rank: rank,
            name: &lifter.name,
            username: &lifter.username,
            instagram: match lifter.instagram {
                None => None,
                Some(ref s) => Some(&s),
            },
            sex: strings.translate_sex(entry.sex),
            age: entry.age,
            equipment: strings.translate_equipment(entry.equipment),
            weightclass: entry
                .weightclasskg
                .as_type(units)
                .in_format(number_format),
            bodyweight: entry
                .bodyweightkg
                .as_type(units)
                .in_format(number_format),

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
            wilks: entry.wilks.in_format(number_format),
        }
    }
}

impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        language: Language,
        langinfo: &'a langpack::LangInfo,
        units: opldb::WeightUnits,
        meet_id: u32,
    ) -> Context<'a> {
        let meet = opldb.get_meet(meet_id);
        let strings = langinfo.get_translations(language);
        let number_format = language.number_format();

        // Display at most one entry for each lifter.
        let groups = opldb
            .get_entries_for_meet(meet_id)
            .into_iter()
            .group_by(|e| e.lifter_id);

        let mut entries: Vec<&opldb::Entry> = groups
            .into_iter()
            .map(|(_key, group)| group.max_by_key(|x| x.wilks).unwrap())
            .collect();

        // Does this meet contain age data?
        // If not, that column will be hidden.
        let mut has_age_data = false;
        for entry in &entries {
            if entry.age != fields::Age::None {
                has_age_data = true;
                break;
            }
        }

        // Get a list of the entries for this meet, highest Wilks first.
        entries.sort_unstable_by(|x, y| {
            x.wilks
                .cmp(&y.wilks)
                .then(x.totalkg.cmp(&y.totalkg))
                .reverse()
        });

        let rows = entries
            .into_iter()
            .zip(1..)
            .map(|(e, i)| ResultsRow::from(opldb, strings, number_format, units, e, i))
            .collect();

        Context {
            page_title: format!(
                "{} {} {}",
                meet.date.year(),
                meet.federation,
                meet.name
            ),
            language: language,
            strings: strings,
            units: units,
            meet: MeetInfo::from(&meet, strings),
            has_age_data: has_age_data,
            rows: rows,
        }
    }
}
