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

use crate::poly5;
use opltypes::*;

fn wilks2020_coefficient_men(bodyweightkg: f64) -> f64 {
    const A: f64 = 47.4617885411949;
    const B: f64 = 8.47206137941125;
    const C: f64 = 0.073694103462609;
    const D: f64 = -0.00139583381094385;
    const E: f64 = 0.00000707665973070743;
    const F: f64 = -0.0000000120804336482315;

    let adjusted = bodyweightkg.clamp(40.0, 200.95);
    600.0 / poly5(F, E, D, C, B, A, adjusted)
}

fn wilks2020_coefficient_women(bodyweightkg: f64) -> f64 {
    const A: f64 = -125.425539779509;
    const B: f64 = 13.7121941940668;
    const C: f64 = -0.0330725063103405;
    const D: f64 = -0.0010504000506583;
    const E: f64 = 0.00000938773881462799;
    const F: f64 = -0.000000023334613884954;

    let adjusted = bodyweightkg.clamp(40.0, 150.95);
    600.0 / poly5(F, E, D, C, B, A, adjusted)
}

/// Calculates Wilks2020 points.
///
/// This formula was updated as of 2020-03-09. The USPA provided us with a formula
/// definition directly sent to them by Robert Wilks. That definition does not match
/// the previously-published Wilks2020 coefficient tables. We assume that Robert Wilks
/// silently updated them.
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
/// The official coefficients are published at
/// <https://powerliftingaustralia.com/wilks-formula/>.
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
        assert!(matches_table(wilks2020_coefficient_men(20.0), 1.3895));

        // Check exact table values.
        assert!(matches_table(wilks2020_coefficient_men(40.0), 1.3895));
        assert!(matches_table(wilks2020_coefficient_men(50.0), 1.1510));
        assert!(matches_table(wilks2020_coefficient_men(60.0), 0.9968));
        assert!(matches_table(wilks2020_coefficient_men(70.0), 0.8923));
        assert!(matches_table(wilks2020_coefficient_men(80.0), 0.8191));
        assert!(matches_table(wilks2020_coefficient_men(90.0), 0.7670));
        assert!(matches_table(wilks2020_coefficient_men(100.0), 0.7294));
        assert!(matches_table(wilks2020_coefficient_men(120.0), 0.6817));
        assert!(matches_table(wilks2020_coefficient_men(140.0), 0.6546));
        assert!(matches_table(wilks2020_coefficient_men(160.0), 0.6361));
        assert!(matches_table(wilks2020_coefficient_men(180.0), 0.6213));

        // Check the upper boundary of the table.
        assert!(matches_table(wilks2020_coefficient_men(200.0), 0.6123));
        assert!(matches_table(wilks2020_coefficient_men(200.95), 0.6122));

        // Out of the table.
        assert!(matches_table(wilks2020_coefficient_men(201.0), 0.6122));
        assert!(matches_table(wilks2020_coefficient_men(400.0), 0.6122));
    }

    /// Test that the coefficients match the published tables, for women.
    #[test]
    fn coefficients_women() {
        // Check below the table (not well-defined).
        assert!(matches_table(wilks2020_coefficient_women(20.0), 1.8486));

        // Check exact table values.
        assert!(matches_table(wilks2020_coefficient_women(40.0), 1.8486));
        assert!(matches_table(wilks2020_coefficient_women(50.0), 1.5091));
        assert!(matches_table(wilks2020_coefficient_women(60.0), 1.3190));
        assert!(matches_table(wilks2020_coefficient_women(70.0), 1.2042));
        assert!(matches_table(wilks2020_coefficient_women(80.0), 1.1318));
        assert!(matches_table(wilks2020_coefficient_women(90.0), 1.0846));
        assert!(matches_table(wilks2020_coefficient_women(100.0), 1.0525));
        assert!(matches_table(wilks2020_coefficient_women(110.0), 1.0286));
        assert!(matches_table(wilks2020_coefficient_women(120.0), 1.0089));
        assert!(matches_table(wilks2020_coefficient_women(130.0), 0.9912));
        assert!(matches_table(wilks2020_coefficient_women(140.0), 0.9753));

        // Check the upper boundary of the table.
        assert!(matches_table(wilks2020_coefficient_women(150.0), 0.9635));
        assert!(matches_table(wilks2020_coefficient_women(150.95), 0.9627));

        // Out of the table.
        assert!(matches_table(wilks2020_coefficient_women(151.0), 0.9627));
        assert!(matches_table(wilks2020_coefficient_women(200.0), 0.9627));
    }
}
