//! Defines the `AgeClass` field for the `entries` table.

#[derive(Clone, Debug, Deserialize, EnumString, Serialize, PartialEq)]
pub enum AgeClass {
    #[serde(rename = "0-16")]
    #[strum(serialize = "0-16")]
    Class0_16,
    #[serde(rename = "17-18")]
    #[strum(serialize = "17-18")]
    Class0_18,
    #[serde(rename = "19-23")]
    #[strum(serialize = "19-23")]
    Class19_23,
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
