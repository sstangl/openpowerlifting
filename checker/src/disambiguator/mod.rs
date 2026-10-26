//! Connects the `disambig` crate to the checker.

use colored::*;
use disambig::DisambigId;
use opltypes::*;
use rustc_hash::FxHashMap;

use crate::meetdata::EntryIndex;
use crate::{AllMeetData, Entry, EntryMap, LifterData, Meet};

/// This crate's wrapper for combined entry and meet information for an entry.
#[derive(Copy, Clone)]
struct DisambigRow<'a> {
    meet: &'a Meet,
    entry: &'a Entry,
    index: EntryIndex,
}

impl<'a> DisambigRow<'a> {
    pub fn from_index(index: EntryIndex, meetdata: &'a AllMeetData) -> Self {
        DisambigRow {
            meet: meetdata.meet(index),
            entry: meetdata.entry(index),
            index,
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
///
/// Returns a list of each disambiguation group with its entries,
/// with groups sorted by earliest EntryDate.
fn disambiguate_one(
    meetdata: &AllMeetData,
    entrymap: &EntryMap,
    username: &Username,
    manual_variants: &[Username],
) -> Vec<Vec<EntryIndex>> {
    // Gather variants matching this username base.
    let rows: Vec<DisambigRow> = {
        let mut acc = Vec::new();
        // Gather the rows with no given variant.
        if let Some(indices) = entrymap.get(&username.without_variant()) {
            for index in indices {
                acc.push(DisambigRow::from_index(*index, meetdata));
            }
        }
        for variant in manual_variants {
            for index in entrymap.get(variant).unwrap() {
                acc.push(DisambigRow::from_index(*index, meetdata));
            }
        }
        acc
    };

    // Disambiguation returns a list like [0, 1, 0, 2, ...], mapping each row to a number.
    // Rows mapped to the same number belong to the same disambiguation group.
    let ids: Vec<DisambigId> = disambig::disambiguate(&rows);

    // Collect each group into a list.
    let groups: Vec<Vec<EntryIndex>> = {
        // Group by DisambigId.
        let mut map: FxHashMap<DisambigId, Vec<EntryIndex>> = FxHashMap::default();
        for (row, id) in rows.iter().zip(ids.iter()) {
            map.entry(*id).or_default().push(row.index);
        }

        // Forget about DisambigId, since that was arbitrary.
        let mut acc: Vec<Vec<EntryIndex>> = map.into_values().collect();

        // Try to make the groups semi-stable (unless historical results are added)
        // by sorting all groups by lowest EntryDate within each group.
        acc.sort_by_cached_key(|entries| {
            entries
                .iter()
                .min_by_key(|&idx| meetdata.entry(*idx).entrydate)
                .map_or_default(|&idx| meetdata.entry(idx).entrydate)
        });
        acc
    };

    groups
}

/// Information about how to label all the lifters in a single group.
struct GroupLabel {
    variant: Option<u32>,
}

/// Calculates labels for the given groups.
///
/// The input list of groups must be sorted in order by EntryDate for this to be stable.
fn make_labels(
    meetdata: &AllMeetData,
    entrymap: &EntryMap,
    groups: &[Vec<EntryIndex>],
    username: &Username,
    manual_variants: &[Username],
) -> Vec<GroupLabel> {
    // The next variant to try. Variants may not be dense, so we will probe before assigning.
    let mut next_variant: u32 = (manual_variants.len() as u32) + 1;
    let base_username = username.without_variant();

    let mut labels = Vec::with_capacity(groups.len());

    for group in groups {
        // Does the group already have a variant manually set? If so, then use that.
        if let Some(&with_variant) = group
            .iter()
            .find(|&&idx| meetdata.entry(idx).username.has_variant())
        {
            let variant = meetdata.entry(with_variant).username.get_variant();
            labels.push(GroupLabel { variant });
            continue;
        }

        // If there's only one group, and there's no manually-assigned variant, don't add a variant.
        if groups.len() == 1 {
            labels.push(GroupLabel { variant: None });
            break;
        }

        // Otherwise, use the next free variant.
        loop {
            // If the variant exists in the EntryMap, it already exists, skip it.
            let variant_str = format!("{}{}", base_username.as_str(), next_variant);
            let variant_username = Username::from_trusted_str(&variant_str).unwrap();

            if entrymap.get(&variant_username).is_some() {
                next_variant += 1;
                continue;
            }

            // It doesn't exist, so we can create it.
            labels.push(GroupLabel {
                variant: Some(next_variant),
            });
            next_variant += 1;
            break;
        }
    }

    labels
}

/// Modifies meet data to reflect the new disambiguation groupings.
fn apply_labels(
    meetdata: &mut AllMeetData,
    entrymap: &mut EntryMap,
    groups: &[Vec<EntryIndex>],
    labels: &[GroupLabel],
    username: &Username,
) {
    // Shouldn't happen, but just for safety.
    if groups.len() <= 1 {
        return;
    }

    for (group, label) in groups.iter().zip(labels.iter()) {
        for &idx in group {
            let entry = meetdata.entry_mut(idx);

            // If a variant is manually assigned, assert that it hasn't changed.
            if let Some(variant) = entry.username.get_variant() {
                assert!(variant == label.variant.unwrap());
                continue;
            }

            // If the label doesn't assign a variant, don't do anything.
            // This should be unreachable, but just for local completeness.
            let Some(label_variant) = label.variant else {
                continue;
            };

            // We need to assign a variant. The username is the base username.
            let variant_username = entry.username.with_variant(label_variant);
            entry.username = variant_username.clone();
            entry.name = format!("{} #{label_variant}", entry.name).into();

            // Drop the mutable reference onto meetdata, then update the entrymap.
            entrymap.insert_entry_for(variant_username, idx, meetdata);
        }
    }

    // The base username should now never be part of the entrymap. Its entries were spread out.
    entrymap.remove(username);
}

/// Disambiguates all lifters sharing the same base username, emitting debug output.
///
/// This allows executing that crate's logic on the real data of a single
/// lifter. This shows what automatic disambiguation would do in the future
/// when implemented for real. It's useful for feedback during development.
pub fn disambiguate_username_debug(
    meetdata: &AllMeetData,
    entrymap: &EntryMap,
    username: &Username,
) {
    // Perform the same calculation as the main disambiguation function.
    let variants: Vec<Username> = entrymap.get_variants_of(username);
    let groups: Vec<Vec<EntryIndex>> = disambiguate_one(meetdata, entrymap, username, &variants);
    let labels: Vec<GroupLabel> = make_labels(meetdata, entrymap, &groups, username, &variants);

    // Figure out a good print order for the labels.
    let mut sorted_indexes: Vec<usize> = (0..labels.len()).collect();
    sorted_indexes.sort_by_key(|&i| labels[i].variant);

    // Output the groups using debug output.
    for index in sorted_indexes {
        let label = &labels[index];
        let group = &groups[index];

        // Vertical whitespace between sections for readability.
        println!();

        // Print the name.
        if let Some(variant) = label.variant {
            println!("{} ({variant})", meetdata.entry(group[0]).name);
        } else {
            println!("{}", meetdata.entry(group[0]).name);
        }

        // Print the matching meets, colorizing the new ones.
        for &idx in group {
            let meet = meetdata.meet(idx);
            if meetdata.entry(idx).username.has_variant() {
                println!("  {}", meet.path.purple());
            } else {
                println!("  {}", meet.path.bold().green());
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
        if data.disambiguation_count.is_none() {
            continue;
        }

        let variants: Vec<Username> = entrymap.get_variants_of(username);
        let groups: Vec<Vec<EntryIndex>> =
            disambiguate_one(meetdata, entrymap, username, &variants);
        let labels: Vec<GroupLabel> = make_labels(meetdata, entrymap, &groups, username, &variants);

        apply_labels(meetdata, entrymap, &groups, &labels, username);
    }
}

/// Disambiguates all usernames.
///
/// This is equivalent to `disambiguate_marked`, but isn't limited to just the names
/// specified in the disambiguation file.
pub fn disambiguate_all(
    meetdata: &mut AllMeetData,
    entrymap: &mut EntryMap,
    lifterdata: &LifterData,
) {
    let orig_entrymap = entrymap.clone(); // Very expensive.

    // TODO: This filtering is inefficient, but it's fast enough for the moment.
    for username in orig_entrymap.keys() {
        // Skip already-disambiguated lifters.
        if username.has_variant() {
            continue;
        }
        // Leave alone usernames that have requested privacy.
        if let Some(data) = lifterdata.get(username)
            && data.privacy
        {
            continue;
        }

        // TODO: This disambiguation breaks lifterdata: IGs, colors, etc.

        let variants: Vec<Username> = entrymap.get_variants_of(username);
        let groups: Vec<Vec<EntryIndex>> =
            disambiguate_one(meetdata, entrymap, username, &variants);
        let labels: Vec<GroupLabel> = make_labels(meetdata, entrymap, &groups, username, &variants);

        apply_labels(meetdata, entrymap, &groups, &labels, username);
    }
}
