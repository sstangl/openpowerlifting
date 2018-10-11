//! Logic for the display of the records page, like a rankings summary.

use opltypes::*;

use std::cmp::Ordering;
use std::ffi::OsStr;
use std::path;
use std::str::FromStr;

use langpack::{self, get_localized_name, Language, Locale, LocalizeNumber};
use opldb::{algorithms, Entry, Lifter, Meet, OplDb};
use pages::selection::*;

/// Query selection descriptor, corresponding to HTML widgets.
#[derive(Copy, Clone, PartialEq, Serialize)]
pub struct RecordsSelection {
    pub equipment: EquipmentSelection,
    pub federation: FederationSelection,
    pub sex: SexSelection,
    pub classkind: ClassKindSelection,
    pub ageclass: AgeClassSelection,
    pub year: YearSelection,
}

impl Default for RecordsSelection {
    fn default() -> RecordsSelection {
        RecordsSelection {
            equipment: EquipmentSelection::RawAndWraps,
            federation: FederationSelection::AllFederations,
            sex: SexSelection::Men,
            classkind: ClassKindSelection::Traditional,
            ageclass: AgeClassSelection::AllAges,
            year: YearSelection::AllYears,
        }
    }
}

impl RecordsSelection {
    /// Converts a RecordSelection to a Selection.
    pub fn to_full_selection(&self) -> Selection {
        Selection {
            equipment: self.equipment,
            federation: self.federation,
            sex: self.sex,
            ageclass: self.ageclass,
            year: self.year,
            ..Selection::default()
        }
    }

    /// Translates a URL path to a RecordSelection.
    pub fn from_path(p: &path::Path) -> Result<Self, ()> {
        let mut ret = RecordsSelection::default();

        // Disallow empty path components.
        if let Some(s) = p.to_str() {
            if s.contains("//") {
                return Err(());
            }
        } else {
            // Failed parsing UTF-8.
            return Err(());
        }

        // Prevent fields from being overwritten or redundant.
        let mut parsed_equipment: bool = false;
        let mut parsed_sex: bool = false;
        let mut parsed_federation: bool = false;
        let mut parsed_classkind: bool = false;
        let mut parsed_ageclass: bool = false;
        let mut parsed_year: bool = false;

        // Iterate over each path component, attempting to determine
        // what kind of data it is.
        for segment in p
            .ancestors()
            .filter_map(|a| a.file_name().and_then(OsStr::to_str))
        {
            // Check whether this is equipment information.
            if let Ok(e) = segment.parse::<EquipmentSelection>() {
                if parsed_equipment {
                    return Err(());
                }
                ret.equipment = e;
                parsed_equipment = true;
            // Check whether this is federation information.
            } else if let Ok(f) = segment.parse::<FederationSelection>() {
                if parsed_federation {
                    return Err(());
                }
                ret.federation = f;
                parsed_federation = true;
            // Check whether this is sex information.
            } else if let Ok(s) = segment.parse::<SexSelection>() {
                if parsed_sex {
                    return Err(());
                }
                ret.sex = s;
                parsed_sex = true;
            // Check whether this is class kind information.
            } else if let Ok(k) = segment.parse::<ClassKindSelection>() {
                if parsed_classkind {
                    return Err(());
                }
                ret.classkind = k;
                parsed_classkind = true;
            // Check whether this is age class information.
            } else if let Ok(c) = segment.parse::<AgeClassSelection>() {
                if parsed_ageclass {
                    return Err(());
                }
                ret.ageclass = c;
                parsed_ageclass = true;
            // Check whether this is year information.
            } else if let Ok(y) = segment.parse::<YearSelection>() {
                if parsed_year {
                    return Err(());
                }
                ret.year = y;
                parsed_year = true;
            // Unknown string, therefore malformed URL.
            } else {
                return Err(());
            }
        }

        Ok(ret)
    }
}

/// Selects what kind of weight classes to use, as opposed to which specific
/// class.
#[derive(Copy, Clone, PartialEq, Serialize)]
pub enum ClassKindSelection {
    Traditional,
    IPF,
    WP,
}

impl FromStr for ClassKindSelection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // No parsing for Traditional: it's the default.
            "ipf-classes" => Ok(ClassKindSelection::IPF),
            "wp-classes" => Ok(ClassKindSelection::WP),
            _ => Err(()),
        }
    }
}

/// The context object passed to `templates/records.html.tera`.
#[derive(Serialize)]
pub struct Context<'db> {
    pub page_title: &'db str,
    pub language: Language,
    pub strings: &'db langpack::Translations,
    pub units: WeightUnits,
    pub selection: RecordsSelection,
    pub tables: Vec<Table<'db>>,
}

// General algorithm:
// 1. Get a struct (or vec? probably vec) containing each weight class.
// 2. Each one of those weight classes maps to a struct that has
//    a bunch of 3-tuples for each event.
// 3. Filter down to the entries in the category.
// 4. For each entry, figure out what weightclass, and call some integrate()
//    method on that struct to see whether this entry displaces any others.

/// Collects a single record.
///
/// Since this is owned by a RecordCollector, it collects a single record
/// in a single weightclass.
pub struct SingleRecordCollector<'db> {
    /// Remembers the top Entries during iteration.
    pub accumulator: [Option<&'db Entry>; 3],
}

impl<'db> Default for SingleRecordCollector<'db> {
    fn default() -> SingleRecordCollector<'db> {
        SingleRecordCollector {
            accumulator: [None; 3],
        }
    }
}

impl<'db> SingleRecordCollector<'db> {
    /// Maybe sort this `Entry` into the `accumulator`.
    pub fn integrate<F>(&mut self, meets: &'db [Meet], entry: &'db Entry, compare: &F)
    where
        F: Fn(&[Meet], &Entry, &Entry) -> Ordering,
    {
        let last = self.accumulator.len() - 1;

        // The accumulator is maintained in sorted order.
        //
        // The incoming entry is compared to the last element in the accumulator,
        // which is the Nth-highest-seen value. If it compares favorably, then
        // it replaces that Entry, and the accumulator is re-sorted.
        if self.accumulator[last]
            .map_or(true, |e| compare(meets, entry, e) == Ordering::Less)
        {
            // This entry matched.
            // Since each lifter is only to be counted once in each category,
            // scan through the accumulator and look to replace an existing entry.
            let same_lifter: Option<usize> = self
                .accumulator
                .iter()
                .position(|opt| opt.map_or(false, |e| e.lifter_id == entry.lifter_id));
            match same_lifter {
                None => {
                    self.accumulator[last] = Some(entry);
                }
                Some(pos) => {
                    let orig = &self.accumulator[pos].unwrap();
                    // Only replace the lifter's entry if this one is better.
                    if compare(meets, entry, orig) == Ordering::Less {
                        self.accumulator[pos] = Some(entry);
                    }
                }
            };

            // Always maintain sorted order.
            self.accumulator.sort_by(|a, b| match (a, b) {
                (None, None) => Ordering::Equal,
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (Some(x), Some(y)) => compare(meets, x, y),
            });
        }
    }
}

/// Collects records in a specific weightclass.
struct RecordCollector<'db> {
    /// The official name of this weightclass.
    pub weightclass_name: WeightClassKg,

    /// The minimum weight of this class, exclusive.
    class_min_exclusive: WeightKg,
    /// The maximum weight of this class, inclusive.
    class_max_inclusive: WeightKg,

    // List of all the records we maintain for a given weightclass.
    pub fullpower_squat: SingleRecordCollector<'db>,
    pub fullpower_bench: SingleRecordCollector<'db>,
    pub fullpower_deadlift: SingleRecordCollector<'db>,
    pub fullpower_total: SingleRecordCollector<'db>,
    pub any_squat: SingleRecordCollector<'db>,
    pub any_bench: SingleRecordCollector<'db>,
    pub any_deadlift: SingleRecordCollector<'db>,
}

impl<'db> RecordCollector<'db> {
    pub fn new(
        weightclass_name: WeightClassKg,
        class_min_exclusive: WeightKg,
        class_max_inclusive: WeightKg,
    ) -> RecordCollector<'db> {
        RecordCollector {
            weightclass_name,
            class_min_exclusive,
            class_max_inclusive,

            fullpower_squat: SingleRecordCollector::default(),
            fullpower_bench: SingleRecordCollector::default(),
            fullpower_deadlift: SingleRecordCollector::default(),
            fullpower_total: SingleRecordCollector::default(),
            any_squat: SingleRecordCollector::default(),
            any_bench: SingleRecordCollector::default(),
            any_deadlift: SingleRecordCollector::default(),
        }
    }

    /// Whether the given Entry is in the weight class this RecordCollector
    /// covers.
    #[inline]
    pub fn entry_in_class(&self, entry: &Entry) -> bool {
        // If bodyweight exists, just go by bodyweight.
        if entry.bodyweightkg.is_non_zero() {
            return entry.bodyweightkg > self.class_min_exclusive
                && entry.bodyweightkg <= self.class_max_inclusive;
        }

        // Otherwise, check for a SHW category with no recorded bodyweight.
        if self.class_max_inclusive == WeightKg::max_value() {
            // Does the minimum weight of the SHW category fit here?
            if let WeightClassKg::Over(w) = entry.weightclasskg {
                return w >= self.class_min_exclusive;
            }
        }

        false
    }

    pub fn integrate(&mut self, meets: &'db [Meet], entry: &'db Entry) {
        debug_assert!(self.entry_in_class(entry));

        if entry.event.is_full_power() {
            self.fullpower_squat
                .integrate(meets, entry, &algorithms::cmp_squat);
            self.fullpower_bench
                .integrate(meets, entry, &algorithms::cmp_bench);
            self.fullpower_deadlift
                .integrate(meets, entry, &algorithms::cmp_deadlift);
            self.fullpower_total
                .integrate(meets, entry, &algorithms::cmp_total);
        }

        if entry.event.has_squat() {
            self.any_squat
                .integrate(meets, entry, &algorithms::cmp_squat);
        }
        if entry.event.has_bench() {
            self.any_bench
                .integrate(meets, entry, &algorithms::cmp_bench);
        }
        if entry.event.has_deadlift() {
            self.any_deadlift
                .integrate(meets, entry, &algorithms::cmp_deadlift);
        }
    }
}

fn make_collectors<'db>(
    sex: SexSelection,
    classkind: ClassKindSelection,
) -> Vec<RecordCollector<'db>> {
    let classes = match classkind {
        // Traditional classes.
        ClassKindSelection::Traditional => {
            if sex == SexSelection::Men {
                vec![
                    WeightClassSelection::T52,
                    WeightClassSelection::T56,
                    WeightClassSelection::T60,
                    WeightClassSelection::T67_5,
                    WeightClassSelection::T75,
                    WeightClassSelection::T82_5,
                    WeightClassSelection::T90,
                    WeightClassSelection::T100,
                    WeightClassSelection::T110,
                    WeightClassSelection::T125,
                    WeightClassSelection::T140,
                    WeightClassSelection::TOver140,
                ]
            } else {
                vec![
                    WeightClassSelection::T44,
                    WeightClassSelection::T48,
                    WeightClassSelection::T52,
                    WeightClassSelection::T56,
                    WeightClassSelection::T60,
                    WeightClassSelection::T67_5,
                    WeightClassSelection::T75,
                    WeightClassSelection::T82_5,
                    WeightClassSelection::T90,
                    WeightClassSelection::TOver90,
                ]
            }
        }

        // IPF new-fangled classes.
        ClassKindSelection::IPF => {
            if sex == SexSelection::Men {
                vec![
                    WeightClassSelection::IpfM53,
                    WeightClassSelection::IpfM59,
                    WeightClassSelection::IpfM66,
                    WeightClassSelection::IpfM74,
                    WeightClassSelection::IpfM83,
                    WeightClassSelection::IpfM93,
                    WeightClassSelection::IpfM105,
                    WeightClassSelection::IpfM120,
                    WeightClassSelection::IpfMOver120,
                ]
            } else {
                vec![
                    WeightClassSelection::IpfF43,
                    WeightClassSelection::IpfF47,
                    WeightClassSelection::IpfF52,
                    WeightClassSelection::IpfF57,
                    WeightClassSelection::IpfF63,
                    WeightClassSelection::IpfF72,
                    WeightClassSelection::IpfF84,
                    WeightClassSelection::IpfFOver84,
                ]
            }
        }

        // World Powerlifting's not-IPF classes.
        ClassKindSelection::WP => {
            if sex == SexSelection::Men {
                vec![
                    WeightClassSelection::WpM62,
                    WeightClassSelection::WpM69,
                    WeightClassSelection::WpM77,
                    WeightClassSelection::WpM85,
                    WeightClassSelection::WpM94,
                    WeightClassSelection::WpM105,
                    WeightClassSelection::WpM120,
                    WeightClassSelection::WpMOver120,
                ]
            } else {
                vec![
                    WeightClassSelection::WpF48,
                    WeightClassSelection::WpF53,
                    WeightClassSelection::WpF58,
                    WeightClassSelection::WpF64,
                    WeightClassSelection::WpF72,
                    WeightClassSelection::WpF84,
                    WeightClassSelection::WpF100,
                    WeightClassSelection::WpFOver100,
                ]
            }
        }
    };

    classes
        .iter()
        .map(|c| {
            let (min, max) = c.to_bounds();
            RecordCollector::new(c.to_weightclasskg(), min, max)
        })
        .collect()
}

fn find_records<'db>(
    opldb: &'db OplDb,
    sel: &RecordsSelection,
) -> Vec<RecordCollector<'db>> {
    // Get a list of all entries corresponding to the selection.
    let indices = algorithms::get_entry_indices_for(&sel.to_full_selection(), opldb);

    // Build a vector of structs that can remember records.
    let mut collectors = make_collectors(sel.sex, sel.classkind);
    let meets = opldb.get_meets();

    // Mapping indices to entries, run the collectors over each Entry.
    for &index in &indices.0 {
        let entry = opldb.get_entry(index);
        if entry.place.is_dq() {
            continue;
        }

        // Each entry can be in at most one weightclass.
        for collector in &mut collectors {
            if collector.entry_in_class(entry) {
                collector.integrate(meets, entry);
                break;
            }
        }
    }

    collectors
}

/// A grouping of rows under a single category.
#[derive(Serialize)]
pub struct Table<'db> {
    pub title: String,
    pub weight_column_label: &'db str,
    pub rows: Vec<RecordsRow<'db>>,
}

impl<'db> Table<'db> {
    pub fn new(title: String, weight_column_label: &'db str) -> Table<'db> {
        Table {
            title,
            weight_column_label,
            rows: vec![],
        }
    }

    /// Append the results from a SingleRecordCollector.
    pub fn append<F>(
        &mut self,
        collector: &SingleRecordCollector<'db>,
        weightclass: langpack::LocalizedWeightClassAny,
        opldb: &'db OplDb,
        locale: &'db Locale,
        lift_selector: F,
    ) where
        F: Fn(&Entry) -> WeightKg,
    {
        let mut rank: u32 = 0;

        for record in collector.accumulator.iter() {
            rank += 1;

            let weightclass_display = if rank == 1 { Some(weightclass) } else { None };

            let row = match record {
                None => RecordsRow {
                    rank,
                    weightclass: weightclass_display,
                    weight_lifted: None,
                    date: None,
                    path: None,
                    federation: None,
                    localized_name: None,
                    lifter: None,
                },
                Some(entry) => {
                    let meet = opldb.get_meet(entry.meet_id);
                    let lifter = opldb.get_lifter(entry.lifter_id);

                    RecordsRow {
                        rank,
                        weightclass: weightclass_display,
                        weight_lifted: Some(
                            lift_selector(entry)
                                .as_type(locale.units)
                                .in_format(locale.number_format),
                        ),
                        date: Some(format!("{}", meet.date)),
                        path: Some(&meet.path),
                        federation: Some(meet.federation),
                        localized_name: Some(get_localized_name(
                            &lifter,
                            locale.language,
                        )),
                        lifter: Some(lifter),
                    }
                }
            };

            self.rows.push(row);
        }
    }
}

/// A row in a records table.
#[derive(Serialize)]
pub struct RecordsRow<'db> {
    pub rank: u32,
    pub weightclass: Option<langpack::LocalizedWeightClassAny>,
    pub weight_lifted: Option<langpack::LocalizedWeightAny>,

    pub date: Option<String>,
    pub path: Option<&'db str>,
    pub federation: Option<Federation>,

    pub localized_name: Option<&'db str>,
    pub lifter: Option<&'db Lifter>,
}

fn prettify_records<'db>(
    records: Vec<RecordCollector<'db>>,
    opldb: &'db OplDb,
    locale: &'db Locale,
) -> Vec<Table<'db>> {
    let strings = &locale.strings;

    let squat_str = &strings.columns.squat;
    let bench_str = &strings.columns.bench;
    let deadlift_str = &strings.columns.deadlift;
    let total_str = &strings.columns.total;

    let full_power = &strings.selectors.event.full_power;
    let all = &strings.selectors.event.all;

    let fullpower_squat_str = format!("{} ({})", strings.columns.squat, full_power);
    let mut fullpower_squat = Table::new(fullpower_squat_str, squat_str);

    let fullpower_bench_str = format!("{} ({})", strings.columns.bench, full_power);
    let mut fullpower_bench = Table::new(fullpower_bench_str, bench_str);

    let fullpower_deadlift_str = format!("{} ({})", strings.columns.deadlift, full_power);
    let mut fullpower_deadlift = Table::new(fullpower_deadlift_str, deadlift_str);

    let mut fullpower_total = Table::new(strings.columns.total.to_string(), total_str);

    let any_squat_str = format!("{} ({})", strings.columns.squat, all);
    let mut any_squat = Table::new(any_squat_str, squat_str);

    let any_bench_str = format!("{} ({})", strings.columns.bench, all);
    let mut any_bench = Table::new(any_bench_str, bench_str);

    let any_deadlift_str = format!("{} ({})", strings.columns.deadlift, all);
    let mut any_deadlift = Table::new(any_deadlift_str, deadlift_str);

    // Collectors are ordered by weight class, ascending.
    for collector in records {
        let class = collector
            .weightclass_name
            .as_type(locale.units)
            .in_format(locale.number_format);

        fullpower_squat.append(
            &collector.fullpower_squat,
            class,
            opldb,
            locale,
            |e: &Entry| e.highest_squatkg(),
        );
        fullpower_bench.append(
            &collector.fullpower_bench,
            class,
            opldb,
            locale,
            |e: &Entry| e.highest_benchkg(),
        );
        fullpower_deadlift.append(
            &collector.fullpower_deadlift,
            class,
            opldb,
            locale,
            |e: &Entry| e.highest_deadliftkg(),
        );
        fullpower_total.append(
            &collector.fullpower_total,
            class,
            opldb,
            locale,
            |e: &Entry| e.totalkg,
        );
        any_squat.append(&collector.any_squat, class, opldb, locale, |e: &Entry| {
            e.highest_squatkg()
        });
        any_bench.append(&collector.any_bench, class, opldb, locale, |e: &Entry| {
            e.highest_benchkg()
        });
        any_deadlift.append(
            &collector.any_deadlift,
            class,
            opldb,
            locale,
            |e: &Entry| e.highest_deadliftkg(),
        );
    }

    // Defines the printed order.
    vec![
        fullpower_squat,
        any_squat,
        fullpower_bench,
        any_bench,
        fullpower_deadlift,
        any_deadlift,
        fullpower_total,
    ]
}

impl<'db> Context<'db> {
    pub fn new(
        opldb: &'db OplDb,
        locale: &'db Locale,
        selection: &RecordsSelection,
    ) -> Context<'db> {
        let records = find_records(opldb, selection);
        let tables = prettify_records(records, opldb, locale);

        Context {
            page_title: &locale.strings.header.records,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            selection: *selection,
            tables,
        }
    }
}
