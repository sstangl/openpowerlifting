//! Types for raw data interchange from Rust to JS.

use langpack::{localized_name, Locale, LocalizeNumber};
use opldb::{Entry, OplDb};
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
    pub vkontakte: Option<&'db str>,
    pub color: Option<&'db str>,
    pub flair: Option<&'db str>,

    pub federation: Federation,
    pub date: String,
    pub country: &'db str,
    pub state: Option<&'db str>,
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
impl<'db> Serialize for JsEntryRow<'db> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.sorted_index)?;
        seq.serialize_element(&self.rank)?;

        seq.serialize_element(&self.name)?;
        seq.serialize_element(&self.username)?;
        seq.serialize_element(&self.instagram)?;
        seq.serialize_element(&self.vkontakte)?;
        seq.serialize_element(&self.color)?;
        seq.serialize_element(&self.flair)?;

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
            vkontakte: lifter.vkontakte.as_deref(),
            color: lifter.color.as_deref(),
            flair: lifter.flair.as_deref(),

            federation: meet.federation,
            date: format!("{}", meet.date),
            country: strings.translate_country(meet.country),
            state: meet.state.as_deref(),
            path: &meet.path,

            sex: strings.translate_sex(entry.sex),
            equipment: strings.translate_equipment(entry.equipment),
            age: PrettyAge::from(entry.age),
            division: entry.division.as_deref(),
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
