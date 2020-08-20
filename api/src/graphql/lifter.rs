//! The Lifter object, expressed for GraphQL.

use crate::graphql::Entry;
use crate::ManagedOplDb;

/// Helper for getting the OplDb.
macro_rules! db {
    ($executor:ident) => {
        &$executor.context().0
    };
}

/// Helper for looking up a [opldb::Lifter].
macro_rules! lifter {
    ($self: ident, $executor:ident) => {
        $executor.context().0.get_lifter($self.0)
    };
}

/// A unique lifter in the database.
///
/// Lifters are uniquely identified by username.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lifter(pub u32);

graphql_object!(Lifter: ManagedOplDb |&self| {
    /// The username that uniquely identifies each lifter.
    field username(&executor) -> &str {
        lifter!(self, executor).username.as_str()
    }

    /// The lifter's name in the Latin character set.
    field latin_name(&executor) -> &str {
        lifter!(self, executor).name.as_str()
    }

    /// The lifter's name in the Cyrillic character set.
    field cyrillic_name(&executor) -> Option<&str> {
        lifter!(self, executor).cyrillic_name.as_deref()
    }

    /// The lifter's name in the Greek character set.
    field greek_name(&executor) -> Option<&str> {
        lifter!(self, executor).greek_name.as_deref()
    }

    /// The lifter's name in the Japanese character set.
    field japanese_name(&executor) -> Option<&str> {
        lifter!(self, executor).japanese_name.as_deref()
    }

    /// The lifter's name in the Korean character set.
    field korean_name(&executor) -> Option<&str> {
        lifter!(self, executor).korean_name.as_deref()
    }

    /// The lifter's Instagram account.
    field instagram(&executor) -> Option<&str> {
        lifter!(self, executor).instagram.as_deref()
    }

    /// The lifter's VKontakte account.
    field vkontakte(&executor) -> Option<&str> {
        lifter!(self, executor).vkontakte.as_deref()
    }

    /// Colorization information.
    field color(&executor) -> Option<&str> {
        lifter!(self, executor).color.as_deref()
    }

    /// Gets a list of all the lifter's entries.
    field entries(&executor) -> Vec<Entry> {
        db!(executor).get_entry_ids_for_lifter(self.0)
            .into_iter()
            .map(Entry)
            .collect()
    }
});
