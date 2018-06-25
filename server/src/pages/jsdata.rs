//! Types for raw data interchange from Rust to JS.

use serde::ser::{Serialize, SerializeSeq, Serializer};

use langpack::{self, Locale};
use opldb::fields;
use opldb::{Entry, OplDb};

pub struct JsEntryRow<'db> {
    pub sorted_index: u32,

    pub name: &'db String,
    pub username: &'db String,
    pub instagram: &'db Option<String>,
    pub vkontakte: &'db Option<String>,
    pub color: &'db Option<String>,

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

/// Serialize to a compact but definitely less-helpful format
/// for JS interchange.
impl<'db> Serialize for JsEntryRow<'db> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.sorted_index)?;

        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.username)?;
        seq.serialize_element(&self.instagram)?;
        seq.serialize_element(&self.vkontakte)?;
        seq.serialize_element(&self.color)?;

        seq.serialize_element(&self.federation)?;
        seq.serialize_element(&self.date)?;
        seq.serialize_element(&self.country)?;
        seq.serialize_element(&self.state)?;
        seq.serialize_element(&self.path)?;

        seq.serialize_element(&self.sex)?;
        seq.serialize_element(&self.equipment)?;
        seq.serialize_element(&self.age)?;
        seq.serialize_element(&self.division)?;
        seq.serialize_element(&self.bodyweight)?;
        seq.serialize_element(&self.weightclass)?;
        seq.serialize_element(&self.squat)?;
        seq.serialize_element(&self.bench)?;
        seq.serialize_element(&self.deadlift)?;
        seq.serialize_element(&self.total)?;
        seq.serialize_element(&self.wilks)?;

        seq.end()
    }
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
            vkontakte: &lifter.vkontakte,
            color: &lifter.color,

            federation: &meet.federation,
            date: format!("{}", meet.date),
            country: strings.translate_country(meet.country),
            state: &meet.state,
            path: &meet.path,

            sex: strings.translate_sex(entry.sex),
            equipment: strings.translate_equipment(entry.equipment),
            age: entry.age,
            division: &entry.division,
            bodyweight: entry.bodyweightkg.as_type(units).in_format(number_format),
            weightclass: entry.weightclasskg.as_type(units).in_format(number_format),
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
