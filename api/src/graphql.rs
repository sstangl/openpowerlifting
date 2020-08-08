//! Exposes a GraphQL interface over the OplDb.

use crate::ManagedOplDb;
use juniper::{EmptyMutation, FieldResult, RootNode};

/// Mark that ManagedOplDb is a valid Context for a GraphQL query.
impl juniper::Context for ManagedOplDb {}

/// A read-only schema over the OplDb.
pub type Schema = RootNode<'static, Query, EmptyMutation<ManagedOplDb>>;

/// Instantiates a new [Schema].
pub fn new_schema() -> Schema {
    Schema::new(Query, EmptyMutation::<ManagedOplDb>::new())
}

/// Helper for getting the OplDb.
macro_rules! db {
    ($executor:ident) => {
        &$executor.context().0
    };
}

/// Helper for looking up an [opldb::Entry].
macro_rules! entry {
    ($self: ident, $executor:ident) => {
        $executor.context().0.get_entry($self.0)
    };
}

/// Helper for looking up a [opldb::Lifter].
macro_rules! lifter {
    ($self: ident, $executor:ident) => {
        $executor.context().0.get_lifter($self.0)
    };
}

/// Helper for looking up a [opldb::Meet].
macro_rules! meet {
    ($self: ident, $executor:ident) => {
        $executor.context().0.get_meet($self.0)
    };
}

/// The query root.
pub struct Query;
graphql_object!(Query: ManagedOplDb |&self| {
    /// Reports the current API version.
    field apiVersion() -> &str {
        "0.1"
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
});

/// A unique entry in the database.
///
/// Each entry corresponds to a division placing.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Entry(u32);

graphql_object!(Entry: ManagedOplDb |&self| {
    /// The meet in which the entry occurred.
    field meet(&executor) -> Meet {
        Meet(entry!(self, executor).meet_id)
    }

    /// The lifter corresponding to this entry.
    field lifter(&executor) -> Lifter {
        Lifter(entry!(self, executor).lifter_id)
    }

    field tested(&executor) -> bool {
        entry!(self, executor).tested
    }
});

/// A unique lifter in the database.
///
/// Lifters are uniquely identified by username.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lifter(u32);

graphql_object!(Lifter: ManagedOplDb |&self| {
    /// The username that uniquely identifies each lifter.
    field username(&executor) -> &str {
        lifter!(self, executor).username.as_str()
    }

    /// Gets a list of all the lifter's entries.
    field entries(&executor) -> Vec<Entry> {
        db!(executor).get_entry_ids_for_lifter(self.0)
            .into_iter()
            .map(|id| Entry(id))
            .collect()
    }
});

/// A unique meet in the database.
///
/// Meets are uniquely identified by path.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meet(u32);

graphql_object!(Meet: ManagedOplDb |&self| {
    /// The path that uniquely identifies each meet.
    field path(&executor) -> &str {
        meet!(self, executor).path.as_str()
    }

    /// The name of the meet.
    field name(&executor) -> &str {
        meet!(self, executor).name.as_str()
    }

    /// Gets a list of all entries from the meet.
    field entries(&executor) -> Vec<Entry> {
        db!(executor).get_entry_ids_for_meet(self.0)
            .into_iter()
            .map(|id| Entry(id))
            .collect()
    }

    /// Gets a list of all lifters that competed in the meet.
    field lifters(&executor) -> Vec<Lifter> {
        db!(executor).get_lifter_ids_for_meet(self.0)
            .into_iter()
            .map(|id| Lifter(id))
            .collect()
    }
});
