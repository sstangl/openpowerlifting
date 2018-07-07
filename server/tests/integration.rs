//! Suite of data integration tests on the compiled database.
//!
//! Not sure how to break this up across files for the moment,
//! so just keeping with a super-generic name.

extern crate server;

use server::opldb::fields::*;
use server::pages::selection::*;

mod common;

/// Checks that all federations known to be fully-tested are
/// marked as "Tested=Yes" in the compiled database.
///
/// If this test fails, the places to check are `scripts/compile`
/// and `src/opldb/fields/federation.rs`.
#[test]
fn tested_federations_are_marked_tested() {
    let db = common::db();
    let meets = db.get_meets();
    let metafed = server::opldb::fields::MetaFederation::AllTested;

    for entry in db.get_entries() {
        if metafed.contains(&entry, &meets) {
            assert_eq!(
                true,
                entry.tested,
                "The Federation {} is part of MetaFederation::AllTested, \
                 but isn't part of TESTED_FEDERATIONS in `scripts/compile`",
                db.get_meet(entry.meet_id).federation
            );
        }
    }
}

/// Checks that the sorting algorithm doesn't include any entries with
/// disqualified or empty values in the category being sorted.
#[test]
fn sorts_only_include_valid_entries() {
    let db = common::db();
    let cache = db.get_static_cache();

    // Use a sort that isn't fully pre-cached.
    let mut selection = Selection::new_default();
    selection.federation = FederationSelection::One(Federation::RPS);
    selection.sort = SortSelection::BySquat;
    let rankings = cache.get_full_sorted_uniqued(&selection, &db);
    for idx in rankings.0.iter() {
        let entry = db.get_entry(*idx);
        assert!(entry.highest_squatkg() > WeightKg(0));
        assert!(!entry.place.is_dq());
    }

    selection = Selection::new_default();
    selection.federation = FederationSelection::One(Federation::RPS);
    selection.sort = SortSelection::ByBench;
    let rankings = cache.get_full_sorted_uniqued(&selection, &db);
    for idx in rankings.0.iter() {
        let entry = db.get_entry(*idx);
        assert!(entry.highest_benchkg() > WeightKg(0));
        assert!(!entry.place.is_dq());
    }

    selection = Selection::new_default();
    selection.federation = FederationSelection::One(Federation::RPS);
    selection.sort = SortSelection::ByDeadlift;
    let rankings = cache.get_full_sorted_uniqued(&selection, &db);
    for idx in rankings.0.iter() {
        let entry = db.get_entry(*idx);
        assert!(entry.highest_deadliftkg() > WeightKg(0));
        assert!(!entry.place.is_dq());
    }

    selection = Selection::new_default();
    selection.federation = FederationSelection::One(Federation::RPS);
    selection.sort = SortSelection::ByTotal;
    let rankings = cache.get_full_sorted_uniqued(&selection, &db);
    for idx in rankings.0.iter() {
        let entry = db.get_entry(*idx);
        assert!(entry.totalkg > WeightKg(0));
        assert!(!entry.place.is_dq());
    }

    selection = Selection::new_default();
    selection.federation = FederationSelection::One(Federation::RPS);
    selection.sort = SortSelection::ByWilks;
    let rankings = cache.get_full_sorted_uniqued(&selection, &db);
    for idx in rankings.0.iter() {
        let entry = db.get_entry(*idx);
        assert!(entry.wilks > Points(0));
        assert!(!entry.place.is_dq());
    }

    // Also test the fully-statically-cached variants.
    selection = Selection::new_default();
    selection.sort = SortSelection::ByWilks;
    let rankings = cache.get_full_sorted_uniqued(&selection, &db);
    for idx in rankings.0.iter() {
        let entry = db.get_entry(*idx);
        assert!(entry.wilks > Points(0));
        assert!(!entry.place.is_dq());
    }

    selection = Selection::new_default();
    selection.sort = SortSelection::BySquat;
    let rankings = cache.get_full_sorted_uniqued(&selection, &db);
    for idx in rankings.0.iter() {
        let entry = db.get_entry(*idx);
        assert!(entry.highest_squatkg() > WeightKg(0));
        assert!(!entry.place.is_dq());
    }
}

/// Tests that meet.num_unique_lifters looks correct.
#[test]
fn num_unique_lifters_is_valid() {
    let db = common::db();

    // Test a meet where each lifter only competed in one division.
    let meet_id = db.get_meet_id("bb/1001").unwrap();
    assert_eq!(db.get_meet(meet_id).num_unique_lifters, 28);

    // Test a meet where some lifters competed more than once.
    // Each lifter should only be counted once.
    let meet_id = db.get_meet_id("spf/1744").unwrap();
    assert_eq!(db.get_meet(meet_id).num_unique_lifters, 59);
}
