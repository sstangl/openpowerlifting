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


# Map of substitutions.
DIVISION_MAP = {
    "Under11": "Youth",
    "Under 11": "Youth",
    "12-13": "Teen 12-13",
    "12*13": "Teen 12-13",
    "14-15": "Teen 14-15",
    "16-17": "Teen 16-17",
    "18-19": "Teen 18-19",
    "20-24": "Juniors 20-24",
    "30-34": "Seniors 30-34",
    "35-39": "Submasters 35-39",
    "40-44": "Masters 40-44",
    "45-49": "Masters 45-49",
    "50-54": "Masters 50-54",
    "55-59": "Masters 55-59",
    "60-64": "Masters 60-64",
    "65-69": "Masters 65-69",
    "70-74": "Masters 70-74",
    "75-79": "Masters 75-79",
    "P/F/M": "Law/Fire/Military"
}


def standardize_division_csv(csv):
    '''Standardizes the Division column.
       Returns true iff something was changed.'''

    if 'Division' not in csv.fieldnames:
        return False
    idx = csv.index('Division')

    changed = False
    for row in csv.rows:
        division = row[idx]
        if division in DIVISION_MAP:
            row[idx] = DIVISION_MAP[division]
            changed = True
        elif division.upper() in DIVISION_MAP:
            row[idx] = DIVISION_MAP[division.upper()]
            changed = True

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
