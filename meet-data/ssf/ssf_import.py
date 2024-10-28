#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Import data from the SSF website
# After running this run ./assign_date (Folder number)


from bs4 import BeautifulSoup
import errno
import os
import sys
import urllib.request
import io

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def write_csv_with_lf(csv_obj, filename):
    with io.StringIO() as buffer:
        csv_obj.write(buffer)
        buffer.seek(0)  # Reset buffer position to the beginning
        with open(filename, 'wb') as file:
            txt = buffer.read().encode('utf-8')
            file.write(txt)


def gethtml(url):
    req = urllib.request.Request(
        url, headers={'User-Agent': 'Mozilla/5.0 (Windows NT 6.1; Win64; x64)'})
    with urllib.request.urlopen(req) as r:
        res = r.read().decode('utf-8')
        return res


def error(msg):
    print(msg, file=sys.stderr)
    raise Exception(msg)


def getdirname(url):
    return url.split('?contestId=')[1]


def getmeetinfo(soup):
    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    name = soup.find('span', id='ContentPlaceHolder2_Header').text

    fed = 'SSF'
    country = 'Sweden'
    state = ''

    # Town and date are listed on the meet list page, leave them blank for now
    town = ''
    date = ''

    name = name.replace(',', ' ').replace('  ', ' ').strip()

    # "SSF" may not be in meet titles, so replace
    # the district abbreviations by their full names
    # The current year may not be part of the title either.
    # Replace by more robust solution later.
    replacements = {
        'ÖSSF': 'Östra Svealand',
        'VSSF': 'Västra Svealand',
        'SSSF': 'Sydsvenska',
        'NNSF': 'Norra Norrland',
        'MNSF': 'Mellersta Norrland',
        'VGSF': 'Västra Götaland',
        'SÖSF': 'Sydöstra',
        '2022': '',
        '2023': '',
        '2024': ''}

    for old, new in replacements.items():
        name.replace(old, new)
    name = name.strip()

    row = [fed, date, country, state, town, name]

    csv.rows = [row]
    return csv


def expand_colspans(tr, enclosing_soup):
    ths = tr.find_all('th')
    for ii in range(len(ths)-1, -1, -1):
        if ths[ii].has_attr('colspan') and int(ths[ii]['colspan']) > 1:
            for jj in range(0, (int(ths[ii]['colspan'])-1)):
                blank_th = enclosing_soup.new_tag('th')
                ths[ii].insert_after(blank_th)
    del ths[ii]['colspan']


def get_division(div_text):
    div_text = div_text.lower()

    if 'senior' in div_text:
        return 'Open'
    elif 'ungdom' in div_text:
        return 'Sub-Juniors'
    elif 'junior' in div_text:
        return 'Juniors'
    elif 'veteran 40-49' in div_text:
        return 'Masters 1'
    elif 'veteran 50-59' in div_text:
        return 'Masters 2'
    elif 'veteran 60-69' in div_text:
        return 'Masters 3'
    elif 'veteran 70+' in div_text:
        return 'Masters 4'
    return ''


def getresults(soup):
    csv = Csv()
    divstate = None
    sexstate = None
    wcstate = None

    # Get the results table.
    results = soup.find_all(
        'table', id='ContentPlaceHolder2_contestTable')
    if len(results) == 0:
        error("Couldn't find the results table.")

    table = results[0]

    trs = table.find_all(['tr'])

    for tr in trs:
        row = [x.text.strip() for x in tr.find_all(['td'])]
        if 'Plac' in row:  # Header
            if csv.fieldnames != []:
                continue

            iterable = iter(range(len(row)))

            for ii in iterable:
                h = row[ii]
                if h == 'Plac':
                    csv.fieldnames += ['Place']
                elif h == 'Namn':
                    csv.fieldnames += ['Name']
                elif h == 'Born':
                    csv.fieldnames += ['BirthYear']
                elif h == 'Förening':
                    csv.fieldnames += ['Team']
                elif h == 'Vikt':
                    csv.fieldnames += ['BodyweightKg']
                elif h == 'Knäböj':
                    csv.fieldnames += ['Best3SquatKg']
                elif h == 'Bänkpress':
                    csv.fieldnames += ['Best3BenchKg']
                elif h == 'Marklyft':
                    csv.fieldnames += ['Best3DeadliftKg']
                elif h == 'Total':
                    csv.fieldnames += ['TotalKg']
                elif h == 'Poäng':
                    csv.fieldnames += ['Wilks']
                elif h == 'Resultat':
                    csv.fieldnames += ['Best3BenchKg']
                elif h == '':
                    csv.fieldnames += ['IGNORE']
                else:
                    error("Unknown column name: \"%s\"" % h)

            # These columns are added from the category rows.
            csv.fieldnames += ['Division', 'Sex', 'WeightClassKg']

        # Rows of length >1 are actual results, of length 1 are categories.
        elif len(row) == 1 and row[0].strip() != '':
            text = row[0].lower()
            if 'viktklass ' in text:
                wcstate = text.replace('kg', '').replace(
                    'viktklass', '').replace(',', '.').strip()
                wcstate = wcstate.replace('.00', '')
                wcstate = wcstate.replace('.50', '.5')
                if wcstate[0] == '+':
                    wcstate = wcstate[1:]+'+'
                continue

            if 'dam' in text:
                sexstate = 'F'
            elif 'herr' in text:
                sexstate = 'M'

            div = get_division(text)

            # Extract division information.
            if div:
                divstate = div
            else:
                error("Unknown state: \"%s\"" % row[0])

        elif row != []:
            assert divstate
            assert sexstate
            assert wcstate

            # Accumulate the row, but we need to look at the class of each td
            # to figure out whether lifts were good or bad.
            row = []
            for td in tr.find_all('td'):
                text = td.text.strip()
                text = text.replace(',', '.')

                if text == '0.00' or text == '0' or text == '-':
                    text = ''

                row.append(text.replace('  ', ' '))

            row = row + [divstate, sexstate, wcstate]

            csv.rows += [row]
    return csv


def fixplace(csv):
    placeidx = csv.index('Place')
    totalidx = csv.index('TotalKg')
    for row in csv.rows:
        if row[placeidx] == '':
            row[placeidx] = 'DQ'
            row[totalidx] = ''  # Instead of a zero.
        if row[totalidx] == '':
            row[placeidx] = 'DQ'


# Equipment is marked in the sheet name
def addequipment(csv, meetcsv):
    meet_name = meetcsv.rows[0][5]

    raw_meet = False
    if any(classic in meet_name.lower() for classic in ['klassisk', 'klbp', 'klsl',
                                                        'kl. sl', 'kl. bp']):
        raw_meet = True

    if 'Equipment' not in csv.fieldnames:
        csv.append_column('Equipment')
    eqpidx = csv.index('Equipment')
    for row in csv.rows:
        if row[eqpidx] == '':
            if raw_meet:
                row[eqpidx] = 'Raw'
            else:
                row[eqpidx] = 'Single-ply'


def capitalise_names(csv):
    nameidx = csv.index('Name')
    for row in csv.rows:
        split_name = [name for name in row[nameidx].split(' ') if name != '']
        for ii in range(0, len(split_name)):
            if len(split_name[ii]) > 3:
                split_name[ii] = split_name[ii].title()
        row[nameidx] = ' '.join(split_name)


def markevent(csv):
    assert 'Event' not in csv.fieldnames
    csv.append_column('Event')

    evtidx = csv.index('Event')

    def getevtindices(csv, fieldl):
        indexlist = []
        for f in fieldl:
            try:
                indexlist.append(csv.index(f))
            except ValueError:
                pass
        return indexlist

    squatidxl = getevtindices(
        csv, ['Squat1Kg', 'Squat2Kg', 'Squat3Kg', 'Best3SquatKg'])
    benchidxl = getevtindices(
        csv, ['Bench1Kg', 'Bench2Kg', 'Bench3Kg', 'Best3BenchKg'])
    deadliftidxl = getevtindices(
        csv, ['Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg', 'Best3DeadliftKg'])

    for row in csv.rows:
        evt = ''
        for i in squatidxl:
            if row[i] != '':
                evt = evt + 'S'
                break
        for i in benchidxl:
            if row[i] != '':
                evt = evt + 'B'
                break
        for i in deadliftidxl:
            if row[i] != '':
                evt = evt + 'D'
                break
        row[evtidx] = evt


def remove_empty_cols_ignore_fieldname(csv):
    def iscolempty(csv, i):
        for row in csv.rows:
            if row[i]:
                return False
        return True

    def getemptyidx(csv):
        for i, col in enumerate(csv.fieldnames):
            if iscolempty(csv, i):
                return i
        return -1

    while 'IGNORE' in csv.fieldnames:
        csv.remove_column_by_name('IGNORE')

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        csv.remove_column_by_index(idx)


def addtotals(csv):

    if 'TotalKg' not in csv.fieldnames:
        csv.append_column('TotalKg')
        placeidx = csv.index('Place')
        totalidx = csv.index('TotalKg')
        for row in csv.rows:
            total = 0
            if row[placeidx] not in ['DQ', 'DD']:
                if ('Best3SquatKg' in csv.fieldnames and
                        row[csv.index('Best3SquatKg')] != ''):
                    total += float(row[csv.index('Best3SquatKg')])
                if ('Best3BenchKg' in csv.fieldnames and
                        row[csv.index('Best3BenchKg')] != ''):
                    total += float(row[csv.index('Best3BenchKg')])
                if ('Best3DeadliftKg' in csv.fieldnames and
                        row[csv.index('Best3DeadliftKg')] != ''):
                    total += float(row[csv.index('Best3DeadliftKg')])
                if total != 0.0:
                    row[totalidx] = str(total)


# Returns None if unsuccessful, otherwise the directory name
def main(url, verbose=True):
    html = gethtml(url)

    soup = BeautifulSoup(html, 'html.parser')

    meetcsv = getmeetinfo(soup)
    dirname = getdirname(url)
    entriescsv = getresults(soup)
    if len(entriescsv.rows) == 0:
        print("No rows found!")
        return None

    addtotals(entriescsv)

    fixplace(entriescsv)
    addequipment(entriescsv, meetcsv)
    capitalise_names(entriescsv)

    remove_empty_cols_ignore_fieldname(entriescsv)

    # Wilks will be automatically calculated later.
    # Feds get it wrong all the time.
    if 'Wilks' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('Wilks')

    # Figure out event information.
    markevent(entriescsv)

    entriescsv.append_column('BirthDate')

    try:
        import shutil
        if os.path.exists(dirname):
            shutil.rmtree(dirname)
        os.makedirs(dirname)
    except OSError as exception:
        if exception.errno != errno.EEXIST:
            raise
        else:
            error("Directory '%s' already exists." % dirname)

    # Use the function for each CSV file
    write_csv_with_lf(entriescsv, os.path.join(dirname, 'entries.csv'))
    write_csv_with_lf(meetcsv, os.path.join(dirname, 'meet.csv'))

    # Writing URL to file
    with open(os.path.join(dirname, 'URL'), 'w', encoding='utf-8', newline='\n') as fd:
        fd.write(url + "\n")

    if verbose:
        print("Imported into %s." % dirname)
        print("Don't forget to run assign_date!")
    return dirname


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: %s url" % sys.argv[0])
    main(sys.argv[1])
