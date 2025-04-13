#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Helper library for Wilks, Schwartz/Malone & Glossbrenner calculation.
#

import math


def wilksCoeff(a, b, c, d, e, f, x):
    return 500 / (a + b * x + c * x**2 + d * x**3 + e * x**4 + f * x**5)


def wilksCoeffMen(x):  # Where x is BodyweightKg.
    a = -216.0475144
    b = 16.2606339
    c = -0.002388645
    d = -0.00113732
    e = 7.01863E-06
    f = -1.291E-08
    x = min(x, 201.9)  # Upper bound to avoid asymptote.
    x = max(x, 40)  # Lower bound to avoid children with huge wilks
    return wilksCoeff(a, b, c, d, e, f, x)


def wilksCoeffWomen(x):  # Where x is BodyweightKg.
    a = 594.31747775582
    b = -27.23842536447
    c = 0.82112226871
    d = -0.00930733913
    e = 0.00004731582
    f = -0.00000009054
    x = min(x, 154.53)  # Cap to avoid asymptote.
    x = max(x, 26.51)  # Lower bound to avoid children with huge wilks
    return wilksCoeff(a, b, c, d, e, f, x)


def smCoeff(a, b, c, x):
    return a*x**b + c


def schwartzCoeff(x):  # Where x is BodyweightKg.
    # Values calculated by fitting to coefficient tables
    a = 3565.902903983125
    b = -2.244917050872728
    c = 0.445775838479913

    # Arbitrary choice of lower bound
    x = max(x, 40)
    return smCoeff(a, b, c, x)


def maloneCoeff(x):  # Where x is BodyweightKg.
    # Values calculated by fitting to coefficient tables
    a = 106.0115863236130
    b = -1.293027130579051
    c = 0.322935585328304

    # Need a lower bound somewhere, so chose when Malone=max(Wilks)
    x = max(x, 29.24)
    return smCoeff(a, b, c, x)


def glossCoeffMen(x):  # Where x is BodyweightKg.
    # Linear coefficients found by fitting to table
    a = -0.000821668402557
    b = 0.676940740094416

    if x < 153.05:  # Gloss function is defined piecewise
        return (schwartzCoeff(x) + wilksCoeffMen(x))/2
    return (schwartzCoeff(x) + a*x + b)/2


def glossCoeffWomen(x):  # Where x is BodyweightKg.
    # Linear coefficients found by fitting to table
    a = -0.000313738002024
    b = 0.852664892884785

    if x < 106.3:  # Gloss function is defined piecewise
        return (maloneCoeff(x) + wilksCoeffWomen(x))/2
    return (maloneCoeff(x) + a*x + b)/2


# Array of age coefficients, such that AGE_COEFFICIENTS[age]
# is the coefficient for that age.
AGE_COEFFICIENTS = [
    # Coefficients in the range of 0-5 are clearly nonsense.
    0.0,  # 0
    0.0,  # 1
    0.0,  # 2
    0.0,  # 3
    0.0,  # 4

    # These coefficients don't actually exist, and are just low-balled best guesses.
    # Kids really shouldn't be competing in this sport...
    # Ranges from age 5 to 13
    1.73,  # 5
    1.67,  # 6
    1.61,  # 7
    1.55,  # 8
    1.49,  # 9
    1.43,  # 10
    1.38,  # 11
    1.33,  # 12
    1.28,  # 13

    # Foster coefficients:
    # http://www.usapl-sd.com/Formulas/Foster.htm
    # Ranges from age 14 to 22
    1.23,  # 14
    1.18,  # 15
    1.13,  # 16
    1.08,  # 17
    1.06,  # 18
    1.04,  # 19
    1.03,  # 20
    1.02,  # 21
    1.01,  # 22

    # Lifters in the range 23-40 receive no handicap.
    1.000,  # 23
    1.000,  # 24
    1.000,  # 25
    1.000,  # 26
    1.000,  # 27
    1.000,  # 28
    1.000,  # 29
    1.000,  # 30
    1.000,  # 31
    1.000,  # 32
    1.000,  # 33
    1.000,  # 34
    1.000,  # 35
    1.000,  # 36
    1.000,  # 37
    1.000,  # 38
    1.000,  # 39
    1.000,  # 40

    # McCulloch coefficients:
    #  http://www.usapl-sd.com/Formulas/Mcculloch.htm (contains some errors).
    # Errors were corrected using the Masters coefficients from:
    #  http://worldpowerliftingcongress.com/wp-content/uploads/2015/02/Glossbrenner.htm
    # Ranges from age 41 to 80.
    1.010,  # 41
    1.020,  # 42
    1.031,  # 43
    1.043,  # 44
    1.055,  # 45
    1.068,  # 46
    1.082,  # 47
    1.097,  # 48
    1.113,  # 49
    1.130,  # 50
    1.147,  # 51
    1.165,  # 52
    1.184,  # 53
    1.204,  # 54
    1.225,  # 55
    1.246,  # 56
    1.268,  # 57
    1.291,  # 58
    1.315,  # 59
    1.340,  # 60
    1.366,  # 61
    1.393,  # 62
    1.421,  # 63
    1.450,  # 64
    1.480,  # 65
    1.511,  # 66
    1.543,  # 67
    1.576,  # 68
    1.610,  # 69
    1.645,  # 70
    1.681,  # 71
    1.718,  # 72
    1.756,  # 73
    1.795,  # 74
    1.835,  # 75
    1.876,  # 76
    1.918,  # 77
    1.961,  # 78
    2.005,  # 79
    2.050,  # 80

    # These coefficients taken from:
    # http://www.usapltwinportsrawopen.com/resources/USAPL+Age+Coefficients.pdf
    # Ranges from age 81 to 90
    2.096,  # 81
    2.143,  # 82
    2.190,  # 83
    2.238,  # 84
    2.287,  # 85
    2.337,  # 86
    2.388,  # 87
    2.440,  # 88
    2.494,  # 89
    2.549,  # 90

    # Coefficients above 90 were just guessed at, and are unstandardized.
    2.605,  # 91
    2.662,  # 92
    2.720,  # 93
    2.779,  # 94
    2.839,  # 95
    2.900,  # 96
    2.962,  # 97
    3.025,  # 98
    3.089,  # 99
    3.154,  # 100
]
assert len(AGE_COEFFICIENTS) == 101

# This is a combinator of the Foster age coefficient for Juniors
# and the McCulloch age coefficient for Masters.
# These coefficients are the same for men and women.


def ageCoeff(age):  # Where age is an integer
    if age >= len(AGE_COEFFICIENTS):
        return 0.0
    return AGE_COEFFICIENTS[age]


def wilks(isMale, bodyweightKg, totalKg):
    if isMale:
        return wilksCoeffMen(bodyweightKg) * totalKg
    return wilksCoeffWomen(bodyweightKg) * totalKg


def mcculloch(isMale, age, bodyweightKg, totalKg):
    return ageCoeff(age) * wilks(isMale, bodyweightKg, totalKg)


def schwartzmalone(isMale, bodyweightKg, totalKg):
    if isMale:
        return schwartzCoeff(bodyweightKg) * totalKg
    return maloneCoeff(bodyweightKg) * totalKg


def glossbrenner(isMale, bodyweightKg, totalKg):
    if isMale:
        return glossCoeffMen(bodyweightKg) * totalKg
    return glossCoeffWomen(bodyweightKg) * totalKg


IPF_COEFFICIENTS = {
    'M': {
        'Raw': {
            'SBD': [310.67, 857.785, 53.216, 147.0835],
            'S': [123.1000, 363.0850, 25.1667, 75.4311],
            'B': [86.4745, 259.155, 17.57845, 53.122],
            'D': [103.5355, 244.7650, 15.3714, 31.5022]
        },
        'Single-ply': {
            'SBD': [387.265, 1121.28, 80.6324, 222.4896],
            'S': [150.4850, 446.4450, 36.5155, 103.7061],
            'B': [133.94, 441.465, 35.3938, 113.0057],
            'D': [110.1350, 263.6600, 14.9960, 23.0110]
        }
    },
    'F': {
        'Raw': {
            'SBD': [125.1435, 228.03, 34.5246, 86.8301],
            'S': [50.4790, 105.6320, 19.1846, 56.2215],
            'B': [25.0485, 43.848, 6.7172, 13.952],
            'D': [47.1360, 67.3490, 9.1555, 13.6700]
        },
        'Single-ply': {
            'SBD': [176.58, 373.315, 48.4534, 110.0103],
            'S': [74.6855, 171.5850, 21.9475, 52.2948],
            'B': [49.106, 124.209, 23.199, 67.492],
            'D': [51.0020, 69.8265, 8.5802, 5.7258]
        }
    }
}


def ipf(sex, equipment, event, bodyweightKg, totalKg):
    # The IPF set lower bounds beyond which points are undefined.
    if bodyweightKg < 40 or totalKg <= 0:
        return 0

    # Normalize equipment to (Raw, Single-ply).
    if equipment == 'Wraps' or equipment == 'Straps':
        equipment = 'Raw'
    elif equipment == 'Multi-ply':
        equipment = 'Single-ply'

    # The IPF formula is only defined for some parameters.
    if equipment not in ['Raw', 'Single-ply']:
        return 0
    if event not in ['SBD', 'S', 'B', 'D']:
        return 0
    if sex not in ['M', 'F']:
        return 0

    # Look up parameters.
    [mean1, mean2, dev1, dev2] = IPF_COEFFICIENTS[sex][equipment][event]

    # Calculate the properties of the normal distribution.
    bwLog = math.log(bodyweightKg)
    mean = mean1 * bwLog - mean2
    dev = dev1 * bwLog - dev2

    # Prevent division by zero.
    if dev == 0.0:
        return 0

    return max(0, 500 + 100 * (totalKg - mean) / dev)
