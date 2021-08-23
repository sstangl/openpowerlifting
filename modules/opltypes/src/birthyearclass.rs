//! Defines the `BirthYearClass` field for the `entries` table.

use crate::BirthYearRange;

/// A BirthYearClass is similar to an AgeClass: instead of being based off Age,
/// it is based off the BirthYear. This is primarily used by IPF federations.
///
/// The order of the definitions matters: OpenIPF uses >= comparisons for class matching.
#[derive(Copy, Clone, Debug, Deserialize, EnumString, Serialize, PartialEq, PartialOrd)]
pub enum BirthYearClass {
    /// No assignable BirthYearClass.
    #[serde(rename = "")]
    #[strum(serialize = "")]
    None,

    /// From the year the lifter turns 14 and throughout the year the lifter turns 18.
    /// This is IPF "Sub-Juniors".
    #[serde(rename = "14-18")]
    #[strum(serialize = "14-18")]
    ClassY14Y18,

    /// From the year the lifter turns 19 and throughout the year the lifter turns 23.
    /// This is IPF "Juniors".
    #[serde(rename = "19-23")]
    #[strum(serialize = "19-23")]
    ClassY19Y23,

    /// From the year the lifter turns 24 and throughout the year the lifter turns 39.
    /// This is IPF "Seniors".
    #[serde(rename = "24-39")]
    #[strum(serialize = "24-39")]
    ClassY24Y39,

    /// From the year the lifter turns 40 and throughout the year the lifter turns 49.
    /// This is IPF "Masters 1".
    #[serde(rename = "40-49")]
    #[strum(serialize = "40-49")]
    ClassY40Y49,

    /// From the year the lifter turns 50 and throughout the year the lifter turns 59.
    /// This is IPF "Masters 2".
    #[serde(rename = "50-59")]
    #[strum(serialize = "50-59")]
    ClassY50Y59,

    /// From the year the lifter turns 60 and throughout the year the lifter turns 69.
    /// This is IPF "Masters 3".
    #[serde(rename = "60-69")]
    #[strum(serialize = "60-69")]
    ClassY60Y69,

    /// From the year the lifter turns 70 and thereafter.
    /// This is IPF "Masters 4".
    #[serde(rename = "70-999")]
    #[strum(serialize = "70-999")]
    ClassY70Y999,
}

impl Default for BirthYearClass {
    fn default() -> BirthYearClass {
        BirthYearClass::None
    }
}

impl BirthYearClass {
    /// Assign a BirthYearClass based on BirthYear.
    pub fn from_birthyear(birth_year: u32, meet_year: u32) -> BirthYearClass {
        // The lifter must have been born.
        if meet_year < birth_year {
            return BirthYearClass::None;
        }

        // Match on the maximum age possibly reached that year.
        match meet_year - birth_year {
            14..=18 => BirthYearClass::ClassY14Y18,
            19..=23 => BirthYearClass::ClassY19Y23,
            24..=39 => BirthYearClass::ClassY24Y39,
            40..=49 => BirthYearClass::ClassY40Y49,
            50..=59 => BirthYearClass::ClassY50Y59,
            60..=69 => BirthYearClass::ClassY60Y69,
            70..=255 => BirthYearClass::ClassY70Y999,
            _ => BirthYearClass::None,
        }
    }

    /// Assign a BirthYearClass based on a range of BirthYears.
    ///
    /// The range generally comes from a configured Division.
    pub fn from_range(range: BirthYearRange, meet_year: u32) -> BirthYearClass {
        let min_year = range.min_year as u32;
        let max_year = range.max_year as u32;

        let class_min = BirthYearClass::from_birthyear(min_year, meet_year);
        let class_max = BirthYearClass::from_birthyear(max_year, meet_year);
        if class_min == class_max {
            class_min
        } else {
            BirthYearClass::None
        }
    }

    /// Whether the given BirthYearClass is a BirthYearClass::None.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::BirthYearClass;
    /// assert!(!BirthYearClass::ClassY19Y23.is_none());
    /// assert!(BirthYearClass::None.is_none());
    /// ```
    pub fn is_none(self) -> bool {
        self == BirthYearClass::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use BirthYearClass::*;

    /// OpenIPF selectors use 40+, 50+, 60+, and 70+ for Masters.
    /// This is implemented using ordered comparison, asserted here.
    #[test]
    fn openipf_class_ranges() {
        assert!(None < ClassY14Y18);
        assert!(ClassY14Y18 < ClassY19Y23);
        assert!(ClassY19Y23 < ClassY24Y39);
        assert!(ClassY24Y39 < ClassY40Y49);
        assert!(ClassY40Y49 < ClassY50Y59);
        assert!(ClassY50Y59 < ClassY60Y69);
        assert!(ClassY60Y69 < ClassY70Y999);
    }
}
