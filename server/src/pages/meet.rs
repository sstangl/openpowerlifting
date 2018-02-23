//! Logic for each meet's individual results page.

use opldb;
use opldb::fields;
use langpack::{self, Language};

#[derive(Serialize)]
pub struct HeaderContext {
    pub num_entries: u32,
    pub num_meets: u32,
}

/// The context object passed to `templates/lifter.html.hbs`.
#[derive(Serialize)]
pub struct Context<'a> {
    pub header: HeaderContext,
    pub meet: MeetInfo<'a>,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: opldb::WeightUnits,

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
    pub fn from(meet: &'a opldb::Meet) -> MeetInfo<'a> {
        MeetInfo {
            path: &meet.path,
            federation: &meet.federation,
            date: format!("{}", &meet.date),
            country: &meet.country,
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
    pub place: String,
    pub name: &'a str,
    pub username: &'a str,
    pub instagram: Option<&'a str>,
    pub sex: &'a str,
    pub age: fields::Age,
    pub equipment: &'a str,
    pub weightclasskg: String,
    pub bodyweightkg: fields::WeightAny,

    pub squatkg: fields::WeightAny,
    pub benchkg: fields::WeightAny,
    pub deadliftkg: fields::WeightAny,
    pub totalkg: fields::WeightAny,
    pub wilks: fields::Points,
}

impl<'a> ResultsRow<'a> {
    fn from(
        opldb: &'a opldb::OplDb,
        strings: &'a langpack::Translations,
        units: opldb::WeightUnits,
        entry: &'a opldb::Entry,
    ) -> ResultsRow<'a> {
        let lifter: &'a opldb::Lifter = opldb.get_lifter(entry.lifter_id);

        ResultsRow {
            place: format!("{}", &entry.place),
            name: &lifter.name,
            username: &lifter.username,
            instagram: match lifter.instagram {
                None => None,
                Some(ref s) => Some(&s),
            },
            sex: strings.translate_sex(entry.sex),
            age: entry.age,
            equipment: strings.translate_equipment(entry.equipment),
            weightclasskg: format!("{}", entry.weightclasskg),
            bodyweightkg: entry.bodyweightkg.as_type(units),

            squatkg: entry.highest_squatkg().as_type(units),
            benchkg: entry.highest_benchkg().as_type(units),
            deadliftkg: entry.highest_deadliftkg().as_type(units),
            totalkg: entry.totalkg.as_type(units),
            wilks: entry.wilks,
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

        // Get a list of the entries for this meet, highest Wilks first.
        let mut entries = opldb.get_entries_for_meet(meet_id);
        entries.sort_unstable_by_key(|e| -1 * e.wilks.0);

        let rows = entries
            .into_iter()
            .map(|e| ResultsRow::from(opldb, strings, units, e))
            .collect();

        Context {
            header: HeaderContext {
                num_entries: opldb.get_entries().len() as u32,
                num_meets: opldb.get_meets().len() as u32,
            },
            language: language,
            strings: strings,
            units: units,
            meet: MeetInfo::from(&meet),
            rows: rows,
        }
    }
}
