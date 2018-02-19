//! Internationalization facilities.

use std::str::FromStr;

/// List of languages accepted by the project.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Language {
    en_US,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en-US" => Ok(Language::en_US),
            _ => Ok(Language::en_US),
        }
    }
}
