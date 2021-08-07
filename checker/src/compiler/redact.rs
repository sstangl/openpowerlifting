//! Applies user-requested redactions from `lifter-data/privacy.csv`.

use opltypes::Username;

use crate::checklib::LifterDataMap;
use crate::{AllMeetData, LifterMap};

/// Applies user-requested redactions from `lifter-data/privacy.csv`.
///
/// Why perform this step after consistency checking? Because it involves a special case
/// for almost all of our sanity-checking paths, and we would like it to be drop-in.
///
/// No other phases (consistency checking, compilation) should need to know about privacy stuff.
///
/// Also! Since redactions don't occur in the source files, we still want age/country/etc
/// debugging tools to keep using the names in the original source.
pub fn redact(
    meetdata: &mut AllMeetData,
    liftermap: &mut LifterMap,
    lifterdata: &mut LifterDataMap,
) {
    // First, iterate over the LifterData, modifying the AllMeetData and LifterMap.
    let mut redacted_usernames: Vec<(Username, Username)> = vec![];
    for (username, data) in lifterdata.iter() {
        // Processing only needed for lifters who requested privacy.
        if !data.privacy {
            continue;
        }

        // Create a new Name and Username.
        let disambiguation_id = redacted_usernames.len() + 1;
        let new_name = format!("Redacted Lifter #{}", disambiguation_id);
        let new_username = Username::from_name(&new_name).unwrap();

        // Mark this username as redacted in our accumulator.
        redacted_usernames.push((username.clone(), new_username.clone()));

        let indices = liftermap.remove(username).unwrap();

        // Redact from all entries.
        for &index in &indices {
            let entry = meetdata.entry_mut(index);
            entry.name = new_name.clone();
            entry.username = new_username.clone();

            entry.cyrillicname = None;
            entry.greekname = None;
            entry.japanesename = None;
            entry.koreanname = None;
        }

        // Re-insert into the LifterMap with the new redacted username.
        liftermap.insert(new_username, indices);
    }

    // Re-key the LifterDataMap to match the new, redacted usernames.
    for (old_username, new_username) in redacted_usernames.drain(..) {
        let data = lifterdata.remove(&old_username).unwrap();
        lifterdata.insert(new_username, data);
    }
}
