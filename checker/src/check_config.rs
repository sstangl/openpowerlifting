//! Checks for CONFIG.toml files.

use opltypes::*;
use toml;

use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use std::io::Read;

use Report;

pub struct CheckResult {
    pub report: Report,
    pub config: Option<Config>,
}

#[derive(Debug)]
pub struct Config {
    pub divisions: Vec<DivisionConfig>,
    pub weightclasses: Vec<WeightClassConfig>,
    pub exemptions: Vec<ExemptionConfig>,
}

#[derive(Debug)]
pub struct DivisionConfig {
    /// The name of the division.
    pub name: String,
    /// The inclusive minimum Age for lifters in this division.
    pub min: Age,
    /// The inclusive maximum Age for lifters in this division.
    pub max: Age,
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

/// Used to exempt a specific meet from some of the checks.
#[derive(Copy, Clone, Debug, EnumString, PartialEq)]
pub enum Exemption {
    /// Exempts the meet from having only known divisions.
    #[strum(to_string = "check_row_division")]
    ExemptDivision,

    /// Exempts the meet from requiring monotonically ascending attempts.
    #[strum(to_string = "check_row_lift_order")]
    ExemptLiftOrder,

    /// Allows lifters of any bodyweight to compete in any weightclass.
    #[strum(to_string = "check_row_weightclass_consistency")]
    ExemptWeightClassConsistency,
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
}

fn parse_divisions(value: &toml::Value, report: &mut Report) -> Vec<DivisionConfig> {
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
        let name: &str = match division.get("name").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => {
                report.error(format!("Value '{}.name' must be a String", key));
                continue;
            }
        };

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

        acc.push(DivisionConfig {
            name: name.to_string(),
            min: min_age,
            max: max_age,
        });
    }

    acc
}

fn parse_weightclasses(
    value: &toml::Value,
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
        let classes = match weightclass.get("classes").and_then(|v| v.as_array()) {
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
        let date_range = match weightclass.get("date_range").and_then(|v| v.as_array()) {
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
        let sex = match weightclass.get("sex").and_then(|v| v.as_str()) {
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
                            Some(div) => {
                                match divisions.iter().position(|ref r| r.name == div) {
                                    Some(idx) => vec.push(idx),
                                    None => {
                                        report.error(format!(
                                            "Invalid division '{}' in {}.divisions",
                                            div, key
                                        ));
                                        continue;
                                    }
                                }
                            }
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

fn parse_exemptions(value: &toml::Value, report: &mut Report) -> Vec<ExemptionConfig> {
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

fn parse_config(
    root: toml::Value,
    mut report: Report,
) -> Result<CheckResult, Box<Error>> {
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
            "divisions" | "exemptions" | "weightclasses" => (),
            _ => {
                report.error(format!("Unknown section '{}'", key));
            }
        }
    }

    Ok(CheckResult {
        report,
        config: Some(Config {
            divisions,
            weightclasses,
            exemptions,
        }),
    })
}

/// Main entry point to CONFIG.toml testing.
pub fn check_config(config: PathBuf) -> Result<CheckResult, Box<Error>> {
    let report = Report::new(config);

    let mut file = File::open(&report.path)?;
    let mut config_str = String::new();
    file.read_to_string(&mut config_str)?;

    // Parse the entire string into TOML Value types.
    let root = config_str.parse::<toml::Value>()?;
    Ok(parse_config(root, report)?)
}
