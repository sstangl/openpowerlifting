//! Internationalization facilities.

use serde_json;

use std::error::Error;
use std::str::FromStr;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

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

#[derive(Deserialize)]
pub struct UnitsTranslations {
    pub lbs: String,
    pub kg: String,
}

#[derive(Deserialize)]
pub struct HeaderTranslations {
    pub rankings: String,
    pub meets: String,
    pub data: String,
    pub faq: String,
    pub contact: String,
    pub supportus: String,
}

#[derive(Deserialize)]
pub struct Translations {
    pub units: UnitsTranslations,
    pub header: HeaderTranslations,
    pub search: String,
}

/// Owner struct of all translation state.
pub struct LangInfo {
    en: Option<Translations>,
    eo: Option<Translations>,
    ru: Option<Translations>,
}

impl LangInfo {
    pub fn new() -> LangInfo {
        LangInfo {
            en: None,
            eo: None,
            ru: None,
        }
    }

    pub fn load_translations(&mut self, language: Language, filename: &str) -> Result<(), Box<Error>> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let trans = serde_json::from_str(&contents)?;

        match language {
            Language::en => self.en = trans,
            Language::eo => self.eo = trans,
            Language::ru => self.ru = trans,
        };

        Ok(())
    }
}
