//! Exposes a GraphQL interface over the OplDb.

use crate::ManagedOplDb;
use juniper::{EmptyMutation, FieldResult, RootNode};

mod entry;
pub use entry::Entry;

pub mod gqltypes;

mod lifter;
pub use lifter::Lifter;

mod meet;
pub use meet::{Meet, MeetFilter, MeetOrderBy};

/// Helper for getting the OplDb.
macro_rules! db {
    ($executor:ident) => {
        &$executor.context().0
    };
}

/// Mark that ManagedOplDb is a valid Context for a GraphQL query.
impl juniper::Context for ManagedOplDb {}

/// A read-only schema over the OplDb.
pub type Schema = RootNode<'static, Query, EmptyMutation<ManagedOplDb>>;

/// Instantiates a new [Schema].
pub fn new_schema() -> Schema {
    Schema::new(Query, EmptyMutation::<ManagedOplDb>::new())
}

/// The query root.
pub struct Query;
graphql_object!(Query: ManagedOplDb |&self| {
    /// Reports the current API version.
    field apiVersion() -> &str {
        "beta"
    }

    /// Looks up a lifter by their unique username.
    field lifter(&executor, username: String) -> FieldResult<Lifter> {
        let db = &executor.context().0;
        let id: u32 = db.get_lifter_id(&username).ok_or("Username does not exist")?;
        Ok(Lifter(id))
    }

    /// Looks up a meet by its unique path.
    field meet(&executor, path: String) -> FieldResult<Meet> {
        let db = &executor.context().0;
        let id: u32 = db.get_meet_id(&path).ok_or("Meet path does not exist")?;
        Ok(Meet(id))
    }

    /// Looks up a range of meets by a filter.
    field meets(
        &executor,
        filter: Option<MeetFilter>,
        order_by: Option<MeetOrderBy>,
        limit: i32,
    ) -> FieldResult<Vec<Meet>> {
        if limit < 1 || limit > 100 {
            return Err("The limit must be between 1 and 100".into());
        }
        let limit = limit as usize;

        // Without sorting or filtering, we can optimize harder.
        if filter.is_none() && order_by.is_none() {
            return Ok(db!(executor).get_meets().iter().enumerate().take(limit)
                .map(|(id, _meet)| Meet(id as u32))
                .collect());
        }

        Err("query::meets() is unimplemented".into())
    }
});
