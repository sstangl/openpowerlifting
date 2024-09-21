#!/usr/bin/env python3
#
# Helper script that reformats the State column to use proper abbreviations.

import sys
import os

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv

# Map of substitutions.
STATE_MAP = {
    "NW": "NRW",
}


def standardize_state_csv(csv):
    '''Standardizes the State column.
       Returns true iff something was changed.'''
    global STATE_MAP

    if 'State' not in csv.fieldnames:
        return False
    idx = csv.index('State')

    changed = False
    for row in csv.rows:
        state = row[idx]
        if state in STATE_MAP:
            row[idx] = STATE_MAP[state]
            changed = True
        elif state.upper() in STATE_MAP:
            row[idx] = STATE_MAP[state.upper()]
            changed = True

    return changed


def standardize_state_filename(filename):
    csv = Csv(filename)
    if standardize_state_csv(csv):
        csv.write_filename(filename)


if __name__ == '__main__':
    if len(sys.argv) > 1:
        standardize_state_filename(sys.argv[1])
    else:
        for dirname, subdirs, files in os.walk(os.getcwd()):
            if "meet-data" in subdirs:
                subdirs[:] = ['meet-data']
            if 'entries.csv' in files:
                filepath = dirname + os.sep + 'entries.csv'
                standardize_state_filename(filepath)
