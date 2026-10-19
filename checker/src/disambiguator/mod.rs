//! Connects the `disambig` crate to the checker.

use colored::*;
use disambig::DisambigId;
use opltypes::*;
use rustc_hash::FxHashMap;

use crate::meetdata::EntryIndex;
use crate::{AllMeetData, Entry, EntryMap, LifterData, Meet};

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
    fn team(&self) -> Option<&str> {
        self.entry.team.as_deref()
    }
}

/// Disambiguates all lifters sharing the same base username.
///
/// This allows executing that crate's logic on the real data of a single
/// lifter. This shows what automatic disambiguation would do in the future
/// when implemented for real. It's useful for feedback during development.
pub fn disambiguate_one(
    meetdata: &AllMeetData,
    entrymap: &EntryMap,
    username: &Username,
    is_debug: bool,
) {
    let manual_variants: Vec<Username> = entrymap.get_variants_of(username);

    // Gather variants matching this username base.
    let rows: Vec<DisambigRow> = {
        let mut acc = Vec::new();
        // Gather the rows with no given variant.
        if let Some(indices) = entrymap.get(&username.without_variant()) {
            for index in indices {
                acc.push(DisambigRow::from_index(*index, meetdata));
            }
        }
        for variant in &manual_variants {
            for index in entrymap.get(variant).unwrap() {
                acc.push(DisambigRow::from_index(*index, meetdata));
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

    // For debug output, print each group one by one.
    if is_debug {
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
}

/// Disambiguates undisambiguated lifters within a disambiguation group.
///
/// This implementation is a limited, hopefully safe subset of auto-disambiguation
/// while the logic is still being tested: For each lifter specified as requiring
/// disambiguation in the `LifterMap`, disambiguation is run on that username group.
pub fn disambiguate_marked(
    meetdata: &mut AllMeetData,
    entrymap: &mut EntryMap,
    lifterdata: &LifterData,
) {
    // TODO: This filtering is inefficient, but it's fast enough for the moment.
    for (username, data) in lifterdata.iter_usernames() {
        if data.disambiguation_count == 0 {
            continue;
        }
        disambiguate_one(meetdata, entrymap, username, false);

        // TODO: Apply the results of the disambiguation.
    }
}
