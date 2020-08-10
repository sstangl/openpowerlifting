//! The Meet object, expressed for GraphQL.

use crate::graphql::{Entry, Lifter};
use crate::ManagedOplDb;

/// Helper for getting the OplDb.
macro_rules! db {
    ($executor:ident) => {
        &$executor.context().0
    };
}

/// Helper for looking up a [opldb::Meet].
macro_rules! meet {
    ($self: ident, $executor:ident) => {
        $executor.context().0.get_meet($self.0)
    };
}

/// A unique meet in the database.
///
/// Meets are uniquely identified by path.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meet(pub u32);

graphql_object!(Meet: ManagedOplDb |&self| {
    /// The path that uniquely identifies each meet.
    field path(&executor) -> &str {
        meet!(self, executor).path.as_str()
    }

    /// The federation that hosted the meet.
    field federation(&executor) -> String {
        format!("{}", meet!(self, executor).federation)
    }

    /// The topmost federation under which the sanction fell.
    field parent_federation(&executor) -> Option<String> {
        let meet = meet!(self, executor);
        meet.federation.sanctioning_body(meet.date).map(|f| format!("{}", f))
    }

    // TODO: Date
    // TODO: Country

    /// The state/province in which the meet was held.
    field state(&executor) -> Option<&str> {
        meet!(self, executor).state.as_deref()
    }

    /// The town/city in which the meet was held.
    field town(&executor) -> Option<&str> {
        meet!(self, executor).town.as_deref()
    }

    /// The name of the meet.
    field name(&executor) -> &str {
        meet!(self, executor).name.as_str()
    }

    /// Counts how many entries were recorded for the meet.
    field num_entries(&executor) -> i32 {
        db!(executor).get_entry_ids_for_meet(self.0).len() as i32
    }

    /// Gets a list of all entries from the meet.
    field entries(&executor) -> Vec<Entry> {
        db!(executor).get_entry_ids_for_meet(self.0)
            .into_iter()
            .map(|id| Entry(id))
            .collect()
    }

    /// Counts how many lifters competed in the meet.
    field num_lifters(&executor) -> i32 {
        meet!(self, executor).num_unique_lifters as i32
    }

    /// Gets a list of all lifters that competed in the meet.
    field lifters(&executor) -> Vec<Lifter> {
        db!(executor).get_lifter_ids_for_meet(self.0)
            .into_iter()
            .map(|id| Lifter(id))
            .collect()
    }
});

/// A range of integers for a filter.
///
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
