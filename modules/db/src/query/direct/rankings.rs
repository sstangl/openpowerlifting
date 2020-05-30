//! Interface for efficiently querying rankings.

use crate::query::direct::*;

/// A query for rankings information.
#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize)]
pub struct RankingsQuery {
    pub filter: EntryFilter,
    pub order_by: OrderBy,
}
