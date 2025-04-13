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
    "13-15": "Juniors 13-15",
    "16-17": "Juniors 16-17",
    "18-19": "Juniors 18-19",
    "20-23": "Juniors 20-23",
    "35-39": "Submasters 35-39",
    "40-44": "Masters 40-44",
    "45-49": "Masters 45-49",
    "50-54": "Masters 50-54",
    "55-59": "Masters 55-59",
    "60-64": "Masters 60-64",
    "65-69": "Masters 65-69",
    "70-74": "Masters 70-74",
    "75-79": "Masters 75-79",
    "Junior 13-15": "Juniors 13-15",
    "Junior 16-17": "Juniors 16-17",
    "Junior 18-19": "Juniors 18-19",
    "Junior 20-23": "Juniors 20-23",
    "Submaster 35-39": "Submasters 35-39",
    "Master 40-44": "Masters 40-44",
    "Master 45-49": "Masters 45-49",
    "Master 50-54": "Masters 50-54",
    "Master 55-59": "Masters 55-59",
    "Master 60-64": "Masters 60-64",
    "Master 65-69": "Masters 65-69",
    "Master 70-74": "Masters 70-74",
    "Master 75-79": "Masters 75-79",
    "Master 80+": "Masters 80+",
    "Master80+": "Masters 80+"
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
