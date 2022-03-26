//! Definition of Hoffman points.
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

/// Calculates Hoffman points.
///
/// Lyle Schwartz [described the Hoffman formula][1] thusly:
///
/// > Imagine two balloons in the shape of a lifter, one larger than the other.
/// > If we can match the big one by blowing air into the smaller, all dimensions
/// > growing in the same proportion, then the original two balloons can be said
/// > to be similar. Body weight in similar objects increases as the cube of any
/// > length (for example height), while strength presumably depends on how big
/// > the muscles are and that increases as the square of a linear dimension.
///
/// The Hoffman formula seems to have been created around 1958. It was used primarily
/// in Olympic weightlifting, but was also used for powerlifting competitions
/// as they started around the 1960s.
///
/// - TODO: This is probably inaccurate. We're going off a description from Schwarz
///   and a single datapoint that someone quoted Hoffman as saying.
/// - TODO: Find out more about the formula. Maybe a coefficient table, at least!
///
/// [1]: <https://web.archive.org/web/20190408071226/https://www.starkcenter.org/igh/igh-v8/igh-v8-n4/igh0804g.pdf>
pub fn hoffman(bodyweight: WeightKg, total: WeightKg) -> Points {
    if bodyweight.is_zero() || total.is_zero() {
        Points::from_i32(0)
    } else {
        // Constant factor chosen to match the one known datapoint.
        const FACTOR: f64 = 30.221682118754234;

        Points::from(FACTOR * f64::from(total) / f64::from(bodyweight).powf(2.0 / 3.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Hoffman is quoted as giving one example:
    ///
    ///   Back in 1942, Davis lifting as a heavyweight for the first time,
    ///   WEIGHING ONLY 200, officially pressed 322(!) in winning the National title
    ///   at the Arena in Philadelphia. This lift would give John a formula
    ///   rating of 218.63, the world’s highest at that time.
    ///
    /// This tests that we match Hoffman's description on that one point.
    ///
    /// # References
    ///
    /// 1. https://web.archive.org/web/20150702230626/http://ditillo2.blogspot.com/2010/11/art-of-press-bob-hoffman.html
    #[test]
    fn example() {
        const KG_FACTOR: f32 = 2.20462262;

        let bw = WeightKg::from_f32(200.0 / KG_FACTOR);
        let total = WeightKg::from_f32(322.0 / KG_FACTOR);

        assert_eq!(hoffman(bw, total), Points::from(218.63));
    }
}
