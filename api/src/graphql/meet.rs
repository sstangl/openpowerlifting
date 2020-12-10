//! The Meet object, expressed for GraphQL.

use crate::graphql::{Entry, Lifter};
use crate::ManagedOplDb;

/// A unique meet in the database.
///
/// Meets are uniquely identified by path.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meet(pub u32);

#[graphql_object(context = ManagedOplDb)]
impl Meet {
    /// The path that uniquely identifies each meet.
    fn path(&self, db: &ManagedOplDb) -> &str {
        db.0.get_meet(self.0).path.as_str()
    }

    /// The federation that hosted the meet.
    fn federation(&self, db: &ManagedOplDb) -> String {
        format!("{}", db.0.get_meet(self.0).federation)
    }

    /// The topmost federation under which the sanction fell.
    fn parent_federation(&self, db: &ManagedOplDb) -> Option<String> {
        let meet = db.0.get_meet(self.0);
        meet.federation
            .sanctioning_body(meet.date)
            .map(|f| format!("{}", f))
    }

    // TODO: Date
    // TODO: Country

    /// The state/province in which the meet was held.
    fn state(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_meet(self.0).state.as_deref()
    }

    /// The town/city in which the meet was held.
    fn town(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_meet(self.0).town.as_deref()
    }

    /// The name of the meet.
    fn name(&self, db: &ManagedOplDb) -> &str {
        db.0.get_meet(self.0).name.as_str()
    }

    /// Counts how many entries were recorded for the meet.
    fn num_entries(&self, db: &ManagedOplDb) -> i32 {
        db.0.get_entry_ids_for_meet(self.0).len() as i32
    }

    /// Gets a list of all entries from the meet.
    fn entries(&self, db: &ManagedOplDb) -> Vec<Entry> {
        db.0.get_entry_ids_for_meet(self.0)
            .into_iter()
            .map(Entry)
            .collect()
    }

    /// Counts how many lifters competed in the meet.
    fn num_lifters(&self, db: &ManagedOplDb) -> i32 {
        db.0.get_meet(self.0).num_unique_lifters as i32
    }

    /// Gets a list of all lifters that competed in the meet.
    fn lifters(&self, db: &ManagedOplDb) -> Vec<Lifter> {
        db.0.get_lifter_ids_for_meet(self.0)
            .into_iter()
            .map(Lifter)
            .collect()
    }
}

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
#[derive(GraphQLEnum, Copy, Clone, PartialEq)]
pub enum MeetOrderBy {
    /// Whatever order the DB prefers at this time.
    Arbitrary,

    /// By date, from earliest to most recent.
    DateAsc,
    /// By date, from most recent to earliest.
    DateDesc,

    /// By number of lifters, from least to most.
    NumLiftersAsc,
    /// By number of lifters, from most to least.
    NumLiftersDesc,
}
