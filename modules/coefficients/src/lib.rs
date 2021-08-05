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

// Allow polynomial functions with many coefficients.
#![allow(clippy::many_single_char_names)]

extern crate opltypes;

mod ah;
pub use crate::ah::ah;

mod dots;
pub use crate::dots::dots;

mod glossbrenner;
pub use crate::glossbrenner::glossbrenner;

mod goodlift;
pub use crate::goodlift::goodlift;

mod hoffman;
pub use crate::hoffman::hoffman;

mod ipf;
pub use crate::ipf::ipf;

mod mcculloch;
pub use crate::mcculloch::mcculloch;

mod nasa;
pub use crate::nasa::nasa;

mod reshel;
pub use crate::reshel::reshel;

mod schwartzmalone;
pub use crate::schwartzmalone::schwartzmalone;

mod wilks;
pub use crate::wilks::wilks;

mod wilks2020;
pub use crate::wilks2020::wilks2020;

/// Multiply and add. On many CPUs, this is a single instruction.
#[inline(always)]
fn madd(a: f64, b: f64, c: f64) -> f64 {
    a * b + c
}

/// Resolves a 4th-degree polynomial using two-phase Horner's Method.
///
/// By splitting the calculation into even/odd degree halves, CPU parallelization is maximized.
///
/// # Formula
/// `ax^4 + bx^3 + cx^2 + dx + e`
#[inline]
pub(crate) fn poly4(a: f64, b: f64, c: f64, d: f64, e: f64, x: f64) -> f64 {
    let x2 = x * x;
    let mut even = madd(a, x2, c); // Ax^2 + C.
    let odd = madd(b, x2, d); // Bx^2 + D.
    even = madd(even, x2, e); // Ax^4 + Cx^2 + E.
    madd(odd, x, even) // Ax^4 + Bx^3 + Cx^2 + Dx + E.
}

/// Resolves a 5th-degree polynomial using two-phase Horner's Method.
///
/// # Formula
/// `ax^5 + bx^4 + cx^3 + dx^2 + ex + f`
#[inline]
pub(crate) fn poly5(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, x: f64) -> f64 {
    let x2 = x * x;
    let mut odd = madd(a, x2, c); // Ax^2 + C.
    let mut even = madd(b, x2, d); // Bx^2 + D.
    odd = madd(odd, x2, e); // Ax^4 + Cx^2 + E.
    even = madd(even, x2, f); // Bx^4 + Dx^2 + F.
    madd(odd, x, even) // Ax^5 + Bx^4 + Cx^3 + Dx^2 + Ex + F.
}
