//! Defines the `AgeClass` field for the `entries` table.

use crate::{Age, AgeRange};

/// The AgeClass used by the server for partitioning into age categories.
#[derive(Copy, Clone, Debug, Deserialize, EnumString, Serialize, PartialEq, Eq)]
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
    #[serde(rename = "80-84")]
    #[strum(serialize = "80-84")]
    Class80_84,
    #[serde(rename = "85-89")]
    #[strum(serialize = "85-89")]
    Class85_89,
    #[serde(rename = "90-999")]
    #[strum(serialize = "90-999")]
    Class90_999,
    #[serde(rename = "")]
    #[strum(serialize = "")]
    None,
}

impl From<Age> for AgeClass {
    /// Assign an AgeClass based on Age.
    ///
    /// Ambiguous cases get assigned to the pessimal class (closest to Senior).
    fn from(age: Age) -> AgeClass {
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
                05..=12 => AgeClass::Class5_12,
                13..=15 => AgeClass::Class13_15,
                16..=17 => AgeClass::Class16_17,
                18..=19 => AgeClass::Class18_19,
                20..=23 => AgeClass::Class20_23,
                24..=34 => AgeClass::Class24_34,
                _ => AgeClass::None,
            }
        } else {
            match min {
                24..=34 => AgeClass::Class24_34,
                35..=39 => AgeClass::Class35_39,
                40..=44 => AgeClass::Class40_44,
                45..=49 => AgeClass::Class45_49,
                50..=54 => AgeClass::Class50_54,
                55..=59 => AgeClass::Class55_59,
                60..=64 => AgeClass::Class60_64,
                65..=69 => AgeClass::Class65_69,
                70..=74 => AgeClass::Class70_74,
                75..=79 => AgeClass::Class75_79,
                80..=84 => AgeClass::Class80_84,
                85..=89 => AgeClass::Class85_89,
                90..=255 => AgeClass::Class90_999,
                _ => AgeClass::None,
            }
        }
    }
}

impl From<AgeRange> for AgeClass {
    /// Assign an AgeClass based on a known AgeRange.
    fn from(range: AgeRange) -> AgeClass {
        let class_min = AgeClass::from(range.min);
        let class_max = AgeClass::from(range.max);

        // If both ends of the range agree, return that AgeClass.
        if class_min == class_max {
            return class_min;
        }

        // If the lower range is unbounded, don't guess.
        // An unbounded upper range can be OK for "Masters 40+".
        if range.min.is_none() {
            return AgeClass::None;
        }

        // If the distance between the max and min is smallish, round toward 30.
        if range.distance().unwrap_or(100) <= 4 {
            // Safe because distance() is only well-defined if not Age::None.
            if range.max.to_u8_option().unwrap_or(30) < 30 {
                return AgeClass::from(range.max);
            }
            return AgeClass::from(range.min);
        }

        AgeClass::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple helper function for making an AgeRange.
    fn range(min: u8, max: u8) -> AgeRange {
        AgeRange::from((Age::Exact(min), Age::Exact(max)))
    }

    /// Approximate ages should round toward 30.
    #[test]
    fn from_approximate() {
        // Round up.
        assert_eq!(AgeClass::from(Age::Approximate(18)), AgeClass::Class18_19);
        assert_eq!(AgeClass::from(Age::Approximate(19)), AgeClass::Class20_23);

        // Round down.
        assert_eq!(AgeClass::from(Age::Approximate(44)), AgeClass::Class40_44);
        assert_eq!(AgeClass::from(Age::Approximate(45)), AgeClass::Class45_49);
    }

    /// We should allow a bit of fuzziness and round "close enough" AgeRanges
    /// toward 30.
    #[test]
    fn fuzzy_ranges() {
        // Check ranges where both ends fall in the same class.
        assert_eq!(AgeClass::from(range(40, 41)), AgeClass::Class40_44);
        assert_eq!(AgeClass::from(range(40, 44)), AgeClass::Class40_44);

        // An AgeRange from an Approximate age that straddles a boundary should
        // round toward 30.
        assert_eq!(AgeClass::from(range(44, 45)), AgeClass::Class40_44);

        // Bigger ranges
        assert_eq!(AgeClass::from(range(44, 48)), AgeClass::Class40_44);
    }
}
