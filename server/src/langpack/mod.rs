//! Internationalization facilities.

use std::str::FromStr;

/// List of languages accepted by the project.
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize)]
pub enum Language {
    /// English, without regional variance (US).
    en,
    /// Esperanto.
    eo,
    /// Russian.
    ru,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Language::en),
            "eo" => Ok(Language::eo),
            "ru" => Ok(Language::ru),
            _ => Err(()),
        }
    }
}
