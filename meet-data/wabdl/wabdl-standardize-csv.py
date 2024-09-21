#!/usr/bin/env python3
#
# Standardizes the WABDL CSV format.
#

import os
import sys

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv


def error(msg):
    print("Error: %s" % msg, file=sys.stderr)
    sys.exit(1)


def standardize_column_event(csv):
    for row in csv.rows:
        if row[csv.index('Event')] == "BP":
            row[csv.index('Event')] = "B"
        elif row[csv.index('Event')] == "DL":
            row[csv.index('Event')] = "D"
        elif row[csv.index('Event')] == "PP":
            row[csv.index('Event')] = "BD"
        else:
            error("Unknown Event '%s'" % row[csv.index('Event')])


def standardize_column_sex(csv):
    for row in csv.rows:
        if row[csv.index('Sex')] in ['M', 'F']:
            continue

        if row[csv.index('Sex')].lower() == "men":
            row[csv.index('Sex')] = "M"
        elif row[csv.index('Sex')].lower() == "women":
            row[csv.index('Sex')] = "F"
        else:
            error("Unknown Sex '%s'" % row[csv.index('Sex')])


def standardize_column_weightclass(csv):
    ci = csv.index('WeightClassLBS')
    for row in csv.rows:
        if row[ci].lower() == 'super':
            row[ci] = 'SHW'


def standardize_column_division(csv):
    di = csv.index('Division')

    for row in csv.rows:
        nosex = row[di].replace('Men', '').replace('Women', '')
        row[di] = nosex.replace('  ', ' ').strip()


def standardize_column_equipment(csv):
    ei = csv.index('Equipment')

    for row in csv.rows:
        if row[ei].lower() == 'raw':
            row[ei] = 'Raw'
        elif row[ei].lower() == 'single':
            row[ei] = 'Single-ply'
        elif row[ei].lower() == 'double':
            row[ei] = 'Multi-ply'
        elif row[ei] == '':
            row[ei] = 'Multi-ply'
        else:
            error("Unknown Equipment '%s'" % row[csv.index('Equipment')])


def standardize_column_name(csv):
    ni = csv.index('Name')
    for row in csv.rows:
        row[ni] = row[ni].replace('.', '')


def remove_zeros(csv, idx):
    for row in csv.rows:
        if row[idx] == '0':
            row[idx] = ''


def find_lift_for(csv, idx, name, equipment):
    nameidx = csv.index('Name')
    eqidx = csv.index('Equipment')

    for row in csv.rows:
        if row[nameidx] == name and row[eqidx] == equipment and row[idx]:
            return row[idx]
    return ""


def integrate_lifts(csv):
    lift = csv.index('LIFT')
    if '4TH' in csv.fieldnames:
        fourth = csv.index('4TH')
    elif 'LIFT-4' in csv.fieldnames:
        fourth = csv.index('LIFT-4')

    bench = csv.index('Best3BenchLBS')
    bench4 = csv.index('Bench4LBS')
    dl = csv.index('Best3DeadliftLBS')
    dl4 = csv.index('Deadlift4LBS')
    total = csv.index('TotalLBS')

    for row in csv.rows:
        event = row[csv.index('Event')]
        if event == 'B':
            row[bench] = row[lift]
            row[bench4] = row[fourth]
            row[total] = row[lift]
        elif event == 'D':
            row[dl] = row[lift]
            row[dl4] = row[fourth]
            row[total] = row[lift]
        elif event == 'BD':
            row[total] = row[lift]
        else:
            error("Unknown Event (in integrate_lifts) '%s'" %
                  row[csv.index('Event')])

    # Push-Pull lifters don't have their Best3BenchLBS and Best3DeadliftLBS.
    # See if we can derive those from the other entries -- usually, yes.
    for row in csv.rows:
        event = row[csv.index('Event')]
        if event == 'BD':
            name = row[csv.index('Name')]
            equipment = row[csv.index('Equipment')]

            row[bench] = find_lift_for(csv, bench, name, equipment)
            row[bench4] = find_lift_for(csv, bench4, name, equipment)
            row[dl] = find_lift_for(csv, dl, name, equipment)
            row[dl4] = find_lift_for(csv, dl4, name, equipment)


def remove_column_if_empty(csv, idx):
    for row in csv.rows:
        if row[idx]:
            return
    csv.remove_column_by_index(idx)


def main():
    csv = oplcsv.Csv()

    # Isolate table lines.
    # The UTF-8 is malformed, so also replace errors.
    found_fieldnames = False
    for line in open("original.csv", errors="backslashreplace"):
        lower = line.lower()

        # Detect the fieldnames line.
        if not found_fieldnames and 'type,' in lower and 'sorted' not in lower:
            found_fieldnames = True
            csv.fieldnames = line.strip().split(',')

        # Data lines contain sex information.
        elif 'men,' in lower or 'women,' in lower:
            csv.rows.append(line.strip().split(','))
        # Data lines contain sex information.
        elif ' men ' in lower or ' women ' in lower:
            csv.rows.append(line.strip().split(','))

    # Remove unnecessary columns.
    csv.remove_column_by_name("SR")  # "State Record"
    csv.remove_column_by_name("RECORD-STATE")  # "State Record"
    csv.remove_column_by_name("NR")  # "National Record"
    csv.remove_column_by_name("RECORD-NAT'L")  # "National Record"
    csv.remove_column_by_name("WR")  # "World Record"
    csv.remove_column_by_name("RECORD-WORLD")  # "World Record"

    # "World" meets don't have sex columns,
    # but that can be derived from the Division.
    if 'SEX' not in csv.fieldnames:
        csv.append_column('Sex')
        for row in csv.rows:
            if 'women' in row[csv.index('DIV')].lower():
                row[csv.index('Sex')] = 'F'
            else:
                row[csv.index('Sex')] = 'M'
    else:
        csv.fieldnames[csv.index('SEX')] = "Sex"

    # Rename columns.
    # The current columns are: "TYPE,SEX,DIV,CLS,PLY,LIFT,4TH,NAME,STATE".
    csv.fieldnames[csv.index('TYPE')] = "Event"
    csv.fieldnames[csv.index('DIV')] = "Division"
    if 'CLS' in csv.fieldnames:
        csv.fieldnames[csv.index('CLS')] = "WeightClassLBS"
    elif 'CLASS' in csv.fieldnames:
        csv.fieldnames[csv.index('CLASS')] = "WeightClassLBS"
    if 'PLY' in csv.fieldnames:
        csv.fieldnames[csv.index('PLY')] = "Equipment"
    else:
        csv.append_column('Equipment')
    csv.fieldnames[csv.index('NAME')] = "Name"
    csv.fieldnames[csv.index('STATE')] = "State"

    # Standardize columns.
    standardize_column_sex(csv)
    standardize_column_event(csv)
    standardize_column_weightclass(csv)
    standardize_column_division(csv)
    standardize_column_equipment(csv)
    standardize_column_name(csv)
    remove_zeros(csv, csv.index('LIFT'))
    if '4TH' in csv.fieldnames:
        remove_zeros(csv, csv.index('4TH'))
    elif 'LIFT-4' in csv.fieldnames:
        remove_zeros(csv, csv.index('LIFT-4'))

    # Move the lifting data into the right columns.
    csv.insert_column(csv.index('LIFT'), "Best3BenchLBS")
    csv.insert_column(csv.index('LIFT'), "Bench4LBS")
    csv.insert_column(csv.index('LIFT'), "Best3DeadliftLBS")
    csv.insert_column(csv.index('LIFT'), "Deadlift4LBS")
    csv.insert_column(csv.index('LIFT'), "TotalLBS")
    integrate_lifts(csv)

    # Remove unused columns.
    csv.remove_column_by_name('LIFT')
    csv.remove_column_by_name('4TH')
    csv.remove_column_by_name('LIFT-4')
    remove_column_if_empty(csv, csv.index('Bench4LBS'))
    remove_column_if_empty(csv, csv.index('Deadlift4LBS'))

    csv.write(sys.stdout)


if __name__ == '__main__':
    main()
