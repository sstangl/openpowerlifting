//! Logic for each lifter's personal page.

use langpack::{self, get_localized_name, Language, Locale};
use opldb::fields::{self, Equipment, Points, WeightKg};
use opldb::{self, Entry};

/// The context object passed to `templates/lifter.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: String,
    pub localized_name: &'a str,
    pub lifter: &'a opldb::Lifter,
    pub lifter_sex: &'a str,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: opldb::WeightUnits,

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
    ) -> PersonalBestsRow<'db> {
        let units = locale.units;
        let format = locale.number_format;

        PersonalBestsRow {
            equipment: equipment,
            squat: squat.map(|kg| kg.as_type(units).in_format(format)),
            bench: bench.map(|kg| kg.as_type(units).in_format(format)),
            deadlift: deadlift.map(|kg| kg.as_type(units).in_format(format)),
            total: total.map(|kg| kg.as_type(units).in_format(format)),
            wilks: wilks.map(|pt| pt.in_format(format)),
        }
    }
}

/// A row in the meet results table.
#[derive(Serialize)]
pub struct MeetResultsRow<'a> {
    pub place: String,
    pub federation: &'a fields::Federation,
    pub date: String,
    pub country: &'a str,
    pub state: Option<&'a str>,
    pub meet_name: &'a str,
    pub meet_path: &'a str,
    pub division: Option<&'a str>,
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

impl<'a> MeetResultsRow<'a> {
    fn from(
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
            wilks: entry.wilks.in_format(number_format),
        }
    }
}

/// Helper function to isolate all the best-calculation logic.
fn calculate_bests<'db>(
    locale: &'db Locale,
    entries: &Vec<&Entry>,
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
        ));
    }

    rows
}

impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        locale: &'a Locale,
        lifter_id: u32,
    ) -> Context<'a> {
        let lifter = opldb.get_lifter(lifter_id);

        // Get a list of the entries for this lifter, oldest entries first.
        let mut entries = opldb.get_entries_for_lifter(lifter_id);
        entries.sort_unstable_by_key(|e| &opldb.get_meet(e.meet_id).date);

        let bests = calculate_bests(&locale, &entries);

        let lifter_sex = locale.strings.translate_sex(entries[0].sex);

        // Display the meet results, most recent first.
        let meet_results = entries
            .into_iter()
            .map(|e| MeetResultsRow::from(opldb, locale, e))
            .rev()
            .collect();

        Context {
            page_title: format!("{}", lifter.name),
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            localized_name: get_localized_name(&lifter, locale.language),
            lifter: lifter,
            lifter_sex: lifter_sex,
            bests: bests,
            meet_results: meet_results,
        }
    }
}
