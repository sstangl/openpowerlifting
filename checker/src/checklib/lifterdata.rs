//! Generates Username maps from files in the lifter-data/ directory.

use usernames::make_username;

use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use crate::Report;

#[derive(Debug)]
pub struct LifterDataCheckResult {
    pub reports: Vec<Report>,
    pub map: LifterDataMap,
}

/// Map from `Username` to `LifterData`.
pub type LifterDataMap = HashMap<String, LifterData>;

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
    report: &mut Report,
    map: &mut LifterDataMap,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: DonatorColorsRow = result?;
        let username = match make_username(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(&username) {
            report.error_on(line, format!("Whitespace error in '{}'", &username));
        }
        if has_whitespace_errors(&row.color) {
            report.error_on(line, format!("Whitespace error in '{}'", &row.color));
        }

        match map.get_mut(&username) {
            Some(data) => {
                data.color = Some(row.color);
            }
            None => {
                let mut data = LifterData::default();
                data.color = Some(row.color);
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
    report: &mut Report,
    map: &mut LifterDataMap,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: FlairRow = result?;
        let username = match make_username(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(&username) {
            report.error_on(line, format!("Whitespace error in '{}'", &username));
        }
        if has_whitespace_errors(&row.flair) {
            report.error_on(line, format!("Whitespace error in '{}'", &row.flair));
        }

        match map.get_mut(&username) {
            Some(data) => {
                data.flair = Some(row.flair);
            }
            None => {
                let mut data = LifterData::default();
                data.flair = Some(row.flair);
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
    report: &mut Report,
    map: &mut LifterDataMap,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: InstagramRow = result?;
        let username = match make_username(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(&username) {
            report.error_on(line, format!("Whitespace error in '{}'", &username));
        }
        if has_whitespace_errors(&row.instagram) {
            report.error_on(line, format!("Whitespace error in '{}'", &row.instagram));
        }

        match map.get_mut(&username) {
            Some(data) => {
                data.instagram = Some(row.instagram);
            }
            None => {
                let mut data = LifterData::default();
                data.instagram = Some(row.instagram);
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
    report: &mut Report,
    map: &mut LifterDataMap,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: VKontakteRow = result?;
        let username = match make_username(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(&username) {
            report.error_on(line, format!("Whitespace error in '{}'", &username));
        }
        if has_whitespace_errors(&row.userpage) {
            report.error_on(line, format!("Whitespace error in '{}'", &row.userpage));
        }

        match map.get_mut(&username) {
            Some(data) => {
                data.vkontakte = Some(row.userpage);
            }
            None => {
                let mut data = LifterData::default();
                data.vkontakte = Some(row.userpage);
                map.insert(username, data);
            }
        }
    }

    Ok(())
}

/// Map from `Username` to a count of disambiguations for that username.
pub type DisambiguationMap = HashMap<String, u32>;

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
    report: &mut Report,
    map: &mut LifterDataMap,
) -> Result<(), Box<dyn Error>> {
    if !report.path.exists() {
        report.error("File does not exist");
        return Ok(());
    }

    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .terminator(csv::Terminator::Any(b'\n'))
        .from_path(&report.path)?;

    for (rownum, result) in rdr.deserialize().enumerate() {
        // Text editors are one-indexed, and the header line was skipped.
        let line = (rownum as u64) + 2;

        let row: NameDisambiguationRow = result?;
        let username = match make_username(&row.name) {
            Ok(s) => s,
            Err(s) => {
                report.error_on(line, s);
                continue;
            }
        };

        if has_whitespace_errors(&username) {
            report.error_on(line, format!("Whitespace error in '{}'", &username));
        }

        match map.get_mut(&username) {
            Some(data) => {
                if data.disambiguation_count > 0 {
                    report
                        .error_on(line, format!("Lifter '{}' is duplicated", &row.name));
                } else {
                    data.disambiguation_count = row.count;
                }
            }
            None => {
                let mut data = LifterData::default();
                data.disambiguation_count = row.count;
                map.insert(username, data);
            }
        };
    }

    Ok(())
}

pub fn check_lifterdata(lifterdir: &Path) -> LifterDataCheckResult {
    let mut reports: Vec<Report> = vec![];
    let mut map = LifterDataMap::new();

    // Check donator-colors.csv.
    // Always create the report in order to catch internal errors.
    let mut report = Report::new(lifterdir.join("donator-colors.csv"));
    match check_donator_colors(&mut report, &mut map) {
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
    match check_flair(&mut report, &mut map) {
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
    match check_social_instagram(&mut report, &mut map) {
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
    match check_social_vkontakte(&mut report, &mut map) {
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
    match check_name_disambiguation(&mut report, &mut map) {
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
