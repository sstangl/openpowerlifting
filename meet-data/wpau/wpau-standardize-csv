#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Standardize the original.csv to the OpenPowerlifting
# internal format.
#

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


# Given the array of lines, split it up into an array per-sheet.
def split_by_sheet(lines):
    for ii in range(len(lines)):
        if 'Sheet' in lines[ii][0]:
            break
    assert ii < len(lines)
    assert 'Sheet' in lines[ii][0]

    sheetlist = []
    sheet = None

    start_ii = ii

    event_types = ['only bench press', 'only deadlift', 'full power',
                   'single bench press', 'single deadlift',
                   'push & pull', 'bench press', 'станова тяга', 'жим штанги лежачи',
                   'bench press (single)', 'deadlift (single)', 'жим без экипировки',
                   'powerlifting']

    exclude_text = ['multy', 'багатоповторний', 'alliance']

    for ii in range(start_ii, len(lines)):
        line = lines[ii]
        # If this line starts a new sheet, generate a new array.
        if 'Sheet' in line[0]:
            sheet = []
            sheetlist.append(sheet)
            sheet.append(line)
        elif (any(event in ''.join(line[0]).lower() for event in event_types) and
                not any(event in ''.join(line[0]).lower() for event in exclude_text) and
                ii > start_ii + 6):
            # Then this is the event listing for an exisiting sheet, Replace
            # the title with it
            if len(sheet) < 3 or not any(is_header(test_line) for test_line in sheet):
                sheet[0][0] = 'Sheet : ' + ''.join(line[0]).lower()
            else:  # Start a new sheet
                sheet = []
                sheetlist.append(sheet)
                sheet.append(['Sheet : ' + ''.join(line[0])])

                # Check that this sheet has a header
                if not is_header(lines[ii + 1]) and not is_header(lines[ii + 2]):
                    # Use the previous sheets header
                    sheet.append(
                        sheetlist[-2][get_header_linenum(sheetlist[-2])])
                    # Change the attempt labels to the right events

        elif ('original in pounds' in ''.join(line[0]).lower() or
                ''.join(line[0]) == 'MULTY REP BENCH' or
                'багатоповторний' in ''.join(line[0]).lower()):
            break
        else:
            sheet.append(line)
    return sheetlist


# Given the name of a sheet, return a dictionary describing the sheet.
def parse_sheetname(s):
    obj = {}
    event = None
    tested = 'No'  # By default, unless otherwise specified.
    equipment = 'Wraps'  # By default, unless otherwise specified.

    s = s.replace(' & ', '&')
    s = s.replace('-', '')
    s = s.replace('_', ' ')

    if 'без екіпірування' in s:
        equipment = 'Raw'
        s = s.replace('без екіпірування', '').strip()

    if 'not drug tested' in s.lower():
        tested = 'No'
        s = s.lower().replace('not drug tested', '').strip()
    elif 'drug tested' in s.lower():
        tested = 'Yes'
        s = s.lower().replace('drug tested', '').strip()
        tested = 'Yes'

    s = s.replace("(", "").replace(")", "").strip()
    # Parse word-by-word to make sure that every word is understood.
    for word in s.lower().split():

        # Events
        if word in ['пл', 'пауэрлифтинг', 'pl', 'power', 'powerlifting',
                    'пауерліфтинг', 'powerlifitng', 'двоборство']:
            assert not event or event == 'SBD'
            event = 'SBD'
        elif word in ['жим', 'им', 'bench', 'bp']:
            assert not event or event == 'B'
            event = 'B'
            if equipment == 'Wraps':
                equipment = 'Raw'
        elif word in ['тяга', 'deadlift', 'dl']:
            assert not event
            event = 'D'
            if equipment == 'Wraps':
                equipment = 'Raw'
        elif word == 'sq':
            assert not event
            event = 'S'
            equipment = 'Wraps'
        elif word in ['двоеборье', 'push&pull', 'pp']:
            assert not event
            event = 'BD'
            if equipment == 'Wraps':
                equipment = 'Raw'

        # Equipment
        elif word == 'экипировке' or word == 'eq':
            equipment = 'Multi-ply'
        elif word == 'raw':
            equipment = 'Raw'

        # Booleans.
        elif word == 'люб.' or word == 'любители' or word == 'amateur':
            tested = 'Yes'
        elif word in ['pro', 'призові']:
            assert tested == 'No'
        elif word == 'дк' or word == 'д_к':
            tested = 'Yes'

        # Grammatical ignorables.
        elif word == 'в':
            pass
        elif word == 'лежа':
            pass
        elif word == 'про':
            pass

        elif word == 'протокол':  # Results
            pass

        # Other ignorables.
        elif word == 'сфо':  # Not sure.
            pass
        elif word == 'станова':  # First word in 'deadlift'.
            pass
        elif word == 'raw':
            pass
        elif word == 'штанги':  # Barbell
            pass
        elif word == 'лежачи':  # Lying (as in bench)
            pass
        elif word == 'single':
            pass
        elif word == 'press':
            pass
        elif word == 'only':
            pass
        elif word == 'full':
            pass
        elif word == 'bdl':
            pass
        elif word == 'original':
            pass
        elif word == 'абсолютна':
            pass
        elif 'sheet' in word:
            pass
        elif word == 'max':
            pass
        elif word.isdigit():
            pass
        else:
            die('Fix parse_sheetname(): Unknown word "%s" in "%s"' % (word, s))

    obj['event'] = event
    obj['tested'] = tested
    obj['equipment'] = equipment

    return obj


# Find the line that contains column information (the line below does also).
def get_header_linenum(sheet):
    for i in range(len(sheet)):
        if is_header(sheet[i]):
            return i
    die("get_header_linenum() failed to find the header.")


# Returns whether a line is a header
def is_header(line):
    if len(line) > 1 and line[1] in ['П.І.', 'Имя', 'NAME', 'Name',
                                     "Прізвище та ім'я", 'Nme', 'Name Surname',
                                     'Фамилия и имя', "Прізвище/Ім'я", 'Full name',
                                     'Lifters']:
        return True
    return False


def parse_fieldnames(sheet, obj):
    fieldnames = []
    headernum = get_header_linenum(sheet)
    header = sheet[headernum]

    # Name all the columns.
    iterable = iter(range(len(header)))

    # First column is place, has no label
    next(iterable)
    fieldnames.append('Place')
    for i in iterable:
        text = header[i]
        text = text.replace('.', ' ')
        text = text.replace('  ', ' ')
        text = text.replace('*', '')
        text = text.lower().strip()

        if text == 'место' or text == 'норматив впа':
            fieldnames.append('Place')
        elif text in ['п і', 'имя', "прізвище та ім'я", 'nme', 'name surname',
                      'name', "прізвище/ім'я", 'фамилия и имя', 'full name', 'lifters']:
            fieldnames.append('CyrillicName')
        elif text in ['city', 'місто', 'місце', 'регіон']:  # City /region
            fieldnames.append('IGNORE')
        elif text in ['город', 'city / team', 'city \\ club', 'city / club',
                      'спорт клуб', 'клуб']:  # Home city/gym
            fieldnames.append('IGNORE')
        elif text in ['дата народ', 'день рождения', 'db', 'дата народження',
                      'birthday', 'bp', 'bd', 'date of birth', 'b date',
                      'дата рождения', 'd birth']:  # Birthdate
            fieldnames.append('BirthDate')
        # Bodyweight
        elif any(check_str in text for check_str in ['власна вага', 'вес', 'pw',
                                                     'weight', 'вл вага']):
            fieldnames.append('BodyweightKg')
        # Wilks
        elif any(check_str in text for check_str in ['сума по коеф', 'абсолют',
                                                     'best', 'absolut', 'gf pts',
                                                     'mf pts', 'f pts',
                                                     'сумма малоуна', 'formula summ',
                                                     'coef summ', 'formulas',
                                                     'coef s/m', 'coef (open)']):
            fieldnames.append('IGNORE')
        elif text in ['виконаний норматив', 'викон норматив',
                      'вик норматив впа україна', 'вик розр',
                      'pts']:  # Some kind of points?
            fieldnames.append('IGNORE')
        elif text in ['страна', 'country', 'країна', 'coutry',
                      'contry', 'nation']:  # Country
            fieldnames.append('Country')
        elif any(check_str in text for check_str in ['звание', 'rank', 'звання',
                                                     'class', 'classif', 'розряд',
                                                     'вик нор-матив впа', 'норматив',
                                                     'вик']):  # Rank
            fieldnames.append('IGNORE')
        elif text == 'best of the best':
            fieldnames.append('IGNORE')
        # Coach
        elif text in ['тренера', 'coach', 'coaches', 'тренер', 'тренери',
                      'країна', 'тренер(и)']:
            fieldnames.append('IGNORE')
        elif text == 'вікова категорія':
            fieldnames.append('Division')
        elif text == 'сп тов':  # Not sure
            fieldnames.append('IGNORE')
        elif text == 'дюсш':  # Youth?
            fieldnames.append('IGNORE')
        elif text in ['к шварца малоуна', 'gf', 'mam', 'mf',
                      'coef', 'коеф', 'f']:  # Coefficient
            fieldnames.append('IGNORE')
        elif text == 'очк':  # No idea
            fieldnames.append('IGNORE')
        elif text == 'mam':  # Age coefficient?
            fieldnames.append('IGNORE')

        elif text in ['сумма', 'total', 'result', 'результат', 'original',
                      'total sum', 'сума', 'підсумок']:
            fieldnames.append('TotalKg')

        elif text in ['присед 1', 'sq 1', 'squat 1', 'присід 1']:
            assert header[i + 1].lower() in ['присед 2', 'sq 2',
                                             'squat 2', 'присід 2']
            assert header[i + 2].lower() in ['присед 3', 'sq 3',
                                             'squat 3', 'присід 3']
            fieldnames.append('Squat1Kg')
            fieldnames.append('Squat2Kg')
            fieldnames.append('Squat3Kg')
            if header[i + 3] == 'Сумма':
                fieldnames.append('Best3SquatKg')
                [next(iterable) for x in range(3)]
            elif header[i + 3].lower() in ['присед 4', 'sq 4', 'squat 4', 'присід 4']:
                fieldnames.append('Squat4Kg')
                [next(iterable) for x in range(3)]
            else:
                [next(iterable) for x in range(2)]
        elif text in ['жим 1', 'bp 1', 'benchpress 1', 'bp1', 'вр1']:
            assert header[i + 1].lower() in ['жим 2', 'bp 2',
                                             'benchpress 2', 'bp2', 'вр2']
            assert header[i + 2].lower() in ['жим 3', 'bp 3',
                                             'benchpress 3', 'bp3', 'вр3']
            fieldnames.append('Bench1Kg')
            fieldnames.append('Bench2Kg')
            fieldnames.append('Bench3Kg')
            if header[i + 3].lower() in ['сумма', 'рез', 'результат']:
                fieldnames.append('Best3BenchKg')
                [next(iterable) for x in range(3)]
            elif header[i + 3].lower() in ['жим 4', 'bp 4', 'benchpress 4', 'bp4']:
                fieldnames.append('Bench4Kg')
                [next(iterable) for x in range(3)]
            else:
                [next(iterable) for x in range(2)]
        elif text in ['тяга 1', 'dl 1', 'deadlift 1']:
            assert header[i + 1].lower() in ['тяга 2', 'dl 2', 'deadlift 2']
            assert header[i + 2].lower() in ['тяга 3', 'dl 3', 'deadlift 3']
            fieldnames.append('Deadlift1Kg')
            fieldnames.append('Deadlift2Kg')
            fieldnames.append('Deadlift3Kg')
            if header[i + 3].lower() == 'сумма':
                fieldnames.append('Best3DeadliftKg')
                [next(iterable) for x in range(3)]
            elif header[i + 3].lower() in ['тяга 4', 'dl 4', 'deadlift 4']:
                fieldnames.append('Deadlift4Kg')
                [next(iterable) for x in range(3)]
            else:
                [next(iterable) for x in range(2)]

        elif text == 'all squat':
            fieldnames.append('Squat1Kg')
            fieldnames.append('Squat2Kg')
            fieldnames.append('Squat3Kg')
            [next(iterable) for x in range(2)]
        elif text == 'all bench press':
            fieldnames.append('Bench1Kg')
            fieldnames.append('Bench2Kg')
            fieldnames.append('Bench3Kg')
            [next(iterable) for x in range(2)]
        elif text == 'all dead lift':
            fieldnames.append('Deadlift1Kg')
            fieldnames.append('Deadlift2Kg')
            fieldnames.append('Deadlift3Kg')
            [next(iterable) for x in range(2)]

        # Events aren't given in header, try to work them out attempt labels
        # event
        elif text in ["1 att"]:
            assert header[i + 1].replace('.', '').lower().strip() in ["2 att"]
            assert header[i + 2].replace('.', '').lower().strip() in ["3 att"]
            if obj['event'] == 'B':
                fieldnames.append('Bench1Kg')
                fieldnames.append('Bench2Kg')
                fieldnames.append('Bench3Kg')
                [next(iterable) for x in range(2)]
        elif text == '':
            fieldnames.append('IGNORE')

        else:
            die('Fix parse_fieldnames(): Unknown column header text: "%s"' % text)

    return fieldnames


# Given a list of lines all of which belong to the same sheet, parse that
# into an OpenPowerlifting-style CSV.
def parse_sheet(sheet):
    assert 'Sheet' in sheet[0][0]
    assert sheet[0][0].count(':') == 1

    csv = oplcsv.Csv()

    title = sheet[0][0].lower()
    # Ignore some sheets that don't contain any powerlifting.
    # Multiple- probably bench for reps
    if ('багатоповторний' in title or 'multy' in title or
            'popular' in title or 'народний' in title):
        return csv

    # Figure out event, equipment, etc., and store in obj.
    obj = parse_sheetname(sheet[0][0].split(':')[1])

    # Look through the sheet for column information and mark up the CSV.
    # All columns are given a name -- the extra ones are removed later.
    csv.fieldnames = parse_fieldnames(sheet, obj)

    # No events listed in sheet titles, see if we can reconstruct them from
    # the header
    if obj['event'] is None:
        event = ''
        if 'Squat1Kg' in csv.fieldnames:
            event += 'S'
        if 'Bench1Kg' in csv.fieldnames:
            event += 'B'
        if 'Deadlift1Kg' in csv.fieldnames:
            event += 'D'
        if event != '':
            obj['event'] = event

    append_div = False

    # The WeightClassKg information is stateful, between rows.
    assert 'WeightClassKg' not in csv.fieldnames
    csv.fieldnames.append('WeightClassKg')
    if 'Division' not in csv.fieldnames:
        csv.fieldnames.append('Division')
        append_div = True
    assert 'Sex' not in csv.fieldnames
    csv.fieldnames.append('Sex')
    assert 'Event' not in csv.fieldnames
    csv.fieldnames.append('Event')
    assert 'Equipment' not in csv.fieldnames
    csv.fieldnames.append('Equipment')
    csv.fieldnames.append('Tested')

    weightclass = ''
    division = ''
    sex = ''

    # There's often a bunch of nonsense at the bottom
    end_line = len(sheet)
    for ii in range(len(sheet) - 1, 0, -1):
        if len([c for c in sheet[ii] if c != '']) > 3:
            break
        else:
            end_line = ii
    # Iterate over each line, skipping the two header lines.
    for line in sheet[get_header_linenum(sheet) + 1:end_line]:
        text = ''.join(line).lower()

        # Skip empty lines.
        if text == '':
            continue

        # Detect lines that set WeightClassKg state.
        if ' кг' in text or ' kg' in text or 'до' in text:
            wc_text = re.findall(r"(\d*\.\d+)|(\d+|\+)", text)

            weightclass = ''.join([str(s[0] + s[1]) for s in wc_text])
            if '+' in weightclass:  # Put the plus at the end
                weightclass = weightclass.replace('+', '') + '+'
            continue

        # Sex and division are given between lifter rows
        if any(search_str in text for search_str in ['жен. (', 'woman (', 'female',
                                                     'women.', 'женщины', 'women']):
            sex = 'F'
            if '(' in text or '\\' in text:
                division = ''.join(re.findall(
                    r"\((.*?)\)|\\(.*)$", text)[0]).strip()
            else:
                division = ''
            continue
        elif any(search_str in text for search_str in ['муж. (', 'men (', 'male',
                                                       'men.', 'мужчины']):
            sex = 'M'
            if '(' in text or '\\' in text:
                division = ''.join(re.findall(
                    r"\((.*?)\)|\\(.*)$", text)[0]).strip()
            else:
                division = ''
            continue

        # Equipment and amateur/pro are given between rows
        if 'raw' in text:
            text.replace('amatur', "amateur")
            if 'S' in obj['event']:
                obj['equipment'] = 'Wraps'
            else:
                obj['equipment'] = 'Raw'
            if 'amateur' in text or ' am' in text:
                obj['tested'] = 'Yes'
            elif 'pro' in text:
                obj['tested'] = 'No'
            if 'doping tested' in text:
                obj['tested'] = 'Yes'
            elif 'not tested' in text:
                obj['tested'] = 'No'
            continue
        elif 'eq' in text:
            text.replace('amatur', "amateur")
            obj['equipment'] = 'Multi-ply'
            if 'amateur' in text:
                obj['tested'] = 'Yes'
            elif 'pro' in text:
                obj['tested'] = 'No'
            if 'doping tested' in text:
                obj['tested'] = 'Yes'
            elif 'not tested' in text:
                obj['tested'] = 'No'
            continue

        if is_header(line):  # Skip extra headers
            continue

        # If we've made it this far, the line should be for a lifter!
        # Make sure they have a name!
        if not line[csv.index('CyrillicName')]:
            continue
        line.append(weightclass)
        if append_div:
            line.append(division)
        line.append(sex)
        line.append(obj['event'])
        line.append(obj['equipment'])
        line.append(obj['tested'])

        assert len(line) == len(csv.fieldnames)
        csv.rows.append(line)

    # Remove all the columns named 'IGNORE' before returning the CSV for
    # integration.
    while 'IGNORE' in csv.fieldnames:
        csv.remove_column_by_name('IGNORE')

    unreverse_names(csv)

    return csv


# Handles the Division and Age columns, which looks like below:
#   Открытая 20-23 (21.11.1986)/30
# Converts to English and separates into Division and Age columns.
def standardize_division_age(csv):

    assert 'Tested' in csv.fieldnames
    assert 'Division' in csv.fieldnames

    csv.append_columns(['BirthYear'])

    dividx = csv.index('Division')
    bdidx = -1
    if 'BirthDate' in csv.fieldnames:
        bdidx = csv.index('BirthDate')
    byidx = csv.index('BirthYear')
    testedidx = csv.index('Tested')

    for row in csv.rows:
        row[dividx] = row[dividx].lower()
        # Fill in the BirthYear.
        if 'BirthDate' in csv.fieldnames:
            bd_split = re.split('[. / -]', row[bdidx])

            # Then we haven't got full year,just 2 digits.
            if len(max(bd_split, key=len)) != 4:
                # This will throw errors if they were born after 2000, but
                # better than nothing
                row[byidx] = '19' + bd_split[-1]
            else:
                row[byidx] = max(bd_split, key=len)

        # Fill in the Division.
        # Handle the divisions with numbers first.
        if '13-15' in row[dividx]:
            division = 'Teen 13-15'
        elif '13-19' in row[dividx]:
            division = 'Teen 13-19'
        elif '16-17' in row[dividx]:
            division = 'Teen 16-17'
        elif '18-19' in row[dividx]:
            division = 'Teen 18-19'
        elif '20-23' in row[dividx]:
            division = 'Juniors 20-23'
        elif '33-39' in row[dividx]:
            division = 'Submasters 33-39'
        elif '40-44' in row[dividx]:
            division = 'Masters 40-44'
        elif '45-49' in row[dividx]:
            division = 'Masters 45-49'
        elif '50-54' in row[dividx]:
            division = 'Masters 50-54'
        elif '55-59' in row[dividx]:
            division = 'Masters 55-59'
        elif '60-64' in row[dividx]:
            division = 'Masters 60-64'
        elif '65-69' in row[dividx]:
            division = 'Masters 65-69'
        elif '70-74' in row[dividx]:
            division = 'Masters 70-74'
        elif '75-79' in row[dividx]:
            division = 'Masters 75-79'
        elif '80+' in row[dividx]:
            division = 'Masters 80+'
        elif '8' in row[dividx]:
            division = 'Children 8'
        elif '9' in row[dividx]:
            division = 'Children 9'
        elif '10' in row[dividx]:
            division = 'Children 10'
        elif '11-12' in row[dividx]:
            division = 'Children 11-12'

        elif 'submaster' in row[dividx]:
            division = 'Submasters'
        elif 'paralympian' in row[dividx]:
            division = 'Paralympian'
        elif 'open' in row[dividx] or 'seniors' in row[dividx]:
            division = 'Open'
        elif 'master' in row[dividx]:
            division = 'Masters'
        elif 'mpf' in row[dividx] or 'm/p/f' in row[dividx]:
            division = 'Military/Fire/Police'
        elif 'юніор' in row[dividx] or 'juniors' in row[dividx]:
            division = 'Juniors'
        elif 'teenagers' in row[dividx]:
            division = 'Teen'
        elif 'teenager' in row[dividx]:
            division = 'Teen'
        elif 'child' in row[dividx]:
            division = 'Children'

        elif row[dividx] == '':
            division = 'Open'

        else:
            die('Fix standardize_division_age(): Unknown division "%s"' %
                row[dividx])

        if row[testedidx] == 'No':
            division = 'Pro ' + division
        else:
            division = 'Amateur ' + division
        row[dividx] = division.replace('  ', ' ').strip()


# Mark DQs properly and make sure that place is an integer.
def cleanup_place(csv):
    place_idx = csv.index('Place')
    total_idx = csv.index('TotalKg')
    for row in csv.rows:
        if '.00' in row[place_idx]:  # Convert place to an integer if it wasn't already
            row[place_idx] = str(int(float(row[place_idx])))

        # Somewhat convoluted way of checking if a lifter is marked DQ and has
        # a nonzero total
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
        elif row[place_idx] == 'DT':
            row[place_idx] = 'DD'
            row[total_idx] = ''
        # Everything else is a regular DQ
        elif (not row[place_idx] or row[place_idx] == '-' or
                row[place_idx] == 'DOC' or row[place_idx] == '—'):
            row[place_idx] = 'DQ'
            row[total_idx] = ''
        elif (row[total_idx].replace('.', '').replace('-', '').isdigit() and
                float(row[total_idx]) == 0.0):
            row[place_idx] = 'DQ'
            row[total_idx] = ''
        elif row[total_idx] == '':
            row[place_idx] = 'DQ'


def cleanup_lift(csv, fieldname):
    if fieldname in csv.fieldnames:
        idx = csv.index(fieldname)

        for row in csv.rows:
            amount = row[idx]

            if amount != '':
                amount = amount.split()[0]

                amount = ''.join(
                    c for c in amount if c.isdigit() or c in ['.', '-'])
                amount = amount.replace('.00', '').replace('.0', '')

                if (amount == 'X' or amount.replace('-', '') == '0' or
                        not any(c.isdigit() for c in amount)):
                    amount = ''

                # Sometimes numbers have more than 2 commas, if so remove the
                # second one
                if len([ii for ii, a in enumerate(amount) if a == '.']) > 1:
                    amount = amount[:amount.rfind(
                        '.')] + amount[amount.rfind('.') + 1:]

                if (amount == 'X' or amount.replace('-', '') == '0' or
                        not any(c.isdigit() for c in amount)):
                    amount = ''

                # Sometimes numbers have more than 2 commas, if so remove the
                # second one
                if len([ii for ii, a in enumerate(amount) if a == '.']) > 1:
                    amount = amount[:amount.rfind(
                        '.')] + amount[amount.rfind('.') + 1:]

                row[idx] = amount


# Names sometimes have something in brackets after them - extra divisions
# maybe?
def cleanup_names(csv):
    if 'CyrillicName' in csv.fieldnames:
        nameidx = csv.index('CyrillicName')
    elif 'Name' in csv.fieldnames:
        nameidx = csv.index('Name')

    for row in csv.rows:
        row[nameidx] = re.sub(r'\(.*\)', '', row[nameidx])
        row[nameidx] = ''.join([c for c in row[nameidx] if not c.isdigit()])
        row[nameidx] = row[nameidx].strip()


def cleanup_bodyweight(csv):
    if 'BodyweightKg' in csv.fieldnames:
        idx = csv.index('BodyweightKg')
        for row in csv.rows:
            row[idx] = row[idx].strip('-')
            if row[idx].replace('.', '').isdigit() and float(row[idx]) == 0.0:
                row[idx] = ''


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

            if total != 0.0:
                row[idx] = str(total)


def unreverse_names(csv):

    if 'CyrillicName' in csv.fieldnames:
        nameidx = csv.index('CyrillicName')
    elif 'Name' in csv.fieldnames:
        nameidx = csv.index('Name')
    for row in csv.rows:
        parts = row[nameidx].split()
        parts = [name.title() for name in parts]
        # The last name is probably the given first name.
        fixed = [parts[-1]] + parts[:-1]
        name = ' '.join(fixed)

        row[nameidx] = name


def main(filename):
    # Since the input is comma-separated, store the file as a list of lists.
    with open(filename) as fd:
        lines = [x.strip().split(',') for x in fd.readlines()]

    # Split the input filename into sheets, each of which is an independent
    # CSV.
    sheetlist = split_by_sheet(lines)

    # Parse each sheet independently, then join them all together into a
    # single CSV.
    csv = oplcsv.Csv()
    for sheet in sheetlist:
        sheetcsv = parse_sheet(sheet)
        csv.cat(sheetcsv)

    for x in ['Squat1Kg', 'Squat2Kg', 'Squat3Kg', 'Squat4Kg',
              'Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Bench4Kg',
              'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'Deadlift4Kg', 'TotalKg']:
        cleanup_lift(csv, x)

    if 'Best3SquatKg' in csv.fieldnames:
        cleanup_lift(csv, 'Best3SquatKg')
    if 'Best3BenchKg' in csv.fieldnames:
        cleanup_lift(csv, 'Best3BenchKg')
    if 'Best3DeadliftKg' in csv.fieldnames:
        cleanup_lift(csv, 'Best3DeadliftKg')

    if 'Best3SquatKg' not in csv.fieldnames and 'Squat1Kg' in csv.fieldnames:
        csv.append_column('Best3SquatKg')
        assign_best(csv, 'Squat')
    if 'Best3BenchKg' not in csv.fieldnames and 'Bench1Kg' in csv.fieldnames:
        csv.append_column('Best3BenchKg')
        assign_best(csv, 'Bench')
    if 'Best3DeadliftKg' not in csv.fieldnames and 'Deadlift1Kg' in csv.fieldnames:
        csv.append_column('Best3DeadliftKg')
        assign_best(csv, 'Deadlift')

    assign_total(csv)

    # Now it's time to standardize the CSV a little bit!
    # We have some temporary columns hanging out.
    standardize_division_age(csv)
    cleanup_place(csv)
    cleanup_names(csv)
    cleanup_bodyweight(csv)

    csv.write(sys.stdout)
    return 0


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s original.csv" % sys.argv[0], file=sys.stderr)
        sys.exit(1)
    main(sys.argv[1])
