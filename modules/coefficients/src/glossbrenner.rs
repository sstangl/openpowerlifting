//! Definition of Glossbrenner points.

use opltypes::*;

use crate::schwartzmalone::{malone_coefficient, schwartz_coefficient};
use crate::wilks::{wilks_coefficient_men, wilks_coefficient_women};

fn glossbrenner_coefficient_men(bodyweightkg: f64) -> f64 {
    // Glossbrenner is defined piecewise.
    if bodyweightkg < 153.05 {
        (schwartz_coefficient(bodyweightkg) + wilks_coefficient_men(bodyweightkg)) / 2.0
    } else {
        // Linear coefficients found by fitting to a table.
        const A: f64 = -0.000821668402557;
        const B: f64 = 0.676940740094416;
        (schwartz_coefficient(bodyweightkg) + A * bodyweightkg + B) / 2.0
    }
}

fn glossbrenner_coefficient_women(bodyweightkg: f64) -> f64 {
    // Glossbrenner is defined piecewise.
    if bodyweightkg < 106.3 {
        (malone_coefficient(bodyweightkg) + wilks_coefficient_women(bodyweightkg)) / 2.0
    } else {
        // Linear coefficients found by fitting to a table.
        const A: f64 = -0.000313738002024;
        const B: f64 = 0.852664892884785;
        (malone_coefficient(bodyweightkg) + A * bodyweightkg + B) / 2.0
    }
}

/// Calculates Glossbrenner points.
///
/// Glossbrenner is the average of two older systems, Schwartz-Malone and Wilks,
/// with a piecewise linear section.
///
/// This points system is most often used by GPC affiliates.
pub fn glossbrenner(sex: Sex, bodyweight: WeightKg, total: WeightKg) -> Points {
    let coefficient: f64 = match sex {
        Sex::M => glossbrenner_coefficient_men(f64::from(bodyweight)),
        Sex::F => glossbrenner_coefficient_women(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coefficients() {
        // Coefficients taken verbatim from the old Python implementation.
        assert_eq!(glossbrenner_coefficient_men(100.0), 0.5848996767118151);
        assert_eq!(glossbrenner_coefficient_women(100.0), 0.7152488066040255);
    }

    #[test]
    fn points() {
        // Point values taken (rounded) from the old Python implementation.
        assert_eq!(
            glossbrenner(Sex::M, WeightKg::from_i32(100), WeightKg::from_i32(1000)),
            Points::from(584.89967)
        );
        assert_eq!(
            glossbrenner(Sex::F, WeightKg::from_i32(60), WeightKg::from_i32(500)),
            Points::from(492.53032)
        );
    }
}
