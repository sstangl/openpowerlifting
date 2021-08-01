//! Checks for CONFIG.toml files.

use opltypes::*;
use toml::{self, Value};

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use std::io::Read;

use crate::Report;

pub struct CheckResult {
    pub report: Report,
    pub config: Option<Config>,
}

#[derive(Debug)]
pub struct Config {
    pub options: Option<OptionConfig>,
    pub divisions: Vec<DivisionConfig>,
    pub weightclasses: Vec<WeightClassConfig>,
    pub exemptions: Vec<ExemptionConfig>,
    pub rulesets: Vec<RuleSetConfig>,
}

#[derive(Debug, Default)]
pub struct OptionConfig {
    /// Option that specifies the config only affects meets after a certain
    /// Date, allowing for partial federation configuration.
    pub valid_since: Option<Date>,

    /// If set to true, pending disambiguations governed by this configuration
    /// become errors.
    pub require_manual_disambiguation: bool,
}

#[derive(Debug)]
pub struct DivisionConfig {
    /// The name of the division.
    pub name: String,
    /// The inclusive minimum Age for lifters in this division.
    pub min: Age,
    /// The inclusive maximum Age for lifters in this division.
    pub max: Age,
    /// Optional restriction of this Division to a single Sex.
    pub sex: Option<Sex>,
    /// Optional restriction of this Division to certain Equipment.
    pub equipment: Option<Vec<Equipment>>,
    /// Optional Tested default for this division. May be overridden by the
    /// Tested column.
    pub tested: Option<bool>,
    /// Specifies a Place that this division must have. Used for Guests.
    pub place: Option<Place>,
}

#[derive(Debug)]
pub struct WeightClassConfig {
    /// The name of the TOML table member.
    ///
    /// For example, `[weightclasses.default_M]` has the name `default_M`.
    pub name: String,

    /// List of weightclasses with the provided parameters.
    pub classes: Vec<WeightClassKg>,
    /// The earliest date at which these weightclasses existed.
    pub date_min: Date,
    /// The last date at which these weightclasses existed.
    pub date_max: Date,
    /// Which sex these weightclasses are for.
    pub sex: Sex,

    /// Specifies that these weightclasses are only for certain divisions.
    ///
    /// These are stored as indices into the Config's `divisions` list.
    pub divisions: Option<Vec<usize>>,
}

#[derive(Debug)]
pub struct RuleSetConfig {
    /// The active RuleSet for the given date range.
    pub ruleset: RuleSet,
    /// The earliest date on which to apply this RuleSet.
    pub date_min: Date,
    /// The last date on which to apply this RuleSet.
    pub date_max: Date,
}

/// Used to exempt a specific meet from some of the checks.
#[derive(Copy, Clone, Debug, EnumString, PartialEq)]
pub enum Exemption {
    /// Exempts the meet from having only known divisions.
    ExemptDivision,

    /// Exempts the meet from requiring monotonically ascending attempts.
    ExemptLiftOrder,

    /// Allows lifters of any bodyweight to compete in any weightclass.
    ExemptWeightClassConsistency,

    /// Allows a meet to contain implausibly young or old lifters.
    ExemptAge,
}

#[derive(Debug)]
pub struct ExemptionConfig {
    /// Name of the folder containing the meet relative to the CONFIG.toml,
    /// like "9804".
    meet_folder: String,
    /// List of tests for which the meet should be exempt.
    exemptions: Vec<Exemption>,
}

impl Config {
    /// Returns an optional list of exemptions for the given folder.
    pub fn exemptions_for(&self, meet_folder: &str) -> Option<&[Exemption]> {
        self.exemptions
            .iter()
            .find(|ec| ec.meet_folder == meet_folder)
            .map(|ec| ec.exemptions.as_slice())
    }

    /// Returns the valid_since option, if present.
    pub fn valid_since(&self) -> Option<Date> {
        self.options.as_ref()?.valid_since
    }

    /// Returns options.require_manual_disambiguation if present, defaulting to
    /// false.
    pub fn does_require_manual_disambiguation(&self) -> bool {
        if let Some(options) = &self.options {
            options.require_manual_disambiguation
        } else {
            false
        }
    }
}

fn parse_options(value: &Value, report: &mut Report) -> Option<OptionConfig> {
    let table = match value.as_table() {
        Some(t) => t,
        None => {
            report.error("Section 'options' must be a Table");
            return None;
        }
    };

    let valid_since: Option<Date> = if let Some(v) = table.get("valid_since") {
        match v.as_str().and_then(|s| s.parse::<Date>().ok()) {
            Some(date) => Some(date),
            None => {
                report.error("Value 'valid_since' must be a Date, like '1999-02-24'");
                None
            }
        }
    } else {
        None
    };

    let mut require_manual_disambiguation = false;
    if let Some(v) = table.get("require_manual_disambiguation") {
        match v.as_bool() {
            Some(b) => {
                require_manual_disambiguation = b;
            }
            None => {
                report.error("Value 'require_manual_disambiguation' must be a boolean");
            }
        }
    }

    Some(OptionConfig {
        valid_since,
        require_manual_disambiguation,
    })
}

fn parse_divisions(value: &Value, report: &mut Report) -> Vec<DivisionConfig> {
    let mut acc = vec![];

    let table = match value.as_table() {
        Some(t) => t,
        None => {
            report.error("Section 'divisions' must be a Table");
            return acc;
        }
    };

    for (key, division) in table {
        // Parse the division name.
        let name: &str = match division.get("name").and_then(Value::as_str) {
            Some(s) => s,
            None => {
                report.error(format!("Value '{}.name' must be a String", key));
                continue;
            }
        };

        // Ensure that the Division name is unique.
        for already_seen in &acc {
            if already_seen.name == name {
                report.error(format!("Division name '{}' must be unique", name));
                break;
            }
        }

        // Parse the minimum age.
        let min_age = match division.get("min") {
            Some(v) => match v.clone().try_into::<Age>() {
                Ok(age) => age,
                Err(e) => {
                    report.error(format!("Failed parsing {}.min: {}", key, e));
                    continue;
                }
            },
            None => {
                report.error(format!("Division '{}' is missing the property 'min'", key));
                continue;
            }
        };

        // Parse the maximum age.
        let max_age = match division.get("max") {
            Some(v) => match v.clone().try_into::<Age>() {
                Ok(age) => age,
                Err(e) => {
                    report.error(format!("Failed parsing {}.max: {}", key, e));
                    continue;
                }
            },
            None => {
                report.error(format!("Division '{}' is missing the property 'max'", key));
                continue;
            }
        };

        // TODO: This fixes the case of {9.5, 10.5}, where is_definitely_less_than
        // fails. TODO: But it could be less of a hack. Maybe define PartialOrd?
        let mut valid_approximate_ages = false;
        if let Age::Approximate(a) = min_age {
            if let Age::Approximate(b) = max_age {
                if a < b {
                    valid_approximate_ages = true;
                }
            }
        }

        // The age range must be nonmonotonically increasing.
        if min_age != max_age
            && !min_age.is_definitely_less_than(max_age)
            && !valid_approximate_ages
        {
            report.error(format!(
                "Division '{}' has an invalid age range '{}-{}'",
                key, min_age, max_age
            ));
            continue;
        }

        // An optional sex restriction may be provided.
        let sex: Option<Sex> = match division.get("sex") {
            Some(v) => match v.clone().try_into::<Sex>() {
                Ok(sex) => Some(sex),
                Err(e) => {
                    report.error(format!("Failed parsing {}.sex: {}", key, e));
                    None
                }
            },
            None => None,
        };

        // An optional list of allowed equipment may be provided.
        let equipment: Option<Vec<Equipment>> = match division.get("equipment") {
            Some(v) => {
                if let Some(array) = v.as_array() {
                    if array.is_empty() {
                        report.error(format!("{}.equipment cannot be empty", key));
                    }

                    let mut vec = Vec::with_capacity(array.len());
                    for value in array {
                        match value.clone().try_into::<Equipment>() {
                            Ok(equipment) => {
                                vec.push(equipment);
                            }
                            Err(e) => {
                                report.error(format!("Error in {}.equipment: {}", key, e));
                            }
                        }
                    }
                    Some(vec)
                } else if let Some(s) = v.as_str() {
                    match s.parse::<Equipment>() {
                        Ok(equipment) => Some(vec![equipment]),
                        Err(e) => {
                            report.error(format!("Error in {}.equipment: {}", key, e));
                            None
                        }
                    }
                } else {
                    report.error(format!("{}.equipment must be a sting or array", key));
                    None
                }
            }
            None => None,
        };

        // Provides a Tested flag which sets some divisions as default-Tested.
        let tested: Option<bool> = match division.get("tested").and_then(Value::as_str) {
            Some(v) => match v {
                "Yes" => Some(true),
                "No" => Some(false),
                _ => {
                    report.error(format!("Failed parsing {}.tested: invalid '{}'", key, v));
                    None
                }
            },
            None => None,
        };

        // Provides a Place value that all entries in the Division must have.
        // This is used to enforce Guest divisions being marked Guest.
        let place: Option<Place> = match division.get("place").and_then(Value::as_str) {
            Some(s) => match s.parse::<Place>() {
                Ok(p) => Some(p),
                Err(e) => {
                    report.error(format!("Failed parsing {}.place: {}", key, e));
                    None
                }
            },
            None => None,
        };

        acc.push(DivisionConfig {
            name: name.to_string(),
            min: min_age,
            max: max_age,
            sex,
            equipment,
            tested,
            place,
        });
    }

    acc
}

fn parse_weightclasses(
    value: &Value,
    divisions: &[DivisionConfig],
    report: &mut Report,
) -> Vec<WeightClassConfig> {
    let mut acc = vec![];

    let table = match value.as_table() {
        Some(t) => t,
        None => {
            report.error("Section 'weightclasses' must be a Table");
            return acc;
        }
    };

    for (key, weightclass) in table {
        // Parse the list of weightclasses.
        let classes = match weightclass.get("classes").and_then(Value::as_array) {
            Some(array) => {
                let mut vec = Vec::with_capacity(array.len());
                for value in array {
                    match value.clone().try_into::<WeightClassKg>() {
                        Ok(class) => {
                            vec.push(class);
                        }
                        Err(e) => {
                            report.error(format!("Error in '{}.classes': {}", key, e));
                        }
                    }
                }
                vec
            }
            None => {
                report.error(format!("Value '{}.classes' must be an Array", key));
                continue;
            }
        };

        // Parse the min and max dates.
        let date_range = match weightclass.get("date_range").and_then(Value::as_array) {
            Some(array) => {
                if array.len() != 2 {
                    report.error(format!("Array '{}.date_range' must have 2 items", key));
                    continue;
                }
                // TODO: These clone() calls can be removed by using Value::as_str().
                let date_min = match array[0].clone().try_into::<Date>() {
                    Ok(date) => date,
                    Err(e) => {
                        report.error(format!("Error in '{}.date_range': {}", key, e));
                        continue;
                    }
                };
                let date_max = match array[1].clone().try_into::<Date>() {
                    Ok(date) => date,
                    Err(e) => {
                        report.error(format!("Error in '{}.date_range': {}", key, e));
                        continue;
                    }
                };
                (date_min, date_max)
            }
            None => {
                report.error(format!("Value '{}.date_range' must be an Array", key));
                continue;
            }
        };

        // Parse the sex restriction.
        let sex = match weightclass.get("sex").and_then(Value::as_str) {
            Some(s) => match s.parse::<Sex>() {
                Ok(sex) => sex,
                Err(e) => {
                    report.error(format!("Error in '{}.sex': {}", key, e));
                    continue;
                }
            },
            None => {
                report.error(format!("Value '{}.sex' must be a String", key));
                continue;
            }
        };

        // Parse the optional division restriction.
        let divindices: Option<Vec<usize>> = match weightclass.get("divisions") {
            Some(v) => match v.as_array() {
                Some(a) => {
                    let mut vec = Vec::with_capacity(a.len());
                    for div in a {
                        match div.as_str() {
                            Some(div) => match divisions.iter().position(|r| r.name == div) {
                                Some(idx) => vec.push(idx),
                                None => {
                                    report.error(format!(
                                        "Invalid division '{}' in {}.divisions",
                                        div, key
                                    ));
                                    continue;
                                }
                            },
                            None => {
                                report.error(format!(
                                    "Array '{}.divisions' may only contain Strings",
                                    key
                                ));
                                continue;
                            }
                        }
                    }
                    Some(vec)
                }
                None => {
                    report.error(format!("Value '{}.divisions' must be an Array", key));
                    continue;
                }
            },
            None => None,
        };

        // The classes must be ordered from least to greatest.
        // This ordering is required for the logic in check_weightclass_consistency.
        for i in 1..classes.len() {
            if classes[i - 1] >= classes[i] {
                report.error(format!(
                    "WeightClassKg '{}' occurs before '{}' in [weightclasses.{}]",
                    classes[i - 1],
                    classes[i],
                    key
                ));
            }
        }

        acc.push(WeightClassConfig {
            name: key.to_string(),
            classes,
            date_min: date_range.0,
            date_max: date_range.1,
            sex,
            divisions: divindices,
        });
    }

    acc
}

fn parse_rulesets(value: &Value, report: &mut Report) -> Vec<RuleSetConfig> {
    let mut acc = vec![];

    let table = match value.as_table() {
        Some(t) => t,
        None => {
            report.error("Section 'rulesets' must be a Table");
            return acc;
        }
    };

    for (key, section) in table {
        // Parse the list of rulesets.
        let ruleset = match section.get("ruleset").and_then(Value::as_array) {
            Some(array) => {
                let mut ruleset = RuleSet::default();
                for value in array {
                    match value.clone().try_into::<Rule>() {
                        Ok(rule) => {
                            ruleset.add(rule);
                        }
                        Err(e) => {
                            report.error(format!("Error in '{}.ruleset': {}", key, e));
                        }
                    }
                }
                ruleset
            }
            None => {
                report.error(format!("Value '{}.ruleset' must be an Array", key));
                continue;
            }
        };

        // Parse the min and max dates.
        let date_range = match section.get("date_range").and_then(Value::as_array) {
            Some(array) => {
                if array.len() != 2 {
                    report.error(format!("Array '{}.date_range' must have 2 items", key));
                    continue;
                }
                // TODO: These clone() calls can be removed by using Value::as_str().
                let date_min = match array[0].clone().try_into::<Date>() {
                    Ok(date) => date,
                    Err(e) => {
                        report.error(format!("Error in '{}.date_range': {}", key, e));
                        continue;
                    }
                };
                let date_max = match array[1].clone().try_into::<Date>() {
                    Ok(date) => date,
                    Err(e) => {
                        report.error(format!("Error in '{}.date_range': {}", key, e));
                        continue;
                    }
                };
                (date_min, date_max)
            }
            None => {
                report.error(format!("Value '{}.date_range' must be an Array", key));
                continue;
            }
        };

        acc.push(RuleSetConfig {
            ruleset,
            date_min: date_range.0,
            date_max: date_range.1,
        });
    }

    acc
}

fn parse_exemptions(value: &Value, report: &mut Report) -> Vec<ExemptionConfig> {
    let mut acc = vec![];

    let table = match value.as_table() {
        Some(t) => t,
        None => {
            report.error("Section 'exemptions' must be a Table");
            return acc;
        }
    };

    for (key, exemptions) in table {
        let exemptions = match exemptions.as_array() {
            Some(a) => a,
            None => {
                report.error(format!("exemptions.{} must be an Array", key));
                continue;
            }
        };

        let mut vec = Vec::with_capacity(exemptions.len());
        for exemption in exemptions {
            let s = match exemption.as_str() {
                Some(s) => s,
                None => {
                    report.error(format!("exemptions.{} must contain Strings", key));
                    continue;
                }
            };

            match s.parse::<Exemption>() {
                Ok(exemption) => {
                    vec.push(exemption);
                }
                Err(e) => {
                    report.error(format!("Error in exemptions.{}: {}", key, e));
                    continue;
                }
            }
        }

        acc.push(ExemptionConfig {
            meet_folder: key.clone(),
            exemptions: vec,
        });
    }

    acc
}

fn parse_config(root: &Value, mut report: Report) -> Result<CheckResult, Box<dyn Error>> {
    // The highest-level Value must be a table.
    let table = match root.as_table() {
        Some(t) => t,
        None => {
            report.error("Root value must be a Table");
            return Ok(CheckResult {
                report,
                config: None,
            });
        }
    };

    // Parse the "options" table.
    let options = table
        .get("options")
        .and_then(|v| parse_options(v, &mut report));

    // Parse the "divisions" table.
    let divisions = match table.get("divisions") {
        Some(v) => parse_divisions(v, &mut report),
        None => {
            report.error("Missing the 'divisions' table");
            return Ok(CheckResult {
                report,
                config: None,
            });
        }
    };

    // Parse the "weightclasses" table.
    let weightclasses = match table.get("weightclasses") {
        Some(v) => parse_weightclasses(v, &divisions, &mut report),
        None => {
            report.error("Missing the 'weightclasses' table");
            return Ok(CheckResult {
                report,
                config: None,
            });
        }
    };

    // Parse the optional "rulesets" table.
    let rulesets = match table.get("rulesets") {
        Some(v) => parse_rulesets(v, &mut report),
        None => vec![],
    };

    // Parse the "exemptions" table.
    let exemptions = match table.get("exemptions") {
        Some(v) => parse_exemptions(v, &mut report),
        None => {
            report.error("Missing the 'exemptions' table");
            return Ok(CheckResult {
                report,
                config: None,
            });
        }
    };

    // Detect unknown sections.
    for key in table.keys() {
        match key.as_str() {
            "options" | "divisions" | "exemptions" | "rulesets" | "weightclasses" => (),
            _ => {
                report.error(format!("Unknown section '{}'", key));
            }
        }
    }

    Ok(CheckResult {
        report,
        config: Some(Config {
            options,
            divisions,
            weightclasses,
            exemptions,
            rulesets,
        }),
    })
}

/// Main entry point to CONFIG.toml testing.
pub fn check_config(config: PathBuf) -> Result<CheckResult, Box<dyn Error>> {
    let report = Report::new(config);

    let mut file = File::open(&report.path)?;
    let mut config_str = String::new();
    file.read_to_string(&mut config_str)?;

    // Parse the entire string into TOML Value types.
    let root = config_str.parse::<Value>()?;
    parse_config(&root, report)
}
