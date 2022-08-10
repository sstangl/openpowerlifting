#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Import data from the APU website.


from bs4 import BeautifulSoup
import errno
import os
import sys
import urllib.request

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def gethtml(url):
    req = urllib.request.Request(
        url, headers={'User-Agent': 'Mozilla/5.0 (Windows NT 6.1; Win64; x64)'})
    with urllib.request.urlopen(req) as r:
        return r.read().decode('utf-8')


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def getmeetinfo(soup):
    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    name = ""
    town = ""
    date = ""

    fed = 'APU'
    country = 'Australia'
    state = ''

    row = [fed, date, country, state, town, name]
    for i, r in enumerate(row):
        row[i] = r.replace(',', ' ').replace('  ', ' ').strip()
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

    if 'open' in div_text:
        return 'O'
    elif 'subjunior' in div_text:
        return 'SJ'
    elif 'junior' in div_text:
        return 'Jr'
    elif 'master 1' in div_text:
        return 'M1'
    elif 'master 2' in div_text:
        return 'M2'
    elif 'master 3' in div_text:
        return 'M3'
    elif 'master 4' in div_text:
        return 'M4'
    elif 'other' in div_text:
        return 'G'
    return div_text


def getresults(soup):

    # Get the results table.
    results_table = soup.find('tbody')
    if not results_table:
        error("Couldn't find the results table.")

    curr_csv = Csv()

    trs = results_table.find_all(['tr'])

    header_tds = trs[1].find_all('td')

    # Get column information.

    headers = [header_tds[ii].text.strip()
               for ii in range(len(header_tds))]

    iterable = iter(range(len(headers)))

    curr_csv.fieldnames = []

    for ii in iterable:
        h = headers[ii].lower()
        h = h.replace('(', '').replace(')', '').replace('  ', ' ')
        if h == '#':
            curr_csv.fieldnames += ['Place']
        elif h == 'name':
            curr_csv.fieldnames += ['Name']
        elif h in ['born', 'year of birth', 'birth year']:
            curr_csv.fieldnames += ['BirthYear']
        elif any(x in h for x in ['age', 'division', 'div', 'cat']):
            curr_csv.fieldnames += ['Division']
        elif h == 'team':
            curr_csv.fieldnames += ['Team']
        elif any(x in h for x in ['bwt', 'bodyweightkg', 'weight kg', 'bw']):
            curr_csv.fieldnames += ['BodyweightKg']
        elif any(x in h for x in ['cls', 'class', 'weightclass kg']):
            curr_csv.fieldnames += ['WeightClassKg']
        elif any(x in h for x in ['sq-1', 'squat1kg', 'squat 1 kg', 'sq 1']):
            curr_csv.fieldnames += ['Squat1Kg', 'Squat2Kg', 'Squat3Kg']
            [next(iterable) for x in range(2)]
        elif any(x in h for x in ['bp-1', 'bench1kg', 'bench 1 kg', 'bp 1']):
            curr_csv.fieldnames += ['Bench1Kg', 'Bench2Kg', 'Bench3Kg']
            [next(iterable) for x in range(2)]
        elif any(x in h for x in ['dl-1', 'deadlift1kg', 'deadlift 1 kg', 'dl 1',
                                  'deadlift 1']):
            curr_csv.fieldnames += ['Deadlift1Kg',
                                    'Deadlift2Kg', 'Deadlift3Kg']
            [next(iterable) for x in range(2)]
        elif any(x in h for x in ['best deadlift']):
            curr_csv.fieldnames += ['Best3DeadliftKg']
        elif any(x in h for x in ['total', 'total kg']):
            curr_csv.fieldnames += ['TotalKg']
        elif any(x in h for x in ['points', 'ipf gl points']):
            curr_csv.fieldnames += ['Wilks']
        elif h in ['place', 'pl code']:
            curr_csv.fieldnames += ['Place']
        elif h == 'sex':
            curr_csv.fieldnames += ['Sex']
        elif any(x in h for x in ['equipment', 'ipf pts code']):
            curr_csv.fieldnames += ['Equipment']
        elif h == 'event':
            curr_csv.fieldnames += ['Event']
        elif h == 'state':
            curr_csv.fieldnames += ['State']
        elif h == '':
            curr_csv.fieldnames += ['IGNORE']
        else:
            error("Unknown column name: \"%s\"" % h)

    for tr in trs[2:]:
        # Accumulate the row, but we need to look at the class of each td
        # to figure out whether lifts were good or bad.
        row = []
        if len(tr.find_all(['td'])) < 3:
            continue

        if 'Referees:' in ''.join([td.text for td in tr.find_all(['td'])]):
            break

        for td in tr.find_all(['td']):
            text = td.text.strip().replace('\r\n', '')
            text = text.replace(',', '.')

            row.append(text.replace('  ', ' '))

        curr_csv.rows += [row]

    return curr_csv

# The weight class is reported as "-72" or "84+", just remove the dash.


def fixclass(csv):
    clsidx = csv.index('WeightClassKg')
    for row in csv.rows:
        row[clsidx] = row[clsidx].replace('-', '')


def fixbirthyear(csv):
    if 'BirthYear' in csv.fieldnames:
        byidx = csv.index('BirthYear')
        for row in csv.rows:
            if row[byidx] != '':
                row[byidx] = row[byidx].split(' ')[0].strip()


def fixdivision(csv):
    if 'Division' in csv.fieldnames:
        dividx = csv.index('Division')
        for row in csv.rows:
            if row[dividx] != '':
                row[dividx] = get_division(row[dividx])


def addequipment(csv):
    if 'Equipment' not in csv.fieldnames:
        csv.append_column('Equipment')
    eqpidx = csv.index('Equipment')
    for row in csv.rows:
        if row[eqpidx] == '':
            row[eqpidx] = 'Raw'

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


def markevent(csv):
    if 'Event' in csv.fieldnames:
        return
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


def main(dirname, url):
    html = gethtml(url)

    soup = BeautifulSoup(html, 'html.parser')

    meetcsv = getmeetinfo(soup)
    entriescsv = getresults(soup)
    if len(entriescsv.rows) == 0:
        error("No rows found!")

    fixclass(entriescsv)
    fixbirthyear(entriescsv)
    fixdivision(entriescsv)
    addequipment(entriescsv)

    if 'Squat1Kg' in entriescsv.fieldnames:
        if ('Squat2Kg' not in entriescsv.fieldnames and
                'Squat3Kg' not in entriescsv.fieldnames):
            entriescsv.fieldnames[entriescsv.index(
                'Squat1Kg')] = 'Best3SquatKg'
    if 'Bench1Kg' in entriescsv.fieldnames:
        if ('Bench2Kg' not in entriescsv.fieldnames and
                'Bench3Kg' not in entriescsv.fieldnames):
            entriescsv.fieldnames[entriescsv.index(
                'Bench1Kg')] = 'Best3BenchKg'
    if 'Deadlift1Kg' in entriescsv.fieldnames:
        if ('Deadlift2Kg' not in entriescsv.fieldnames and
                'Deadlift3Kg' not in entriescsv.fieldnames):
            entriescsv.fieldnames[entriescsv.index(
                'Deadlift1Kg')] = 'Best3DeadliftKg'

    # Wilks will be automatically calculated later.
    # Feds get it wrong all the time.
    if 'Wilks' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('Wilks')

    if ('Squat1Kg' in entriescsv.fieldnames and
            'Best3SquatKg' not in entriescsv.fieldnames):
        calc_best_lift(entriescsv, 'Best3SquatKg', [
                       'Squat1Kg', 'Squat2Kg', 'Squat3Kg'])
    if ('Bench1Kg' in entriescsv.fieldnames and
            'Best3BenchKg' not in entriescsv.fieldnames):
        calc_best_lift(entriescsv, 'Best3BenchKg', [
                       'Bench1Kg', 'Bench2Kg', 'Bench3Kg'])
    if ('Deadlift1Kg' in entriescsv.fieldnames and
            'Best3DeadliftKg' not in entriescsv.fieldnames):
        calc_best_lift(entriescsv, 'Best3DeadliftKg', [
                       'Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg'])

    entriescsv.append_column('BirthDate')

    # Figure out event information.
    markevent(entriescsv)

    try:
        os.makedirs(dirname)
    except OSError as exception:
        if exception.errno != errno.EEXIST:
            raise
        else:
            error("Directory '%s' already exists." % dirname)

    with open(dirname + os.sep + 'entries.csv', 'w') as fd:
        entriescsv.write(fd)
    with open(dirname + os.sep + 'meet.csv', 'w') as fd:
        meetcsv.write(fd)
    with open(dirname + os.sep + 'URL', 'w') as fd:
        fd.write(url + "\n")

    print("Imported into %s." % dirname)


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: %s dirname url" % sys.argv[0])
    main(sys.argv[1], sys.argv[2])
