//! Definition of Wilks points.
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

/// Helper function for the common fifth-degree Wilks polynomial.
///
/// Since Points and WeightKg have at most 2 decimal places
/// and are unlikely to exceed 2000, coefficients must be accurate
/// to 7 decimal places, requiring the use of `f64`.
fn wilks_coefficient(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, x: f64) -> f64 {
    500.0 / (a + b * x + c * x.powi(2) + d * x.powi(3) + e * x.powi(4) + f * x.powi(5))
}

pub fn wilks_coefficient_men(bodyweightkg: f64) -> f64 {
    const A: f64 = -216.0475144;
    const B: f64 = 16.2606339;
    const C: f64 = -0.002388645;
    const D: f64 = -0.00113732;
    const E: f64 = 7.01863E-06;
    const F: f64 = -1.291E-08;

    // Upper bound avoids asymptote.
    // Lower bound avoids children with huge coefficients.
    let adjusted = bodyweightkg.clamp(40.0, 201.9);

    wilks_coefficient(A, B, C, D, E, F, adjusted)
}

pub fn wilks_coefficient_women(bodyweightkg: f64) -> f64 {
    const A: f64 = 594.31747775582;
    const B: f64 = -27.23842536447;
    const C: f64 = 0.82112226871;
    const D: f64 = -0.00930733913;
    const E: f64 = 0.00004731582;
    const F: f64 = -0.00000009054;

    // Upper bound avoids asymptote.
    // Lower bound avoids children with huge coefficients.
    let adjusted = bodyweightkg.clamp(26.51, 154.53);

    wilks_coefficient(A, B, C, D, E, F, adjusted)
}

/// Calculates Wilks points.
pub fn wilks(sex: Sex, bodyweight: WeightKg, total: WeightKg) -> Points {
    if bodyweight.is_zero() || total.is_zero() {
        return Points::from_i32(0);
    }
    let coefficient: f64 = match sex {
        Sex::M | Sex::Mx => wilks_coefficient_men(f64::from(bodyweight)),
        Sex::F => wilks_coefficient_women(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coefficients() {
        // Coefficients taken verbatim from the old Python implementation.
        assert_eq!(wilks_coefficient_men(100.0), 0.608589071906651);
        assert_eq!(wilks_coefficient_women(100.0), 0.8325833167368221);
    }

    #[test]
    fn points() {
        // Point values taken (rounded) from the old Python implementation.
        assert_eq!(
            wilks(Sex::M, WeightKg::from_i32(100), WeightKg::from_i32(1000)),
            Points::from(608.58907)
        );
        assert_eq!(
            wilks(Sex::F, WeightKg::from_i32(60), WeightKg::from_i32(500)),
            Points::from(557.4434)
        );
    }
}
