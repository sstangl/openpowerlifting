//! Definition of Dots points.

use opltypes::*;

/// Helper function for the common fourth-degree Dots polynomial.
fn dots_coefficient(a: f64, b: f64, c: f64, d: f64, e: f64, x: f64) -> f64 {
    500.0 / (a * x.powi(4) + b * x.powi(3) + c * x.powi(2) + d * x + e)
}

pub fn dots_coefficient_men(bodyweightkg: f64) -> f64 {
    const A: f64 = -0.0000010930;
    const B: f64 = 0.0007391293;
    const C: f64 = -0.1918759221;
    const D: f64 = 24.0900756;
    const E: f64 = -307.75076;

    // Bodyweight bounds are defined; bodyweights out of range match the boundaries.
    let adjusted = bodyweightkg.max(40.0).min(210.0);
    dots_coefficient(A, B, C, D, E, adjusted)
}

pub fn dots_coefficient_women(bodyweightkg: f64) -> f64 {
    const A: f64 = -0.0000010706;
    const B: f64 = 0.0005158568;
    const C: f64 = -0.1126655495;
    const D: f64 = 13.6175032;
    const E: f64 = -57.96288;

    // Bodyweight bounds are defined; bodyweights out of range match the boundaries.
    let adjusted = bodyweightkg.max(40.0).min(150.0);
    dots_coefficient(A, B, C, D, E, adjusted)
}

/// Calculates Dots points.
///
/// Dots were introduced by the German IPF Affiliate BVDK after the IPF switched to
/// IPF Points, which do not allow comparing between sexes. The BVDK hosts team
/// competitions that allow lifters of all sexes to compete on a singular team.
///
/// Since Wilks points have been ostracized from the IPF, and IPF Points are
/// unsuitable, German lifters therefore came up with their own formula.
///
/// The author of the Dots formula is Tim Konertz <tim.konertz@outlook.com>.
pub fn dots(sex: Sex, bodyweight: WeightKg, total: WeightKg) -> Points {
    let coefficient: f64 = match sex {
        Sex::M => dots_coefficient_men(f64::from(bodyweight)),
        Sex::F => dots_coefficient_women(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}
