//! Definition of Dots points.
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

/// Definition of the fourth-degree Dots polynomial.
///
/// This implementation is unused, because we prefer the faster implementation below.
/// However, this version is easier to read.
#[allow(dead_code)]
fn dots_coefficient_poly(a: f64, b: f64, c: f64, d: f64, e: f64, x: f64) -> f64 {
    500.0 / (a * x.powi(4) + b * x.powi(3) + c * x.powi(2) + d * x + e)
}

/// Multiply and add.
#[inline(always)]
fn madd(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}

/// Two-phase Horner's method, by splitting into even/odd degrees.
///
/// This results in codegen that is about 5.5% faster than the poly approach.
/// Compiling with a modern CPU target, it's about 20% faster.
fn dots_coefficient_horner2(a: f64, b: f64, c: f64, d: f64, e: f64, x: f64) -> f64 {
    let x2: f64 = x * x;
    let mut even = madd(a, x2, c); // Ax^2 + C.
    let odd = madd(b, x2, d); // Bx^2 + D.
    even = madd(even, x2, e); // Ax^4 + Cx^2 + E.
    let join = madd(odd, x, even); // Ax^4 + Bx^3 + Cx^2 + Dx + E.
    500.0 / join
}

pub fn dots_coefficient_men(bodyweightkg: f64) -> f64 {
    const A: f64 = -0.0000010930;
    const B: f64 = 0.0007391293;
    const C: f64 = -0.1918759221;
    const D: f64 = 24.0900756;
    const E: f64 = -307.75076;

    // Bodyweight bounds are defined; bodyweights out of range match the boundaries.
    let adjusted = bodyweightkg.clamp(40.0, 210.0);
    dots_coefficient_horner2(A, B, C, D, E, adjusted)
}

pub fn dots_coefficient_women(bodyweightkg: f64) -> f64 {
    const A: f64 = -0.0000010706;
    const B: f64 = 0.0005158568;
    const C: f64 = -0.1126655495;
    const D: f64 = 13.6175032;
    const E: f64 = -57.96288;

    // Bodyweight bounds are defined; bodyweights out of range match the boundaries.
    let adjusted = bodyweightkg.clamp(40.0, 150.0);
    dots_coefficient_horner2(A, B, C, D, E, adjusted)
}

/// Calculates Dots points.
///
/// Dots were introduced by the German IPF Affiliate BVDK after the IPF switched to
/// IPF Points, which do not allow comparing between sexes. The BVDK hosts team
/// competitions that allow lifters of all sexes to compete on a singular team.
///
/// Since Wilks points have been ostracized from the IPF, and IPF Points are
/// unsuitable, German lifters therefore came up with their own formula.
///
/// The author of the Dots formula is Tim Konertz <tim.konertz@outlook.com>.
///
/// Tim says that Dots is an acronym for "Dynamic Objective Team Scoring,"
/// but that they chose the acronym before figuring out the expansion.
pub fn dots(sex: Sex, bodyweight: WeightKg, total: WeightKg) -> Points {
    if bodyweight.is_zero() || total.is_zero() {
        return Points::from_i32(0);
    }
    let coefficient: f64 = match sex {
        Sex::M | Sex::Mx => dots_coefficient_men(f64::from(bodyweight)),
        Sex::F => dots_coefficient_women(f64::from(bodyweight)),
    };
    Points::from(coefficient * f64::from(total))
}
