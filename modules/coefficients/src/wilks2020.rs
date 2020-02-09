//! Definition of Wilks2020 points.
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

/// Helper function for the common fifth-degree Wilks2020 polynomial.
fn wilks2020_coefficient(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, x: f64) -> f64 {
    600.0 / (a + b * x + c * x.powi(2) + d * x.powi(3) + e * x.powi(4) + f * x.powi(5))
}

fn wilks2020_coefficient_men(bodyweightkg: f64) -> f64 {
    const A: f64 = -99.22855411;
    const B: f64 = 14.40581421;
    const C: f64 = -0.015415771;
    const D: f64 = -769734E-09;
    const E: f64 = 497549E-11;
    const F: f64 = -9.35418E-09;

    let adjusted = bodyweightkg.max(40.0).min(200.95);
    wilks2020_coefficient(A, B, C, D, E, F, adjusted)
}

fn wilks2020_coefficient_women(bodyweightkg: f64) -> f64 {
    const A: f64 = -219.6791486;
    const B: f64 = 19.55345493;
    const C: f64 = -0.167792909;
    const D: f64 = 406937E-09;
    const E: f64 = 184095E-11;
    const F: f64 = -8.31427E-09;

    let adjusted = bodyweightkg.max(40.0).min(150.95);
    wilks2020_coefficient(A, B, C, D, E, F, adjusted)
}

/// Calculates Wilks2020 points.
///
/// In 2020, Robert Wilks announced that he was updating his formula based on new data.
/// He published large coefficient tables, and apparently refused to tell anyone the
/// underlying formula for calculating the coefficients. He also suggested that the
/// formula was not stable, and that he would be adjusting the coefficients over time.
///
/// Wilks2020 unfortunately produces points on an intentionally different scale than
/// the original Wilks formula: instead of being based around 500 points, it uses 600.
/// The scale difference means that the new formula is not a drop-in replacement for
/// the old one, and therefore the old formula will continue to be used. We handle
/// this by just treating the new version as a completely different formula, and have
/// named it "Wilks2020" to note the distinction.
///
/// One problem we will have is that if Robert Wilks continues to update the coefficients,
/// we will have to make a new formula each time he does so, because the display of old
/// meet results requires us to calculate the formula in use *at that time for that meet*.
/// If we updated this formula, we would be incorrect for historical results.
///
/// The official coefficients are published at:
///   https://powerliftingaustralia.com/wilks-formula/
///
/// The formula comes via York Stanham, who converted it from a PA/WP spreadsheet.
pub fn wilks2020(sex: Sex, bodyweight: WeightKg, total: WeightKg) -> Points {
    if bodyweight.is_zero() || total.is_zero() {
        return Points::from_i32(0);
    }
    let coefficient: f64 = match sex {
        Sex::M | Sex::Mx => wilks2020_coefficient_men(f64::from(bodyweight)),
        Sex::F => wilks2020_coefficient_women(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests whether two floating-point numbers are equal to four decimal places,
    /// as published in the official Wilks2020 coefficient tables.
    fn matches_table(a: f64, b: f64) -> bool {
        const FIGS: f64 = 10000.0;
        (a * FIGS).round() == (b * FIGS).round()
    }

    /// Test that the coefficients match the published tables, for men.
    #[test]
    fn coefficients_men() {
        // Check below the table (not well-defined).
        assert!(matches_table(wilks2020_coefficient_men(20.0), 1.4463));

        // Check exact table values.
        assert!(matches_table(wilks2020_coefficient_men(40.0), 1.4463));
        assert!(matches_table(wilks2020_coefficient_men(50.0), 1.1662));
        assert!(matches_table(wilks2020_coefficient_men(60.0), 0.9991));
        assert!(matches_table(wilks2020_coefficient_men(70.0), 0.8911));
        assert!(matches_table(wilks2020_coefficient_men(80.0), 0.8179));
        assert!(matches_table(wilks2020_coefficient_men(90.0), 0.7668));
        assert!(matches_table(wilks2020_coefficient_men(100.0), 0.7304));
        assert!(matches_table(wilks2020_coefficient_men(120.0), 0.6847));
        assert!(matches_table(wilks2020_coefficient_men(140.0), 0.6582));
        assert!(matches_table(wilks2020_coefficient_men(160.0), 0.6396));
        assert!(matches_table(wilks2020_coefficient_men(180.0), 0.6245));
        assert!(matches_table(wilks2020_coefficient_men(200.0), 0.6155));

        // Check the upper boundary of the table.
        assert!(matches_table(wilks2020_coefficient_men(200.0), 0.6155));
        assert!(matches_table(wilks2020_coefficient_men(200.95), 0.6153));

        // Out of the table.
        assert!(matches_table(wilks2020_coefficient_men(201.0), 0.6153));
        assert!(matches_table(wilks2020_coefficient_men(400.0), 0.6153));
    }

    /// Test that the coefficients match the published tables, for women.
    #[test]
    fn coefficients_women() {
        // Check below the table (not well-defined).
        assert!(matches_table(wilks2020_coefficient_women(20.0), 1.8524));

        // Check exact table values.
        assert!(matches_table(wilks2020_coefficient_women(40.0), 1.8524));
        assert!(matches_table(wilks2020_coefficient_women(50.0), 1.5065));
        assert!(matches_table(wilks2020_coefficient_women(60.0), 1.3194));
        assert!(matches_table(wilks2020_coefficient_women(70.0), 1.208));
        assert!(matches_table(wilks2020_coefficient_women(80.0), 1.138));
        assert!(matches_table(wilks2020_coefficient_women(90.0), 1.0922));
        assert!(matches_table(wilks2020_coefficient_women(100.0), 1.0608));
        assert!(matches_table(wilks2020_coefficient_women(110.0), 1.0378));
        assert!(matches_table(wilks2020_coefficient_women(120.0), 1.0194));
        assert!(matches_table(wilks2020_coefficient_women(130.0), 1.0038));
        assert!(matches_table(wilks2020_coefficient_women(140.0), 0.9905));

        // Check the upper boundary of the table.
        // Note: 150.0 does not exactly match (returns 0.9803 instead of 0.9804).
        assert!(matches_table(wilks2020_coefficient_women(150.05), 0.9803));
        assert!(matches_table(wilks2020_coefficient_women(150.95), 0.9796));

        // Out of the table.
        assert!(matches_table(wilks2020_coefficient_women(151.0), 0.9796));
        assert!(matches_table(wilks2020_coefficient_women(200.0), 0.9796));
    }
}
