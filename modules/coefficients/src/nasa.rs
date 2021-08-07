//! Definition of NASA points.
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

/// Calculates NASA points.
///
/// NASA points are the bodyweight multiple times a coefficient.
/// They're defined [by a coefficient table][1].
///
/// [1]: <http://nasa-sports.com/coefficient-system/>
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
