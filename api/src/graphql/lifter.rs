//! The Lifter object, expressed for GraphQL.

use crate::graphql::Entry;
use crate::ManagedOplDb;

/// A unique lifter in the database.
///
/// Lifters are uniquely identified by username.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lifter(pub u32);

#[graphql_object(context = ManagedOplDb)]
impl Lifter {
    /// The username that uniquely identifies each lifter.
    fn username(&self, db: &ManagedOplDb) -> &str {
        db.0.get_lifter(self.0).username.as_str()
    }

    /// The lifter's name in the Latin character set.
    fn latin_name(&self, db: &ManagedOplDb) -> &str {
        db.0.get_lifter(self.0).name.as_str()
    }

    /// The lifter's name in the Cyrillic character set.
    fn cyrillic_name(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_lifter(self.0).cyrillic_name.as_deref()
    }

    /// The lifter's name in the Greek character set.
    fn greek_name(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_lifter(self.0).greek_name.as_deref()
    }

    /// The lifter's name in the Japanese character set.
    fn japanese_name(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_lifter(self.0).japanese_name.as_deref()
    }

    /// The lifter's name in the Korean character set.
    fn korean_name(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_lifter(self.0).korean_name.as_deref()
    }

    /// The lifter's Instagram account.
    fn instagram(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_lifter(self.0).instagram.as_deref()
    }

    /// Colorization information.
    fn color(&self, db: &ManagedOplDb) -> Option<&str> {
        db.0.get_lifter(self.0).color.as_deref()
    }

    /// Gets a list of all the lifter's entries.
    fn entries(&self, db: &ManagedOplDb) -> Vec<Entry> {
        db.0.get_entry_ids_for_lifter(self.0)
            .into_iter()
            .map(Entry)
            .collect()
    }
}
