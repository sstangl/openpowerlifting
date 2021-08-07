//! Generates Username maps from files in the lifter-data/ directory.

use fxhash::{FxBuildHasher, FxHashMap};
use opltypes::Username;
use serde_derive::Deserialize;
use toml::de;

use std::error::Error;
use std::fs;
use std::path::Path;

use crate::Report;

/// Sex consistency exemptions, in the `[sex]` table of `exemptions.toml`.
#[derive(Deserialize)]
pub struct SexExemptions {
    usernames: Vec<Username>,
}

/// Bodyweight consistency exemptions, in the `[bodyweight]` table of `exemptions.toml`.
#[derive(Deserialize)]
pub struct BodyweightExemptions {
    usernames: Vec<Username>,
}

/// Deserialization target for `lifter-data/exemptions.toml`.
#[derive(Deserialize)]
pub struct ExemptionConfig {
    sex: SexExemptions,
    bodyweight: BodyweightExemptions,
}

#[derive(Debug)]
pub struct LifterDataCheckResult {
    pub reports: Vec<Report>,
    pub map: LifterDataMap,
}

/// Map from `Username` to `LifterData`.
pub type LifterDataMap = FxHashMap<Username, LifterData>;

/// A struct containing all `lifter-data/` metadata for a single Username.
#[derive(Debug, Default)]
pub struct LifterData {
    /// CSS class, for lifters who donate to the project.
    pub color: Option<String>,

    /// Extra metadata for showing symbols next to a lifter's name.
    ///
    /// This was added as a promotion for Boss of Bosses, showing the BBBC logo
    /// next to lifters' names for a year. It is currently unused.
    pub flair: Option<String>,

    /// The lifter's Instagram.
    pub instagram: Option<String>,

    /// The lifter's VKontakte name (a Russian clone of Facebook).
    pub vkontakte: Option<String>,

    /// Number of known lifters sharing the same username.
    pub disambiguation_count: u32,

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

/// Helper function to look for common whitespace errors.
fn has_whitespace_errors(s: &str) -> bool {
    s.contains("  ") || s.starts_with(' ') || s.ends_with(' ')
}

/// Specifies CSS classes for when the server renders the lifter's Name.
///
/// The data exists in `lifter-data/donator-colors.csv`.
#[derive(Deserialize)]
struct DonatorColorsRow {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Color")]
    pub color: String,
}

/// Checks `lifter-data/donator-colors.csv`, mutating the LifterDataMap.
fn check_donator_colors(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    map: &mut LifterDataMap,
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
            report.error_on(line, format!("Whitespace error in '{}'", &row.color));
        }

        match map.get_mut(&username) {
            Some(data) => {
                data.color = Some(row.color);
            }
            None => {
                let data = LifterData {
                    color: Some(row.color),
                    ..Default::default()
                };
                map.insert(username, data);
            }
        }
    }

    Ok(())
}

/// Specifies a special flair to the right of the lifter's Name.
///
/// The data exists in `lifter-data/flair.csv`.
#[derive(Deserialize)]
struct FlairRow {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Flair")]
    pub flair: String,
}

/// Checks `lifter-data/flair.csv`, mutating the LifterDataMap.
fn check_flair(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    map: &mut LifterDataMap,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = reader.from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: FlairRow = result?;
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
        if has_whitespace_errors(&row.flair) {
            report.error_on(line, format!("Whitespace error in '{}'", &row.flair));
        }

        match map.get_mut(&username) {
            Some(data) => {
                data.flair = Some(row.flair);
            }
            None => {
                let data = LifterData {
                    flair: Some(row.flair),
                    ..Default::default()
                };
                map.insert(username, data);
            }
        }
    }

    Ok(())
}

/// Loads an `exemptions.toml` file.
///
/// This allows specifying a bunch of exemptions in a single place.
/// There should only be one such file, in `lifter-data/`.
pub fn load_exemptions(report: &mut Report, map: &mut LifterDataMap) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let file_contents: String = fs::read_to_string(&report.path)?;
    let exemptions: ExemptionConfig = de::from_str(file_contents.as_str())?;

    // Handle the [sex] section.
    for username in exemptions.sex.usernames.into_iter() {
        match map.get_mut(&username) {
            Some(data) => {
                data.exempt_sex = true;
            }
            None => {
                let data = LifterData {
                    exempt_sex: true,
                    ..Default::default()
                };
                map.insert(username, data);
            }
        }
    }

    // Handle the [bodyweight] section.
    for username in exemptions.bodyweight.usernames.into_iter() {
        match map.get_mut(&username) {
            Some(data) => {
                data.exempt_bodyweight = true;
            }
            None => {
                let data = LifterData {
                    exempt_bodyweight: true,
                    ..Default::default()
                };
                map.insert(username, data);
            }
        }
    }

    Ok(())
}

/// Specifies lifters who requested name redaction.
///
/// The data exists in `lifter-data/privacy.csv`
#[derive(Deserialize)]
struct PrivacyRow {
    #[serde(rename = "Name")]
    pub name: String,
}

/// Checks `lifter-data/privacy.csv`, mutating the LifterDataMap.
fn check_privacy(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    map: &mut LifterDataMap,
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

        match map.get_mut(&username) {
            Some(data) => {
                data.privacy = true;
            }
            None => {
                let data = LifterData {
                    privacy: true,
                    ..Default::default()
                };
                map.insert(username, data);
            }
        }
    }

    Ok(())
}

/// Specifices a lifter's Instagram public account name.
///
/// The data exists in `lifter-data/social-instagram.csv`.
#[derive(Deserialize)]
struct InstagramRow {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Instagram")]
    pub instagram: String,
}

/// Checks `lifter-data/social-instagram.csv`, mutating the LifterDataMap.
fn check_social_instagram(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    map: &mut LifterDataMap,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = reader.from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: InstagramRow = result?;
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
        if has_whitespace_errors(&row.instagram) {
            report.error_on(line, format!("Whitespace error in '{}'", &row.instagram));
        }

        match map.get_mut(&username) {
            Some(data) => {
                data.instagram = Some(row.instagram);
            }
            None => {
                let data = LifterData {
                    instagram: Some(row.instagram),
                    ..Default::default()
                };
                map.insert(username, data);
            }
        }
    }

    Ok(())
}

/// Specifices a lifter's VKontakte public account name.
///
/// VKontakte is a social networking site for Russia, like Facebook.
/// The data exists in `lifter-data/social-instagram.csv`.
#[derive(Deserialize)]
struct VKontakteRow {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Userpage")]
    pub userpage: String,
}

/// Checks `lifter-data/social-vkontakte.csv`, mutating the LifterDataMap.
fn check_social_vkontakte(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    map: &mut LifterDataMap,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = reader.from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: VKontakteRow = result?;
        let username = match Username::from_name(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(username.as_str()) {
            report.error_on(line, format!("Whitesace error in '{}'", username.as_str()));
        }
        if has_whitespace_errors(&row.userpage) {
            report.error_on(line, format!("Whitespace error in '{}'", &row.userpage));
        }

        match map.get_mut(&username) {
            Some(data) => {
                data.vkontakte = Some(row.userpage);
            }
            None => {
                let data = LifterData {
                    vkontakte: Some(row.userpage),
                    ..Default::default()
                };
                map.insert(username, data);
            }
        }
    }

    Ok(())
}

/// Map from `Username` to a count of disambiguations for that username.
pub type DisambiguationMap = FxHashMap<String, u32>;

/// Specifies a Name (which is translated into a Username) and a Count of the
/// number of disambiguations for people sharing that same Username.
///
/// The list of manual disambiguations is hand-curated and used as a cache,
/// so that parsing meet-data/ can be skipped to get a list of disambiguated
/// usernames.
#[derive(Deserialize)]
struct NameDisambiguationRow {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Count")]
    pub count: u32,
}

/// Checks `lifter-data/name-disambiguation.csv`, making a HashMap of all
/// usernames requiring disambiguation.
fn check_name_disambiguation(
    reader: &csv::ReaderBuilder,
    report: &mut Report,
    map: &mut LifterDataMap,
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

        match map.get_mut(&username) {
            Some(data) => {
                if data.disambiguation_count > 0 {
                    report.error_on(line, format!("Lifter '{}' is duplicated", &row.name));
                } else {
                    data.disambiguation_count = row.count;
                }
            }
            None => {
                let data = LifterData {
                    disambiguation_count: row.count,
                    ..Default::default()
                };
                map.insert(username, data);
            }
        };
    }

    Ok(())
}

pub fn check_lifterdata(reader: &csv::ReaderBuilder, lifterdir: &Path) -> LifterDataCheckResult {
    let mut reports: Vec<Report> = vec![];
    let mut map = LifterDataMap::with_hasher(FxBuildHasher::default());

    // Check donator-colors.csv.
    // Always create the report in order to catch internal errors.
    let mut report = Report::new(lifterdir.join("donator-colors.csv"));
    match check_donator_colors(reader, &mut report, &mut map) {
        Ok(()) => (),
        Err(e) => {
            report.error(e);
        }
    }
    if report.has_messages() {
        reports.push(report)
    }

    // Check flair.csv.
    let mut report = Report::new(lifterdir.join("flair.csv"));
    match check_flair(reader, &mut report, &mut map) {
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
    match load_exemptions(&mut report, &mut map) {
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
    match check_privacy(reader, &mut report, &mut map) {
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
    match check_social_instagram(reader, &mut report, &mut map) {
        Ok(()) => (),
        Err(e) => {
            report.error(e);
        }
    }
    if report.has_messages() {
        reports.push(report)
    }

    // Check social-vkontakte.csv.
    let mut report = Report::new(lifterdir.join("social-vkontakte.csv"));
    match check_social_vkontakte(reader, &mut report, &mut map) {
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
    match check_name_disambiguation(reader, &mut report, &mut map) {
        Ok(()) => (),
        Err(e) => {
            report.error(e);
        }
    }
    if report.has_messages() {
        reports.push(report)
    }

    LifterDataCheckResult { reports, map }
}
