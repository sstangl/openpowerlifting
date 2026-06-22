//! Implements heuristic-based disambiguation.
//!
//! Disambiguation distinguishes lifters with the same name by grouping them
//! and applying a label to each group.

use opltypes::*;
use rustc_hash::FxHashMap;

use std::fmt;

/// Accessor for all disambiguation information needed for a single entry.
///
/// Having this as a trait enables it to work with both the format in the Checker
/// and in this crate's test suite without requiring an intermediary data format.
pub trait DisambigEntry {
    // Meet information.
    fn federation(&self) -> Federation;
    fn date(&self) -> Date;
    fn meet_country(&self) -> Country;

    // Entry information.
    fn username(&self) -> Username;
    fn sex(&self) -> Sex;
    fn birth_date(&self) -> Option<Date>;
    fn birth_year(&self) -> Option<u32>;
    fn age(&self) -> Age;
}

/// A disambiguation group ID assigned to a particular entry.
///
/// All entries in the same group are assigned the same ID. This ID is not intended
/// for output: it is an internal designation only for purposes of reporting.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DisambigId(u32);

impl fmt::Debug for DisambigId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The extra quotes help it line up in GroupAssertError output.
        write!(f, "\"{}\"", self.0)
    }
}

/// An arbitrary heuristic-based scoring system for distance between entries.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Score(i32);

impl Score {
    /// No negative information was gained, also no positive information.
    const NO_CONTRADICTION: Score = Score(0);
    /// Strong negative information was gained.
    const DEFINITE_MISMATCH: Score = Score(i32::MIN);

    const MODERATELY_POSITIVE: Score = Score(100);
    const STRONGLY_POSITIVE: Score = Score(1000);
}

impl std::ops::Add<Score> for Score {
    type Output = Score;

    fn add(self, rhs: Score) -> Self::Output {
        Score(self.0.saturating_add(rhs.0))
    }
}

impl std::ops::AddAssign<Score> for Score {
    fn add_assign(&mut self, rhs: Score) {
        self.0 = self.0.saturating_add(rhs.0)
    }
}

/// Given a list of rows, returns a list of corresponding group assignations.
pub fn disambiguate<E: DisambigEntry>(rows: &[E]) -> Vec<DisambigId> {
    // The next disambiguation ID to be assigned to a new group.
    let mut next_disambig_id = DisambigId(1);

    // First, form groups based on any manually-provided disambiguation data.
    let mut groups: FxHashMap<DisambigId, Vec<usize>> = {
        // A hashmap is used instead of a vec for allocation safety.
        // Otherwise, disambiguation numbers would correspond to allocation size.
        let mut map: FxHashMap<DisambigId, Vec<usize>> = FxHashMap::default();

        for (i, row) in rows.iter().enumerate() {
            if let (_base, Some(variant)) = row.username().to_parts() {
                next_disambig_id = DisambigId(next_disambig_id.0.max(variant.saturating_add(1)));
                map.entry(DisambigId(variant)).or_default().push(i);
            }
        }
        map
    };

    let mut acc = Vec::with_capacity(rows.len());

    // For each entry that has no variant, determine the group it most
    // closely matches, or create a new group for it. This is O(n^2) but n is low.
    for (i, row) in rows.iter().enumerate() {
        if let Some(variant) = row.username().to_parts().1 {
            acc.push(DisambigId(variant)); // Manually disambiguated.
        } else {
            if let Some(best_group) = find_best_group(row, rows, &groups) {
                acc.push(best_group);
                groups.entry(best_group).or_default().push(i);
            } else {
                acc.push(next_disambig_id);
                groups.entry(next_disambig_id).or_default().push(i);
                next_disambig_id = DisambigId(next_disambig_id.0.saturating_add(1));
            }
        }
    }

    acc
}

/// Given a row and a list of groups, determine with which group the row belongs, if any.
///
/// A return value of `None` signifies that the entry should form a new group.
fn find_best_group<E: DisambigEntry>(
    row: &E,
    rows: &[E],
    groups: &FxHashMap<DisambigId, Vec<usize>>,
) -> Option<DisambigId> {
    let mut best_score = Score::default();
    let mut best_group_id: Option<DisambigId> = None;

    for (group_id, group_rows) in groups.iter() {
        for group_row in group_rows {
            let score = calculate_heuristic_score(row, &rows[*group_row]);
            if score > Score::default() && score > best_score {
                best_score = score;
                best_group_id = Some(*group_id);
            }
        }
    }
    best_group_id
}

/// The heart of the heuristic-based disambiguation logic.
fn calculate_heuristic_score<E: DisambigEntry>(a: &E, b: &E) -> Score {
    let mut score = Score::default();
    score += score_federation(a, b);
    score += score_date(a, b);
    score += score_location(a, b);
    score += score_sex(a, b);
    score += score_age(a, b);
    score
}

fn score_federation<E: DisambigEntry>(a: &E, b: &E) -> Score {
    let (a_fed, b_fed) = (a.federation(), b.federation());

    // Competing in exactly the same federation provides mildly positive evidence.
    if a_fed == b_fed {
        return Score(50);
    }

    // If the two federations have the same sanctioning body, that's mildly positive.
    if a_fed.sanctioning_body(a.date()).is_some()
        && a_fed.sanctioning_body(a.date()) == b_fed.sanctioning_body(b.date())
    {
        return Score(50);
    }

    // Otherwise the federations are distinct, mildly negative.
    Score(-20)
}

fn score_date<E: DisambigEntry>(a: &E, b: &E) -> Score {
    let (a_date, b_date) = (a.date(), b.date());
    let earlier = a_date.min(b_date);
    let later = a_date.max(b_date);

    // Meets further apart provide increasingly contradictory evidence.
    let distance_years = later.year().saturating_sub(earlier.year());
    Score(-20 * distance_years as i32)
}

fn score_location<E: DisambigEntry>(a: &E, b: &E) -> Score {
    // If in the same country, mildly positive evidence.
    if a.meet_country() == b.meet_country() {
        return Score(50);
    }

    if a.meet_country().contains(b.meet_country()) || b.meet_country().contains(a.meet_country()) {
        return Score(50);
    }

    // If one of the federations is international, ignore this.
    if a.federation().home_country().is_none() || b.federation().home_country().is_none() {
        return Score::NO_CONTRADICTION;
    }

    // TODO: US state logic, maybe with distances.

    // Otherwise, in different countries and non-international.
    Score(-50)
}

fn score_sex<E: DisambigEntry>(a: &E, b: &E) -> Score {
    // A match is not strong positive evidence.
    if a.sex() == b.sex() {
        return Score::NO_CONTRADICTION;
    }

    // Lifters may compete freely in the Mx class. It provides no information.
    if a.sex() == Sex::Mx || b.sex() == Sex::Mx {
        return Score::NO_CONTRADICTION;
    }

    // A mismatch otherwise is strong negative information.
    Score::DEFINITE_MISMATCH
}

fn score_age<E: DisambigEntry>(a: &E, b: &E) -> Score {
    // Do both have birthdates?
    if let (Some(a_birth_date), Some(b_birth_date)) = (a.birth_date(), b.birth_date()) {
        if a_birth_date == b_birth_date {
            return Score::STRONGLY_POSITIVE;
        } else {
            return Score::DEFINITE_MISMATCH;
        }
    }

    // Do both have birth years?
    if let (Some(a_birth_year), Some(b_birth_year)) = (a.birth_year(), b.birth_year()) {
        if a_birth_year == b_birth_year {
            return Score::MODERATELY_POSITIVE;
        } else {
            return Score::DEFINITE_MISMATCH;
        }
    }

    // Do both contain age information?
    // If so, that can be reduced down to BirthYearRanges and intersected.
    if let (Some(a_byr), Some(b_byr)) = (
        a.age().to_birthyearrange(a.date()),
        b.age().to_birthyearrange(b.date()),
    ) {
        if a_byr.intersect(b_byr) == BirthYearRange::default() {
            return Score::DEFINITE_MISMATCH;
        } else {
            return Score::MODERATELY_POSITIVE;
        }
    }

    Score(0)
}
