//! Logic for each lifter's personal page.

use opldb;

#[derive(Serialize)]
pub struct HeaderContext {
    pub num_entries: u32,
    pub num_meets: u32,
}

/// The context object passed to `templates/lifter.html.hbs`.
#[derive(Serialize)]
pub struct Context<'a> {
    pub header: HeaderContext,
    pub lifter: &'a opldb::Lifter,
}

impl<'a> Context<'a> {
    pub fn new(opldb: &'a opldb::OplDb, lifter_id: u32) -> Context<'a> {
        let lifter = opldb.get_lifter(lifter_id);
        let entries = opldb.get_entries_for_lifter(lifter_id);

        Context {
            header: HeaderContext {
                num_entries: opldb.get_entries().len() as u32,
                num_meets: opldb.get_meets().len() as u32,
            },
            lifter: lifter,
        }
    }
}
