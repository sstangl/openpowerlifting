//! The OpenPowerlifting API.

#[macro_use]
extern crate juniper;
#[macro_use]
extern crate serde_derive;

pub mod beta;
pub mod graphql;

/// Wrapper struct for the OplDb.
///
/// This is necessary in order to implement the juniper::Context trait
/// without making GraphQL a dependency of the DB itself.
#[cfg(not(test))]
pub struct ManagedOplDb(pub opldb::OplDb);
#[cfg(test)]
pub struct ManagedOplDb(pub &'static opldb::OplDb);
