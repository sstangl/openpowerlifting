//! Logic for each lifter's personal page.

use langpack::{get_localized_name, Language, Locale, LocalizeNumber};
use opldb::{self, Entry};
use opltypes::*;

use crate::pages::meet::points_column_title; // FIXME: This should not be defined there.

/// The context object passed to `templates/lifter.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub urlprefix: &'static str,
    pub page_title: &'a str,
    pub page_description: &'a str,
    pub localized_name: &'a str,
    pub lifter: &'a opldb::Lifter,
    pub lifter_sex: &'a str,
    pub show_sex_column: bool,
    pub show_attempts: bool,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: WeightUnits,
    pub points_column_title: &'a str,

    pub bests: Vec<PersonalBestsRow<'a>>,
    pub meet_results: Vec<MeetResultsRow<'a>>,
}

/// A row in the Best Lifts table.
#[derive(Serialize)]
pub struct PersonalBestsRow<'db> {
    pub equipment: &'db str,
    pub squat: Option<langpack::LocalizedWeightAny>,
    pub bench: Option<langpack::LocalizedWeightAny>,
    pub deadlift: Option<langpack::LocalizedWeightAny>,
    pub total: Option<langpack::LocalizedWeightAny>,
    pub points: Option<langpack::LocalizedPoints>,
}

impl<'db> PersonalBestsRow<'db> {
    fn new(
        locale: &'db Locale,
        equipment: &'db str,
        squat: Option<WeightKg>,
        bench: Option<WeightKg>,
        deadlift: Option<WeightKg>,
        total: Option<WeightKg>,
        points: Option<Points>,
    ) -> PersonalBestsRow<'db> {
        let units = locale.units;
        let format = locale.number_format;

        PersonalBestsRow {
            equipment,
            squat: squat.map(|kg| kg.as_type(units).in_format(format)),
            bench: bench.map(|kg| kg.as_type(units).in_format(format)),
            deadlift: deadlift.map(|kg| kg.as_type(units).in_format(format)),
            total: total.map(|kg| kg.as_type(units).in_format(format)),
            points: points.map(|pt| pt.in_format(format)),
        }
    }
}

/// A row in the meet results table.
#[derive(Serialize)]
pub struct MeetResultsRow<'a> {
    pub place: langpack::LocalizedPlace,
    pub federation: &'a Federation,
    pub date: String,
    pub country: &'a str,
    pub state: Option<&'a str>,
    pub meet_name: &'a str,
    pub meet_path: &'a str,
    pub division: Option<&'a str>,
    pub age: PrettyAge,
    pub sex: &'a str,
    pub equipment: &'a str,
    pub weightclass: langpack::LocalizedWeightClassAny,
    pub bodyweight: langpack::LocalizedWeightAny,

    // Bests, including 4th attempts
    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,

    // Total, excluding 4th attempts.
    pub total: langpack::LocalizedWeightAny,

    // Best of first 3 attempts.
    //
    // This is needed when the template wants to show attempts,
    // but the row only includes, for example, Best3SquatKg and Squat4Kg.
    pub best3squat: langpack::LocalizedWeightAny,
    pub best3bench: langpack::LocalizedWeightAny,
    pub best3deadlift: langpack::LocalizedWeightAny,

    // Attempts.
    // Remember that federations might only report bests!
    pub squat1: langpack::LocalizedWeightAny,
    pub squat2: langpack::LocalizedWeightAny,
    pub squat3: langpack::LocalizedWeightAny,
    pub squat4: langpack::LocalizedWeightAny,
    pub bench1: langpack::LocalizedWeightAny,
    pub bench2: langpack::LocalizedWeightAny,
    pub bench3: langpack::LocalizedWeightAny,
    pub bench4: langpack::LocalizedWeightAny,
    pub deadlift1: langpack::LocalizedWeightAny,
    pub deadlift2: langpack::LocalizedWeightAny,
    pub deadlift3: langpack::LocalizedWeightAny,
    pub deadlift4: langpack::LocalizedWeightAny,

    // Points.
    pub points: langpack::LocalizedPoints,
}

impl<'a> MeetResultsRow<'a> {
    pub fn from(
        opldb: &'a opldb::OplDb,
        locale: &'a Locale,
        points_system: PointsSystem,
        entry: &'a opldb::Entry,
    ) -> MeetResultsRow<'a> {
        let meet: &'a opldb::Meet = opldb.get_meet(entry.meet_id);

        let strings = locale.strings;
        let number_format = locale.number_format;
        let units = locale.units;

        MeetResultsRow {
            place: locale.place(entry.place, entry.sex),
            federation: &meet.federation,
            date: format!("{}", meet.date),
            country: strings.translate_country(meet.country),
            state: meet.state.as_ref().map(|s| s as _),
            meet_name: &meet.name,
            meet_path: &meet.path,
            division: entry.division.as_ref().map(|d| d as _),
            age: PrettyAge::from(entry.age),
            sex: strings.translate_sex(entry.sex),
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

            best3squat: entry.best3squatkg.as_type(units).in_format(number_format),
            best3bench: entry.best3benchkg.as_type(units).in_format(number_format),
            best3deadlift: entry
                .best3deadliftkg
                .as_type(units)
                .in_format(number_format),

            squat1: entry.squat1kg.as_type(units).in_format(number_format),
            squat2: entry.squat2kg.as_type(units).in_format(number_format),
            squat3: entry.squat3kg.as_type(units).in_format(number_format),
            squat4: entry.squat4kg.as_type(units).in_format(number_format),
            bench1: entry.bench1kg.as_type(units).in_format(number_format),
            bench2: entry.bench2kg.as_type(units).in_format(number_format),
            bench3: entry.bench3kg.as_type(units).in_format(number_format),
            bench4: entry.bench4kg.as_type(units).in_format(number_format),
            deadlift1: entry.deadlift1kg.as_type(units).in_format(number_format),
            deadlift2: entry.deadlift2kg.as_type(units).in_format(number_format),
            deadlift3: entry.deadlift3kg.as_type(units).in_format(number_format),
            deadlift4: entry.deadlift4kg.as_type(units).in_format(number_format),

            points: entry.points(points_system, units).in_format(number_format),
        }
    }
}

/// Helper function to isolate all the best-calculation logic.
fn calculate_bests<'db>(
    locale: &'db Locale,
    points_system: PointsSystem,
    entries: &[&Entry],
) -> Vec<PersonalBestsRow<'db>> {
    // Best lifts must ignore disqualified entries.
    let non_dq: Vec<&&Entry> = entries.iter().filter(|e| !e.place.is_dq()).collect();

    // Calculate best lifts.
    let raw_squat: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Raw)
        .map(|e| e.highest_squatkg())
        .max();

    let raw_bench: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Raw || e.equipment == Equipment::Wraps)
        .map(|e| e.highest_benchkg())
        .max();

    let raw_deadlift: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Raw || e.equipment == Equipment::Wraps)
        .map(|e| e.highest_deadliftkg())
        .max();

    let raw_total: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Raw)
        .map(|e| e.totalkg)
        .max();

    let raw_points: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Raw)
        .map(|e| e.points(points_system, locale.units))
        .max();

    let wraps_squat: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Wraps)
        .map(|e| e.highest_squatkg())
        .max();

    let wraps_total: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Wraps)
        .map(|e| e.totalkg)
        .max();

    let wraps_points: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Wraps)
        .map(|e| e.points(points_system, locale.units))
        .max();

    let single_squat: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Single)
        .map(|e| e.highest_squatkg())
        .max();

    let single_bench: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Single)
        .map(|e| e.highest_benchkg())
        .max();

    let single_deadlift: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Single)
        .map(|e| e.highest_deadliftkg())
        .max();

    let single_total: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Single)
        .map(|e| e.totalkg)
        .max();

    let single_points: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Single)
        .map(|e| e.points(points_system, locale.units))
        .max();

    let multi_squat: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Multi)
        .map(|e| e.highest_squatkg())
        .max();

    let multi_bench: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Multi)
        .map(|e| e.highest_benchkg())
        .max();

    let multi_deadlift: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Multi)
        .map(|e| e.highest_deadliftkg())
        .max();

    let multi_total: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Multi)
        .map(|e| e.totalkg)
        .max();

    let multi_points: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Multi)
        .map(|e| e.points(points_system, locale.units))
        .max();

    let unlimited_squat: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Unlimited)
        .map(|e| e.highest_squatkg())
        .max();

    let unlimited_bench: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Unlimited)
        .map(|e| e.highest_benchkg())
        .max();

    let unlimited_deadlift: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.equipment == Equipment::Unlimited)
        .map(|e| e.highest_deadliftkg())
        .max();

    let unlimited_total: Option<WeightKg> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Unlimited)
        .map(|e| e.totalkg)
        .max();

    let unlimited_points: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Unlimited)
        .map(|e| e.points(points_system, locale.units))
        .max();

    let mut rows = Vec::with_capacity(4);

    if raw_squat.is_some() || raw_bench.is_some() || raw_deadlift.is_some() || raw_total.is_some() {
        rows.push(PersonalBestsRow::new(
            locale,
            &locale.strings.equipment.raw,
            raw_squat,
            raw_bench,
            raw_deadlift,
            raw_total,
            raw_points,
        ));
    }

    if wraps_squat.is_some() || wraps_total.is_some() {
        rows.push(PersonalBestsRow::new(
            locale,
            &locale.strings.equipment.wraps,
            wraps_squat,
            None,
            None,
            wraps_total,
            wraps_points,
        ));
    }

    if single_squat.is_some()
        || single_bench.is_some()
        || single_deadlift.is_some()
        || single_total.is_some()
    {
        rows.push(PersonalBestsRow::new(
            locale,
            &locale.strings.equipment.single,
            single_squat,
            single_bench,
            single_deadlift,
            single_total,
            single_points,
        ));
    }

    if multi_squat.is_some()
        || multi_bench.is_some()
        || multi_deadlift.is_some()
        || multi_total.is_some()
    {
        rows.push(PersonalBestsRow::new(
            locale,
            &locale.strings.equipment.multi,
            multi_squat,
            multi_bench,
            multi_deadlift,
            multi_total,
            multi_points,
        ));
    }

    if unlimited_squat.is_some()
        || unlimited_bench.is_some()
        || unlimited_deadlift.is_some()
        || unlimited_total.is_some()
    {
        rows.push(PersonalBestsRow::new(
            locale,
            &locale.strings.equipment.unlimited,
            unlimited_squat,
            unlimited_bench,
            unlimited_deadlift,
            unlimited_total,
            unlimited_points,
        ));
    }

    rows
}

impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        locale: &'a Locale,
        lifter_id: u32,
        points_system: PointsSystem,
        entry_filter: Option<fn(&'a opldb::OplDb, &'a Entry) -> bool>, /* For use by
                                                                        * distributions.
                                                                        */
    ) -> Context<'a> {
        let lifter = opldb.get_lifter(lifter_id);
        let mut entries = opldb.get_entries_for_lifter(lifter_id);

        // Do all the entries have the same Sex?
        // If not, we want to show a "Sex" column for debugging.
        // This must be done before we apply any entry_filter.
        let mut consistent_sex: bool = true;
        for entry in entries.iter().skip(1) {
            if entry.sex != entries[0].sex {
                consistent_sex = false;
                break;
            }
        }

        // Filter and sort the entries, oldest entries first.
        if let Some(f) = entry_filter {
            entries = entries.into_iter().filter(|e| f(opldb, *e)).collect();
        }
        entries.sort_unstable_by_key(|e| &opldb.get_meet(e.meet_id).date);

        // Display sex information from the most recent meet.
        let lifter_sex = match entries.last() {
            Some(entry) => locale.strings.translate_sex(entry.sex),
            None => "?",
        };

        let bests = calculate_bests(locale, points_system, &entries);

        // Determine if any of the entries have attempt information.
        // If a federation only reports Bests, we don't want lots of empty columns.
        let has_attempts = entries.iter().any(|&e| {
            e.squat1kg.is_non_zero()
                || e.squat2kg.is_non_zero()
                || e.squat3kg.is_non_zero()
                || e.squat4kg.is_non_zero()
                || e.bench1kg.is_non_zero()
                || e.bench2kg.is_non_zero()
                || e.bench3kg.is_non_zero()
                || e.bench4kg.is_non_zero()
                || e.deadlift1kg.is_non_zero()
                || e.deadlift2kg.is_non_zero()
                || e.deadlift3kg.is_non_zero()
                || e.deadlift4kg.is_non_zero()
        });

        // Display the meet results, most recent first.
        let meet_results = entries
            .into_iter()
            .map(|e| MeetResultsRow::from(opldb, locale, points_system, e))
            .rev()
            .collect();

        Context {
            urlprefix: "/",
            page_title: get_localized_name(lifter, locale.language),
            page_description: &locale.strings.html_header.description,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            localized_name: get_localized_name(lifter, locale.language),
            lifter,
            lifter_sex,
            show_sex_column: !consistent_sex,
            show_attempts: has_attempts,
            points_column_title: points_column_title(points_system, locale, points_system),
            bests,
            meet_results,
        }
    }
}
