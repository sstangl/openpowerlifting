//! Suite of data integration tests on the compiled database.
//!
//! Not sure how to break this up across files for the moment,
//! so just keeping with a super-generic name.

use opldb::algorithms;
use opldb::query::direct::*;
use opltypes::*;

mod common;

/// Checks that the sorting algorithm doesn't include any entries with
/// disqualified or empty values in the category being sorted.
#[test]
fn sorts_only_include_valid_entries() {
    let db = common::db();

    // Use a sort that isn't fully pre-cached.
    let mut query = RankingsQuery::default();
    query.filter.federation = FederationFilter::One(Federation::RPS);
    query.order_by = OrderBy::Squat;
    let rankings = algorithms::full_sorted_uniqued(&query, &db);
    for idx in rankings.0.iter() {
        let entry = db.entry(*idx);
        assert!(entry.highest_squatkg() > WeightKg::from_i32(0));
        assert!(!entry.place.is_dq());
    }

    query = RankingsQuery::default();
    query.filter.federation = FederationFilter::One(Federation::RPS);
    query.order_by = OrderBy::Bench;
    let rankings = algorithms::full_sorted_uniqued(&query, &db);
    for idx in rankings.0.iter() {
        let entry = db.entry(*idx);
        assert!(entry.highest_benchkg() > WeightKg::from_i32(0));
        assert!(!entry.place.is_dq());
    }

    query = RankingsQuery::default();
    query.filter.federation = FederationFilter::One(Federation::RPS);
    query.order_by = OrderBy::Deadlift;
    let rankings = algorithms::full_sorted_uniqued(&query, &db);
    for idx in rankings.0.iter() {
        let entry = db.entry(*idx);
        assert!(entry.highest_deadliftkg() > WeightKg::from_i32(0));
        assert!(!entry.place.is_dq());
    }

    query = RankingsQuery::default();
    query.filter.federation = FederationFilter::One(Federation::RPS);
    query.order_by = OrderBy::Total;
    let rankings = algorithms::full_sorted_uniqued(&query, &db);
    for idx in rankings.0.iter() {
        let entry = db.entry(*idx);
        assert!(entry.totalkg > WeightKg::from_i32(0));
        assert!(!entry.place.is_dq());
    }

    query = RankingsQuery::default();
    query.filter.federation = FederationFilter::One(Federation::RPS);
    query.order_by = OrderBy::Wilks;
    let rankings = algorithms::full_sorted_uniqued(&query, &db);
    for idx in rankings.0.iter() {
        let entry = db.entry(*idx);
        assert!(entry.wilks > Points::from_i32(0));
        assert!(!entry.place.is_dq());
    }

    // Also test the fully-statically-cached variants.
    query = RankingsQuery::default();
    query.order_by = OrderBy::Wilks;
    let rankings = algorithms::full_sorted_uniqued(&query, &db);
    for idx in rankings.0.iter() {
        let entry = db.entry(*idx);
        assert!(entry.wilks > Points::from_i32(0));
        assert!(!entry.place.is_dq());
    }

    query = RankingsQuery::default();
    query.order_by = OrderBy::Squat;
    let rankings = algorithms::full_sorted_uniqued(&query, &db);
    for idx in rankings.0.iter() {
        let entry = db.entry(*idx);
        assert!(entry.highest_squatkg() > WeightKg::from_i32(0));
        assert!(!entry.place.is_dq());
    }
}

/// Tests that meet.num_unique_lifters looks correct.
#[test]
fn num_unique_lifters_is_valid() {
    let db = common::db();

    // Test a meet where each lifter only competed in one division.
    let meet_id = db.meet_id("bb/1001").unwrap();
    assert_eq!(db.meet(meet_id).num_unique_lifters, 28);

    // Test a meet where some lifters competed more than once.
    // Each lifter should only be counted once.
    let meet_id = db.meet_id("spf/1744").unwrap();
    assert_eq!(db.meet(meet_id).num_unique_lifters, 59);
}
