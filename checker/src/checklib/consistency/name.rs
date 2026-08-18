//! Checks for consistency errors across entries per lifter.

use crate::{AllMeetData, Entry, EntryIndex, LifterMap, Report};

/// Checks that Name fields are consistent for this lifter.
fn check_name_one(indices: &[EntryIndex], meetdata: &AllMeetData, report: &mut Report) {
    let first_entry: &Entry = meetdata.entry(indices[0]);

    let name = &first_entry.name;
    let username = &first_entry.username;

    let mut chinesename = &first_entry.chinesename;
    let mut cyrillicname = &first_entry.cyrillicname;
    let mut greekname = &first_entry.greekname;
    let mut japanesename = &first_entry.japanesename;
    let mut koreanname = &first_entry.koreanname;

    for index in indices.iter().skip(1) {
        let entry = &meetdata.entry(*index);

        // The Name field must exactly match for the same username.
        if name != &entry.name {
            let msg = format!(
                "Name conflict for '{username}': '{name}' vs '{}'",
                entry.name
            );
            report.error(msg);
        }

        // If this is the first time seeing an optional name field, remember it.
        if chinesename.is_none() && entry.chinesename.is_some() {
            chinesename = &entry.chinesename;
        }
        if cyrillicname.is_none() && entry.cyrillicname.is_some() {
            cyrillicname = &entry.cyrillicname;
        }
        if greekname.is_none() && entry.greekname.is_some() {
            greekname = &entry.greekname;
        }
        if japanesename.is_none() && entry.japanesename.is_some() {
            japanesename = &entry.japanesename;
        }
        if koreanname.is_none() && entry.koreanname.is_some() {
            koreanname = &entry.koreanname;
        }

        // Check ChineseName consistency.
        // TODO: Re-enable after fixing data.
        // if let Some(entry_zh_name) = &entry.chinesename
        //     && let Some(zh_name) = chinesename
        //     && zh_name != entry_zh_name
        // {
        //     let msg =
        //         format!("ChineseName conflict for {username}: '{zh_name}' vs '{entry_zh_name}'",);
        //     report.error(msg);
        // }

        // Check CyrillicName consistency.
        if let Some(entry_cr_name) = &entry.cyrillicname
            && let Some(cr_name) = cyrillicname
            && cr_name != entry_cr_name
        {
            let msg =
                format!("CyrillicName conflict for {username}: '{cr_name}' vs '{entry_cr_name}'",);
            report.error(msg);
        }

        // Check GreekName consistency.
        if let Some(entry_el_name) = &entry.greekname
            && let Some(el_name) = greekname
            && el_name != entry_el_name
        {
            let msg =
                format!("GreekName conflict for {username}: '{el_name}' vs '{entry_el_name}'",);
            report.error(msg);
        }

        // Check JapaneseName consistency.
        if let Some(entry_jp_name) = &entry.japanesename
            && let Some(jp_name) = japanesename
            && jp_name != entry_jp_name
        {
            let msg =
                format!("JapaneseName conflict for {username}: '{jp_name}' vs '{entry_jp_name}'",);
            report.error(msg);
        }

        // Check KoreanName consistency.
        if let Some(entry_ko_name) = &entry.koreanname
            && let Some(ko_name) = koreanname
            && ko_name != entry_ko_name
        {
            let msg =
                format!("KoreanName conflict for {username}: '{ko_name}' vs '{entry_ko_name}'",);
            report.error(msg);
        }
    }
}

/// Checks Name consistency for all lifters.
pub fn check_name_all(liftermap: &LifterMap, meetdata: &AllMeetData, reports: &mut Vec<Report>) {
    let mut report = Report::new("[Name Consistency]".into());

    for lifter_indices in liftermap.values() {
        check_name_one(lifter_indices, meetdata, &mut report);
    }

    if report.has_messages() {
        reports.push(report);
    }
}
