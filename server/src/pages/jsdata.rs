//! Types for raw data interchange from Rust to JS.

use langpack::{localized_name, Locale, LocalizeNumber};
use opldb::{Entry, OplDb};
use opltypes::states::State;
use opltypes::*;
use serde::ser::{Serialize, SerializeSeq, Serializer};

/// Represents one row of JS data.
///
/// This struct is manually serialized. When adding members to this struct,
/// also remember to update the Serialize implementation.
pub struct JsEntryRow<'db> {
    pub sorted_index: u32,
    pub rank: langpack::LocalizedOrdinal,

    pub name: &'db str,
    pub username: &'db str,
    pub instagram: Option<&'db str>,
    pub color: Option<&'db str>,

    pub lifter_country: Option<&'db str>,
    pub lifter_state: Option<State>,

    pub federation: Federation,
    pub date: String,
    pub meet_country: &'db str,
    pub meet_state: Option<&'db str>,
    pub path: &'db str,

    pub sex: &'db str,
    pub equipment: &'db str,
    pub age: PrettyAge,
    pub division: Option<&'db str>,
    pub bodyweight: langpack::LocalizedWeightAny,
    pub weightclass: langpack::LocalizedWeightClassAny,
    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,
    pub total: langpack::LocalizedWeightAny,

    /// Any kind of points: Wilks, McCulloch, etc.
    /// Only one points system is used at a time.
    pub points: langpack::LocalizedPoints,
}

/// Serialize to a compact but definitely less-helpful format
/// for JS interchange.
impl Serialize for JsEntryRow<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.sorted_index)?;
        seq.serialize_element(&self.rank)?;

        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.username)?;
        seq.serialize_element(&self.instagram)?;
        seq.serialize_element(&self.color)?;

        seq.serialize_element(&self.lifter_country)?;
        seq.serialize_element(&self.lifter_state)?;

        seq.serialize_element(&self.federation)?;
        seq.serialize_element(&self.date)?;
        seq.serialize_element(&self.meet_country)?;
        seq.serialize_element(&self.meet_state)?;
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
        seq.serialize_element(&self.points)?;

        seq.end()
    }
}

impl<'db> JsEntryRow<'db> {
    pub fn from(
        opldb: &'db OplDb,
        locale: &'db Locale,
        entry: &'db Entry,
        sorted_index: u32,
        points_system: PointsSystem,
    ) -> JsEntryRow<'db> {
        let meet = opldb.meet(entry.meet_id);
        let lifter = opldb.lifter(entry.lifter_id);

        let strings = locale.strings;
        let number_format = locale.number_format;
        let units = locale.units;

        JsEntryRow {
            sorted_index,

            rank: locale.ordinal(sorted_index + 1, entry.sex),

            name: localized_name(lifter, locale.language),
            username: lifter.username.as_str(),
            instagram: lifter.instagram.as_deref(),
            color: lifter.color.as_deref(),

            lifter_country: entry.lifter_country.map(|c| strings.translate_country(c)),
            lifter_state: entry.lifter_state,

            federation: meet.federation,
            date: format!("{}", meet.date),
            meet_country: strings.translate_country(meet.country),
            meet_state: meet.state.as_deref(),
            path: &meet.path,

            sex: strings.translate_sex(entry.sex),
            equipment: strings.translate_equipment(entry.equipment),
            age: PrettyAge::from(entry.age),
            division: entry.division.map(|symbol| symbol.as_str()),
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
            points: entry.points(points_system, units).in_format(number_format),
        }
    }
}
