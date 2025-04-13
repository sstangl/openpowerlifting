#!/usr/bin/env python3
#
# Tries to create a well-formatted Division column from surrounding information.

import sys
import os

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv


# Map from in-file division to final division
DIVMAP = {
    "O": "O",
    "J": "J",
    "T1": "T1",
    "T2": "T2",
    "T3": "T3",
    "M1": "M1",
    "M2": "M2",
    "M3": "M3",
    "M4": "M4",
    "M5": "M5",
    "M6": "M6",
    "M7": "M7",
    "M8": "M8",
    "M9": "M9",

    "T13-15": "T1",
    "T16-17": "T2",
    "T18-19": "T3",
    "Juniors": "J",
    "40-44": "M1",
    "45-49": "M2",
    "50-54": "M3",
    "55-59": "M4",
    "60-64": "M5",
    "65-69": "M6",
    "70-74": "M7",
    "75-79": "M8",
    "80-84": "M9"
}

EQPMAP = {
    "Raw": "R",
    "Wraps": "CR",
    "Single-ply": "ES",
    "Multi-ply": "EM"
}


def main(filename):
    csv = oplcsv.Csv(filename)
    csv.append_column('NewDivision')

    newdividx = csv.index('NewDivision')
    sexidx = csv.index('Sex')
    dividx = csv.index('Division')
    eqpidx = csv.index('Equipment')

    has_tested = 'Tested' in csv.fieldnames

    for row in csv.rows:
        sex = row[sexidx]
        div = row[dividx]
        eqp = EQPMAP[row[eqpidx]]
        fed = "WPC"

        if has_tested:
            testedidx = csv.index('Tested')
            if row[testedidx] == 'Yes':
                fed = 'AWPC'

        if div not in DIVMAP:
            print("DIVMAP is missing '%s'" % div)
            return

        div = DIVMAP[div]

        if len(div) == 2:
            row[newdividx] = "%s_%s%s_%s_%s" % (sex, div[0], eqp, div[1], fed)
        else:
            row[newdividx] = "%s_%s%s_%s" % (sex, div, eqp, fed)

    csv.write_filename(filename)


if __name__ == "__main__":
    main(sys.argv[1])
