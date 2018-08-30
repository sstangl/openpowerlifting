//! Defines the `AgeClass` field for the `entries` table.

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
    Classblank,
}
