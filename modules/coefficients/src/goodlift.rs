//! Definition of GOODLIFT Points.
//
// Copyright (c) 2020 The OpenPowerlifting Project
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and/or associated documentation files (the
// "Materials"), to deal in the Materials without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Materials, and to
// permit persons to whom the Materials are furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Materials.
//
// THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// MATERIALS OR THE USE OR OTHER DEALINGS IN THE MATERIALS.

use opltypes::*;

/// Hardcoded formula parameters: `(A, B, C)`.
type Parameters = (f64, f64, f64);

/// Gets formula parameters from what is effectively a lookup table.
fn parameters(sex: Sex, equipment: Equipment, event: Event) -> Parameters {
    // Since the formula was made for the IPF, it only covers Raw and Single-ply.
    // We do our best and just reuse those for Wraps and Multi-ply, respectively.
    let equipment = match equipment {
        Equipment::Raw | Equipment::Wraps | Equipment::Straps => Equipment::Raw,
        Equipment::Single | Equipment::Multi | Equipment::Unlimited => Equipment::Single,
    };

    // Points are only specified for Sex::M and Sex::F.
    let dichotomous_sex = match sex {
        Sex::M | Sex::Mx => Sex::M,
        Sex::F => Sex::F,
    };

    const SBD: Event = Event::sbd();
    const B: Event = Event::b();

    match (event, dichotomous_sex, equipment) {
        (SBD, Sex::M, Equipment::Raw) => (1199.72839, 1025.18162, 0.009210),
        (SBD, Sex::M, Equipment::Single) => (1236.25115, 1449.21864, 0.01644),
        (SBD, Sex::F, Equipment::Raw) => (610.32796, 1045.59282, 0.03048),
        (SBD, Sex::F, Equipment::Single) => (758.63878, 949.31382, 0.02435),

        (B, Sex::M, Equipment::Raw) => (320.98041, 281.40258, 0.01008),
        (B, Sex::M, Equipment::Single) => (381.22073, 733.79378, 0.02398),
        (B, Sex::F, Equipment::Raw) => (142.40398, 442.52671, 0.04724),
        (B, Sex::F, Equipment::Single) => (221.82209, 357.00377, 0.02937),

        _ => (0.0, 0.0, 0.0),
    }
}

/// Calculates IPF GOODLIFT Points.
pub fn goodlift(
    sex: Sex,
    equipment: Equipment,
    event: Event,
    bodyweight: WeightKg,
    total: WeightKg,
) -> Points {
    // Look up parameters.
    let (a, b, c) = parameters(sex, equipment, event);

    // Exit early for undefined cases.
    if a == 0.0 || bodyweight < WeightKg::from_i32(35) || total.is_zero() {
        return Points::from_i32(0);
    }

    // A - B * e^(-C * Bwt).
    let e_pow = (-1.0 * c * f64::from(bodyweight)).exp();
    let denominator = a - (b * e_pow);

    // Prevent division by zero.
    if denominator == 0.0 {
        return Points::from_i32(0);
    }

    // Calculate GOODLIFT points.
    // We add the requirement that the value be non-negative.
    let points: f64 = f64::from(total) * (0.0_f64).max(100.0 / denominator);
    Points::from(points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn published_examples() {
        // Dmitry Inzarkin from 2019 IPF World Open Men's Championships.
        let weight = WeightKg::from_f32(92.04);
        let total = WeightKg::from_f32(1035.0);
        assert_eq!(
            goodlift(Sex::M, Equipment::Single, Event::sbd(), weight, total),
            Points::from(112.85)
        );

        // Susanna Torronen from 2019 World Open Classic Bench Press Championships.
        let weight = WeightKg::from_f32(70.50);
        let total = WeightKg::from_f32(122.5);
        assert_eq!(
            goodlift(Sex::F, Equipment::Raw, Event::b(), weight, total),
            Points::from(96.78)
        );
    }
}
