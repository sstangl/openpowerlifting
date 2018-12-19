//! Definition of IPF Points.

use opltypes::*;

/// Hardcoded formula parameters: `(mean_1, mean_2, deviation_1, deviation_2)`.
type Parameters = (f64, f64, f64, f64);

/// Gets formula parameters from what is effectively a lookup table.
fn get_parameters(sex: Sex, equipment: Equipment, event: Event) -> Parameters {
    // Since the formula was made for the IPF, it only covers Raw and Single-ply.
    // We do our best and just reuse those for Wraps and Multi-ply, respectively.
    let equipment = match equipment {
        Equipment::Raw | Equipment::Wraps | Equipment::Straps => Equipment::Raw,
        Equipment::Single | Equipment::Multi => Equipment::Single,
    };

    const SBD: Event = Event::sbd();
    const B: Event = Event::b();

    match (event, sex, equipment) {
        (SBD, Sex::M, Equipment::Raw) => (310.67, 857.785, 53.216, 147.0835),
        (SBD, Sex::M, Equipment::Single) => (387.265, 1121.28, 80.6324, 222.4896),
        (SBD, Sex::F, Equipment::Raw) => (125.1435, 228.03, 34.5246, 86.8301),
        (SBD, Sex::F, Equipment::Single) => (176.58, 373.315, 48.4534, 110.0103),
        (B, Sex::M, Equipment::Raw) => (86.4745, 259.155, 17.57845, 53.122),
        (B, Sex::M, Equipment::Single) => (133.94, 441.465, 35.3938, 113.0057),
        (B, Sex::F, Equipment::Raw) => (25.0485, 43.848, 6.7172, 13.952),
        (B, Sex::F, Equipment::Single) => (49.106, 124.209, 23.199, 67.492),
        _ => (0.0, 0.0, 0.0, 0.0),
    }
}

/// Calculates IPF Points.
///
/// The IPF formula is a normal distribution with a mean of 500 and a standard
/// deviation of 100.
pub fn ipf(
    sex: Sex,
    equipment: Equipment,
    event: Event,
    bodyweight: WeightKg,
    total: WeightKg,
) -> Points {
    // Look up parameters.
    let (mean1, mean2, dev1, dev2) = get_parameters(sex, equipment, event);

    // Exit early for undefined cases.
    if mean1 == 0.0 || bodyweight <= WeightKg::from_i32(0) {
        return Points::from_i32(0);
    }

    // Calculate the properties of the normal distribution.
    let bw_log = f64::from(bodyweight).ln();
    let mean = mean1 * bw_log - mean2;
    let dev = dev1 * bw_log - dev2;

    // Prevent division by zero.
    if dev == 0.0 {
        return Points::from_i32(0);
    }

    // Calculate IPF points.
    // We add the requirement that the value be non-negative.
    // Although this breaks from the formal definition of the formula,
    // it looks to have been the IPF's intention.
    let points: f64 = (0.0_f64).max(500.0 + 100.0 * (f64::from(total) - mean) / dev);
    Points::from(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A simple helper to pass some defaults, so the testcases aren't too long.
    fn test_helper(bodyweight: WeightKg, total: WeightKg) -> Points {
        ipf(Sex::M, Equipment::Raw, Event::sbd(), bodyweight, total)
    }

    #[test]
    fn expected_values() {
        assert_eq!(
            test_helper(WeightKg::from_f32(93.0), WeightKg::from_f32(777.5)),
            Points::from(741.32)
        );

        let weight = WeightKg::from_f32(73.0);
        let total = WeightKg::from_f32(337.5);
        assert_eq!(
            ipf(Sex::F, Equipment::Raw, Event::sbd(), weight, total),
            Points::from(546.67)
        );
    }

    #[test]
    fn edge_cases() {
        // Zero bodyweight shouldn't crash.
        assert_eq!(
            test_helper(WeightKg::from_f32(0.0), WeightKg::from_f32(400.0)),
            Points::from(0.0)
        );

        // Negative bodyweight shouldn't crash.
        assert_eq!(
            test_helper(WeightKg::from_f32(-100.0), WeightKg::from_f32(400.0)),
            Points::from(0.0)
        );

        // Zero total shouldn't crash.
        assert_eq!(
            test_helper(WeightKg::from_f32(100.0), WeightKg::from_f32(0.0)),
            Points::from(0.0)
        );

        // Negative total shouldn't crash.
        assert_eq!(
            test_helper(WeightKg::from_f32(100.0), WeightKg::from_f32(-100.0)),
            Points::from(0.0)
        );
    }
}
