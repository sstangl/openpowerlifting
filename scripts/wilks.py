#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Helper library for Wilks calculation.
#

import sys

def wilksCoeff(a, b, c, d, e, f, x):
    return 500 / (a + b*x + c*x**2 + d*x**3 + e*x**4 + f*x**5)

def wilksCoeffMen(x): # Where x is BodyweightKg.
    a = -216.0475144
    b = 16.2606339
    c = -0.002388645
    d = -0.00113732
    e = 7.01863E-06
    f = -1.291E-08
    return wilksCoeff(a, b, c, d, e, f, x)

def wilksCoeffWomen(x): # Where x is BodyweightKg.
    a = 594.31747775582
    b = -27.23842536447
    c = 0.82112226871
    d = -0.00930733913
    e = 0.00004731582
    f = -0.00000009054
    return wilksCoeff(a, b, c, d, e, f, x)

# These coefficients don't actually exist, and are just low-balled best guesses.
# Kids really shouldn't be competing in this sport...
# Ranges from age 5 to 13
PRETEEN_COEFF = [
    1.73,
    1.67,
    1.61,
    1.55,
    1.49,
    1.43,
    1.38,
    1.33,
    1.28    
]

# Foster coefficients:
# http://www.usapl-sd.com/Formulas/Foster.htm
# Ranges from age 14 to 22
FOSTER_COEFF = [ 
    1.23,
    1.18,
    1.13,
    1.08,
    1.06,
    1.04,
    1.03,
    1.02,
    1.01
]

# McCulloch coefficients:
# http://www.usapl-sd.com/Formulas/Mcculloch.htm
# Ranges from age 41 to 80
MCCULLOCH_COEFF = [
    1.01,
    1.02,
    1.031,
    1.043,
    1.055,
    1.068,
    1.082,
    1.097,
    1.113,
    1.130,
    1.147,
    1.165,
    1.184,
    1.204,
    1.225,
    1.246,
    1.258,
    1.292,
    1.315,
    1.340,
    1.366,
    1.393,
    1.421,
    1.450,
    1.480,
    1.511,
    1.543,
    1.578,
    1.610,
    1.645,
    1.681,
    1.718,
    1.756,
    1.795,
    1.835,
    1.876,
    1.918,
    1.961,
    2.005,
    2.050
]

# These coefficients taken from:
# http://www.usapltwinportsrawopen.com/resources/USAPL+Age+Coefficients.pdf
# Ranges from age 81 to 90
EIGHTIES_COEF = [
    2.096,
    2.143,
    2.190,
    2.238,
    2.287,
    2.337,
    2.388,
    2.440,
    2.494,
    2.549
]

# This is a combinator of the Foster age coefficient for Juniors
# and the McCulloch age coefficient for Masters.
# These coefficients are the same for men and women.
def ageCoeff(age): # Where age is an integer
    if age < 5:
        pass
    elif age < 14:
        return PRETEEN_COEFF[age - 5]
    elif age < 23:
        return FOSTER_COEFF[age - 14]
    elif age < 41:
        # 23-40 don't receive a handicap.
        return 1.00
    elif age < 81:
        return MCCULLOCH_COEFF[age - 41]
    elif age < 91:
        return EIGHTIES_COEF[age - 81]

    print("Missing age coefficient for age %d" % age, file=sys.stderr)
    return 0.0


def wilks(isMale, bodyweightKg, totalKg):
    if isMale:
        return wilksCoeffMen(bodyweightKg) * totalKg
    return wilksCoeffWomen(bodyweightKg) * totalKg

def mcculloch(isMale, age, bodyweightKg, totalKg):
    return ageCoeff(age) * wilks(isMale, bodyweightKg, totalKg)
