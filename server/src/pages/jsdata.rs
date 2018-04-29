//! Types for raw data interchange from Rust to JS.

use langpack::{self, Locale};
use opldb::fields;
use opldb::{Entry, OplDb};

#[derive(Serialize)]
pub struct JsEntryRow<'db> {
    pub sorted_index: u32,

    pub name: &'db String,
    pub username: &'db String,
    pub instagram: &'db Option<String>,

    pub federation: &'db fields::Federation,
    pub date: String,
    pub country: &'db str,
    pub state: &'db Option<String>,
    pub path: &'db String,

    pub sex: &'db str,
    pub equipment: &'db str,
    pub age: fields::Age,
    pub division: &'db Option<String>,
    pub bodyweight: langpack::LocalizedWeightAny,
    pub weightclass: langpack::LocalizedWeightClassAny,
    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,
    pub total: langpack::LocalizedWeightAny,
    pub wilks: langpack::LocalizedPoints,
}

impl<'db> JsEntryRow<'db> {
    pub fn from(
        opldb: &'db OplDb,
        locale: &'db Locale,
        entry: &'db Entry,
        sorted_index: u32,
    ) -> JsEntryRow<'db> {
        let meet = opldb.get_meet(entry.meet_id);
        let lifter = opldb.get_lifter(entry.lifter_id);

        let strings = locale.strings;
        let number_format = locale.number_format;
        let units = locale.units;

        JsEntryRow {
            sorted_index,

            name: &lifter.name,
            username: &lifter.username,
            instagram: &lifter.instagram,

            federation: &meet.federation,
            date: format!("{}", meet.date),
            country: strings.translate_country(meet.country),
            state: &meet.state,
            path: &meet.path,

            sex: strings.translate_sex(entry.sex),
            equipment: strings.translate_equipment(entry.equipment),
            age: entry.age,
            division: &entry.division,
            bodyweight: entry
                .bodyweightkg
                .as_type(units)
                .in_format(number_format),
            weightclass: entry
                .weightclasskg
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
