//! Exposes a GraphQL interface over the OplDb.

use crate::ManagedOplDb;
use juniper::{EmptyMutation, FieldResult, RootNode};

mod entry;
pub use entry::Entry;

pub mod gqltypes;

mod lifter;
pub use lifter::Lifter;

mod meet;
pub use meet::Meet;

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
        Err("query::meets() is unimplemented")?
    }
});

/// The final range is the intersection of all provided parameters.
#[derive(GraphQLInputObject)]
pub struct IntRange {
    /// All values < x.
    less_than: Option<i32>,

    /// All values <= x.
    less_than_or_equal: Option<i32>,

    /// All values > x.
    greater_than: Option<i32>,

    /// All values >= x.
    greater_than_or_equal: Option<i32>,
}

/// A filter for meet data.
#[derive(GraphQLInputObject)]
pub struct MeetFilter {
    num_entries: Option<IntRange>,
}

/// An ordering on meet data.
#[derive(GraphQLEnum)]
pub enum MeetOrderBy {
    /// By date, from earliest to most recent.
    DateAsc,
    /// By date, from most recent to earliest.
    DateDesc,
}

/// The GraphQL variant of [opltypes::Equipment].
#[derive(GraphQLEnum)]
enum Equipment {
    Raw,
    Wraps,
    SinglePly,
    MultiPly,
    Unlimited,
    Straps,
}

impl From<opltypes::Equipment> for Equipment {
    fn from(o: opltypes::Equipment) -> Equipment {
        match o {
            opltypes::Equipment::Raw => Equipment::Raw,
            opltypes::Equipment::Wraps => Equipment::Wraps,
            opltypes::Equipment::Single => Equipment::SinglePly,
            opltypes::Equipment::Multi => Equipment::MultiPly,
            opltypes::Equipment::Unlimited => Equipment::Unlimited,
            opltypes::Equipment::Straps => Equipment::Straps,
        }
    }
}
