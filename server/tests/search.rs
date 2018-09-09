//! Suite of tests for the search function on the compiled database.

extern crate server;

use server::pages::api_search::*;
use server::pages::selection::*;
use server::opldb::algorithms;

mod common;

/// Checks that basic rankings search functionality works.
#[test]
fn basic_rankings_search() {
    let db = common::db();
    let selection = Selection::default();

    // Perform the search.
    let res = search_rankings(&db, &selection, 0, "Sean Stangl");
    let row = res.next_index.unwrap();

    // Check that the result is for the specified lifter.
    let list = algorithms::get_full_sorted_uniqued(&selection, &db);
    let lifter = db.get_lifter(db.get_entry(list.0[row]).lifter_id);
    assert_eq!(lifter.name, "Sean Stangl");
}

/// Checks that searching in "Lastname Firstname" order works.
#[test]
fn backwards_name_search() {
    let db = common::db();
    let selection = Selection::default();

    // Perform the search.
    let res = search_rankings(&db, &selection, 0, "stangl sean");
    let row = res.next_index.unwrap();

    // Check that the result is for the specified lifter.
    let list = algorithms::get_full_sorted_uniqued(&selection, &db);
    let lifter = db.get_lifter(db.get_entry(list.0[row]).lifter_id);
    assert_eq!(lifter.name, "Sean Stangl");
}

/// Checks that searching by Instagram works.
#[test]
fn instagram_search() {
    let db = common::db();
    let selection = Selection::default();

    // Perform the search.
    let res = search_rankings(&db, &selection, 0, "Ferruix");
    let row = res.next_index.unwrap();

    // Check that the result is for the specified lifter.
    let list = algorithms::get_full_sorted_uniqued(&selection, &db);
    let lifter = db.get_lifter(db.get_entry(list.0[row]).lifter_id);
    assert_eq!(lifter.name, "Sean Stangl");
}
