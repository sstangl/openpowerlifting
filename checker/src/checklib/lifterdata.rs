//! Generates Username maps from files in the lifter-data/ directory.

use opltypes::Username;
use rustc_hash::FxHashMap;
use serde_derive::Deserialize;
use toml::de;

use std::error::Error;
use std::fs;
use std::path::Path;

use crate::Report;

#[derive(Debug)]
pub struct LifterDataCheckResult {
    pub reports: Vec<Report>,
    pub partial_data: LifterData,
}

#[derive(Debug)]
pub struct LifterData {
    /// Map from `Username` to `UsernameData`.
    map: FxHashMap<Username, UsernameData>,
}

/// A struct containing all `lifter-data/` metadata for a single Username.
#[derive(Debug, Default)]
pub struct UsernameData {
    /// CSS class, for lifters who donate to the project.
    pub color: Option<Box<str>>,

    /// The lifter's Instagram.
    pub instagram: Option<Box<str>>,

    /// Number of manually-specified lifters sharing the same username.
    ///
    /// Automatic disambiguation may produce a number higher than this amount.
    pub disambiguation_count: Option<u32>,

    /// True iff a sex conflict with this username is expected.
    ///
    /// This is used to suppress errors where one lifter is marked as two
    /// conflicting sexes, in cases where we know that the data is correct.
    pub exempt_sex: bool,

    /// True iff bodyweight consistency checks for this username are disabled.
    pub exempt_bodyweight: bool,

    /// True iff the lifter requested that their name be redacted on the website.
    pub privacy: bool,
}

impl LifterData {
    /// Returns lifter data for the given username.
    pub fn get(&self, u: &Username) -> Option<&UsernameData> {
        self.map.get(u)
    }

    /// Mutably returns lifter data for the given username.
    pub fn get_mut(&mut self, u: &Username) -> Option<&mut UsernameData> {
        self.map.get_mut(u)
    }

    /// Returns an iterator over all usernames with lifter data.
    pub fn iter_usernames(&self) -> std::collections::hash_map::Iter<'_, Username, UsernameData> {
        self.map.iter()
    }

    /// Renames the given username if present in the lifter data.
    pub fn rename(&mut self, old: &Username, new: Username) {
        if let Some(udata) = self.map.remove(old) {
            self.map.insert(new, udata);
        }
    }
}

/// Helper function to look for common whitespace errors.
fn has_whitespace_errors(s: &str) -> bool {
    s.contains("  ") || s.starts_with(' ') || s.ends_with(' ')
}

/// Sex consistency exemptions, in the `[sex]` table of `exemptions.toml`.
#[derive(Deserialize)]
struct SexExemptions {
    usernames: Vec<Username>,
}

/// Bodyweight consistency exemptions, in the `[bodyweight]` table of `exemptions.toml`.
#[derive(Deserialize)]
struct BodyweightExemptions {
    usernames: Vec<Username>,
}

/// Deserialization target for `lifter-data/exemptions.toml`.
#[derive(Deserialize)]
struct ExemptionConfig {
    sex: SexExemptions,
    bodyweight: BodyweightExemptions,
}

/// Specifies CSS classes for when the server renders the lifter's Name.
///
/// The data exists in `lifter-data/donator-colors.csv`.
#[derive(Deserialize)]
struct DonatorColorsRow {
    #[serde(rename = "Name")]
    pub name: Box<str>,
    #[serde(rename = "Color")]
    pub color: Box<str>,
}

/// Checks `lifter-data/donator-colors.csv`, mutating the LifterDataMap.
fn check_donator_colors(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    data: &mut LifterData,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = reader.from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: DonatorColorsRow = result?;
        let username = match Username::from_name(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(username.as_str()) {
            report.error_on(line, format!("Whitespace error in '{}'", username.as_str()));
        }
        if has_whitespace_errors(&row.color) {
            report.error_on(line, format!("Whitespace error in '{}'", row.color));
        }

        data.map.entry(username).or_default().color = Some(row.color);
    }

    Ok(())
}

/// Loads an `exemptions.toml` file.
///
/// This allows specifying a bunch of exemptions in a single place.
/// There should only be one such file, in `lifter-data/`.
pub fn load_exemptions(report: &mut Report, data: &mut LifterData) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let file_contents: String = fs::read_to_string(&report.path)?;
    let exemptions: ExemptionConfig = de::from_str(file_contents.as_str())?;

    // Handle the [sex] section.
    for username in exemptions.sex.usernames.into_iter() {
        data.map.entry(username).or_default().exempt_sex = true;
    }

    // Handle the [bodyweight] section.
    for username in exemptions.bodyweight.usernames.into_iter() {
        data.map.entry(username).or_default().exempt_bodyweight = true;
    }

    Ok(())
}

/// Specifies lifters who requested name redaction.
///
/// The data exists in `lifter-data/privacy.csv`
#[derive(Deserialize)]
struct PrivacyRow {
    #[serde(rename = "Name")]
    pub name: Box<str>,
}

/// Checks `lifter-data/privacy.csv`, mutating the LifterDataMap.
fn check_privacy(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    data: &mut LifterData,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = reader.from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: PrivacyRow = result?;
        let username = match Username::from_name(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(username.as_str()) {
            report.error_on(line, format!("Whitespace error in '{}'", username.as_str()));
        }

        data.map.entry(username).or_default().privacy = true;
    }

    Ok(())
}

/// Specifices a lifter's Instagram public account name.
///
/// The data exists in `lifter-data/social-instagram.csv`.
#[derive(Deserialize)]
struct InstagramRow {
    #[serde(rename = "Name")]
    pub name: Box<str>,
    #[serde(rename = "Instagram")]
    pub instagram: Box<str>,
}

/// Checks `lifter-data/social-instagram.csv`, mutating the LifterDataMap.
fn check_social_instagram(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    data: &mut LifterData,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    /// Checks that a character is valid in an Instagram handle.
    fn is_valid_ig_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_' || c == '.'
    }

    let mut rdr = reader.from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: InstagramRow = result?;
        let name = &row.name;
        let ig = &row.instagram;

        let username = match Username::from_name(name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(username.as_str()) {
            report.error_on(line, format!("Whitespace error in '{name}'"));
        }

        if ig.is_empty() {
            report.error_on(line, "Instagram column is empty");
        }
        if !ig.chars().all(is_valid_ig_char) {
            report.error_on(line, format!("'{ig}' has invalid characters"));
        }

        match data.get_mut(&username) {
            Some(data) => {
                if data.instagram.is_some() {
                    report.error_on(line, format!("Lifter '{name}' has multiple Instagrams."));
                }
                data.instagram = Some(row.instagram);
            }
            None => {
                let udata = UsernameData {
                    instagram: Some(row.instagram),
                    ..Default::default()
                };
                data.map.insert(username, udata);
            }
        }
    }

    Ok(())
}

/// Specifies a Name (which is translated into a Username) and a Count of the
/// number of disambiguations for people sharing that same Username.
///
/// The list of manual disambiguations is hand-curated and used as a cache,
/// so that parsing meet-data/ can be skipped to get a list of disambiguated
/// usernames.
#[derive(Deserialize)]
struct NameDisambiguationRow {
    #[serde(rename = "Name")]
    pub name: Box<str>,
    #[serde(rename = "Count")]
    pub count: u32,
}

/// Checks `lifter-data/name-disambiguation.csv`, making a HashMap of all
/// usernames requiring disambiguation.
fn check_name_disambiguation(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    data: &mut LifterData,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = reader.from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: NameDisambiguationRow = result?;
        let name = &row.name;

        let username = match Username::from_name(name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(name) {
            report.error_on(line, format!("Whitespace error in '{name}'"));
        }
        if name.contains('#') {
            report.error_on(line, format!("Name '{name}' cannot contain '#'"));
        }

        let entry = data.map.entry(username).or_default();
        if entry.disambiguation_count.is_some() {
            let msg = format!(
                "Lifter '{}' appears multiple times in name-disambiguation.csv",
                row.name
            );
            report.error_on(line, msg);
        }
        entry.disambiguation_count = Some(row.count);
    }

    Ok(())
}

pub fn check_lifterdata(reader: &csv::ReaderBuilder, lifterdir: &Path) -> LifterDataCheckResult {
    let mut reports: Vec<Report> = vec![];
    let mut data = LifterData {
        map: Default::default(),
    };

    // Check donator-colors.csv.
    // Always create the report in order to catch internal errors.
    let mut report = Report::new(lifterdir.join("donator-colors.csv"));
    match check_donator_colors(reader, &mut report, &mut data) {
        Ok(()) => (),
        Err(e) => {
            report.error(e);
        }
    }
    if report.has_messages() {
        reports.push(report)
    }

    // Load exemptions from exemptions.toml.
    let mut report = Report::new(lifterdir.join("exemptions.toml"));
    match load_exemptions(&mut report, &mut data) {
        Ok(()) => (),
        Err(e) => {
            report.error(e);
        }
    }
    if report.has_messages() {
        reports.push(report)
    }

    // Check privacy.csv.
    let mut report = Report::new(lifterdir.join("privacy.csv"));
    match check_privacy(reader, &mut report, &mut data) {
        Ok(()) => (),
        Err(e) => {
            report.error(e);
        }
    }
    if report.has_messages() {
        reports.push(report)
    }

    // Check social-instagram.csv.
    let mut report = Report::new(lifterdir.join("social-instagram.csv"));
    match check_social_instagram(reader, &mut report, &mut data) {
        Ok(()) => (),
        Err(e) => {
            report.error(e);
        }
    }
    if report.has_messages() {
        reports.push(report)
    }

    // Check name-disambiguation.csv and produce a `HashMap<Username, Count>`.
    let mut report = Report::new(lifterdir.join("name-disambiguation.csv"));
    match check_name_disambiguation(reader, &mut report, &mut data) {
        Ok(()) => (),
        Err(e) => {
            report.error(e);
        }
    }
    if report.has_messages() {
        reports.push(report)
    }

    LifterDataCheckResult {
        reports,
        partial_data: data,
    }
}
