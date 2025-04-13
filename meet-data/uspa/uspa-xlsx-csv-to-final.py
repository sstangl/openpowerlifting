#!/usr/bin/env python3
#
# Parses the new-style USPA format (including attempts) to our entries.csv format.
# The input CSV comes from the XLSX that Steve Denison sends in: therefore we don't
# have column alignment issues.
#
# This script is called by the "uspa-xlsx" script.
#

import os
import sys

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv


FIELDNAME_MAP = {
    "": "",
    "Name": "Name",
    "Country": "Country",
    "State": "State",
    "Class": "WeightClassKg",
    "Weight": "BodyweightKg",
    "Age": "Age",
    "SQ1": "Squat1Kg",
    "SQ2": "Squat2Kg",
    "SQ3": "Squat3Kg",
    "BP1": "Bench1Kg",
    "BP2": "Bench2Kg",
    "BP3": "Bench3Kg",
    "DL1": "Deadlift1Kg",
    "DL2": "Deadlift2Kg",
    "DL3": "Deadlift3Kg",
    "Total Kg": "TotalKg",
    "Wilks Total": "DELETEME",
    "Dots Total": "DELETEME",
    "McC Total": "DELETEME"
}


def fixup_fieldnames(csv):
    for (i, field) in enumerate(csv.fieldnames):
        csv.fieldnames[i] = FIELDNAME_MAP[field]
    csv.fieldnames[0] = 'Place'


def fixup_lifts(csv):
    for row in csv.rows:
        for (j, cell) in enumerate(row):
            # Skipped lifts have a bunch of dashes, usually 5 or 6.
            if cell.startswith('---'):
                row[j] = ''

            elif cell == '0' or cell == '0.0' or cell == '0.00':
                row[j] = ''

            # Remove unnecessary ".0"
            elif cell.endswith(".0"):
                row[j] = cell[:-2]


# Removes textual info often added by meet directors to the bottom of the spreadsheet
def remove_extra_info(csv):
    found_index = 0
    titles = ['best lifters', 'meet director']

    for (i, row) in enumerate(csv.rows):
        if any(title in row[1].lower() for title in titles):
            found_index = i
            break

    if found_index > 0:
        csv.rows = csv.rows[:found_index]


def integrate_4ths(csv):
    csv.append_columns(["Squat4Kg", "Bench4Kg", "Deadlift4Kg"])

    for (i, row) in enumerate(csv.rows):
        for (j, cell) in enumerate(row):
            if "4th" in cell or \
                    (row[0] == '' and cell.startswith('(') and cell.endswith(')')):
                # Appears like "4th: 122.5"
                fourth = cell.replace("4th: ", "")

                # Sometimes appears like "(122.5)"
                fourth = cell.replace("(", "").replace(")", "")

                # Look vertically to see what kind of a column this is.
                fieldname = csv.fieldnames[j]
                target = ''
                if 'Squat' in fieldname:
                    target = 'Squat4Kg'
                elif 'Bench' in fieldname:
                    target = 'Bench4Kg'
                elif 'Deadlift' in fieldname:
                    target = 'Deadlift4Kg'
                else:
                    raise Exception("Unknown column type '%s'" % fieldname)

                # The lift gets credited to the lifter in the previous row.
                assert i > 0
                csv.rows[i-1][csv.index(target)] = fourth


DIVISION_MAP = {
    "Jr": "Youth",
    "JR": "Youth",
    "Jr 10-12": "Juniors 10-12",
    "Jr 13-15": "Juniors 13-15",
    "Jr 16-17": "Juniors 16-17",
    "Junior 16-17": "Juniors 16-17",
    "Jr 18-19": "Juniors 18-19",
    "Jr 20-23": "Juniors 20-23",
    "Junior 20-23": "Juniors 20-23",
    "Jr 15-19": "Juniors 15-19",
    "Open": "Open",
    "Submaster": "Submasters 35-39",
    "Master 40-44": "Masters 40-44",
    "Master 45-49": "Masters 45-49",
    "Master 50-54": "Masters 50-54",
    "Master 55-59": "Masters 55-59",
    "Master 60-64": "Masters 60-64",
    "Master 65-69": "Masters 65-69",
    "Master 70-74": "Masters 70-74",
    "Master 75-79": "Masters 75-79",
    "Master 80+": "Masters 80+"
}


def integrate_stateful_data(csv):
    csv.append_columns(["Event", "Equipment", "Sex", "Division"])

    state_event = ''
    state_equipment = ''
    state_sex = ''
    state_division = ''

    for row in csv.rows:
        # Rows that change state put their information in the Name(1) column.
        if row[0] == '' and row[1] != '':
            # This is a stateful row. There are two kinds:
            # 1) Competition rows, like "Women Classic Raw Powerlifting"
            #    These always begin with "Women" or "Men".
            # 2) Division rows, like "67.5kg Master 50-54".
            #    These always begin with a weightclass (can be "SHW").
            stateinfo = row[1].replace('  ', ' ')
            if stateinfo.startswith('Women') or stateinfo.startswith('Men'):
                # It's a competition row. Reset everything.
                state_event = ''
                state_equipment = ''
                state_sex = ''
                state_division = ''

                # Every string must be understood.
                for part in stateinfo.split():
                    if part == 'Women':
                        state_sex = 'F'
                    elif part == 'Men':
                        state_sex = 'M'
                    elif part == 'Classic':
                        state_equipment = 'Wraps'
                    elif part == 'Raw':
                        if 'Classic' in stateinfo:
                            state_equipment = 'Wraps'
                        else:
                            state_equipment = 'Raw'
                    elif part == 'Single':
                        state_equipment = 'Single-ply'
                    elif part == 'Multi':
                        state_equipment = 'Unlimited'
                    elif part == 'Ply':
                        pass
                    elif part == 'Powerlifting':
                        state_event = 'SBD'
                    elif part == 'Push-Pull':
                        state_event = 'BD'
                    elif part == 'Squat':
                        state_event = 'S'
                    elif part == 'Bench':
                        state_event = 'B'
                    elif part == 'Deadlift':
                        state_event = 'D'
                    elif part == 'Only':
                        pass
                    else:
                        raise Exception("Unknown word '%s' in '%s'" % (part, stateinfo))
            else:
                # It's a division row. Ignore the weightclass.
                division = stateinfo[stateinfo.index(' ')+1:].strip()
                # Sometimes the "kg" might be its own word.
                if division.startswith("kg "):
                    division = division[3:].strip()
                state_division = DIVISION_MAP[division]

        # Handle rows with lifter information.
        elif row[0] != '' and row[1] != '':
            assert state_event
            assert state_equipment
            assert state_sex
            assert state_division

            row[csv.index('Event')] = state_event
            row[csv.index('Equipment')] = state_equipment
            row[csv.index('Sex')] = state_sex
            row[csv.index('Division')] = state_division


def remove_non_lifter_rows(csv):
    csv.rows = list(filter(lambda r: r[0] != '', csv.rows))


def fixup_weightclasskg(csv):
    wtclsidx = csv.index('WeightClassKg')
    sexidx = csv.index('Sex')

    for row in csv.rows:
        row[wtclsidx] = row[wtclsidx].replace('kg', '')

        if row[wtclsidx] == 'SHW':
            if row[sexidx] == 'F':
                row[wtclsidx] = '90+'
            else:
                row[wtclsidx] = '140+'


def main(filename):
    # Input.
    csv = oplcsv.Csv(filename)

    # Simple processing.
    fixup_fieldnames(csv)
    fixup_lifts(csv)
    remove_extra_info(csv)
    integrate_4ths(csv)

    # Complex processing.
    integrate_stateful_data(csv)
    remove_non_lifter_rows(csv)

    # Simple processing that might depend on the stateful data.
    fixup_weightclasskg(csv)

    # Add an empty BirthDate column to reduce future git churn.
    csv.remove_empty_columns()
    csv.insert_column(csv.index('Age'), 'BirthDate')

    # Output.
    while 'DELETEME' in csv.fieldnames:
        csv.remove_column_by_name('DELETEME')
    csv.write(sys.stdout)


if __name__ == "__main__":
    main(sys.argv[1])
