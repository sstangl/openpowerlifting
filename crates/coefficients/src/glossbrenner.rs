//! Definition of Glossbrenner points.
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
    if bodyweight.is_zero() || total.is_zero() {
        return Points::from_i32(0);
    }
    let coefficient: f64 = match sex {
        Sex::M | Sex::Mx => glossbrenner_coefficient_men(f64::from(bodyweight)),
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
        assert_eq!(glossbrenner_coefficient_men(100.0), 0.5812707859533183);
        assert_eq!(glossbrenner_coefficient_women(100.0), 0.7152488066040259);
    }

    #[test]
    fn points() {
        // Point values taken (rounded) from the old Python implementation.
        assert_eq!(
            glossbrenner(Sex::M, WeightKg::from_i32(100), WeightKg::from_i32(1000)),
            Points::from(581.27)
        );
        assert_eq!(
            glossbrenner(Sex::F, WeightKg::from_i32(60), WeightKg::from_i32(500)),
            Points::from(492.53032)
        );

        // Zero bodyweight should be treated as "unknown bodyweight".
        assert_eq!(
            glossbrenner(Sex::M, WeightKg::from_i32(0), WeightKg::from_i32(500)),
            Points::from_i32(0)
        );
    }
}
