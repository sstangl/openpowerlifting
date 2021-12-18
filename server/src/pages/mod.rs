//! The backend logic for each HTML page.

// Common objects.
pub mod jsdata;

// Template context providers.
pub mod contact;
pub mod data;
pub mod disambiguation;
pub mod faq;
pub mod lifter;
pub mod lifter_csv;
pub mod meet;
pub mod meetlist;
pub mod rankings;
pub mod records;
pub mod status;

// API providers.
pub mod api_rankings;
pub mod api_search;

// Development pages (mounted under /dev).
pub mod checker;

/// Error type for `from_path()` impls.
#[derive(Debug)]
pub enum FromPathError {
    /// Utf8 parsing failed.
    NotUtf8,
    /// Some part of the path contained no information.
    EmptyComponent,
    /// Some component kind occurred more than once.
    ConflictingComponent,
    /// Some component could not be parsed into any type.
    UnknownComponent,
}
