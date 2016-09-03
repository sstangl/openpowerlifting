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

# This is a combinator of the Foster age coefficient for Juniors
# and the McCulloch age coefficient for Masters.
# These coefficients are the same for men and women.
def ageCoeff(age): # Where age is an integer
    # These coefficients don't actually exist, and are just low-balled best guesses.
    # Kids really shouldn't be competing in this sport...
    if age is  5: return 1.73
    if age is  6: return 1.67
    if age is  7: return 1.61
    if age is  8: return 1.55
    if age is  9: return 1.49
    if age is 10: return 1.43
    if age is 11: return 1.38
    if age is 12: return 1.33
    if age is 13: return 1.28

    # Foster coefficients:
    # http://www.usapl-sd.com/Formulas/Foster.htm
    if age is 14: return 1.23
    if age is 15: return 1.18
    if age is 16: return 1.13
    if age is 17: return 1.08
    if age is 18: return 1.06
    if age is 19: return 1.04
    if age is 20: return 1.03
    if age is 21: return 1.02
    if age is 22: return 1.01

    # 23-40 don't receive a handicap.
    if age > 22 and age < 41: return 1.00

    # McCulloch coefficients:
    # http://www.usapl-sd.com/Formulas/Mcculloch.htm
    if age is 41: return 1.01
    if age is 42: return 1.02
    if age is 43: return 1.031
    if age is 44: return 1.043
    if age is 45: return 1.055
    if age is 46: return 1.068
    if age is 47: return 1.082
    if age is 48: return 1.097
    if age is 49: return 1.113
    if age is 50: return 1.130
    if age is 51: return 1.147
    if age is 52: return 1.165
    if age is 53: return 1.184
    if age is 54: return 1.204
    if age is 55: return 1.225
    if age is 56: return 1.246
    if age is 57: return 1.258
    if age is 58: return 1.292
    if age is 59: return 1.315
    if age is 60: return 1.340
    if age is 61: return 1.366
    if age is 62: return 1.393
    if age is 63: return 1.421
    if age is 64: return 1.450
    if age is 65: return 1.480
    if age is 66: return 1.511
    if age is 67: return 1.543
    if age is 68: return 1.578
    if age is 69: return 1.610
    if age is 70: return 1.645
    if age is 71: return 1.681
    if age is 72: return 1.718
    if age is 73: return 1.756
    if age is 74: return 1.795
    if age is 75: return 1.835
    if age is 76: return 1.876
    if age is 77: return 1.918
    if age is 78: return 1.961
    if age is 79: return 2.005
    if age is 80: return 2.050

    # These coefficients taken from:
    # http://www.usapltwinportsrawopen.com/resources/USAPL+Age+Coefficients.pdf
    if age is 81: return 2.096
    if age is 82: return 2.143
    if age is 83: return 2.190
    if age is 84: return 2.238
    if age is 85: return 2.287
    if age is 86: return 2.337
    if age is 87: return 2.388
    if age is 88: return 2.440
    if age is 89: return 2.494
    if age is 90: return 2.549

    print("Missing age coefficient for age %d" % age, file=sys.stderr)
    return 0.0


def wilks(isMale, bodyweightKg, totalKg):
    if isMale:
        return wilksCoeffMen(bodyweightKg) * totalKg
    return wilksCoeffWomen(bodyweightKg) * totalKg

def mcculloch(isMale, age, bodyweightKg, totalKg):
    return ageCoeff(age) * wilks(isMale, bodyweightKg, totalKg)
