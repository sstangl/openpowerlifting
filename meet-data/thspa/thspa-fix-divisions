#!/usr/bin/env python3

import oplcsv
import sys
import os


# Map of substitutions.
DIVISION_MAP = {
    "Freshmen Boys": "Freshmen",
    "FRESHMEN BOYS": "Freshmen",
    "Boys Ninth Grade": "Freshmen",
    "Boys Ninth grade": "Freshmen",
    "Boys Freshman": "Freshmen",
    "Boys Freshmen": "Freshmen",
    "Boys-Freshmen Division": "Freshmen",
    "Boys Ninth": "Freshmen",
    "BOYS FRESHMEN": "Freshmen",
    "BOYS FRESHMAN": "Freshmen",
    "Freshman Boys": "Freshmen",
    "BOYS": "Boys",
    "High School Boys": "Boys",
    "Rookies": "Boys",
    "BOYS DIVISION": "Boys",
    "Green": "Boys",
    "White": "Boys",
    "Boys Division": "Boys",
    "Boys JV": "Junior Varsity",
    "J.V. BOYS": "Junior Varsity",
    "Boys J.V.": "Junior Varsity",
    "Junior Varsity Boys": "Junior Varsity",
    "JV": "Junior Varsity",
    "Boys-JV": "Junior Varsity",
    "JV Boys Division": "Junior Varsity",
    "Boys JV Division": "Junior Varsity",
    "JV Boys": "Junior Varsity",
    "JV Boys (B Team)": "Junior Varsity",
    "JV BOYS": "Junior Varsity",
    "Boys (Junior Varsity)": "Junior Varsity",
    "Juniors Varsity": "Junior Varsity",
    "Boys Varsity": "Varsity",
    "Varsity Boys": "Varsity",
    "VAR BOYS": "Varsity",
    "Boys- Varsity": "Varsity",
    "Boys (Varsity)": "Varsity",
    "Boys Varsiity": "Varsity",
    "VARSITY BOYS": "Varsity",
    "Boys V": "Varsity",
    "Boys Division 1": "Div 1",
    "Division 1": "Div 1",
    "Div 2 Boys": "Div 2",
    "2 Region 6": "Div 2",
    "DIVISION 2": "Div 2",
    "Div 2 (White)": "Div 2",
    "Boys Div 2": "Div 2",
    "Boys Div 3": "Div 3",
    "Div 3 (Green)": "Div 3",

    # Divisions for Girls (lets this script be used for THSWPA).
    "THSPA Girls": "Girls",
    "GIRLS": "Girls",
    "Varsity Girls": "Varsity",
    "Girls Varsity": "Varsity",
    "JV Girls": "Junior Varsity",
    "Junior Varsity Girls": "Junior Varsity",
    "Girls Division 1": "Div 1",
    "Girls Ninth Grade": "Freshmen",
    "Girls Ninth grade": "Freshmen",
    "Girls J.V.": "Junior Varsity",
    "Girls Ninth": "Freshmen",
    "Girls JV": "Junior Varsity",
    "Region 5 Division 2": "Div 2",
    "Region 5 Division 3": "Div 3",
    "Division 2": "Div 2",
    "Girls Division 2": "Div 2",
    "Division 3": "Div 3"
}


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
            continue
        elif division.upper() in DIVISION_MAP:
            row[idx] = DIVISION_MAP[division.upper()]
            changed = True
            continue

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
