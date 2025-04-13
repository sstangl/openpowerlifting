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
    "Amateur Juniors": "Amateur Juniors 20-23",
    "Am Open": "Amateur Open",
    "Amateur Masters (40-44)": "Amateur Masters 40-44",
    "Amateur Masters (50-54)": "Amateur Masters 50-54",
    "Pro Juniors": "Pro Juniors 20-23",
    "Pro Submasters": "Pro Submasters 33-39",
    "Pro SubMasters": "Pro Submasters 33-39",
    "Amateur SubMasters": "Amateur Submasters 33-39",
    "Amateur SubMasters 33-39": "Amateur Submasters 33-39",
    "Amateur Submasters": "Amateur Submasters 33-39",
    "Amateur Masters 80-84": "Amateur Masters 80+",
    "Professional Open": "Pro Open",
    "Professional Juniors": "Pro Juniors 20-23",
    "Amateur Masters 44-49": "Amateur Masters 45-49",
    "Pro SubMasters 33-39": "Pro Submasters 33-39",
    "Pro Police Open": "Pro Police",
    "Am Police": "Amateur Police",
    "Am Police/ Corrections": "Amateur Police",
    "Am Sub Masters": "Amateur Submasters 33-39",
    "Am Juniors": "Amateur Juniors 20-23",
    "Am Teen 14-15": "Amateur Teen 14-15",
    "Am Teen 16-17": "Amateur Teen 16-17",
    "Am Masters 40-44": "Amateur Masters 40-44",
    "Am Masters 45-49": "Amateur Masters 45-49",
    "Am Masters 50-54": "Amateur Masters 50-54",
    "Am Teen 18-19": "Amateur Teen 18-19",
    "Amateur Up to 13": "Amateur Teen 13",
    "Amateur Teen Up to 13 Years": "Amateur Teen 13",
    "Amateur Teen Up to 13 years": "Amateur Teen 13",
    "Amateur 16-17": "Amateur Teen 16-17",
    "Amateur 18-19": "Amateur Teen 18-19",
    "50-59": "Pro Masters 50-59",
    "18-19": "Pro Teen 18-19",
    "70-79": "Pro Masters 70-79",
    "16-17": "Pro Teen 16-17",
    "14-15": "Pro Teen 14-15",
    "60-69": "Pro Masters 60-69",
    "0-13": "Pro Teen 13",
    "Open Am": "Amateur Open",
    "Open Pro": "Pro Open",
    "Masters 40-44 Pro": "Pro Masters 40-44",
    "Open": "Pro Open",
    "Juniors": "Pro Juniors 20-23",
    "Masters 40-44": "Pro Masters 40-44",
    "Masters 40-49": "Pro Masters 40-49",
    "Masters 45-49": "Pro Masters 45-49",
    "Masters 50-54": "Pro Masters 50-54",
    "Masters 55-59": "Pro Masters 55-59",
    "Masters 60-64": "Pro Masters 60-64",
    "Masters 65-69": "Pro Masters 65-69",
    "Masters 70-74": "Pro Masters 70-74",
    "Masters 75-79": "Pro Masters 75-79",
    "Pro Junior": "Pro Juniors 20-23",
    "Am Teen  18-19": "Amateur Teen 18-19",
    "Police": "Pro Police",
    "Am Submasters": "Amateur Submasters 33-39",
    "Amateur Teen up to 13": "Amateur Teen 13",
    "Teen 18-19": "Pro Teen 18-19",
    "Amateur Teen Up to 13": "Amateur Teen 13",
    "Amateur Youth": "Amateur Teen 13",
    "Elite Am Open": "Elite Amateur Open",
    "E Am Open": "Elite Amateur Open",
    "Am Elite Open": "Elite Amateur Open",
    "Elite Open": "Elite Amateur Open",
    "E. Am Open": "Elite Amateur Open",
    "E. Am Juniors": "Elite Amateur Juniors 20-23",
    "Am Sub": "Amateur Submasters 33-39",
    "Am Masters": "Amateur Masters",
    "Amateur Junior": "Amateur Juniors 20-23",
    "Amateur Teen 13-15": "Amateur Teen 14-15",
    "M-JrR": "Pro Junior 20-23",
    "M-SmE": "Pro Submasters 33-39",
    "M-SmR": "Pro Submasters 33-39",
    "M-OpR": "Pro Open",
    "M-T3R": "Pro Teen 18-19",
    "Pro Junior 20-23": "Pro Juniors 20-23",
    "M-OpE": "Pro Open",
    "Am Teen": "Amateur Teen",
    "Teen": "Pro Teen",
    "Masters": "Pro Masters",
    "Pro Master 45-49": "Pro Masters 45-49",
    "Amateur Submaster 35-39": "Amateur Submasters 35-39",
    "Amateur Master 50-54": "Amateur Masters 50-54",
    "Law/Fire": "Pro Police",
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

        # Remove parentheses.
        if '(' in division or ')' in division:
            division = division.replace('(', ' ')
            division = division.replace(')', ' ')
            division = division.replace('  ', ' ')
            division = division.strip()
            row[idx] = division
            changed = True

        # Consult the map.
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
