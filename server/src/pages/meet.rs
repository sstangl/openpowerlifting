//! Logic for each meet's individual results page.

use opldb;
use opldb::fields;

#[derive(Serialize)]
pub struct HeaderContext {
    pub num_entries: u32,
    pub num_meets: u32,
}

/// The context object passed to `templates/lifter.html.hbs`.
#[derive(Serialize)]
pub struct Context {
    pub header: HeaderContext,
}

impl Context {
    pub fn new(opldb: &opldb::OplDb, meet_id: u32) -> Context {
        Context {
            header: HeaderContext {
                num_entries: opldb.get_entries().len() as u32,
                num_meets: opldb.get_meets().len() as u32,
            },
        }
    }
}
