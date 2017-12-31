//! Defines custom structs for the variables used by the HBS templates.

use schema::{Entry, Meet};

/// Variables used by templates/header.hbs.
#[derive(Serialize)]
pub struct Header {
    /// Length of the "entries" table.
    pub num_entries: i64,

    /// Length of the "meets" table.
    pub num_meets: i64,
}

/// Variables used by templates/desktop-base.hbs.
#[derive(Serialize)]
pub struct Base<'a> {
    /// The page title, in the HTML <head>.
    pub title: &'a str,

    pub header: Header,
}

/// Variables used by templates/faq.html.hbs.
#[derive(Serialize)]
pub struct FaqContext<'a> {
    pub base: Base<'a>,
}

/// Variables used by templates/lifter.html.hbs.
#[derive(Serialize)]
pub struct LifterContext<'a> {
    /// Lifter name with possible Instagram link, as HTML.
    pub lifter_nameurl_html: &'a str,

    pub entries: &'a Vec<(Entry, Meet)>,

    pub best_raw_squat: Option<f32>,
    pub best_raw_bench: Option<f32>,
    pub best_raw_deadlift: Option<f32>,
    pub best_raw_total: Option<f32>,
    pub best_raw_wilks: Option<f32>,

    pub base: Base<'a>,
}

/// Variables used by templates/meet.html.hbs.
#[derive(Serialize)]
pub struct MeetContext<'a> {
    pub meet_display_string: &'a str,
    pub meetpath: &'a str,

    pub entries: &'a Vec<Entry>,

    pub base: Base<'a>,
}
