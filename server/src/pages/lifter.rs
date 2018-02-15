//! Logic for each lifter's personal page.

use serde;

use opldb;

/// The context object passed to `templates/lifter.html.hbs`.
#[derive(Serialize)]
pub struct Context<'a> {
    lifter: &'a opldb::Lifter,
}

impl<'a> Context<'a> {
    pub fn new(opldb: &'a opldb::OplDb, lifter_id: u32) -> Context<'a> {
        let lifter = opldb.get_lifter(lifter_id);

        Context {
            lifter: lifter,
        }
    }
}
