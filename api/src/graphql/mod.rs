//! Exposes a GraphQL interface over the OplDb.

use crate::ManagedOplDb;
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

mod entry;
pub use entry::Entry;

pub mod gqltypes;

mod lifter;
pub use lifter::Lifter;

mod meet;
pub use meet::{Meet, MeetFilter, MeetOrderBy};

/// Mark that ManagedOplDb is a valid Context for a GraphQL query.
impl juniper::Context for ManagedOplDb {}

/// A read-only schema over the OplDb.
pub type Schema =
    RootNode<'static, Query, EmptyMutation<ManagedOplDb>, EmptySubscription<ManagedOplDb>>;

/// Instantiates a new [Schema].
pub fn new_schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<ManagedOplDb>::new(),
        EmptySubscription::<ManagedOplDb>::new(),
    )
}

/// The query root.
pub struct Query;

#[graphql_object(context = ManagedOplDb)]
impl Query {
    /// Reports the current API version.
    fn apiVersion() -> &str {
        "beta"
    }

    /// Looks up a lifter by their unique username.
    fn lifter(db: &ManagedOplDb, username: String) -> FieldResult<Lifter> {
        let id: u32 =
            db.0.get_lifter_id(&username)
                .ok_or("Username does not exist")?;
        Ok(Lifter(id))
    }

    /// Looks up a meet by its unique path.
    fn meet(db: &ManagedOplDb, path: String) -> FieldResult<Meet> {
        let id: u32 = db.0.get_meet_id(&path).ok_or("Meet path does not exist")?;
        Ok(Meet(id))
    }

    /// Looks up a range of meets by a filter.
    fn meets(
        db: &ManagedOplDb,
        filter: Option<MeetFilter>,
        order_by: Option<MeetOrderBy>,
        limit: i32,
    ) -> FieldResult<Vec<Meet>> {
        if !(1..=100).contains(&limit) {
            return Err("The limit must be between 1 and 100".into());
        }
        let limit = limit as usize;

        // Without sorting or filtering, we can optimize harder.
        if filter.is_none() && order_by.is_none() {
            return Ok(db
                .0
                .get_meets()
                .iter()
                .enumerate()
                .take(limit)
                .map(|(id, _meet)| Meet(id as u32))
                .collect());
        }

        Err("query::meets() is unimplemented".into())
    }
}
