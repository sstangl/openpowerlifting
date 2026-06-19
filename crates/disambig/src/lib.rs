//! Implements heuristic-based disambiguation.
//!
//! Disambiguation distinguishes lifters with the same name by grouping them
//! and applying a label to each group.

use opltypes::*;

use std::fmt;

/// Accessor for all disambigutaion information needed for a single entry.
///
/// Having this as a trait enables it to work with both the format in the Checker
/// and in this crate's test suite without requiring an intermediary data format.
pub trait DisambigEntry {
    fn sex(&self) -> Sex;
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DisambigId(u32);

impl fmt::Debug for DisambigId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The extra quotes help it line up in GroupAssertError output.
        write!(f, "\"{}\"", self.0)
    }
}

/// Given a list of rows, returns a list of corresponding group assignations.
pub fn disambiguate<E: DisambigEntry>(rows: &[E]) -> Vec<DisambigId> {
    let mut acc = Vec::new();

    // For the moment, report that every entry is in the same group.
    for _ in 0..rows.len() {
        acc.push(DisambigId(0));
    }
    acc
}
