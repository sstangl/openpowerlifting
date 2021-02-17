#!/usr/bin/env python3
# Given a original.csv as outputted by rpu-parse, parse each sheet one at a time
# and join all the sheets together into an OpenPowerlifting-formatted CSV file.

import sys
import os
import re

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv


def die(s):
    print(s, file=sys.stderr)
    sys.exit(1)


# Find the line that contains column information (the line below does also).
def get_header_linenum(sheet):
    for ii in range(len(sheet)):
        if is_header(sheet[ii]):
            return ii

    return -1


def is_header(line):
    if any(x in ''.join(line).lower() for x in ['№', 'место', 'мес-']):
        return True
    return False


# Header is two merged rows, recombine these back into one row here
def fix_headers(sheet):
    headernum = get_header_linenum(sheet)
    if headernum != -1:
        header = sheet[headernum]

        linebelow = sheet[headernum + 1]

        header = [(header[ii] + " " + linebelow[ii]).strip()
                  for ii in range(0, len(header))]
        sheet[headernum - 1] = ['' for x in linebelow]

        sheet[headernum] = header

    return sheet


def parse_fieldnames(sheet):
    fieldnames = []

    headernum = get_header_linenum(sheet)
    if headernum != -1:

        header = sheet[headernum]
        # Name all the columns.
        iterable = iter(range(len(header)))

        for i in iterable:
            text = header[i].lower().replace('.', '').replace('-', '')
            text = text.lower()

            if '№' in text or 'номер' in text:
                fieldnames.append('IGNORE')
            elif 'место' in text or 'мес то' in text or text == 'зан м':
                fieldnames.append('Place')
            elif any(x in text for x in ['фамилия имя', 'участник']):
                fieldnames.append('CyrillicName')
            elif any(x in text for x in ['год родж', 'год рожд', 'грожд', 'год рож',
                                         'гр', 'г/р', 'год', 'дата рожд']):
                fieldnames.append('BirthYear')
            elif any(x in text for x in ['разряд', 'вып разр', 'звание',
                                         'раз ряд', 'рд зв',
                                         'разр']):  # Rank
                fieldnames.append('IGNORE')
            elif 'регион' in text:  # Region
                fieldnames.append('IGNORE')
            elif any(x in text for x in ['город', 'субьект рф',
                                         'область', 'субъект рф', 'мо',
                                         'субьект']):  # City/Region
                fieldnames.append('IGNORE')
            elif 'вес' in text:
                fieldnames.append('BodyweightKg')
            elif 'ком очки' in text:
                fieldnames.append('IGNORE')
            # Wilks/IPF Points
            elif text in ['очки ipf', 'вилкс', 'абс кооф', 'коэф',
                          'очки абс', 'виллк',
                          'коэф виллс', 'очки по вилксу', 'wilks', 'оч к', 'очки ipfgl',
                          'очки ipf gl']:
                fieldnames.append('IGNORE')
            elif 'тренер' in text:  # Coach
                fieldnames.append('IGNORE')
            # Individual points
            elif text in ['очки', 'оч', 'баллы'] or any(x in text for x in ['оч кт',
                                                                            'жре бий',
                                                                            'оч ки',
                                                                            'вып рд',
                                                                            'очки ком']):
                fieldnames.append('IGNORE')
            elif text == 'дсо':  # Something to do with referees?
                fieldnames.append('IGNORE')
            elif text in ['возраст кат', 'категория', 'воз кат']:
                fieldnames.append('Division')
            elif any(x in text for x in ['вуз', 'команда (во / род войск)',
                                         'спортивная школа', 'команда', 'дсо кфк',
                                         'клуб']):
                fieldnames.append('Team')
            elif text == 'мст':
                fieldnames.append('IGNORE')

            elif text in ['жим 1', '1'] and header[i + 3].lower() in ['жим 2', '2']:
                # assert header[i + 3].lower() in ['жим 2', '2']
                # assert header[i + 6].lower() in ['жим 3', '3']
                # assert header[i + 9].lower() in ['рез-т жим', 'рез-т']
                fieldnames.append('Bench1Kg')
                fieldnames.append('IGNORE')
                fieldnames.append('IGNORE')
                fieldnames.append('Bench2Kg')
                fieldnames.append('IGNORE')
                fieldnames.append('IGNORE')
                fieldnames.append('Bench3Kg')
                fieldnames.append('IGNORE')
                fieldnames.append('IGNORE')
                fieldnames.append('Best3BenchKg')
                [next(iterable) for x in range(9)]

            # FPIO
            elif text in ['жим лежа 1', 'жим 1'] and header[i + 1].lower() in ['2']:
                assert header[i + 1].lower() in ['2']
                assert header[i + 2].lower() in ['3']
                assert header[i + 3].lower() in ['жим итог', 'итог']
                fieldnames.append('Bench1Kg')
                fieldnames.append('Bench2Kg')
                fieldnames.append('Bench3Kg')
                fieldnames.append('Best3BenchKg')
                [next(iterable) for x in range(3)]
            # Rostov
            elif text in ['приседание'] and header[i + 1] == '':
                assert header[i + 2] == ''
                fieldnames.append('Squat1Kg')
                fieldnames.append('Squat2Kg')
                fieldnames.append('Squat3Kg')
                [next(iterable) for x in range(2)]
            elif text in ['жим лёжа'] and header[i + 1] == '':
                assert header[i + 2] == ''
                fieldnames.append('Bench1Kg')
                fieldnames.append('Bench2Kg')
                fieldnames.append('Bench3Kg')
                [next(iterable) for x in range(2)]
            elif text in ['становая тяга'] and header[i + 1] == '':
                assert header[i + 2] == ''
                fieldnames.append('Deadlift1Kg')
                fieldnames.append('Deadlift2Kg')
                fieldnames.append('Deadlift3Kg')
                [next(iterable) for x in range(2)]
            elif text in ['тяга 1'] and header[i + 3].lower() in ['тяга 2']:
                # assert header[i + 3].lower() in ['тяга 2', '2']
                # assert header[i + 6].lower() in ['тяга 3', '3']
                # assert header[i + 9].lower() in ['рез-т тяга', 'рез-т']
                fieldnames.append('Deadlift1Kg')
                fieldnames.append('IGNORE')
                fieldnames.append('IGNORE')
                fieldnames.append('Deadlift2Kg')
                fieldnames.append('IGNORE')
                fieldnames.append('IGNORE')
                fieldnames.append('Deadlift3Kg')
                fieldnames.append('IGNORE')
                fieldnames.append('IGNORE')
                fieldnames.append('Best3DeadliftKg')
                [next(iterable) for x in range(9)]
            elif text in ['прис', 'пр'] or \
                    any(x in text for x in ['рез-т присед', 'резт присед',
                                            'результат присед', 'прис резт',
                                            'присед итог', 'присед', 'прис результат',
                                            'результат прис', 'при сед']):
                fieldnames.append('Best3SquatKg')
            elif text in ['жим', 'ж/л'] or \
                    any(x in text for x in ['рез-т жим', 'резт жим', 'жим резт',
                                            'жим итог']):
                fieldnames.append('Best3BenchKg')
            elif text in ['тяга', 'стт'] or \
                    any(x in text for x in ['рез-т тяга', 'резт тяга',
                                            'тяга резт', 'тяга итог', 'бал итог']):
                fieldnames.append('Best3DeadliftKg')

            elif 'сумма троеб' in text or text == 'сумма':
                fieldnames.append('TotalKg')
            elif text == 'нмомер':  # Lot
                fieldnames.append('IGNORE')
            elif text in ['абс', 'абссумма']:  # Absolute Place
                fieldnames.append('IGNORE')

            # FPR fills their spreadsheets with lots of merged blank columns
            elif text == '':
                fieldnames.append('IGNORE')

            else:
                die('Fix parse_fieldnames(): Unknown column header text: "%s"' % text)
    else:  # No header was given, use the default
        fieldnames = ['Place', 'IGNORE', 'CyrillicName']
        fieldnames.extend(['IGNORE'] * 10)
        fieldnames.append('BirthYear')
        fieldnames.extend(['IGNORE'] * 26)
        fieldnames.append('BodyweightKg')
        fieldnames.extend(['IGNORE'] * 2)
        fieldnames.append('Best3SquatKg')
        fieldnames.extend(['IGNORE'] * 2)
        fieldnames.append('Best3BenchKg')
        fieldnames.extend(['IGNORE'] * 2)
        fieldnames.append('Best3DeadliftKg')
        fieldnames.extend(['IGNORE'] * 2)
        fieldnames.append('TotalKg')

        # fieldnames = ['Place','CyrillicNameName','BirthYear',
        #               'IGNORE','IGNORE','IGNORE','BodyweightKg','Squat1Kg',
        #               'Squat2Kg','Squat3Kg']

    return fieldnames


# Finds the division of a meet from the preamble
def get_division(sheet):
    headernum = get_header_linenum(sheet)
    division = 'Open'
    for row in sheet[0:headernum]:
        rowtext = ''.join(row).lower()
        if 'юниорок' in rowtext or 'средиюниоров' in rowtext:
            division = 'Juniors'
        elif 'юношей' in rowtext or 'девушек' in rowtext:
            division = 'Sub-Juniors'

    return division

# Search the preamble for equipment


def get_equipment(sheet):
    headernum = get_header_linenum(sheet)
    equipment = 'Single-ply'
    for row in sheet[0:headernum]:
        rowtext = ''.join(row).lower()
        if 'классическому' in rowtext:
            equipment = 'Raw'

    return equipment


# Search above the header for sex
def get_initial_sex(sheet):
    headernum = get_header_linenum(sheet)
    sex = 'F'
    for row in sheet[0:headernum]:
        rowtext = ''.join(row).lower()
        if 'мужчины' in rowtext:
            sex = 'M'
        elif 'женщины' in rowtext:
            sex = 'F'

    return sex


# Given a list of lines all of which belong to the same sheet, parse that
# into an OpenPowerlifting-style CSV.
def parse_csv(sheet):

    csv = oplcsv.Csv()

    # FPR splits headers across multiple lines,fix this
    sheet = fix_headers(sheet)

    # Look through the sheet for column information and mark up the CSV.
    # All columns are given a name -- the extra ones are removed later.
    csv.fieldnames = parse_fieldnames(sheet)

    csv.fieldnames.append('WeightClassKg')
    csv.fieldnames.append('Event')
    csv.fieldnames.append('Equipment')
    csv.fieldnames.append('Division')
    csv.fieldnames.append('Sex')

    weightclass = ''
    event = None
    equipment = get_equipment(sheet)
    division = get_division(sheet)
    sex = get_initial_sex(sheet)

    if event is None:  # Might add code to search for events in titles later
        event = ''
        if 'Squat1Kg' in csv.fieldnames or 'Best3SquatKg' in csv.fieldnames:
            event += 'S'
        if 'Bench1Kg' in csv.fieldnames or 'Best3BenchKg' in csv.fieldnames:
            event += 'B'
        if 'Deadlift1Kg' in csv.fieldnames or 'Best3DeadliftKg' in csv.fieldnames:
            event += 'D'

    iterable = iter(range(get_header_linenum(sheet)+2, len(sheet)))

    for ii in iterable:
        line = sheet[ii]
        text = ''.join(line)

        # Skip empty lines.
        if text == '':
            continue

        # Detect lines that set WeightClassKg state.
        if 'Весовая категория' in text:
            weightclass = ''.join(re.findall(r"\d*\.\d+|(\d+|\+)", text))
            if '+' in weightclass:  # Put the plus at the end
                weightclass = weightclass.replace('+', '') + '+'
            continue

        # Detect lines that set sex
        if 'mужчины' in text.lower() or 'мужчины' in text.lower():
            sex = 'M'
            continue
        elif 'женщины' in text.lower() or 'жюри' in text.lower():
            sex = 'F'
            continue

        if 'судейская' in text:
            continue

        # Skip lines without a number, I think that these are always not lifter
        # data
        if line[0].strip() == '':
            continue

        # Skip lines without a name
        if line[csv.index('CyrillicName')].strip() == '':
            continue

        if is_header(line):
            continue

        line.append(weightclass)
        line.append(event)
        line.append(equipment)
        line.append(division)
        line.append(sex)

        if len(line) != len(csv.fieldnames):
            csv.fieldnames.extend(
                ['IGNORE'] * (len(line) - len(csv.fieldnames)))

        assert len(line) == len(csv.fieldnames)
        csv.rows.append(line)

    # Remove all the columns named 'IGNORE' before returning the CSV for
    # integration.
    while 'IGNORE' in csv.fieldnames:
        csv.remove_column_by_name('IGNORE')

    unreverse_names(csv)

    return csv


# Mark DQs properly and make sure that place is an integer.
def cleanup_place(csv):
    if 'Place' in csv.fieldnames:
        place_idx = csv.index('Place')
        div_idx = csv.index('Division')
        total_idx = csv.index('TotalKg')
        for row in csv.rows:
            if 'М1' in row[place_idx]:  # There are weird Russian M's
                row[place_idx] = row[place_idx].replace('М1', '').strip()
                row[div_idx] = 'Masters 1'
            elif 'М2' in row[place_idx]:
                row[place_idx] = row[place_idx].replace('М2', '').strip()
                row[div_idx] = 'Masters 2'
            elif 'М3' in row[place_idx]:
                row[place_idx] = row[place_idx].replace('М3', '').strip()
                row[div_idx] = 'Masters 3'
            elif 'М4' in row[place_idx]:
                row[place_idx] = row[place_idx].replace('М4', '').strip()
                row[div_idx] = 'Masters 4'

            # Convert place to an integer if it wasn't already
            if '.00' in row[place_idx]:
                row[place_idx] = str(int(float(row[place_idx])))

            # Somewhat convoluted way of checking if a lifter is marked DQ and
            # has a nonzero total
            if (row[place_idx] == 'DQ' and
                    row[total_idx].replace('.', '').replace('-', '').isdigit() and
                    float(row[total_idx]) != 0.0):
                row[place_idx] = 'DD'
                row[total_idx] = ''
            elif (row[place_idx] == '' and
                    row[total_idx].replace('.', '').replace('-', '').isdigit() and
                    float(row[total_idx]) != 0.0):
                row[place_idx] = 'DD'
                row[total_idx] = ''
            # Everything else is a regular DQ
            elif not row[place_idx] or row[place_idx] == '-':
                row[place_idx] = 'DQ'
            elif row[place_idx] == 'дск':
                row[place_idx] = 'DQ'
            elif (row[total_idx].replace('.', '').replace('-', '').isdigit() and
                    float(row[total_idx]) == 0.0):
                row[place_idx] = 'DQ'
                row[total_idx] = ''
            elif row[total_idx] == '':
                row[place_idx] = 'DQ'


def unreverse_names(csv):

    if 'CyrillicName' in csv.fieldnames:
        nameidx = csv.index('CyrillicName')
    elif 'Name' in csv.fieldnames:
        nameidx = csv.index('Name')
    for row in csv.rows:
        parts = row[nameidx].split()
        if parts[-1] == 'дк':
            parts.pop()

        parts = [name.title() for name in parts]
        # The last name is probably the given first name.
        fixed = [parts[-1]] + parts[:-1]
        name = ' '.join(fixed)

        row[nameidx] = name

# Names sometimes have something in brackets after them


def cleanup_names(csv):
    if 'CyrillicName' in csv.fieldnames:
        nameidx = csv.index('CyrillicName')
    elif 'Name' in csv.fieldnames:
        nameidx = csv.index('Name')

    for row in csv.rows:
        row[nameidx] = re.sub(r'\(.*\)', '', row[nameidx])
        row[nameidx] = row[nameidx].strip()


def cleanup_lift(csv, fieldname):
    if fieldname in csv.fieldnames:
        idx = csv.index(fieldname)

        for row in csv.rows:
            amount = row[idx]

            amount = ''.join(c for c in amount if c.isdigit()
                             or c in ['.', '-'])
            amount = amount.replace('.00', '').replace('.0', '')

            if (amount == 'X' or amount.replace('-', '') == '0' or
                    not any(c.isdigit() for c in amount)):
                amount = ''

            # Sometimes numbers have more than 2 commas, if so remove the
            # second one
            if len([ii for ii, a in enumerate(amount) if a == '.']) > 1:
                amount = amount[:amount.rfind(
                    '.')] + amount[amount.rfind('.') + 1:]

            row[idx] = amount


# Remove '.0' from weightclasses
def cleanup_weightclass(csv):
    if 'WeightClassKg' in csv.fieldnames:
        idx = csv.index('WeightClassKg')
        for row in csv.rows:
            if '.0' in row[idx]:
                row[idx] = row[idx].replace('.0', '')

# Sometimes weight class is also given after bodyweight


def cleanup_bodyweight(csv):
    if 'BodyweightKg' in csv.fieldnames:
        idx = csv.index('BodyweightKg')
        for row in csv.rows:
            row[idx] = row[idx].replace(
                'сн.вр.', '').replace('снята врачом', '')
            row[idx] = row[idx].replace('снят врачом', '').replace('снят', '')
            row[idx] = row[idx].replace('неявка', '')
            if row[idx] != '':
                row[idx] = row[idx].split()[0]


def assign_total(csv):

    if 'TotalKg' not in csv.fieldnames:
        csv.append_column('TotalKg')
    idx = csv.index('TotalKg')

    def weight(str):
        try:
            return float(str)
        except ValueError:
            return 0.0

    for row in csv.rows:
        if row[idx] == '':
            total = 0.0
            if 'Best3SquatKg' in csv.fieldnames:
                total += weight(row[csv.index('Best3SquatKg')])
            if 'Best3BenchKg' in csv.fieldnames:
                total += weight(row[csv.index('Best3BenchKg')])
            if 'Best3DeadliftKg' in csv.fieldnames:
                total += weight(row[csv.index('Best3DeadliftKg')])

            if ('Best3SquatKg' in csv.fieldnames and
                (weight(row[csv.index('Best3SquatKg')]) < 0 or
                    row[csv.index('Best3SquatKg')] == '')):
                total = 0.0
            elif ('Best3BenchKg' in csv.fieldnames and
                    (weight(row[csv.index('Best3BenchKg')]) < 0 or
                     row[csv.index('Best3BenchKg')] == '')):
                total = 0.0
            elif ('Best3DeadliftKg' in csv.fieldnames and
                    (weight(row[csv.index('Best3DeadliftKg')]) < 0 or
                     row[csv.index('Best3DeadliftKg')] == '')):
                total = 0.0

            if total != 0.0:
                row[idx] = str(total)


def fix_sex(csv):

    # Finds the sex of the lifters in a meet from the weight classes
    mens_classes = ['53', '59', '66', '74',
                    '83', '93', '105', '120', '120+']
    # Don't have -52 as this overlaps with an old class
    womens_classes = ['43', '47', '57', '63', '72', '84', '84+']

    old_mens_only = ['100', '110', '125', '125+']
    old_womens_only = ['44', '48', '90+']

    all_mens = mens_classes + old_mens_only
    all_womens = womens_classes + old_womens_only
    wc_idx = csv.index('WeightClassKg')
    sex_idx = csv.index('Sex')
    for row in csv.rows:
        wc = row[wc_idx]
        if wc in all_mens:
            row[sex_idx] = 'M'
        elif wc in all_womens:
            row[sex_idx] = 'F'


def main(filename):
    # Since the input is comma-separated, store the file as a list of lists.
    with open(filename) as fd:
        lines = [x.strip().split(',') for x in fd.readlines()]

    csv = parse_csv(lines)

    for x in ['Squat1Kg', 'Squat2Kg', 'Squat3Kg', 'Squat4Kg', 'Best3SquatKg',
              'Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Bench4Kg', 'Best3BenchKg',
              'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'Deadlift4Kg',
              'Best3DeadliftKg', 'TotalKg']:
        cleanup_lift(csv, x)

    assign_total(csv)

    # Now it's time to standardize the CSV a little bit!
    cleanup_place(csv)
    cleanup_names(csv)
    cleanup_weightclass(csv)
    cleanup_bodyweight(csv)
    fix_sex(csv)

    csv.append_columns(['BirthDate'])

    csv.write(sys.stdout)
    return 0


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s original.csv > entries.csv" % sys.argv[0])
        sys.exit(1)
    sys.exit(main(sys.argv[1]))
