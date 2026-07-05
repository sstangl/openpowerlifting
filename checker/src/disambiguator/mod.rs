//! Groups lifters based on various properties

use colored::*;
use disambig::DisambigId;
use opltypes::*;
use rustc_hash::FxHashMap;

use crate::meetdata::EntryIndex;
use crate::{AllMeetData, Entry, LifterMap, Meet};

mod group_age;
pub use group_age::{group_age_debug_for, group_by_age};

/// This crate's wrapper for combined entry and meet information for an entry.
#[derive(Copy, Clone)]
struct DisambigRow<'a> {
    meet: &'a Meet,
    entry: &'a Entry,
}

impl<'a> DisambigRow<'a> {
    pub fn from_index(index: EntryIndex, meetdata: &'a AllMeetData) -> Self {
        DisambigRow {
            meet: meetdata.meet(index),
            entry: meetdata.entry(index),
        }
    }
}

impl<'a> disambig::DisambigEntry for DisambigRow<'a> {
    fn federation(&self) -> Federation {
        self.meet.federation
    }
    fn date(&self) -> Date {
        self.meet.date
    }
    fn meet_country(&self) -> Country {
        self.meet.country
    }
    fn meet_state(&self) -> Option<State> {
        self.meet.state
    }
    fn username(&self) -> Username {
        self.entry.username.clone() // TODO: Reference OK in trait?
    }
    fn sex(&self) -> Sex {
        self.entry.sex
    }
    fn birth_date(&self) -> Option<Date> {
        self.entry.birthdate
    }
    fn birth_year(&self) -> Option<u32> {
        self.entry.birthyearrange.exact_birthyear()
    }
    fn age(&self) -> Age {
        self.entry.age
    }
}

/// Debug wrapper around the `disambig` crate.
///
/// This allows executing that crate's logic on the real data of a single
/// lifter. This shows what automatic disambiguation would do in the future
/// when implemented for real. It's useful for feedback during development.
pub fn disambiguate_debug_for(meetdata: &AllMeetData, liftermap: &LifterMap, username: &Username) {
    // Gather variants matching this username.
    let rows: Vec<DisambigRow> = {
        let mut acc = Vec::new();
        // Gather the rows with no given variant.
        if let Some(indices) = liftermap.get(&username.without_variant()) {
            for index in indices {
                acc.push(DisambigRow::from_index(*index, meetdata));
            }
        }
        // While variants keep getting found, gather their rows too.
        for i in 1.. {
            if let Some(indices) = liftermap.get(&username.with_variant(i)) {
                for index in indices {
                    acc.push(DisambigRow::from_index(*index, meetdata));
                }
            } else {
                break;
            }
        }
        acc
    };

    let ids: Vec<DisambigId> = disambig::disambiguate(&rows);

    // Collect each group into a list.
    let groups: FxHashMap<DisambigId, Vec<DisambigRow<'_>>> = {
        let mut map: FxHashMap<DisambigId, Vec<DisambigRow<'_>>> = FxHashMap::default();
        for (row, id) in rows.iter().zip(ids.iter()) {
            map.entry(*id).or_default().push(*row);
        }
        map
    };

    // Sort the IDs and remove duplicates.
    let sorted_ids: Vec<DisambigId> = {
        let mut acc = ids.clone();
        acc.sort();
        acc.dedup();
        acc
    };

    // Output each group one by one.
    for (i, id) in sorted_ids.iter().enumerate() {
        let group: &[DisambigRow] = groups.get(id).unwrap();

        // Vertical whitespace between sections for readability.
        if i > 0 {
            println!();
        }

        // Is there any hardcoded variant in this group? Use that name if so.
        if let Some(with_variant) = group.iter().find(|row| row.entry.username.has_variant()) {
            println!("{}", with_variant.entry.name.bold());
        } else {
            println!("{} {:?}", group[0].entry.name.bold(), id);
        }

        // Beneath, print the matching meets, colorizing the new ones.
        for row in group {
            if row.entry.username.has_variant() {
                println!("  {}", row.meet.path.purple());
            } else {
                println!("  {}", row.meet.path.bold().green());
            }
        }
    }
}
