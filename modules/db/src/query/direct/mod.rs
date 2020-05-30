//! A simple interface for common queries.
//!
//! The interface is limited to queries for which the database is guaranteed by construction
//! to have an efficient response.

mod filter;
pub use filter::*;

mod rankings;
pub use rankings::*;
