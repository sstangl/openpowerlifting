//! Definition of Schwartz-Malone points.

use opltypes::*;

/// Calculated the Schwartz coefficient, used for men.
pub fn schwartz_coefficient(bodyweightkg: f64) -> f64 {
    // Values calculated by fitting to coefficient tables.
    const A: f64 = 3565.902903983125;
    const B: f64 = -2.244917050872728;
    const C: f64 = 0.445775838479913;

    // Arbitrary choice of lower bound.
    let adjusted = bodyweightkg.max(40.0);

    A * adjusted.powf(B) + C
}

/// Calculates the Malone coefficient, used for women.
pub fn malone_coefficient(bodyweightkg: f64) -> f64 {
    // Values calculated by fitting to coefficient tables.
    const A: f64 = 106.0115863236130;
    const B: f64 = -1.293027130579051;
    const C: f64 = 0.322935585328304;

    // Lower bound chosen at point where Malone = max(Wilks).
    let adjusted = bodyweightkg.max(29.24);

    A * adjusted.powf(B) + C
}

/// Calculates Schwartz-Malone points.
///
/// Schwartz-Malone is an older system that was superseded by Wilks.
pub fn schwartzmalone(sex: Sex, bodyweight: WeightKg, total: WeightKg) -> Points {
    let coefficient: f64 = match sex {
        Sex::M => schwartz_coefficient(f64::from(bodyweight)),
        Sex::F => malone_coefficient(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coefficients() {
        // Coefficients taken verbatim from the old Python implementation.
        assert_eq!(schwartz_coefficient(100.0), 0.5612102815169793);
        assert_eq!(malone_coefficient(100.0), 0.597914296471229);
    }

    #[test]
    fn points() {
        // Points taken verbatim from the old Python implementation.
        assert_eq!(
            schwartzmalone(Sex::M, WeightKg::from_i32(93), WeightKg::from_i32(500)),
            Points::from(290.82)
        );
        assert_eq!(
            schwartzmalone(Sex::F, WeightKg::from_i32(74), WeightKg::from_i32(500)),
            Points::from(364.40)
        );
    }
}
