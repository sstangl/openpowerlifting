//! Defines custom structs for the variables used by the HBS templates.

use schema;
use schema::{Entry, Meet};

fn render_weight(opt: Option<f32>) -> Option<String> {
    match opt {
        Some(w) => {
            let mut s = format!("{:.1}", w);

            // Remove trailing ".0" characters.
            if s.ends_with(".0") {
                let len = s.len();
                s.truncate(len - 2);
            }

            Some(s)
        },
        None => None
    }
}

/// Function for rendering Wilks and McCulloch points.
/// Trailing zeroes are not truncated.
fn render_score(opt: Option<f32>) -> Option<String> {
    match opt {
        Some(w) => Some(format!("{:.2}", w)),
        None => None
    }
}

/// Like schema::Entry, but with hardcoded strings instead of floats.
/// FIXME: We can get rid of this with Rocket v0.4.0.
#[derive(Serialize)]
pub struct StringifiedEntry {
    pub id: i32,
    pub meet_id: i32,
    pub lifter_id: i32,
    pub sex: schema::Sex,
    pub event: Option<String>,
    pub equipment: schema::Equipment,
    pub age: Option<String>,
    pub division: Option<String>,
    pub bodyweightkg: Option<String>,
    pub weightclasskg: Option<String>,
    pub squat1kg: Option<String>,
    pub squat2kg: Option<String>,
    pub squat3kg: Option<String>,
    pub squat4kg: Option<String>,
    pub bestsquatkg: Option<String>,
    pub bench1kg: Option<String>,
    pub bench2kg: Option<String>,
    pub bench3kg: Option<String>,
    pub bench4kg: Option<String>,
    pub bestbenchkg: Option<String>,
    pub deadlift1kg: Option<String>,
    pub deadlift2kg: Option<String>,
    pub deadlift3kg: Option<String>,
    pub deadlift4kg: Option<String>,
    pub bestdeadliftkg: Option<String>,
    pub totalkg: Option<String>,
    pub place: Option<String>,
    pub wilks: Option<String>,
    pub mcculloch: Option<String>,
}

impl<'a> From<Entry> for StringifiedEntry {
	fn from(entry: Entry) -> Self {
		Self {
			id: entry.id,
            meet_id: entry.meet_id,
            lifter_id: entry.lifter_id,
            sex: entry.sex,
            event: entry.event,
            equipment: entry.equipment,
            age: Some(String::from("FIXME")),
            division: entry.division,
            bodyweightkg: render_weight(entry.bodyweightkg),
            weightclasskg: render_weight(entry.weightclasskg),
            squat1kg: render_weight(entry.squat1kg),
            squat2kg: render_weight(entry.squat2kg),
            squat3kg: render_weight(entry.squat3kg),
            squat4kg: render_weight(entry.squat4kg),
            bestsquatkg: render_weight(entry.bestsquatkg),
            bench1kg: render_weight(entry.bench1kg),
            bench2kg: render_weight(entry.bench2kg),
            bench3kg: render_weight(entry.bench3kg),
            bench4kg: render_weight(entry.bench4kg),
            bestbenchkg: render_weight(entry.bestbenchkg),
            deadlift1kg: render_weight(entry.deadlift1kg),
            deadlift2kg: render_weight(entry.deadlift2kg),
            deadlift3kg: render_weight(entry.deadlift3kg),
            deadlift4kg: render_weight(entry.deadlift4kg),
            bestdeadliftkg: render_weight(entry.bestdeadliftkg),
            totalkg: render_weight(entry.totalkg),
            place: entry.place,
            wilks: render_score(entry.wilks),
            mcculloch: render_score(entry.mcculloch),
		}
	}
}

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

    pub entries: &'a Vec<(StringifiedEntry, Meet)>,

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
