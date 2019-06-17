//! Logic for each lifter's personal page.

use opltypes::*;

use crate::langpack::{self, get_localized_name, Language, Locale, LocalizeNumber};
use crate::opldb::{self, Entry};

/// The context object passed to `templates/lifter.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub urlprefix: &'static str,
    pub page_title: &'a str,
    pub localized_name: &'a str,
    pub lifter: &'a opldb::Lifter,
    pub lifter_sex: &'a str,
    pub show_sex_column: bool,
    pub show_attempts: bool,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: WeightUnits,

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
    pub wilks: Option<langpack::LocalizedPoints>,
    pub ipfpoints: Option<langpack::LocalizedPoints>,
}

impl<'db> PersonalBestsRow<'db> {
    fn new(
        locale: &'db Locale,
        equipment: &'db str,
        squat: Option<WeightKg>,
        bench: Option<WeightKg>,
        deadlift: Option<WeightKg>,
        total: Option<WeightKg>,
        wilks: Option<Points>,
        ipfpoints: Option<Points>,
    ) -> PersonalBestsRow<'db> {
        let units = locale.units;
        let format = locale.number_format;

        PersonalBestsRow {
            equipment,
            squat: squat.map(|kg| kg.as_type(units).in_format(format)),
            bench: bench.map(|kg| kg.as_type(units).in_format(format)),
            deadlift: deadlift.map(|kg| kg.as_type(units).in_format(format)),
            total: total.map(|kg| kg.as_type(units).in_format(format)),
            wilks: wilks.map(|pt| pt.in_format(format)),
            ipfpoints: ipfpoints.map(|pt| pt.in_format(format)),
        }
    }
}

/// A row in the meet results table.
#[derive(Serialize)]
pub struct MeetResultsRow<'a> {
    pub place: String,
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

    // Bests.
    pub squat: langpack::LocalizedWeightAny,
    pub bench: langpack::LocalizedWeightAny,
    pub deadlift: langpack::LocalizedWeightAny,
    pub total: langpack::LocalizedWeightAny,

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
    pub wilks: langpack::LocalizedPoints,
    pub ipfpoints: langpack::LocalizedPoints,
}

impl<'a> MeetResultsRow<'a> {
    pub fn from(
        opldb: &'a opldb::OplDb,
        locale: &'a Locale,
        entry: &'a opldb::Entry,
    ) -> MeetResultsRow<'a> {
        let meet: &'a opldb::Meet = opldb.get_meet(entry.meet_id);

        let strings = locale.strings;
        let number_format = locale.number_format;
        let units = locale.units;

        MeetResultsRow {
            place: format!("{}", &entry.place),
            federation: &meet.federation,
            date: format!("{}", meet.date),
            country: strings.translate_country(meet.country),
            state: match meet.state {
                None => None,
                Some(ref s) => Some(&s),
            },
            meet_name: &meet.name,
            meet_path: &meet.path,
            division: match entry.division {
                None => None,
                Some(ref s) => Some(&s),
            },
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

            wilks: entry.wilks.in_format(number_format),
            ipfpoints: entry.ipfpoints.in_format(number_format),
        }
    }
}

/// Helper function to isolate all the best-calculation logic.
fn calculate_bests<'db>(
    locale: &'db Locale,
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

    let raw_wilks: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Raw)
        .map(|e| e.wilks)
        .max();

    let raw_ipfpoints: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Raw)
        .map(|e| e.ipfpoints)
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

    let wraps_wilks: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Wraps)
        .map(|e| e.wilks)
        .max();

    let wraps_ipfpoints: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Wraps)
        .map(|e| e.ipfpoints)
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

    let single_wilks: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Single)
        .map(|e| e.wilks)
        .max();

    let single_ipfpoints: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Single)
        .map(|e| e.ipfpoints)
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

    let multi_wilks: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Multi)
        .map(|e| e.wilks)
        .max();

    let multi_ipfpoints: Option<Points> = non_dq
        .iter()
        .filter(|e| e.event.is_full_power() && e.equipment == Equipment::Multi)
        .map(|e| e.ipfpoints)
        .max();

    let mut rows = Vec::with_capacity(4);

    if raw_squat.is_some()
        || raw_bench.is_some()
        || raw_deadlift.is_some()
        || raw_total.is_some()
    {
        rows.push(PersonalBestsRow::new(
            &locale,
            &locale.strings.equipment.raw,
            raw_squat,
            raw_bench,
            raw_deadlift,
            raw_total,
            raw_wilks,
            raw_ipfpoints,
        ));
    }

    if wraps_squat.is_some() || wraps_total.is_some() {
        rows.push(PersonalBestsRow::new(
            &locale,
            &locale.strings.equipment.wraps,
            wraps_squat,
            None,
            None,
            wraps_total,
            wraps_wilks,
            wraps_ipfpoints,
        ));
    }

    if single_squat.is_some()
        || single_bench.is_some()
        || single_deadlift.is_some()
        || single_total.is_some()
    {
        rows.push(PersonalBestsRow::new(
            &locale,
            &locale.strings.equipment.single,
            single_squat,
            single_bench,
            single_deadlift,
            single_total,
            single_wilks,
            single_ipfpoints,
        ));
    }

    if multi_squat.is_some()
        || multi_bench.is_some()
        || multi_deadlift.is_some()
        || multi_total.is_some()
    {
        rows.push(PersonalBestsRow::new(
            &locale,
            &locale.strings.equipment.multi,
            multi_squat,
            multi_bench,
            multi_deadlift,
            multi_total,
            multi_wilks,
            multi_ipfpoints,
        ));
    }

    rows
}

impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        locale: &'a Locale,
        lifter_id: u32,
        entry_filter: Option<fn(&'a opldb::OplDb, &'a Entry) -> bool>, /* For use by
                                                                        * distributions.
                                                                        */
    ) -> Context<'a> {
        let lifter = opldb.get_lifter(lifter_id);

        // Get a list of the entries for this lifter, oldest entries first.
        let mut entries = opldb.get_entries_for_lifter(lifter_id);
        if let Some(f) = entry_filter {
            entries = entries.into_iter().filter(|e| f(opldb, *e)).collect();
        }
        entries.sort_unstable_by_key(|e| &opldb.get_meet(e.meet_id).date);

        let bests = calculate_bests(&locale, &entries);

        // Do all the entries have the same Sex?
        // If not, we want to show a "Sex" column for debugging.
        let mut consistent_sex: bool = true;
        for entry in entries.iter().skip(1) {
            if entry.sex != entries[0].sex {
                consistent_sex = false;
                break;
            }
        }

        let lifter_sex = if consistent_sex {
            locale.strings.translate_sex(entries[0].sex)
        } else {
            "?"
        };

        // Determine if any of the entries have attempt information.
        // If a federation only reports Bests, we don't want lots of empty columns.
        let has_attempts = entries.iter().find(
            |&e| {
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
            }
        ).is_some();

        // Display the meet results, most recent first.
        let meet_results = entries
            .into_iter()
            .map(|e| MeetResultsRow::from(opldb, locale, e))
            .rev()
            .collect();

        Context {
            urlprefix: "/",
            page_title: get_localized_name(&lifter, locale.language),
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            localized_name: get_localized_name(&lifter, locale.language),
            lifter,
            lifter_sex,
            show_sex_column: !consistent_sex,
            show_attempts: has_attempts,
            bests,
            meet_results,
        }
    }
}
