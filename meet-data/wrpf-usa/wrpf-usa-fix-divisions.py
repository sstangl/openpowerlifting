#!/usr/bin/env python3
#
# Helper script that makes known division transformations to all
# entries.csv files in the given tree.
#
# When run as a script, it fixes the "Division" columns.
#
# This is also set up to be importable as a library, so that importation
# scripts can use it directly.

import sys
import os

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv


# Map of substitutions, after some computation has been applied.
DIVISION_MAP = {
    "Open": "Open",
    "Junior (13-15)": "J13-15",
    "Junior (16-17)": "J16-17",
    "Junior (18-19)": "J18-19",
    "Junior (20-23)": "J20-23",
    "Sub-Master (35-39)": "S35-39",
    "Master (40-44)": "M40-44",
    "Master (45-49)": "M45-49",
    "Master (50-54)": "M50-54",
    "Master (55-59)": "M55-59",
    "Master (60-64)": "M60-64",
    "Master (65-69)": "M65-69",
    "Master (70-74)": "M70-74",
    "Master (75-79)": "M75-79",
}


# Given a division like "Men's Drug Tested Raw Open", make that just "Open".
#
# This is standardizing LiftingCast output, primarily.

def remove_common_elements(division):
    division = division.replace("Men's ", "")
    division = division.replace("Women's ", "")
    division = division.replace("Drug Tested ", "")
    division = division.replace("Untested ", "")
    division = division.strip()

    division = division.replace("Classic Raw ", "")
    division = division.replace("Raw ", "")
    division = division.replace("With Wraps ", "")
    division = division.replace("Single Ply ", "")
    division = division.strip()

    division = division.replace("Squat Only", "")
    division = division.replace("Bench Only", "")
    division = division.replace("Deadlift Only", "")
    division = division.replace("Push/Pull", "")
    division = division.strip()

    return division


def standardize_division_csv(csv):
    '''Standardizes the Division column.
       Returns true iff something was changed.'''
    global DIVISION_MAP

    if 'Division' not in csv.fieldnames:
        return False
    idx = csv.index('Division')

    changed = False
    for row in csv.rows:
        division = row[idx]
        if division in DIVISION_MAP:
            row[idx] = DIVISION_MAP[division]
            changed = True
        elif remove_common_elements(division) in DIVISION_MAP:
            row[idx] = DIVISION_MAP[remove_common_elements(division)]
            changed = True
        elif division.upper() in DIVISION_MAP:
            row[idx] = DIVISION_MAP[division.upper()]
            changed = True

        # This uses PLU/USPC format, which I think looks nicer.
        if "Tested" in division:
            row[idx] = row[idx] + "T"

    return changed


def standardize_division_filename(filename):
    csv = oplcsv.Csv(filename)
    if standardize_division_csv(csv):
        csv.write_filename(filename)


if __name__ == '__main__':
    if len(sys.argv) > 1:
        standardize_division_filename(sys.argv[1])
    else:
        for dirname, subdirs, files in os.walk(os.getcwd()):
            if "meet-data" in subdirs:
                subdirs[:] = ['meet-data']
            if 'entries.csv' in files:
                filepath = dirname + os.sep + 'entries.csv'
                standardize_division_filename(filepath)
