//! Suite of data integration tests on the compiled database.
//!
//! Not sure how to break this up across files for the moment,
//! so just keeping with a super-generic name.

extern crate server;

mod common;

/// Checks that all federations known to be fully-tested are
/// marked as "Tested=Yes" in the compiled database.
///
/// If this test fails, the places to check are `scripts/compile`
/// and `src/opldb/fields/federation.rs`.
#[test]
fn tested_federations_are_marked_tested() {
    let db = common::db();
    let metafed = server::opldb::fields::MetaFederation::AllTested;

    for entry in db.get_entries() {
        if metafed.contains(&entry, &db) {
            assert_eq!(true, entry.tested,
                       "The Federation {} is part of MetaFederation::AllTested, \
                        but isn't part of TESTED_FEDERATIONS in `scripts/compile`",
                       db.get_meet(entry.meet_id).federation);
        }
    }
}
