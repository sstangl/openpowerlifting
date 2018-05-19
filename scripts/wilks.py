#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Helper library for Wilks calculation.
#


def wilksCoeff(a, b, c, d, e, f, x):
    return 500 / (a + b * x + c * x**2 + d * x**3 + e * x**4 + f * x**5)


def wilksCoeffMen(x):  # Where x is BodyweightKg.
    a = -216.0475144
    b = 16.2606339
    c = -0.002388645
    d = -0.00113732
    e = 7.01863E-06
    f = -1.291E-08
    if x < 201.9:
        return wilksCoeff(a, b, c, d, e, f, x)
    else:
        return wilksCoeff(a, b, c, d, e, f, 201.9)        


def wilksCoeffWomen(x):  # Where x is BodyweightKg.
    a = 594.31747775582
    b = -27.23842536447
    c = 0.82112226871
    d = -0.00930733913
    e = 0.00004731582
    f = -0.00000009054
    if x < 154.53:
        return wilksCoeff(a, b, c, d, e, f, x)
    else:
        return wilksCoeff(a, b, c, d, e, f, 154.53)


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
