#!/usr/bin/env python3
# Given a original.csv as outputted by wrpf-parse, parse each sheet one at a time
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


# Given the array of lines, split it up into an array per-sheet.
def split_by_sheet(lines):
    # Skip past the overview.
    assert lines[0][0] == 'Overview'
    for i in range(len(lines)):
        if 'Sheet' in lines[i][0]:
            break
    assert i < len(lines)
    assert 'Sheet' in lines[i][0]

    sheetlist = []
    sheet = None

    for line in lines[i:]:
        # If this line starts a new sheet, generate a new array.
        if 'Sheet' in line[0]:
            sheet = []
            sheetlist.append(sheet)
        sheet.append(line)

    return sheetlist


# Given the name of a sheet, return a dictionary describing the sheet.
def parse_sheetname(s):
    obj = {}

    federation = ''
    event = None
    amateur = 'Pro'  # By default, unless otherwise specified.
    tested = 'No'  # By default, unless otherwise specified.
    equipment = 'Wraps'  # By default, unless otherwise specified.

    # Parse word-by-word to make sure that every word is understood.
    s = s.lower()

    if 'без экип' in s:  # Need to do 'without equipment' seperately
        equipment = 'Raw'
        # We want to remove all strings that start with 'без экип'
        s = s.replace('без', '')
        split_str = s.split()
        index = [ii for ii, s in enumerate(split_str) if 'экип' in s][0]
        del split_str[index]
        s = ' '.join(split_str)

    if 'без дк' in s:  # Do 'Without drug test' seperately
        s = s.replace('без дк', '')

    s = s.replace('.', ' ').replace('"', '').replace(
        '_', ' ').replace('(', ' ').replace(')', ' ').replace('-', '')
    s = s.replace(' & ', '&')
    s = s.replace('д к', 'дк')  # Drug tested
    s = s.replace('многопетельнаядк', 'многопетельная дк')

    for word in s.split():
        # Federations
        if word in ['wrpf', 'wrpfpro', 'wprf']:
            assert not federation
            federation = 'WRPF'
        elif word in ['wepf']:
            federation = 'WEPF'
        elif word == 'спр':  # Some other federation that does multi-ply.
            assert not federation
            federation = 'SPR'
        elif word in ['gpa', 'gpaipod', 'gpaipo', 'ipo']:
            assert not federation
            federation = 'GPA'
        elif word == 'ipl':
            assert not federation
            federation = 'IPL'

        # Events
        elif word in ['пл', 'пауэрлифтинг', 'pl', 'пауэрифтинг', 'пауэлифтинг',
                      'чапл', 'капл', 'паурлифтинг', 'пауэрлифинг']:
            assert not event
            event = 'SBD'
        elif word == 'присед' or word == 'sq':
            assert not event
            event = 'S'
        elif word in ['жим', 'bp', 'bench', 'жд', 'press', 'лёжа', 'стоя']:
            assert not event or event == 'B'
            event = 'B'
            if equipment == 'Wraps':
                equipment = 'Raw'
        elif word == 'тяга' or word == 'dl':
            assert not event
            event = 'D'
            if equipment == 'Wraps':
                equipment = 'Raw'
        elif word in ['двоеборье', 'pp', 'ж+т', 'bp+dl', 'push&pull', 'cил', 'сд']:
            assert not event
            event = 'BD'
            if equipment == 'Wraps':
                equipment = 'Raw'

        elif word in ['софт', 'slinghot', 'slingshot', 'soft',
                      'однопетельная', 'многопетельная'] and event == 'B':
            equipment = 'Unlimited'
        # Equipment
        elif word in ['однослой', 'однослое', 'односл', 's ply', 'однослойной',
                      '1ой', '1ply', 'однослойная', 'однослойный', 'однолойной',
                      '1сл', 'односл']:
            equipment = 'Single-ply'
        elif word in ['экипировке', 'm ply', 'eq', 'многослой', 'экипировка',
                      'мн', 'эипировке']:
            if equipment == '':  # Otherwise we might accidently mark single as multi
                equipment = 'Multi-ply'
        elif word in ['СПР', 'облегченная', 'облегченной', 'слинг']:
            equipment = 'Multi-ply'
            event = 'B'
        elif 'бинт' in word or word == 'wraps':
            equipment = 'Wraps'
        elif word in ['безкипировки', 'raw', 'безэкип', 'классический', 'классич']:
            die('Fix parse_sheetname(): Meet contains raw and wraps original, '
                'this needs to be done manually.')
            equipment = 'Wraps'

        # Booleans.
        elif word == 'люб.' or word == 'любители':
            amateur = 'Amateur'
        elif word == 'pro' or word == 'pro' or word == 'pr':
            assert amateur == 'Pro'
        elif word in ['дк', 'dr test', 'dt', 'д к ', 'dc']:
            tested = 'Yes'
            amateur = 'Amateur'

        # Grammatical ignorables.
        elif word == 'в':
            pass
        elif word == 'лежа':
            pass
        elif word == 'на':
            pass
        elif word == 'и':
            pass
        elif word == 'с':
            pass
        elif word == 'ст':
            pass

        # Other ignorables.
        elif word == 'сфо':  # Not sure.
            pass
        elif word == 'становая' or word == 'стан':  # First word in 'deadlift'.
            pass
        elif 'экип' in word:  # Equipment
            pass
        # We don't seperately track Paralympic bench right now
        elif word == 'handicaped':
            pass
        # Extra terms for slingshot bench
        elif word in ['spr', 'ultra', 'standart', 'стандарт', 'ультра',
                      'шоте', 'жесткая']:
            pass
        elif word == 'фжд':  # Don't know what this means
            pass
        elif word == 'любители' or word == 'любит' or word == 'люб':  # Amateur /Fan?
            pass
        elif word.strip() == 'макс' or word == 'максимум':  # Maximum
            pass
        elif word == 'силовое':  # Power.
            pass
        elif word == 'двоеб':  # Double
            pass
        elif word == 'дивизион':  # Division
            pass
        elif word == 'мужчины':  # Men
            pass
        elif word == 'воен':  # Military
            pass
        elif word == 'лежанародные':  # Something to do with amateur bench, not sure
            pass
        elif word == 'забавы':  # Fun? Maybe amateurs
            pass
        elif word == 'класс':  # Class
            pass
        elif word == 'сл':  # Layer
            pass
        elif word == 'женщины':  # Women
            pass
        elif word in ['сотрудники', 'сотрудн', 'клиенты']:  # Employees/Members
            pass
        elif word in ['шоу']:  # Show
            pass
        else:
            die('Fix parse_sheetname(): Unknown word "%s" in "%s"' % (word, s))
    assert event

    obj['federation'] = federation
    obj['event'] = event
    obj['amateur'] = amateur
    obj['tested'] = tested
    obj['equipment'] = equipment

    return obj


# Find the line that contains column information (the line below does also).
def get_header_linenum(sheet):
    for ii in range(len(sheet)):
        if ('Место' in sheet[ii][0] or 'Place' in sheet[ii][0] or
                'Місце' in sheet[ii][0]):
            sheet[ii][0] = 'Место'
            return ii
    # Sometimes place is not labelled, if not find name
    for ii in range(len(sheet)):
        if sheet[ii][1] == 'ФИО':
            sheet[ii][0] = 'Место'
            return ii
    die("get_header_linenum() failed to find the header.")

# Header is two merged rows, recombine these back into one row here


def fix_headers(sheet):
    headernum = get_header_linenum(sheet)
    header = sheet[headernum]

    lineabove = sheet[headernum - 1]  # Need some error checking here
    linebelow = sheet[headernum + 1]

    # Name has split below rest of data
    if header[1] == '' and lineabove[1] != '':
        header = [(lineabove[ii] + " " + header[ii]).strip()
                  for ii in range(0, len(header))]
        sheet[headernum] = ['' for x in header]
        headernum = headernum - 1
    elif header[1] == '' and linebelow[1] != '':
        header = [(header[ii] + " " + linebelow[ii]).strip()
                  for ii in range(0, len(header))]
        sheet[headernum - 1] = ['' for x in linebelow]
    else:  # Attempt numbers always split below
        header = [header[0]] + [header[1]] + \
            [(header[ii] + " " + linebelow[ii]).strip()
             for ii in range(2, len(header))]
        sheet[headernum - 1] = ['' for x in linebelow]

    sheet[headernum] = header

    return sheet


# Sometimes we get country/city/state instead of just country
def split_country(csv):
    if 'Country/City/State' in csv.fieldnames:
        idx = csv.index('Country/City/State')
        for row in csv.rows:
            parts = row[idx].split('/')

            row[idx] = parts[0]
        csv.fieldnames[idx] = 'Country'


def parse_fieldnames(sheet):
    fieldnames = []

    headernum = get_header_linenum(sheet)
    header = sheet[headernum]
    # Name all the columns.
    iterable = iter(range(len(header)))
    for i in iterable:
        text = header[i].lower().replace('.', '')

        if text == 'место' or text == 'place':
            fieldnames.append('Place')
        elif 'фио' in text or 'имя' in text or 'прізвище' in text:
            fieldnames.append('CyrillicName')
        elif text == 'name':
            fieldnames.append('Name')
        # There are a stupid number of ways to write this column so using this
        # as a catch-all
        elif any(x in text for x in ['возра', 'возро', 'категорія', 'дивизион',
                                     'age categoty']):
            fieldnames.append('Division-Age')
        # Catch all for birthdate if this isn't a division-age column
        elif 'дата' in text:
            fieldnames.append('BirthDate')
        elif text in ['собств вес', 'собств', 'соб', 'вес', 'собственный вес',
                      'собственный', 'body weight', 'собcтвенный вес', 'собст вес',
                      'соб вес', 'собстввес', 'вага', 'св', 'собственнный вес']:
            fieldnames.append('BodyweightKg')
        elif text == 'весовая категория':
            fieldnames.append('WeightClassKg')
        elif text == 'team':
            fieldnames.append('Team')
        elif any(x in text for x in ['gloss', 'коэф', 'resh', 'залуцкий', 'рейшел',
                                     'залутский']):
            # This is not the Wilks, but the WilksCoefficient.
            fieldnames.append('IGNORE')
        elif text == 'команда' or text == 'coach':  # Coach
            fieldnames.append('IGNORE')
        elif text in ['сountry/city/state', 'страна/город/область', 'страна/город']:
            fieldnames.append('Country/City/State')
        elif text in ['country/town', 'город/страна']:
            fieldnames.append('Country/City/State')
        elif 'город' in text or 'регіон' in text:
            # We don't care about city-level information.
            fieldnames.append('IGNORE')
        elif text in ['разряд/звание', 'разряд']:  # Rank
            fieldnames.append('IGNORE')

        elif text in ['присед 1', 'squat 1', 'присяд 1', 'приседание 1', 'sq1',
                      'писед 1', 'приседания 1']:
            assert header[i + 1].lower() in ['2', 'sq2']
            assert header[i + 2].lower() in ['3', 'sq3']
            fieldnames.append('Squat1Kg')
            fieldnames.append('Squat2Kg')
            fieldnames.append('Squat3Kg')
            if header[i + 3].lower().replace('.', '') in ['рек', 'rec', 'sq4', '4']:
                fieldnames.append('Squat4Kg')
                [next(iterable) for x in range(3)]
            else:
                [next(iterable) for x in range(2)]

        elif text in ['жим', 'жим 1', 'benchpress 1', 'жим макс кг 1',
                      'жим на максимум 1', 'жим вес', 'жим лежа 1', 'bp1', 'жим лёжа 1',
                      'жим стоя 1']:
            assert header[i + 1].lower() in ['2', 'bp2']
            assert header[i + 2].lower() in ['3', 'bp3']
            fieldnames.append('Bench1Kg')
            fieldnames.append('Bench2Kg')
            fieldnames.append('Bench3Kg')
            if header[i + 3].lower().replace('.', '') in ['рек', 'rec', 'bp4', '4']:
                fieldnames.append('Bench4Kg')
                [next(iterable) for x in range(3)]
            else:
                [next(iterable) for x in range(2)]

        elif text in ['тяга', 'тяга 1', 'deadlift 1', 'становая тяга 1', 'dl1',
                      'тяга становая 1']:
            assert header[i + 1].lower() in ['2', 'dl2']
            assert header[i + 2].lower() in ['3', 'dl3']
            fieldnames.append('Deadlift1Kg')
            fieldnames.append('Deadlift2Kg')
            fieldnames.append('Deadlift3Kg')
            if header[i + 3].lower().replace('.', '') in ['рек', 'rec', 'dl4', '4']:
                fieldnames.append('Deadlift4Kg')
                [next(iterable) for x in range(3)]
            else:
                [next(iterable) for x in range(2)]

        elif text in ['сумма', 'total', 'result', 'сумма баллов', 'резульат',
                      'результат bp', 'результат', 'итог', 'рек']:
            fieldnames.append('TotalKg')

        # This is the Wilks. But we prefer to re-calculate it ourselves.
        elif text in ['wilks', 'wpoints', 'willks', 'очки', 'points', 'vilks',
                      'вилкс', 'абс', 'pts']:
            fieldnames.append('IGNORE')

        elif text == 'тренер':
            fieldnames.append('IGNORE')

        elif text == 'страна/клуб':  # City/Club ?
            fieldnames.append('IGNORE')
        elif text == 'страна':
            fieldnames.append('Country')
        elif text == 'норматив спр':  # Not sure what this is
            fieldnames.append('IGNORE')
        elif text == '':
            fieldnames.append('IGNORE')

        else:
            die('parse_fieldnames(): Unknown column header text: "%s"' % text)

    return fieldnames


# Given a list of lines all of which belong to the same sheet, parse that
# into an OpenPowerlifting-style CSV.
def parse_sheet(sheet):
    assert 'Sheet' in sheet[0][0]
    assert sheet[0][0].count(':') == 1

    csv = oplcsv.Csv()
    title = sheet[0][0].lower()
    # Ignore some sheets that don't contain any powerlifting.
    if 'судейская кол' in title:
        return csv
    elif 'нж' in title or 'народный' in title:
        # This is the "People's Bench", which I think is bench-for-reps.
        return csv
    elif 'пауэрспорт' in title or 'powersport' in title:
        return csv  # "Power sport", overhead-press + bicep-curl
    elif 'roll' in title or 'ролл' in title:
        return csv
    elif 'командный зачет' in title or 'комндное первенство' in title:  # Team points
        return csv
    elif 'командное первенство' in title:
        return csv
    elif 'apollo' in title or 'appol' in title or 'аполл' in title:
        return csv
    elif 'hub' in title or 'хаб' in title:
        return csv
    elif 'grip' in title:
        return csv
    elif 'excalibur' in title or 'эскалибур' in title:
        return csv
    elif 'bullet' in title:
        return csv
    elif 'block' in title or 'блок' in title or 'кирпич' in title:
        return csv
    elif 'ось' in title:  # axle
        return csv
    elif 'saxon' in title:
        return csv
    elif 'рулетка' in title:  # From the Russian Roulette division haha
        return csv
    elif any(x in title for x in ['фжд', 'военный', '1 вес', 'жд любители',
                                  'армейский', 'тяговое', 'военный',
                                  '1_2 веса', 'многоповтор', 'жд']):  # Bench reps
        return csv
    elif 'богатырский' in title:  # Heroic Press ?? Some kind of overhead press maybe
        return csv
    elif any(x in title for x in ['корпус', 'судейски', 'командный зачёт',
                                  'абсолютный', 'судейство', 'судейская',
                                  'коллегия']):  # Results
        return csv
    elif 'арм' in title:  # Arm wrestling
        return csv
    elif 'парная' in title:  # Tandem deadlift
        return csv
    elif 'records' in title or 'рекорды' in title:
        return csv
    elif 'логлифт' in title:  # Loglift
        return csv
    elif 'крж' in title:  # ??
        return csv
    elif 'чд' in title:  # Not sure what theses pages are, some kind of reps contest
        return csv
    elif 'бицепс' in title:  # Curl
        return csv
    elif 'isf' in title:  # Some kind of calesthetics competition
        return csv
    elif 'спр' in title or 'wsf' in title:  # Strongman
        return csv
    elif 'лямках' in title:  # Straps
        return csv

    # Figure out event, equipment, etc., and store in obj.
    obj = parse_sheetname(sheet[0][0].split(':')[1])

    # wrpf often splits headers across multiple lines,fix this
    sheet = fix_headers(sheet)

    # Look through the sheet for column information and mark up the CSV.
    # All columns are given a name -- the extra ones are removed later.
    csv.fieldnames = parse_fieldnames(sheet)

    # The WeightClassKg information is stateful, between rows.
    if 'WeightClassKg' not in csv.fieldnames:
        csv.fieldnames.append('WeightClassKg')
    assert 'Event' not in csv.fieldnames
    csv.fieldnames.append('Event')
    assert 'Equipment' not in csv.fieldnames
    csv.fieldnames.append('Equipment')
    csv.fieldnames.append('Amateur')
    csv.fieldnames.append('Tested')
    csv.fieldnames.append('Federation')

    weightclass = None

    # Iterate over each line, skipping the two header lines.
    for line in sheet[get_header_linenum(sheet) + 2:]:
        text = ''.join(line)
        # Stop iteration once the 'Best Lifters' section is reached.
        if ('Абсолютный' in text or 'List absolute winners' in text or
                text == 'Мужчины' or 'Возрастная группа' in text or text == 'Женщины'):
            break
        # Skip empty lines.
        if text == '':
            continue

        # Detect lines that set WeightClassKg state.
        if ('ВЕСОВАЯ КАТЕГОРИЯ' in text or 'Body Weight Category' in text or
                'Мужчины до' in text):
            wc_text = re.findall(r"(\d*\.\d+)|(\d+|\+)", text)

            weightclass = ''.join([str(s[0] + s[1]) for s in wc_text])
            if '+' in weightclass:  # Put the plus at the end
                weightclass = weightclass.replace('+', '') + '+'
            continue

        # If we've made it this far, the line should be for a lifter!
        # Make sure they have a name!
        if ('CyrillicName' in csv.fieldnames and
                not line[csv.index('CyrillicName')]):
            continue
        if 'Name' in csv.fieldnames and not line[csv.index('Name')]:
            continue

        line.append(weightclass)
        line.append(obj['event'])
        line.append(obj['equipment'])
        line.append(obj['amateur'])
        line.append(obj['tested'])
        line.append(obj['federation'])

        assert len(line) == len(csv.fieldnames)
        csv.rows.append(line)

    # Remove all the columns named 'IGNORE' before returning the CSV for
    # integration.
    while 'IGNORE' in csv.fieldnames:
        csv.remove_column_by_name('IGNORE')

    unreverse_names(csv)

    return csv


# Handles the Division-Age column, which looks like below:
#   Открытая 20-23 (21.11.1986)/30
# Converts to English and separates into Division and Age columns.
def standardize_division_age(csv):
    assert 'Division-Age' in csv.fieldnames
    assert 'Amateur' in csv.fieldnames
    assert 'Division' not in csv.fieldnames
    assert 'Age' not in csv.fieldnames

    csv.append_columns(['Division', 'Age', 'BirthDate'])

    idx = csv.index('Division-Age')
    dividx = csv.index('Division')
    ageidx = csv.index('Age')
    amateuridx = csv.index('Amateur')
    bdidx = csv.index('BirthDate')

    for row in csv.rows:
        # Fill in the Age.
        if '/' in row[idx]:

            row[ageidx] = row[idx].split('/')[1].strip()

        row[idx] = row[idx].replace(' - ', '-')
        row[idx] = row[idx].lower().strip()
        row[bdidx] = row[idx].split('/')[0]
        if '(' in row[bdidx]:
            row[bdidx] = re.search(r"(?<=\().*?(?=\))", row[bdidx]).group(0)
            parts = row[bdidx].split('.')
            if len(parts) == 3:
                row[bdidx] = parts[2]+'-'+parts[1]+'-'+parts[0]

        # Fill in the Division.
        # Handle the divisions with numbers first.
        if 'открытая 20-23' in row[idx]:
            division = 'Juniors 20-23'
        elif 'открытая 24-39' in row[idx]:
            division = 'Open'
        elif ('юниорки 20-23' in row[idx] or 'juniors 20-23' in row[idx] or
                'юниор 20-23' in row[idx] or 'юниоры 20-23' in row[idx] or
                'юнирки 20-23' in row[idx]):
            division = 'Juniors 20-23'
        elif 'девушки 14-16' in row[idx]:
            division = 'Teen 14-16'
        elif 'юноши 13-15' in row[idx] or 'teen 13-15' in row[idx]:
            division = 'Teen 13-15'
        elif 'teen 13-19' in row[idx] or 'юноши 13-19' in row[idx]:
            division = 'Teen 13-19'
        elif ('юноши 15-19' in row[idx] or 'teenage 15-19' in row[idx] or
                'teens 15-19' in row[idx]):
            division = 'Teen 15-19'
        elif 'юноши 14-16' in row[idx]:
            division = 'Teen 14-16'
        elif 'юноши 14-18' in row[idx]:
            division = 'Teen 14-18'
        elif 'юноши 16-17' in row[idx] or 'teen 16-17' in row[idx]:
            division = 'Teen 16-17'
        elif 'юноши 18-19' in row[idx] or 'teen 18-19' in row[idx]:
            division = 'Teen 18-19'
        elif 'юноши 17-19' in row[idx] or 'teen 17-19' in row[idx]:
            division = 'Sub-Juniors 17-19'
        elif 'тeenager t1' in row[idx] or 't1' in row[idx]:
            division = 'Teen T1'
        elif 'тeenager t2' in row[idx] or 't2' in row[idx]:
            division = 'Teen T2'
        elif 'тeenager t2' in row[idx] or 't3' in row[idx]:
            division = 'Teen T3'
        elif 'sub masters 33-39' in row[idx]:
            division = 'Sub Masters 33-39'
        elif 'masters 40+' in row[idx]:
            division = 'Masters 40+'
        elif 'мастера 40-44' in row[idx] or 'masters 40-44' in row[idx]:
            division = 'Masters 40-44'
        elif 'masters 40-45' in row[idx]:
            division = 'Masters 40-45'
        elif ('master 40-49' in row[idx] or 'masters 40-49' in row[idx] or
                'мастера 40-49' in row[idx]):
            division = 'Masters 40-49'
        elif 'мастера 45-49' in row[idx] or 'masters 45-49' in row[idx]:
            division = 'Masters 45-49'
        elif 'masters 45-50' in row[idx]:
            division = 'Masters 45-50'
        elif 'мастера 50-54' in row[idx] or 'masters 50-54' in row[idx]:
            division = 'Masters 50-54'
        elif ('master 50-59' in row[idx] or 'masters 50-59' in row[idx] or
                'мастера 50-59' in row[idx]):
            division = 'Masters 50-59'
        elif 'мастера 55-59' in row[idx] or 'masters 55-59' in row[idx]:
            division = 'Masters 55-59'
        elif 'мастера 60-64' in row[idx] or 'masters 60-64' in row[idx]:
            division = 'Masters 60-64'
        elif 'master 60-69' in row[idx] or 'мастера 60-69' in row[idx]:
            division = 'Masters 60-69'
        elif 'masters 65-69' in row[idx] or 'мастера 65-69' in row[idx]:
            division = 'Masters 65-69'
        elif 'masters 70-74' in row[idx]:
            division = 'Masters 70-74'
        elif 'мастера 70-79' in row[idx] or 'master 70-79' in row[idx]:
            division = 'Masters 70-79'
        elif 'мастера 75-79' in row[idx]:
            division = 'Masters 75-79'
        elif 'мастера 1' in row[idx] or 'm1' in row[idx]:
            division = 'Masters 1'
        elif 'мастера 2' in row[idx] or 'm2' in row[idx]:
            division = 'Masters 2'
        elif 'мастера 3' in row[idx] or 'm3' in row[idx]:
            division = 'Masters 3'
        elif 'мастера 4' in row[idx] or 'm4' in row[idx]:
            division = 'Masters 4'

        # Catch all for remaining divisions
        elif 'teen' in row[idx] or 'юноши' in row[idx]:
            division = 'Teen'
        elif 'мастера' in row[idx] or 'master' in row[idx]:
            division = 'Masters'
        elif any(div in row[idx] for div in ['открытая', 'open', 'открыая', 'откртыая',
                                             'отрытая', 'открытый']):
            division = 'Open'
        elif 'юниор' in row[idx] or 'junior' in row[idx] or 'j' in row[idx]:
            division = 'Juniors 20-23'
        elif 'девушки' in row[idx]:
            division = 'Sub-Juniors 17-19'
        elif 'ветераны' in row[idx]:
            division = 'Masters'
        elif row[idx] != '':
            die('Fix standardize_division_age(): Unknown division "%s"' %
                row[idx])
        else:  # Sometimes division is left blank for whatever reason
            division = ''

        division = row[amateuridx] + ' ' + division
        row[dividx] = division.replace('  ', ' ').strip()

    # Remove the now-extraneous columns.
    csv.remove_column_by_name('Division-Age')
    csv.remove_column_by_name('Amateur')

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
        # Everything else is a regular DQ
        elif not row[place_idx] or row[place_idx] == '-':
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
        parts = [name.title() for name in parts]

        # The last name is probably the given first name.
        fixed = [parts[-1]] + parts[:-1]
        name = ' '.join(fixed)

        row[nameidx] = name

# Names sometimes have something in brackets after them - extra divisions
# maybe?


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
    idx = csv.index('WeightClassKg')
    for row in csv.rows:
        if row[idx]:
            if '.0' in row[idx]:
                row[idx] = row[idx].replace('.0', '')
        else:
            row[idx] = ''


# Sometimes weight class is also given after bodyweight
def cleanup_bodyweight(csv):
    idx = csv.index('BodyweightKg')
    for row in csv.rows:
        if row[idx] != '':
            row[idx] = row[idx].split()[0]


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

    # Now it's time to standardize the CSV a little bit!
    # We have some temporary columns hanging out.
    standardize_division_age(csv)
    cleanup_place(csv)
    split_country(csv)
    cleanup_names(csv)
    cleanup_weightclass(csv)
    cleanup_bodyweight(csv)

    # At the moment, this is not useful information.
    csv.remove_column_by_name('Federation')

    csv.write(sys.stdout)
    return 0


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s original.csv > entries.csv" % sys.argv[0])
        sys.exit(1)
    sys.exit(main(sys.argv[1]))
