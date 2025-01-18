use std::fmt;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

use crate::types::{Attempt, Placing};

#[derive(Debug, Deserialize)]
pub enum Gender {
    M,
    F,
}

impl Gender {
    pub fn to_openpowerlifting(&self) -> String {
        let value = match self {
            Gender::M => "M",
            Gender::F => "F",
        };

        value.to_owned()
    }
}

#[derive(Debug)]
pub enum WeightClass {
    // Female weight classes
    Under43,
    Under47,
    Under52,
    Under57,
    Under63,
    Under69,
    Under76,
    Under84,
    Over84,

    // Male weight classes
    Under53,
    Under59,
    Under66,
    Under74,
    Under83,
    Under93,
    Under105,
    Under120,
    Over120,
}

impl WeightClass {
    pub fn to_openpowerlifting(&self) -> String {
        let value = match self {
            WeightClass::Under43 => "43",
            WeightClass::Under47 => "47",
            WeightClass::Under52 => "52",
            WeightClass::Under57 => "57",
            WeightClass::Under63 => "63",
            WeightClass::Under69 => "69",
            WeightClass::Under76 => "76",
            WeightClass::Under84 => "84",
            WeightClass::Over84 => "84+",

            WeightClass::Under53 => "53",
            WeightClass::Under59 => "59",
            WeightClass::Under66 => "66",
            WeightClass::Under74 => "74",
            WeightClass::Under83 => "83",
            WeightClass::Under93 => "93",
            WeightClass::Under105 => "105",
            WeightClass::Under120 => "120",
            WeightClass::Over120 => "120+",
        };

        value.to_owned()
    }
}

struct WeightClassVisitor;

impl<'de> Visitor<'de> for WeightClassVisitor {
    type Value = WeightClass;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid weight class")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let Some(weight_class) = value.strip_suffix("kg") else {
            return Err(E::custom(format!(
                "weight class {value} does not end with 'kg'"
            )));
        };

        let Some(numerical) = weight_class
            .strip_prefix('-')
            .or_else(|| weight_class.strip_suffix('+'))
        else {
            return Err(E::custom(format!(
                "weight class {weight_class} does not start with '-' or end with '+'"
            )));
        };

        let variant = match numerical {
            "43" => WeightClass::Under43,
            "47" => WeightClass::Under47,
            "52" => WeightClass::Under52,
            "57" => WeightClass::Under57,
            "63" => WeightClass::Under63,
            "69" => WeightClass::Under69,
            "76" => WeightClass::Under76,
            "84" => {
                if weight_class.starts_with('-') {
                    WeightClass::Under84
                } else {
                    WeightClass::Over84
                }
            }

            "53" => WeightClass::Under53,
            "59" => WeightClass::Under59,
            "66" => WeightClass::Under66,
            "74" => WeightClass::Under74,
            "83" => WeightClass::Under83,
            "93" => WeightClass::Under93,
            "105" => WeightClass::Under105,
            "120" => {
                if weight_class.starts_with('-') {
                    WeightClass::Under120
                } else {
                    WeightClass::Over120
                }
            }
            _ => return Err(E::custom(format!("invalid weight class {value}"))),
        };

        Ok(variant)
    }
}

impl<'de> Deserialize<'de> for WeightClass {
    fn deserialize<D>(deserializer: D) -> Result<WeightClass, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WeightClassVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum Event {
    B,
    SBD,
}

impl Event {
    pub fn to_openpowerlifting(&self) -> String {
        let value = match self {
            Self::B => "B",
            Self::SBD => "SBD",
        };

        value.to_owned()
    }
}

#[derive(Debug, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum Equipment {
    EQ,
    RAW,
}

impl Equipment {
    pub fn to_openpowerlifting(&self) -> String {
        match self {
            Equipment::EQ => "Single-ply",
            Equipment::RAW => "Raw",
        }
        .to_owned()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Row {
    pub firstname: String,
    pub surname: String,
    pub country: String,
    pub gender: Gender,
    pub dob: String,
    pub age: u8,
    pub division: String,
    pub weight_class: WeightClass,
    #[serde(rename = "BODY_WEIGHT")]
    pub bodyweight: f32,
    #[serde(rename = "SQ1_kg")]
    pub squat1: Attempt,
    #[serde(rename = "SQ2_kg")]
    pub squat2: Attempt,
    #[serde(rename = "SQ3_kg")]
    pub squat3: Attempt,
    #[serde(rename = "SQ_BEST_KG")]
    pub best_squat: Attempt,
    #[serde(rename = "BP1_kg")]
    pub bench1: Attempt,
    #[serde(rename = "BP2_kg")]
    pub bench2: Attempt,
    #[serde(rename = "BP3_kg")]
    pub bench3: Attempt,
    #[serde(rename = "BP_BEST_KG")]
    pub best_bench: Attempt,
    #[serde(rename = "DL1_kg")]
    pub deadlift1: Attempt,
    #[serde(rename = "DL2_kg")]
    pub deadlift2: Attempt,
    #[serde(rename = "DL3_kg")]
    pub deadlift3: Attempt,
    #[serde(rename = "DL_BEST_KG")]
    pub best_deadlift: Attempt,
    pub total_kg: Attempt,
    pub total_rank: Placing,
    pub event: Event,
    pub equipment: Equipment,

    // Should be the same for every row, meet level data
    pub event_federation: String,
    pub event_title: String,
    pub event_country: String,
    pub event_city: String,
    pub event_date_begin: String,
}
