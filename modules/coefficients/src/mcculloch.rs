//! Definition of McCulloch points.

use opltypes::*;

use crate::wilks::{wilks_coefficient_men, wilks_coefficient_women};

/// Lookup table of age coefficients, used as AGE_COEFFICIENTS[age].
const AGE_COEFFICIENTS: [f64; 101] = [
    // Coefficients in the range of 0-4 are clearly nonsense.
    0.0, // 0
    0.0, // 1
    0.0, // 2
    0.0, // 3
    0.0, // 4
    // These coefficients don't actually exist, and are just low-balled best guesses.
    // Kids really shouldn't be competing in this sport...
    // Ranges from age 5 to 13.
    1.73, // 5
    1.67, // 6
    1.61, // 7
    1.55, // 8
    1.49, // 9
    1.43, // 10
    1.38, // 11
    1.33, // 12
    1.28, // 13
    // Foster coefficients:
    // http://www.usapl-sd.com/Formulas/Foster.htm
    // Ranges from age 14 to 22.
    1.23, // 14
    1.18, // 15
    1.13, // 16
    1.08, // 17
    1.06, // 18
    1.04, // 19
    1.03, // 20
    1.02, // 21
    1.01, // 22
    // Lifters in the range 23-40 receive no handicap.
    1.00, // 23
    1.00, // 24
    1.00, // 25
    1.00, // 26
    1.00, // 27
    1.00, // 28
    1.00, // 29
    1.00, // 30
    1.00, // 31
    1.00, // 32
    1.00, // 33
    1.00, // 34
    1.00, // 35
    1.00, // 36
    1.00, // 37
    1.00, // 38
    1.00, // 39
    1.00, // 40
    // McCulloch coefficients:
    //  http://www.usapl-sd.com/Formulas/Mcculloch.htm (contains some errors).
    // Errors were corrected using the Masters coefficients from:
    //  http://worldpowerliftingcongress.com/wp-content/uploads/2015/02/Glossbrenner.htm
    // Ranges from age 41 to 80.
    1.010, // 41
    1.020, // 42
    1.031, // 43
    1.043, // 44
    1.055, // 45
    1.068, // 46
    1.082, // 47
    1.097, // 48
    1.113, // 49
    1.130, // 50
    1.147, // 51
    1.165, // 52
    1.184, // 53
    1.204, // 54
    1.225, // 55
    1.246, // 56
    1.268, // 57
    1.291, // 58
    1.315, // 59
    1.340, // 60
    1.366, // 61
    1.393, // 62
    1.421, // 63
    1.450, // 64
    1.480, // 65
    1.511, // 66
    1.543, // 67
    1.576, // 68
    1.610, // 69
    1.645, // 70
    1.681, // 71
    1.718, // 72
    1.756, // 73
    1.795, // 74
    1.835, // 75
    1.876, // 76
    1.918, // 77
    1.961, // 78
    2.005, // 79
    2.050, // 80
    // These coefficients taken from:
    // http://www.usapltwinportsrawopen.com/resources/USAPL+Age+Coefficients.pdf
    // Ranges from age 81 to 90.
    2.096, // 81
    2.143, // 82
    2.190, // 83
    2.238, // 84
    2.287, // 85
    2.337, // 86
    2.388, // 87
    2.440, // 88
    2.494, // 89
    2.549, // 90
    // Coefficients above 90 were just guessed at, and are unstandardized.
    2.605, // 91
    2.662, // 92
    2.720, // 93
    2.779, // 94
    2.839, // 95
    2.900, // 96
    2.962, // 97
    3.025, // 98
    3.089, // 99
    3.154, // 100
];

/// Calculates an appropriate age coefficient.
fn age_coeff(age: Age) -> f64 {
    match age {
        // Exact ages perform table lookup.
        Age::Exact(age) => {
            if age as usize > AGE_COEFFICIENTS.len() {
                AGE_COEFFICIENTS[AGE_COEFFICIENTS.len() - 1]
            } else {
                AGE_COEFFICIENTS[usize::from(age)]
            }
        }

        // Approximate ages round in the direction of least generosity.
        Age::Approximate(age) => {
            // Lifters around age 30 receive no handicap.
            if age < 30 {
                // For Juniors, assume the higher age.
                AGE_COEFFICIENTS[usize::from(age) + 1]
            } else {
                // For Masters, assume the lower age.
                if usize::from(age) > AGE_COEFFICIENTS.len() {
                    AGE_COEFFICIENTS[AGE_COEFFICIENTS.len() - 1]
                } else {
                    AGE_COEFFICIENTS[usize::from(age)]
                }
            }
        }

        // If no age is known, don't affect the score.
        Age::None => 1.0,
    }
}

/// Calculates McCulloch points.
///
/// "McCulloch" specifically refers to only a specific range of Masters age coefficients,
/// but the name was popularized by the USPA as the general term for Age-Adjusted Wilks.
pub fn mcculloch(sex: Sex, bodyweight: WeightKg, total: WeightKg, age: Age) -> Points {
    // Wilks coefficients are used directly to avoid Points boxing/unboxing overhead.
    let wilks_coefficient: f64 = match sex {
        Sex::M => wilks_coefficient_men(f64::from(bodyweight)),
        Sex::F => wilks_coefficient_women(f64::from(bodyweight)),
    };

    let age_coefficient: f64 = age_coeff(age);
    Points::from(wilks_coefficient * age_coefficient * f64::from(total))
}
