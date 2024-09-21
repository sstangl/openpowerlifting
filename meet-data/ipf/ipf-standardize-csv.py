#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Standardize the original.csv to the OpenPowerlifting
# internal format.
#

import sys
import os
import shutil

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
    for row in csv.rows:
        for i, x in enumerate(row):
            row[i] = x.strip().replace('  ', ' ')


def assign_class(csv):
    curclass = None
    totalidx = csv.index('TotalKg')
    wtclsidx = csv.index('WeightClassKg')

    for row in csv.rows:
        if 'kg' in row[0]:
            curclass = row[0].replace('-', '').replace('kg', '')
        elif curclass and row[totalidx]:
            row[wtclsidx] = curclass

# Try and get the name of the meet from the csv file


def get_meet_name(csv):
    meet_info = [""]
    for ii in range(0, 3):
        if len(csv.rows[ii]) != 0:
            meet_info = csv.rows[ii][0].split('.')
            if len(meet_info) > 3:  # Looks like we've found the meet info
                break
    if len(meet_info) != 0:
        return meet_info[0]

    return ""


def assign_division(csv):
    curdiv = None
    totalidx = csv.index('TotalKg')
    dividx = csv.index('Division')

    for row in csv.rows[4:]:  # Skip the title
        if (row[0] == 'Open' or 'Sub' in row[0] or 'Juniors' in row[0] or
                'Masters' in row[0]):
            break

    # Try and work out the division of single division meets.
    # Masters meets always have multiple divisions.
    if curdiv is None:
        if 'Open' in get_meet_name(csv):
            curdiv = 'Open'
        elif 'Sub' in get_meet_name(csv):
            curdiv = 'Sub-Junior'
        elif 'Junior' in get_meet_name(csv):
            curdiv = 'Junior'

    for row in csv.rows:
        # So we can correctly strip rows in Bench Only meets
        if (any(title in row[0].lower() for title in
                ['best lifters', 'list of', 'champion of'])):
            curdiv = 'NON-LIFTER-ROW'
        # and 'best' not in row[0].lower():
        elif (row[0] == 'Open' or 'Sub' in row[0] or 'Juniors' in row[0] or
                'Masters' in row[0]):
            curdiv = row[0].replace('Women', '').replace('Men', '').strip()
        elif curdiv and row[totalidx]:
            row[dividx] = curdiv


def remove_non_lifter_rows(csv):
    # All lifter rows at this point, even if DQ'd, have totals given.
    totalidx = csv.index('TotalKg')
    # For Bench Only meets we check the division to see if this is a lifter
    dividx = csv.index('Division')

    def getbadidx(csv):
        for i, row in enumerate(csv.rows):
            if not row[totalidx] or row[dividx] == 'NON-LIFTER-ROW':
                return i
        return -1

    while True:
        idx = getbadidx(csv)
        if idx == -1:
            break
        del csv.rows[idx]

    if csv.rows[0][0] in ['PL.', 'Rnk']:
        del csv.rows[0]


def fixup_lift(csv, fieldname):
    idx = csv.index(fieldname)
    placeidx = csv.index('Place')

    for row in csv.rows:
        amount = row[idx]

        # Records are like: '234-w'.
        # Failed record attempts aren't marked as records.
        if amount and amount[0] != '-' and '-' in amount:
            amount = amount.split('-')[0]

        # Some older meets have records as 123/w
        if amount and amount[0] != '/' and '/' in amount:
            amount = amount.split('/')[0]

        if ' ' in amount:
            amount = amount.split()[0]

        if amount == 'X' or amount == 'DSQ' or amount == 'TD':
            amount = ''

        # Special handling for disqualification due to doping.
        if amount == 'DD':
            row[placeidx] = 'DD'
            amount = ''

        row[idx] = amount


# In Bench meets doping disqualifications and bombs are marked in the total
def fixup_total(csv):
    totalidx = csv.index('TotalKg')
    placeidx = csv.index('Place')
    for row in csv.rows:
        if row[totalidx] == 'DD':
            row[totalidx] = ''
            row[placeidx] = 'DD'
        elif not row[totalidx].replace('.', '', 1).isdigit():
            row[totalidx] = ''

        # There are some older IPF meets where despite a DQ the total is not
        # blank
        if row[placeidx] in ['DQ', 'DD', 'RD']:
            row[totalidx] = ''


def assign_best(csv, liftname):
    idx1 = csv.index('%s1Kg' % liftname)
    idx2 = csv.index('%s2Kg' % liftname)
    idx3 = csv.index('%s3Kg' % liftname)
    bestidx = csv.index('Best3%sKg' % liftname)

    def weight(str):
        try:
            return float(str)
        except ValueError:
            return 0.0

    for row in csv.rows:
        best = max(weight(row[idx1]), weight(row[idx2]), weight(row[idx3]))

        if float(best) > 0:
            row[bestidx] = str(best)


def unreverse_names(csv):
    nameidx = csv.index('Name')
    for row in csv.rows:
        parts = row[nameidx].split()
        parts = [name.title() for name in parts]

        # The last name is probably the given first name.
        fixed = [parts[-1]] + parts[:-1]
        name = ' '.join(fixed)

        name = name.replace('Jr.', 'Jr')
        name = name.replace('Sr.', 'Sr')

        row[nameidx] = name


def add_events(csv, events):
    eventidx = csv.index('Event')
    for row in csv.rows:
        row[eventidx] = events


def add_sex(csv, sex):
    sexidx = csv.index('Sex')
    for row in csv.rows:
        row[sexidx] = sex


# Finds the sex of the lifters in a meet from the weight classes
def meet_sex(csv):
    mens_classes = ['-53', '-59', '-66', '-74',
                    '-83', '-93', '-105', '-120', '120+']
    # Don't have -52 as this overlaps with an old class
    womens_classes = ['-43', '-47', '-57', '-63', '-72', '-84', '84+']

    old_mens_only = ['-100', '-110', '-125', '125+']
    old_womens_only = ['-44', '48', '90+']

    all_mens = mens_classes + old_mens_only
    all_womens = womens_classes + old_womens_only
    for row in csv.rows:
        if any(row[0].startswith(x) for x in all_mens):
            return 'M'
        elif any(row[0].startswith(x) for x in all_womens):
            return 'F'

    # Couldn't work out the sex of lifters from the weight classes for some
    # reason
    return ''


# See if we can determine whether this meet was single-ply
def add_equipment(csv):
    equip_type = ''

    # Check the field that usually has the meet name to see if it tells us the
    # equipment
    meet_info = [""]
    for ii in range(0, 3):
        if len(csv.rows[ii]) != 0:
            meet_info = csv.rows[ii][0].split('.')
            if len(meet_info) > 3:  # Looks like we've found the meet info
                break
    meet_name = meet_info[0]

    if 'open' in meet_name.lower() or 'equipped' in meet_name.lower():
        equip_type = 'Single-ply'
    elif 'classic' in meet_name.lower():
        equip_type = 'Raw'

    # Check the year, if < 2011 then it was single-ply
    meet_year = 9999
    if len(meet_info) > 3:
        meet_year = meet_info[-1]

    if meet_year.isdigit() and int(meet_year) < 2011:
        equip_type = 'Single-ply'

    eqpidx = csv.index('Equipment')
    for row in csv.rows:
        row[eqpidx] = equip_type


def standardize_full_meet(csv):
    # Fieldnames given for the detailed spreadsheet.
    fieldnames = []

    fieldnames.append('Place')
    fieldnames.append('Name')
    fieldnames.append('BirthYear')
    fieldnames.append('Country')
    fieldnames.append('BodyweightKg')
    fieldnames.append('XXX_wilksfactor')

    if len(csv.rows[0]) == 22:  # Lot number is not always given
        fieldnames.append('XXX_lot')

    fieldnames.append('Squat1Kg')
    fieldnames.append('Squat2Kg')
    fieldnames.append('Squat3Kg')
    fieldnames.append('XXX_squatplace')

    fieldnames.append('Bench1Kg')
    fieldnames.append('Bench2Kg')
    fieldnames.append('Bench3Kg')
    fieldnames.append('XXX_benchplace')

    fieldnames.append('Deadlift1Kg')
    fieldnames.append('Deadlift2Kg')
    fieldnames.append('Deadlift3Kg')
    fieldnames.append('XXX_deadliftplace')

    fieldnames.append('TotalKg')
    fieldnames.append('XXX_wilks')  # Recalculate anyway.
    fieldnames.append('XXX_points')

    csv.fieldnames = fieldnames

    csv.remove_column_by_name('XXX_wilksfactor')
    csv.remove_column_by_name('XXX_lot')
    csv.remove_column_by_name('XXX_squatplace')
    csv.remove_column_by_name('XXX_benchplace')
    csv.remove_column_by_name('XXX_deadliftplace')
    csv.remove_column_by_name('XXX_wilks')
    csv.remove_column_by_name('XXX_points')

    csv.fieldnames = fieldnames

    csv.remove_column_by_name('XXX_wilksfactor')
    csv.remove_column_by_name('XXX_lot')
    csv.remove_column_by_name('XXX_squatplace')
    csv.remove_column_by_name('XXX_benchplace')
    csv.remove_column_by_name('XXX_deadliftplace')
    csv.remove_column_by_name('XXX_wilks')
    csv.remove_column_by_name('XXX_points')

    csv.insert_column(csv.index('Squat3Kg') + 1, 'Best3SquatKg')
    csv.insert_column(csv.index('Bench3Kg') + 1, 'Best3BenchKg')
    csv.insert_column(csv.index('Deadlift3Kg') + 1, 'Best3DeadliftKg')
    csv.append_column('WeightClassKg')
    csv.append_column('Division')
    csv.append_column('Event')
    csv.append_column('Sex')
    csv.append_column('Equipment')

    assign_class(csv)
    assign_division(csv)
    add_events(csv, 'SBD')
    add_sex(csv, meet_sex(csv))
    add_equipment(csv)

    remove_non_lifter_rows(csv)

    for x in ['Squat1Kg', 'Squat2Kg', 'Squat3Kg', 'Bench1Kg', 'Bench2Kg', 'Bench3Kg',
              'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'TotalKg']:
        fixup_lift(csv, x)
    fixup_total(csv)

    for x in ['Squat', 'Bench', 'Deadlift']:
        assign_best(csv, x)

    strip_whitespace(csv)
    unreverse_names(csv)


def standardize_bench_only(csv):
    # Fieldnames given for the bench spreadsheet.
    fieldnames = []

    fieldnames.append('Place')
    fieldnames.append('Name')
    fieldnames.append('BirthYear')
    fieldnames.append('Country')
    fieldnames.append('BodyweightKg')

    if len(csv.rows[0]) > 12:
        fieldnames.append('XXX_wilksfactor')
    fieldnames.append('XXX_lot')

    fieldnames.append('Bench1Kg')
    fieldnames.append('Bench2Kg')
    fieldnames.append('Bench3Kg')

    fieldnames.append('TotalKg')

    if len(csv.rows[0]) == 14:  # Some IPF meets mark WRs and DQs in a seperate column
        fieldnames.append('WR_DQ')

    fieldnames.append('XXX_wilks')
    fieldnames.append('XXX_points')

    csv.fieldnames = fieldnames

    csv.remove_column_by_name('XXX_wilksfactor')
    csv.remove_column_by_name('XXX_lot')
    csv.remove_column_by_name('WR_DQ')
    csv.remove_column_by_name('XXX_wilks')
    csv.remove_column_by_name('XXX_points')

    csv.insert_column(csv.index('Bench3Kg') + 1, 'Best3BenchKg')
    csv.append_column('WeightClassKg')
    csv.append_column('Division')
    csv.append_column('Event')
    csv.append_column('Sex')
    csv.append_column('Equipment')

    assign_class(csv)
    assign_division(csv)
    add_events(csv, 'B')
    add_sex(csv, meet_sex(csv))
    add_equipment(csv)

    remove_non_lifter_rows(csv)

    for x in ['Bench1Kg', 'Bench2Kg', 'Bench3Kg']:
        fixup_lift(csv, x)
    fixup_total(csv)

    assign_best(csv, 'Bench')

    strip_whitespace(csv)
    unreverse_names(csv)


def main(filename):
    csv = Csv()
    with open(filename, 'r') as fd:
        csv.rows = [x.strip().split(',') for x in fd.readlines()]

    # Fieldnames given for the detailed spreadsheet.
    if len(csv.rows[0]) in [21, 22]:
        standardize_full_meet(csv)
    elif len(csv.rows[0]) in [12, 13, 14]:  # Bench only
        standardize_bench_only(csv)
    else:  # Leave the csv exactly the same as the original file
        with open(filename, "r") as fd:
            shutil.copyfileobj(fd, sys.stdout)
        return

    csv.append_column('BirthDate')
    csv.write(sys.stdout)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s original.csv" % sys.argv[0], file=sys.stderr)
        sys.exit(1)
    main(sys.argv[1])
