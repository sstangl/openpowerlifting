//! Definition of NASA points.

use opltypes::*;

/// Calculates NASA points.
///
/// NASA points are the bodyweight multiple times a coefficient.
/// They're defined by a coefficient table: http://nasa-sports.com/coefficient-system/
pub fn nasa(bodyweight: WeightKg, total: WeightKg) -> Points {
    // Arbitrary lower bound, and avoid division by zero.
    if bodyweight < WeightKg::from_i32(30) || total.is_zero() {
        return Points::from_i32(0);
    }

    // The function was determined using fitting in GNUPlot:
    //
    // Final set of parameters            Asymptotic Standard Error
    // =======================            ==========================
    // m               = 0.00620912       +/- 1.265e-06    (0.02037%)
    // b               = 0.565697         +/- 0.0001322    (0.02337%)
    const M: f64 = 0.00620912;
    const B: f64 = 0.565697;
    let bw = f64::from(bodyweight);
    Points::from((f64::from(total) / bw) * (M * bw + B))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points() {
        assert_eq!(
            nasa(WeightKg::from_i32(90), WeightKg::from_i32(500)),
            Points::from(6.25)
        );
    }
}
