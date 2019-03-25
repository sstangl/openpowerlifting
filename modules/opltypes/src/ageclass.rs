//! Defines the `AgeClass` field for the `entries` table.

use crate::Age;

/// The AgeClass column, used for rankings and records by the server.
#[derive(Copy, Clone, Debug, Deserialize, EnumString, Serialize, PartialEq)]
pub enum AgeClass {
    #[serde(rename = "5-12")]
    #[strum(serialize = "5-12")]
    Class5_12,
    #[serde(rename = "13-15")]
    #[strum(serialize = "13-15")]
    Class13_15,
    #[serde(rename = "16-17")]
    #[strum(serialize = "16-17")]
    Class16_17,
    #[serde(rename = "18-19")]
    #[strum(serialize = "18-19")]
    Class18_19,
    #[serde(rename = "20-23")]
    #[strum(serialize = "20-23")]
    Class20_23,
    #[serde(rename = "24-34")]
    #[strum(serialize = "24-34")]
    Class24_34,
    #[serde(rename = "35-39")]
    #[strum(serialize = "35-39")]
    Class35_39,
    #[serde(rename = "40-44")]
    #[strum(serialize = "40-44")]
    Class40_44,
    #[serde(rename = "45-49")]
    #[strum(serialize = "45-49")]
    Class45_49,
    #[serde(rename = "50-54")]
    #[strum(serialize = "50-54")]
    Class50_54,
    #[serde(rename = "55-59")]
    #[strum(serialize = "55-59")]
    Class55_59,
    #[serde(rename = "60-64")]
    #[strum(serialize = "60-64")]
    Class60_64,
    #[serde(rename = "65-69")]
    #[strum(serialize = "65-69")]
    Class65_69,
    #[serde(rename = "70-74")]
    #[strum(serialize = "70-74")]
    Class70_74,
    #[serde(rename = "75-79")]
    #[strum(serialize = "75-79")]
    Class75_79,
    #[serde(rename = "80-999")]
    #[strum(serialize = "80-999")]
    Class80_999,
    #[serde(rename = "")]
    #[strum(serialize = "")]
    None,
}

impl AgeClass {
    /// Whether a given Age is definitely contained within the AgeClass.
    pub fn contains(self, age: Age) -> bool {
        let (min, max) = match age {
            Age::Exact(n) => (n, n),
            Age::Approximate(n) => (n, n + 1),
            Age::None => {
                return false;
            }
        };

        match self {
            AgeClass::Class5_12 => min >= 5 && max <= 12,
            AgeClass::Class13_15 => min >= 13 && max <= 15,
            AgeClass::Class16_17 => min >= 16 && max <= 17,
            AgeClass::Class18_19 => min >= 18 && max <= 19,
            AgeClass::Class20_23 => min >= 20 && max <= 23,
            AgeClass::Class24_34 => min >= 24 && max <= 34,
            AgeClass::Class35_39 => min >= 35 && max <= 39,
            AgeClass::Class40_44 => min >= 40 && max <= 44,
            AgeClass::Class45_49 => min >= 45 && max <= 49,
            AgeClass::Class50_54 => min >= 50 && max <= 54,
            AgeClass::Class55_59 => min >= 55 && max <= 59,
            AgeClass::Class60_64 => min >= 60 && max <= 64,
            AgeClass::Class65_69 => min >= 65 && max <= 69,
            AgeClass::Class70_74 => min >= 70 && max <= 74,
            AgeClass::Class75_79 => min >= 75 && max <= 79,
            AgeClass::Class80_999 => min >= 80,
            AgeClass::None => false,
        }
    }
}
