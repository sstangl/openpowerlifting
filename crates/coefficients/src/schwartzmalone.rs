//! Definition of Schwartz-Malone points.
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

/// Calculates the Schwartz coefficient, used for men.
///
/// The Schwartz formula replaced the Hoffman formula.
/// Schwartz was proposed around February 1971.
///
/// The exact formula was found in the magazine Powerlifting USA,
/// Vol.6, No.2, August 1982, on page 61. That text is reproduced below:
///
/// Computerized Schwartz Formula...Dr. Lyle Schwartz has often been
/// asked for a means by which the formula he has given Powerlifting can be
/// programmed into a computer or a hand held calculator with sufficient
/// memory. To obtain the Schwartz Formula (SF) for bodyweights (BW) bet-
/// ween 40 and 126 kg, the expression is: SF = 0.631926 exp(+01) -
/// 0.262349 exp(+00) (BW) + 0.511550 exp(-02) (BW)^2 - 0.519738
/// exp(-04) (BW)^3 + 0.267626 exp(-06) (BW)^4 - 0.540132 exp(-09)
/// (BW)^5 - 0.728875 exp(-13) (BW)^6. For higher bodyweights, the follow-
/// ing simple formulae are used: for BW 126-136, SF = 0.5210-0.0012
/// (BW - 125), for BW 136-146, SF = 0.5090-0011 (BW - 135), for BW
/// 146-156, SF = 0.4980-0.0010 (BW - 145), and for BW 156-166, SF =
/// 0.4880-0.0090 (BW - 156)
///
/// Schwartz is quoted as saying about the formula's development,
///
/// "Since powerlifting was still a young sport in the early 1970s
/// there was uneven development in the three lifts on the part of most
/// self-trained athletes. I compensated for such unevenness by creating
/// artificial 'best' totals by adding together the current records in the
/// individual lifts. A 'best' total would have been achieved by that ideal
/// lifter who could match the best performances to date in all three
/// powerlifts. Then I fitted these data to an artificial curve and picked
/// off numbers from the curve."
pub fn schwartz_coefficient(bodyweightkg: f64) -> f64 {
    let adjusted = bodyweightkg.clamp(40.0, 166.0);

    if adjusted <= 126.0 {
        let x0 = 0.631926 * 10_f64;
        let x1 = 0.262349 * adjusted;
        let x2 = 0.511550 * 10_f64.powi(-2) * adjusted.powi(2);
        let x3 = 0.519738 * 10_f64.powi(-4) * adjusted.powi(3);
        let x4 = 0.267626 * 10_f64.powi(-6) * adjusted.powi(4);
        let x5 = 0.540132 * 10_f64.powi(-9) * adjusted.powi(5);
        let x6 = 0.728875 * 10_f64.powi(-13) * adjusted.powi(6);
        x0 - x1 + x2 - x3 + x4 - x5 - x6
    } else if adjusted <= 136.0 {
        0.5210 - 0.0012 * (adjusted - 125.0)
    } else if adjusted <= 146.0 {
        0.5090 - 0.0011 * (adjusted - 135.0)
    } else if adjusted <= 156.0 {
        0.4980 - 0.0010 * (adjusted - 145.0)
    } else {
        // The final formula as published for this piece does not match
        // the coefficient tables.
        //
        // From the tables, the step is exactly 0.0004 per pound, which
        // has been converted to kg below.
        //
        // For reference, the published original is:
        //   0.4880 - 0.0090 * (adjusted - 156.0)
        0.4879 - 0.00088185 * (adjusted - 155.0)
    }
}

/// Calculates the Malone coefficient, used for women.
pub fn malone_coefficient(bodyweightkg: f64) -> f64 {
    // Values calculated by fitting to coefficient tables.
    const A: f64 = 106.011586323613;
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
    if bodyweight.is_zero() || total.is_zero() {
        return Points::from_i32(0);
    }
    let coefficient: f64 = match sex {
        Sex::M | Sex::Mx => schwartz_coefficient(f64::from(bodyweight)),
        Sex::F => malone_coefficient(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests whether two floating-point numbers are equal to 4 decimal places,
    /// as published in the official Schwartz coefficient tables.
    fn matches_table(a: f64, b: f64) -> bool {
        const FIGS: f64 = 10000.0;
        (a * FIGS).round() == (b * FIGS).round()
    }

    #[test]
    fn coefficients() {
        // Coefficients taken verbatim from the old Python implementation.
        assert_eq!(malone_coefficient(100.0), 0.597914296471229);
    }

    /// Test whether the Schwartz coefficient calculation matches the
    /// officially-published tables.
    ///
    /// The official tables are published with bodyweights in pounds.
    #[test]
    fn schwartz_coefficient_table() {
        // The conversion factor that Schwartz used.
        let kg = 2.20462262;

        // Test the polynomial.
        assert!(matches_table(schwartz_coefficient(94.0 / kg), 1.2124));
        assert!(matches_table(schwartz_coefficient(110.0 / kg), 0.9991));
        assert!(matches_table(schwartz_coefficient(162.0 / kg), 0.6753));
        assert!(matches_table(schwartz_coefficient(220.0 / kg), 0.5545));

        // Test the second piece (bounded by 136kg).
        // Note that 286 and 287 fail due to rounding.
        assert!(matches_table(schwartz_coefficient(288.0 / kg), 0.5142));

        // Test the third piece (bounded by 146kg).
        // Note that 315 fails due to rounding.
        assert!(matches_table(schwartz_coefficient(316.0 / kg), 0.4998));

        // Test the fourth piece (bounded by 156kg).
        assert!(matches_table(schwartz_coefficient(337.0 / kg), 0.4901));
        assert!(matches_table(schwartz_coefficient(343.0 / kg), 0.4874));

        // Test the final piece (bounded by 166kg).
        // Note that some later values fail due to rounding.
        assert!(matches_table(schwartz_coefficient(344.0 / kg), 0.4870));
        assert!(matches_table(schwartz_coefficient(345.0 / kg), 0.4866));
        assert!(matches_table(schwartz_coefficient(346.0 / kg), 0.4862));
        assert!(matches_table(schwartz_coefficient(347.0 / kg), 0.4858));
        assert!(matches_table(schwartz_coefficient(348.0 / kg), 0.4854));
    }

    #[test]
    fn points() {
        // Points taken verbatim from the old Python implementation.
        assert_eq!(
            schwartzmalone(Sex::M, WeightKg::from_i32(93), WeightKg::from_i32(500)),
            Points::from(287.15)
        );
        assert_eq!(
            schwartzmalone(Sex::F, WeightKg::from_i32(74), WeightKg::from_i32(500)),
            Points::from(364.40)
        );
    }
}
