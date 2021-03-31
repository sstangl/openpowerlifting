//! GraphQL API regression tests.

#[macro_use]
extern crate juniper;

use api::graphql;
use api::ManagedOplDb;

use opldb::OplDb;

use std::sync::Once;

static mut OPLDB_GLOBAL: Option<ManagedOplDb> = None;
static OPLDB_INIT: Once = Once::new();

fn db() -> &'static ManagedOplDb {
    const LIFTERS_CSV: &str = "../build/lifters.csv";
    const MEETS_CSV: &str = "../build/meets.csv";
    const ENTRIES_CSV: &str = "../build/entries.csv";

    unsafe {
        OPLDB_INIT.call_once(|| {
            let db = OplDb::from_csv(LIFTERS_CSV, MEETS_CSV, ENTRIES_CSV).unwrap();
            OPLDB_GLOBAL = Some(ManagedOplDb(db));
        });

        OPLDB_GLOBAL.as_ref().unwrap()
    }
}

type ExecutionError = juniper::ExecutionError<juniper::DefaultScalarValue>;

/// Executes a single GraphQL API query against the OplDb.
fn execute<'a>(
    schema: &'a api::graphql::Schema,
    query: &'a str,
) -> Result<(juniper::Value, Vec<ExecutionError>), juniper::GraphQLError<'a>> {
    juniper::execute_sync(query, None, &schema, &juniper::Variables::new(), &db())
}

/// Executes a query and asserts that it matches a given result.
fn assert_execute(query: &str, expected: juniper::Value) {
    let schema = graphql::new_schema();
    let (res, _errors) = execute(&schema, query).unwrap();
    assert_eq!(res, expected);
}

/// Look up a single meet by MeetPath.
#[test]
fn meet_lookup() {
    let query = r#"query {
        meet(path: "wrpf-usa/bob4") {
            name
        }
    }"#;
    let expected = graphql_value!({
        "meet": {
            "name": "Boss of Bosses 4",
        }
    });
    assert_execute(query, expected);
}

/// Look up a single lifter by Username.
#[test]
fn lifter_lookup() {
    let query = r#"query {
        lifter(username: "seanstangl") {
            latinName,
            cyrillicName
        }
    }"#;
    let expected = graphql_value!({
        "lifter": {
            "latinName": "Sean Stangl",
            "cyrillicName": "Шон Стангл",
        }
    });
    assert_execute(query, expected);
}
