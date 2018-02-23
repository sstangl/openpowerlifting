//! Logic for each lifter's personal page.

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
    pub lifter: &'a opldb::Lifter,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: opldb::WeightUnits,

    pub meet_results: Vec<MeetResultsRow<'a>>,
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

    pub squat_is_pr: bool,
    pub bench_is_pr: bool,
    pub deadlift_is_pr: bool,
    pub total_is_pr: bool,
}

impl<'a> MeetResultsRow<'a> {
    fn from(
        opldb: &'a opldb::OplDb,
        strings: &'a langpack::Translations,
        units: opldb::WeightUnits,
        entry: &'a opldb::Entry,
        prmarker: PrMarker,
    ) -> MeetResultsRow<'a> {
        let meet: &'a opldb::Meet = opldb.get_meet(entry.meet_id);

        MeetResultsRow {
            place: format!("{}", &entry.place),
            federation: &meet.federation,
            date: format!("{}", meet.date),
            country: &meet.country,
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

            squat_is_pr: prmarker.squat_is_pr,
            bench_is_pr: prmarker.bench_is_pr,
            deadlift_is_pr: prmarker.deadlift_is_pr,
            total_is_pr: prmarker.total_is_pr,
        }
    }
}

/// A simple temporary struct to be zipped up with the entries iterator.
struct PrMarker {
    pub squat_is_pr: bool,
    pub bench_is_pr: bool,
    pub deadlift_is_pr: bool,
    pub total_is_pr: bool,
}

impl PrMarker {
    pub fn new() -> PrMarker {
        PrMarker {
            squat_is_pr: false,
            bench_is_pr: false,
            deadlift_is_pr: false,
            total_is_pr: false,
        }
    }
}

/// Given a list of Entries in sorted order (oldest first),
/// mark which lifts are PRs, taking equipment into consideration.
///
/// Weightclasses are not considered.
fn mark_prs(entries: &Vec<&opldb::Entry>) -> Vec<PrMarker> {
    let mut best_squat_raw = fields::WeightKg(0);
    let mut best_bench_raw = fields::WeightKg(0);
    let mut best_deadlift_raw = fields::WeightKg(0);
    let mut best_total_raw = fields::WeightKg(0);

    let mut best_squat_wraps = fields::WeightKg(0);
    let mut best_total_wraps = fields::WeightKg(0);

    let mut best_squat_single = fields::WeightKg(0);
    let mut best_bench_single = fields::WeightKg(0);
    let mut best_deadlift_single = fields::WeightKg(0);
    let mut best_total_single = fields::WeightKg(0);

    let mut best_squat_multi = fields::WeightKg(0);
    let mut best_bench_multi = fields::WeightKg(0);
    let mut best_deadlift_multi = fields::WeightKg(0);
    let mut best_total_multi = fields::WeightKg(0);

    let mut acc = Vec::with_capacity(entries.len());

    for i in 0..entries.len() {
        let entry = &entries[i];

        // TODO FIXME -- If the lifter competed in multiple divisions on
        // the same day, PRs should be shared across them.

        let mut prmarker = PrMarker::new();

        let squat = entry.highest_squatkg();
        let bench = entry.highest_benchkg();
        let deadlift = entry.highest_deadliftkg();

        match entry.equipment {
            fields::Equipment::Raw => {
                if squat > best_squat_raw {
                    prmarker.squat_is_pr = true;
                    best_squat_raw.0 = squat.0;
                }
                if bench > best_bench_raw {
                    prmarker.bench_is_pr = true;
                    best_bench_raw.0 = bench.0;
                }
                if deadlift > best_deadlift_raw {
                    prmarker.deadlift_is_pr = true;
                    best_deadlift_raw.0 = deadlift.0;
                }
                if entry.totalkg > best_total_raw {
                    prmarker.total_is_pr = true;
                    best_total_raw.0 = entry.totalkg.0;
                }
            }
            fields::Equipment::Wraps => {
                if squat > best_squat_wraps {
                    prmarker.squat_is_pr = true;
                    best_squat_wraps.0 = squat.0;
                }
                if bench > best_bench_raw {
                    prmarker.bench_is_pr = true;
                    best_bench_raw.0 = bench.0;
                }
                if deadlift > best_deadlift_raw {
                    prmarker.deadlift_is_pr = true;
                    best_deadlift_raw.0 = deadlift.0;
                }
                if entry.totalkg > best_total_wraps {
                    prmarker.total_is_pr = true;
                    best_total_wraps.0 = entry.totalkg.0;
                }
            }
            fields::Equipment::Single => {
                if squat > best_squat_single {
                    prmarker.squat_is_pr = true;
                    best_squat_single.0 = squat.0;
                }
                if bench > best_bench_single {
                    prmarker.bench_is_pr = true;
                    best_bench_single.0 = bench.0;
                }
                if deadlift > best_deadlift_single {
                    prmarker.deadlift_is_pr = true;
                    best_deadlift_single.0 = deadlift.0;
                }
                if entry.totalkg > best_total_single {
                    prmarker.total_is_pr = true;
                    best_total_single.0 = entry.totalkg.0;
                }
            }
            fields::Equipment::Multi => {
                if squat > best_squat_multi {
                    prmarker.squat_is_pr = true;
                    best_squat_multi.0 = squat.0;
                }
                if bench > best_bench_multi {
                    prmarker.bench_is_pr = true;
                    best_bench_multi.0 = bench.0;
                }
                if deadlift > best_deadlift_multi {
                    prmarker.deadlift_is_pr = true;
                    best_deadlift_multi.0 = deadlift.0;
                }
                if entry.totalkg > best_total_multi {
                    prmarker.total_is_pr = true;
                    best_total_multi.0 = entry.totalkg.0;
                }
            }
            fields::Equipment::Straps => {}
        };

        acc.push(prmarker);
    }

    acc
}

impl<'a> Context<'a> {
    pub fn new(
        opldb: &'a opldb::OplDb,
        language: Language,
        langinfo: &'a langpack::LangInfo,
        units: opldb::WeightUnits,
        lifter_id: u32,
    ) -> Context<'a> {
        let lifter = opldb.get_lifter(lifter_id);
        let strings = langinfo.get_translations(language);

        // Get a list of the entries for this lifter, oldest entries first.
        let mut entries = opldb.get_entries_for_lifter(lifter_id);
        entries.sort_unstable_by_key(|e| &opldb.get_meet(e.meet_id).date);

        let prmarkers = mark_prs(&entries);

        // Display the meet results, most recent first.
        let meet_results = entries
            .into_iter()
            .zip(prmarkers.into_iter())
            .map(|(e, pr)| MeetResultsRow::from(opldb, strings, units, e, pr))
            .rev()
            .collect();

        Context {
            header: HeaderContext {
                num_entries: opldb.get_entries().len() as u32,
                num_meets: opldb.get_meets().len() as u32,
            },
            language: language,
            strings: strings,
            units: units,
            lifter: lifter,
            meet_results: meet_results,
        }
    }
}
