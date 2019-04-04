//! Defines the `AgeClass` field for the `entries` table.

use crate::Age;

/// The AgeClass used by the server for partitioning into age categories.
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

impl Default for AgeClass {
    fn default() -> AgeClass {
        AgeClass::None
    }
}

impl AgeClass {
    /// Assign an AgeClass based on Age.
    ///
    /// Ambiguous cases get assigned to the pessimal class (closest to Senior).
    pub fn from_age(age: Age) -> AgeClass {
        let (min, max) = match age {
            Age::Exact(n) => (n, n),
            Age::Approximate(n) => (n, n + 1),
            Age::None => {
                return AgeClass::None;
            }
        };

        // Handle the sub-senior classes, which round up.
        if max < 30 {
            match max {
                05...12 => AgeClass::Class5_12,
                13...15 => AgeClass::Class13_15,
                16...17 => AgeClass::Class16_17,
                18...19 => AgeClass::Class18_19,
                20...23 => AgeClass::Class20_23,
                24...34 => AgeClass::Class24_34,
                _ => AgeClass::None,
            }
        } else {
            match min {
                24...34 => AgeClass::Class24_34,
                35...39 => AgeClass::Class35_39,
                40...44 => AgeClass::Class40_44,
                45...49 => AgeClass::Class45_49,
                50...54 => AgeClass::Class50_54,
                55...59 => AgeClass::Class55_59,
                60...64 => AgeClass::Class60_64,
                65...69 => AgeClass::Class65_69,
                70...74 => AgeClass::Class70_74,
                75...79 => AgeClass::Class75_79,
                80...255 => AgeClass::Class80_999,
                _ => AgeClass::None,
            }
        }
    }

    /// Assign an AgeClass based on a range of Ages.
    ///
    /// The range generally comes from a configured Division.
    ///
    /// TODO: Note that because of the limitation in AgeClass, this cannot
    /// TODO: handle Divisions like 40-49.
    pub fn from_range(min: Age, max: Age) -> AgeClass {
        let class_min = AgeClass::from_age(min);
        let class_max = AgeClass::from_age(max);
        if class_min == class_max {
            class_min
        } else {
            AgeClass::None
        }
    }

    /// Returns a tuple of the inclusive Age range bounds for the AgeClass.
    pub fn to_range(self) -> Option<(Age, Age)> {
        match self {
            AgeClass::Class5_12 => Some((Age::Exact(5), Age::Exact(12))),
            AgeClass::Class13_15 => Some((Age::Exact(13), Age::Exact(15))),
            AgeClass::Class16_17 => Some((Age::Exact(16), Age::Exact(17))),
            AgeClass::Class18_19 => Some((Age::Exact(18), Age::Exact(19))),
            AgeClass::Class20_23 => Some((Age::Exact(20), Age::Exact(23))),
            AgeClass::Class24_34 => Some((Age::Exact(24), Age::Exact(34))),
            AgeClass::Class35_39 => Some((Age::Exact(35), Age::Exact(39))),
            AgeClass::Class40_44 => Some((Age::Exact(40), Age::Exact(44))),
            AgeClass::Class45_49 => Some((Age::Exact(45), Age::Exact(49))),
            AgeClass::Class50_54 => Some((Age::Exact(50), Age::Exact(54))),
            AgeClass::Class55_59 => Some((Age::Exact(55), Age::Exact(59))),
            AgeClass::Class60_64 => Some((Age::Exact(60), Age::Exact(64))),
            AgeClass::Class65_69 => Some((Age::Exact(65), Age::Exact(69))),
            AgeClass::Class70_74 => Some((Age::Exact(70), Age::Exact(74))),
            AgeClass::Class75_79 => Some((Age::Exact(75), Age::Exact(79))),
            AgeClass::Class80_999 => Some((Age::Exact(80), Age::Exact(255))),
            AgeClass::None => None,
        }
    }

    /// Whether the given AgeClass is an AgeClass::None.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::AgeClass;
    /// assert!(!AgeClass::Class20_23.is_none());
    /// assert!(AgeClass::None.is_none());
    /// ```
    pub fn is_none(self) -> bool {
        self == AgeClass::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_age() {
        let approx_40 = Age::Approximate(40);
        assert_eq!(AgeClass::from_age(approx_40), AgeClass::Class40_44);
    }
}
