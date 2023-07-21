#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Standardize the original.csv to the OpenPowerlifting
# internal format.
#

import os
import sys

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def error(msg):
    print("Error: %s" % msg, file=sys.stderr)
    sys.exit(1)


def strip_whitespace(csv):
    for i, x in enumerate(csv.fieldnames):
        csv.fieldnames[i] = x.strip().replace('  ', ' ')

    for row in csv.rows:
        for i, x in enumerate(row):
            row[i] = x.strip().replace('  ', ' ')


def remove_record_info(csv):
    for row in csv.rows:
        for i, x in enumerate(row):
            x = x.lower()
            if x == 'ar' or x == 'ajr':  # Australian Record
                row[i] = ''
            if x == 'jr' or x == 'sjr':  # Junior Record
                row[i] = ''
            if x == 'm1r' or x == 'm2r' or x == 'm3r' or x == 'm4r' or x == 'm3':
                row[i] = ''
            if x == 'wr' or x == 'jwr' or x == 'wjr':
                row[i] = ''
            if x == 'w' or x == 'am1' or x == 'am2' or x == 'am3':
                row[i] = ''


def remove_empty_rows(csv):
    def getemptyidx(csv):
        for i, row in enumerate(csv.rows):
            if ''.join(row) == '':
                return i
        return -1

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        del csv.rows[idx]


def remove_bottom_meet_info(csv):
    def getemptyidx(csv):
        for i, row in enumerate(csv.rows):
            if 'Referees' in ','.join(row):
                return i
            if 'Referrees' in ','.join(row):
                return i
            if 'Referee\'s' in ','.join(row):
                return i
            if 'Referess' in ','.join(row):
                return i
            if 'Jury:' in ','.join(row):
                return i
            if '=' and 'record' in ','.join(row).lower():
                return i
        return -1

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return

        # Delete everything after and including row `idx`.
        while len(csv.rows) > idx:
            del csv.rows[idx]


# Some meets have "x" or "X" when lifts are missed or not taken.
def remove_xs(csv):
    nameidx = csv.index('Name')
    for row in csv.rows:
        for i, x in enumerate(row):
            if i <= nameidx:
                continue
            if x == 'x' or x == 'X':
                row[i] = ''


# Some meets have "x" or "X" when lifts are missed or not taken.
def remove_strings(csv, sl):
    nameidx = csv.index('Name')
    for row in csv.rows:
        for i, x in enumerate(row):
            if i <= nameidx:
                continue
            if x in sl:
                row[i] = ''


def remove_empty_cols(csv):
    def iscolempty(csv, i):
        if csv.fieldnames[i]:
            return False
        for row in csv.rows:
            if row[i]:
                return False
        return True

    def getemptyidx(csv):
        for i, col in enumerate(csv.fieldnames):
            if iscolempty(csv, i):
                return i
        return -1

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        csv.remove_column_by_index(idx)


def remove_rows_by_empty_column(csv, colname):
    def getemptyidx(csv, colidx):
        for i, row in enumerate(csv.rows):
            if row[colidx] == '':
                return i
        return -1

    colidx = csv.index(colname)
    if colidx < 0:
        error("Column %s not found in remove_rows_by_empty_column()." % colname)

    while True:
        idx = getemptyidx(csv, colidx)
        if idx == -1:
            return
        del csv.rows[idx]


def remove_zeros(csv):
    for row in csv.rows:
        for i, x in enumerate(row):
            # The CSV conversion via LibreOffice already standardized
            # all decimal forms of 0.00 and such to just '0'.
            if x == '0':
                row[i] = ''


def remove_dashes(csv):
    for row in csv.rows:
        for i, x in enumerate(row):
            # The CSV conversion via LibreOffice already standardized
            # all decimal forms of 0.00 and such to just '0'.
            if x == '-':
                row[i] = ''


def standardize_column_names(csv):
    for i, col in enumerate(csv.fieldnames):
        if col == '':
            pass
        elif col == 'NAME':
            csv.fieldnames[i] = 'Name'
        elif col == 'DOB':
            csv.fieldnames[i] = 'BirthYear'
        elif col == 'M/F' or col == 'M / F':
            csv.fieldnames[i] = 'Sex'
        elif col == 'BWT' or col == 'BW':
            csv.fieldnames[i] = 'BodyweightKg'
        elif col == 'SQ 1' or col == 'SQ-1' or col == 'SQ1' or col == 'Squat1Kg':
            csv.fieldnames[i] = 'Squat1Kg'
        elif col == 'SQ 2' or col == 'SQ-2' or col == 'SQ2' or col == 'Squat2Kg':
            csv.fieldnames[i] = 'Squat2Kg'
        elif col == 'SQ 3' or col == 'SQ-3' or col == 'SQ3' or col == 'Squat3Kg':
            csv.fieldnames[i] = 'Squat3Kg'
        elif col == 'SQUAT':
            if not csv.fieldnames[i + 1] and not csv.fieldnames[i + 2]:
                csv.fieldnames[i] = 'Squat1Kg'
                csv.fieldnames[i + 1] = 'Squat2Kg'
                csv.fieldnames[i + 2] = 'Squat3Kg'
            else:
                error("Old-style Squat parsing error.")
        elif col == 'BP 1' or col == 'BP-1' or col == 'BP1' or col == 'Bench1Kg':
            csv.fieldnames[i] = 'Bench1Kg'
        elif col == 'BP 2' or col == 'BP-2' or col == 'BP2' or col == 'Bench2Kg':
            csv.fieldnames[i] = 'Bench2Kg'
        elif col == 'BP 3' or col == 'BP-3' or col == 'BP3' or col == 'Bench3Kg':
            csv.fieldnames[i] = 'Bench3Kg'
        elif col == 'BENCH PRESS':
            if not csv.fieldnames[i + 1] and not csv.fieldnames[i + 2]:
                csv.fieldnames[i] = 'Bench1Kg'
                csv.fieldnames[i + 1] = 'Bench2Kg'
                csv.fieldnames[i + 2] = 'Bench3Kg'
            else:
                error("Old-style Bench parsing error.")
        elif col == 'DL 1' or col == 'DL-1' or col == 'DL1' or col == 'Deadlift1Kg':
            csv.fieldnames[i] = 'Deadlift1Kg'
        elif col == 'DL 2' or col == 'DL-2' or col == 'DL2' or col == 'Deadlift2Kg':
            csv.fieldnames[i] = 'Deadlift2Kg'
        elif col == 'DL 3' or col == 'DL-3' or col == 'DL3' or col == 'Deadlift3Kg':
            csv.fieldnames[i] = 'Deadlift3Kg'
        elif col == 'DEADLIFT':
            if not csv.fieldnames[i + 1] and not csv.fieldnames[i + 2]:
                csv.fieldnames[i] = 'Deadlift1Kg'
                csv.fieldnames[i + 1] = 'Deadlift2Kg'
                csv.fieldnames[i + 2] = 'Deadlift3Kg'
            else:
                error("Old-style Deadlift parsing error.")
        elif col == 'TOTAL' or col == 'Total' or col == 'TOT' or col == 'PL Total':
            csv.fieldnames[i] = 'TotalKg'
        elif col.lower() == 'wilks' or col == 'Wilks Total':
            csv.fieldnames[i] = 'Wilks'
        elif col == 'NATION' or col == "Country":
            csv.fieldnames[i] = 'Country'
        elif col == 'CLASS' or col == 'DIV':
            csv.fieldnames[i] = 'WeightClassKg'
        elif col == 'CAT':
            csv.fieldnames[i] = 'Division'
        elif col.lower() == 'place' or col == 'PLACING' or col == 'PL':
            csv.fieldnames[i] = 'Place'
        elif col == 'TEAM':
            csv.fieldnames[i] = 'Team'
        elif col == 'SUB':
            csv.fieldnames[i] = 'SubTotalDELETE'
        elif col == 'NAT':
            csv.fieldnames[i] = 'Country'
        elif col == 'AGE GROUP':
            csv.fieldnames[i] = 'Division'
        elif col == 'WC':
            csv.fieldnames[i] = 'WeightClassKg'
        else:
            error("Unknown column: \"%s\"" % col)


def isint(s):
    try:
        int(s)
        return True
    except ValueError:
        return False


def consume_first_column(csv):
    csv.fieldnames[0] = 'Place'
    assert 'Equipment' not in csv.fieldnames
    assert 'WeightClassKg' not in csv.fieldnames
    assert 'Event' not in csv.fieldnames
    csv.append_columns(['Equipment', 'WeightClassKg', 'Event'])

    # Sex information is statefully present also, but there's usually a M/F
    # column.
    assert 'Sex' in csv.fieldnames

    eqidx = csv.index('Equipment')
    wtclsidx = csv.index('WeightClassKg')
    evtidx = csv.index('Event')

    # Walk down the rows, storing and writing state from the first column.
    # Nuke the stateful rows as they're walked, for remove_empty_rows() later.
    equipment = 'Raw'  # PA stopped supplying equipment info by default.
    weightclasskg = None
    event = 'SBD'

    for row in csv.rows:
        # If this is a placement, then write information.
        if isint(row[0]) or row[0] == '-' or row[0] == 'DQ':
            assert equipment
            assert weightclasskg

            row[eqidx] = equipment
            row[wtclsidx] = weightclasskg
            row[evtidx] = event

            if row[0] == '-':
                row[0] = 'DQ'

        # Otherwise, change state and clear the row.
        else:
            if row[0] == '':
                pass
            elif row[0] == 'RAW':
                equipment = 'Raw'
                event = 'SBD'
            elif row[0] == 'WOMEN':
                pass
            elif row[0] == 'MEN':
                pass
            elif 'KG' in row[0]:
                weightclasskg = row[0].replace('KG', '').strip()
            elif row[0] == 'BP' or row[0] == 'BENCH PRESS' or row[0] == 'APC BP' \
                    or row[0] == 'BENCH PRESS ONLY':
                event = 'B'
                equipment = 'Raw'
            elif row[0] == 'EBP' or row[0] == 'EQP BP' or row[0] == 'BP EQP':
                event = 'B'
                equipment = 'Single-ply'
            elif row[0] == 'PUSH/PULL':
                event = 'BD'
            elif (row[0] == 'EQP' or row[0] == 'EQ' or row[0] == 'EQD' or
                    row[0] == 'EQD MEN'):
                event = 'SBD'
                equipment = 'Single-ply'
            elif row[0].lower() == 'equipped':
                event = 'SBD'
                equipment = 'Single-ply'
            elif row[0] == 'MASTERS 1' or row[0] == 'MASTERS 2' or row[0] == 'MASTERS 3':
                pass
            elif row[0] == 'MASTERS 4' or row[0] == 'OPEN':
                pass
            elif row[0] == 'SUBJUNIOR' or row[0] == 'JUNIOR' or row[0] == 'SUB JUNIOR':
                pass
            elif row[0] == 'PUSH / PULL':
                event = 'BD'
            elif row[0] == "DEADLIFT ONLY":
                event = 'D'
            elif row[0] == "EQUIPPED BENCH PRESS":
                event = 'B'
                equipment = 'Single-ply'
            elif row[0] == "EQP D/L":
                event = 'D'
                equipment = 'Single-ply'
            else:
                error("Unknown state change in first column: \"%s\"" % row[0])

            row[0] = ''


# For documents <= 2013, the place isn't given. Instead, there is a CLASS column,
# and the event/gear is specified statefully by rows with colspan=16.
def consume_old_format(csv):
    assert csv.fieldnames[0] == 'Name'
    assert 'WeightClassKg' in csv.fieldnames
    assert 'Sex' in csv.fieldnames
    assert 'Equipment' not in csv.fieldnames
    assert 'Event' not in csv.fieldnames

    csv.append_columns(['Equipment', 'Event'])

    sexidx = csv.index('Sex')
    eqidx = csv.index('Equipment')
    evtidx = csv.index('Event')

    equipment = 'Single-ply'
    event = 'SBD'

    for row in csv.rows:
        if row[0] == 'Equipped Powerlifting':
            equipment = 'Single-ply'
            event = 'SBD'
            row[0] = ''
        elif row[0] == 'Equipped Bench Press' or row[0] == 'Equipped Bench Press Only':
            equipment = 'Single-ply'
            event = 'B'
            row[0] = ''
        elif row[0] == 'Raw Powerlifting':
            equipment = 'Raw'
            event = 'SBD'
            row[0] = ''
        elif row[0] == 'Raw Bench Press' or row[0].lower() == 'raw bench press only':
            equipment = 'Raw'
            event = 'B'
            row[0] = ''
        elif row[0] == 'Bench Press Only' or row[0].lower() == 'bench press' \
                or row[0].lower() == 'bench only':
            event = 'B'
            row[0] = ''
        elif row[0].lower() == 'deadlift only':
            event = 'D'
            row[0] = ''
        elif row[0] == 'Squat and Bench Press Only':
            event = 'SB'
            row[0] = ''
        # If no sex is specified, this is probabaly an unknown control row.
        elif not row[sexidx]:
            error("Unknown state change in old-style first column: \"%s\"" %
                  row[0])
        else:
            assert equipment
            assert event
            row[eqidx] = equipment
            row[evtidx] = event


# Sometimes a lifter that DQ'd will still be given a place, but Wilks or Total
# will be marked "BMB" or "disq" or something like that.
# Make a best effort to hunt that down.
def hunt_dq_info(csv):
    nameidx = csv.index('Name')
    placeidx = csv.index('Place')

    for row in csv.rows:
        # Look just to the right of the Name row, to not get false positives
        # from weird names.
        s = ','.join(row[nameidx + 1:]).lower()
        if ',bmb' in s or ',disq' in s or ',dq' in s:
            row[placeidx] = 'DQ'


# Attempts are given, but not the Best3SquatKg columns, etc.
def calc_best_lift(csv, col, attemptlist):
    if col in csv.fieldnames:
        return

    for k in attemptlist:
        assert k in csv.fieldnames

    csv.insert_column(csv.index(attemptlist[-1]) + 1, col)

    for row in csv.rows:
        best = 0
        for k in attemptlist:
            try:
                attempt = float(row[csv.index(k)])
            except ValueError:
                attempt = 0
            if attempt > best:
                best = attempt
        if best > 0:
            row[csv.index(col)] = str(best)


# Old-style formats use the same labels for weight class and division,
# so we label everything WeightClassKg then hunt for a fix-up here.
def try_label_division_column(csv):
    if 'Division' in csv.fieldnames:
        return
    if 'WeightClassKg' not in csv.fieldnames:
        return

    def hasdiv(csv, i):
        for row in csv.rows:
            if row[i] == 'O':
                return True
        return False

    for i, field in enumerate(csv.fieldnames):
        if field == 'WeightClassKg' and hasdiv(csv, i):
            csv.fieldnames[i] = 'Division'
            break


def main(filename):
    csv = Csv()
    with open(filename) as fd:
        # Ignore all the bogus lines until we reach the 'NAME' row.
        line = fd.readline()
        while line and 'NAME' not in line:
            line = fd.readline()
        if not line:
            error("No column with 'NAME' found.")
        lines = fd.readlines()

    csv.fieldnames = line.strip().split(',')
    csv.rows = [x.strip().split(',') for x in lines]

    strip_whitespace(csv)
    remove_record_info(csv)
    remove_empty_rows(csv)
    remove_empty_cols(csv)

    standardize_column_names(csv)

    # Some referee information gets added to the bottom, sometimes.
    remove_bottom_meet_info(csv)

    # At this point the leftmost column contains place information
    # and stateful weight class / equipment information.
    if csv.fieldnames[0] == '':
        consume_first_column(csv)
    # Otherwise, this is an old-style (2013 and earlier) document,
    elif csv.fieldnames[0] == 'Name':
        consume_old_format(csv)
    else:
        error("Unknown format.")
    remove_empty_rows(csv)

    remove_zeros(csv)
    remove_dashes(csv)
    remove_xs(csv)

    # Old-style documents don't have a Place column, so we'll have to run
    # calc-place after.
    if 'Place' in csv.fieldnames:
        hunt_dq_info(csv)
    remove_strings(csv, ['BMB', 'Disq'])
    remove_strings(csv, ['????', '???', '??'])  # Usually in BirthYear.

    try_label_division_column(csv)

    # Columns with no header are used to mark records.
    while '' in csv.fieldnames:
        csv.remove_column_by_name('')

    # Wilks will be automatically calculated later.
    # Feds get it wrong all the time.
    if 'Wilks' in csv.fieldnames:
        csv.remove_column_by_name('Wilks')

    if 'Squat1Kg' in csv.fieldnames:
        calc_best_lift(csv, 'Best3SquatKg', [
                       'Squat1Kg', 'Squat2Kg', 'Squat3Kg'])
    if 'Bench1Kg' in csv.fieldnames:
        calc_best_lift(csv, 'Best3BenchKg', [
                       'Bench1Kg', 'Bench2Kg', 'Bench3Kg'])
    if 'Deadlift1Kg' in csv.fieldnames:
        calc_best_lift(csv, 'Best3DeadliftKg', [
                       'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg'])

    if 'BirthDate' not in csv.fieldnames:
        csv.insert_column(csv.index('BirthYear'), 'BirthDate')
    if 'Division' not in csv.fieldnames:
        csv.insert_column(csv.index('BodyweightKg'), 'Division')

    csv.write(sys.stdout)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s original.csv" % sys.argv[0], file=sys.stderr)
        sys.exit(1)
    main(sys.argv[1])
