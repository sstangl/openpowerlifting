//! Internationalization facilities.

use std::str::FromStr;

/// List of languages accepted by the project.
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize)]
pub enum Language {
    #[serde(rename = "en-US")]
    en_US,
    ru,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en-US" => Ok(Language::en_US),
            "ru" => Ok(Language::ru),
            _ => Err(()),
        }
    }
}
