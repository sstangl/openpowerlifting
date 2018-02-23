//! Internationalization facilities.

use serde_json;

use std::error::Error;
use std::str::FromStr;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use opldb;
use opldb::fields;

/// List of languages accepted by the project.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Serialize)]
pub enum Language {
    /// English, without regional variance (US).
    en,
    /// Esperanto.
    eo,
    /// Spanish.
    es,
    /// Russian.
    ru,
}

impl Language {
    /// Returns the units associated with the language.
    pub fn default_units(self) -> opldb::WeightUnits {
        match self {
            Language::en => opldb::WeightUnits::Lbs,
            _ => opldb::WeightUnits::Kg,
        }
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Language::en),
            "eo" => Ok(Language::eo),
            "es" => Ok(Language::es),
            "ru" => Ok(Language::ru),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UnitsTranslations {
    pub lbs: String,
    pub kg: String,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentTranslations {
    pub raw: String,
    pub wraps: String,
    pub single: String,
    pub multi: String,
    pub straps: String,
}

#[derive(Serialize, Deserialize)]
pub struct SexTranslations {
    pub m: String,
    pub f: String,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderTranslations {
    pub rankings: String,
    pub meets: String,
    pub data: String,
    pub faq: String,
    pub contact: String,
    pub supportus: String,
}

#[derive(Serialize, Deserialize)]
pub struct ColumnTranslations {
    pub place: String,
    pub formulaplace: String,
    pub liftername: String,
    pub federation: String,
    pub date: String,
    pub location: String,
    pub meetname: String,
    pub division: String,
    pub sex: String,
    pub age: String,
    pub equipment: String,
    pub weightclass: String,
    pub bodyweight: String,
    pub squat: String,
    pub bench: String,
    pub deadlift: String,
    pub total: String,
    pub wilks: String,
}

#[derive(Serialize, Deserialize)]
pub struct Translations {
    pub units: UnitsTranslations,
    pub equipment: EquipmentTranslations,
    pub sex: SexTranslations,
    pub header: HeaderTranslations,
    pub columns: ColumnTranslations,
    pub search: String,
}

/// Owner struct of all translation state.
pub struct LangInfo {
    en: Option<Translations>,
    eo: Option<Translations>,
    es: Option<Translations>,
    ru: Option<Translations>,
}

impl LangInfo {
    pub fn new() -> LangInfo {
        LangInfo {
            en: None,
            eo: None,
            es: None,
            ru: None,
        }
    }

    pub fn load_translations(
        &mut self,
        language: Language,
        filename: &str,
    ) -> Result<(), Box<Error>> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let trans = serde_json::from_str(&contents)?;

        match language {
            Language::en => self.en = trans,
            Language::eo => self.eo = trans,
            Language::es => self.es = trans,
            Language::ru => self.ru = trans,
        };

        Ok(())
    }

    pub fn get_translations<'a>(&'a self, language: Language) -> &'a Translations {
        match language {
            Language::en => self.en.as_ref().unwrap(),
            Language::eo => self.eo.as_ref().unwrap(),
            Language::es => self.es.as_ref().unwrap(),
            Language::ru => self.ru.as_ref().unwrap(),
        }
    }
}

impl Translations {
    pub fn translate_equipment<'a>(&'a self, equip: fields::Equipment) -> &'a str {
        match equip {
            fields::Equipment::Raw => &self.equipment.raw,
            fields::Equipment::Wraps => &self.equipment.wraps,
            fields::Equipment::Single => &self.equipment.single,
            fields::Equipment::Multi => &self.equipment.multi,
            fields::Equipment::Straps => &self.equipment.straps,
        }
    }

    pub fn translate_sex<'a>(&'a self, sex: fields::Sex) -> &'a str {
        match sex {
            fields::Sex::M => &self.sex.m,
            fields::Sex::F => &self.sex.f,
        }
    }
}
