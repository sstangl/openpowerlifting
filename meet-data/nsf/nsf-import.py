#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Import data from the extremely well-designed NSF website.
# Since they use a database for all their meets, everything
# is imported very easily, as with the USAPL.
#
# Except these guys actually show all of their data, and seem
# to take it seriously.
#


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
    with urllib.request.urlopen(url) as r:
        return r.read().decode('utf-8', errors='replace')


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


# URL will be like "/?page=protokoll_vis&id=3508".
def getdirname(url):
    return url.split('id=')[1]


def getmeetinfo(soup):
    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    # Get the facts table.
    fakta = soup.find_all('h4', text='Fakta')
    if len(fakta) != 1:
        error("Couldn't find the facts table.")

    # Go through td -> tr -> tbody.
    tbody = fakta[0].parent.parent.parent

    # Get the date.
    dato = tbody.find('td', text='Dato')
    # Skip to the next <td> in that row.
    dotdate = dato.find_next('td').text
    # The date is formatted as dd.mm.yyyy.
    assert '.' in dotdate
    [day, month, year] = dotdate.split('.')
    date = '%s-%s-%s' % (year, month, day)

    # Get the town.
    sted = tbody.find('td', text='Sted')
    # Skip to the next <td> in that row.
    town = sted.next.next.next.text

    # Get the competition name.
    h2s = soup.find_all('h2')
    if len(h2s) != 1:
        error("Need a better way to figure out the competition name.")
    name = h2s[0].text
    # Get the organising club name
    organiser = tbody.find('td', text='Arrangør')
    org = organiser.find_next('td').text
    name = org + ' ' + name

    fed = 'NSF'
    country = 'Norway'
    state = ''

    row = [fed, date, country, state, town, name]
    for i, r in enumerate(row):
        row[i] = r.replace(',', ' ').replace('  ', ' ').strip()
    csv.rows = [row]
    return csv


def getresults(soup):
    csv = Csv()

    # Get the results table.
    resultater = soup.find_all('h4', text='Resultater')
    if len(resultater) != 1:
        error("Couldn't find the results table.")

    table = resultater[0].find_next('table')

    trs = table.find_all('tr')

    # Get column information.
    headers = [x.text for x in trs[0].find_all('td')]

    csv.fieldnames = []
    for h in headers:
        if h == '#':
            csv.fieldnames += ['Place']
        elif h == 'Kat.':
            csv.fieldnames += ['WeightClassKg']
        elif h == 'Navn':
            csv.fieldnames += ['Name', 'Equipment']
        elif h == 'Klubb':
            csv.fieldnames += ['Team']
        elif h == 'Kr.vekt':
            csv.fieldnames += ['BodyweightKg']
        elif h == 'Knebøy':
            csv.fieldnames += ['Squat1Kg', 'Squat2Kg', 'Squat3Kg']
        elif h == 'Benkpress':
            csv.fieldnames += ['Bench1Kg', 'Bench2Kg', 'Bench3Kg']
        elif h == 'Markløft':
            csv.fieldnames += ['Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg']
        elif h == 'Totalt':
            csv.fieldnames += ['TotalKg']
        elif h == 'Poeng':
            csv.fieldnames += ['Wilks']
        else:
            error("Unknown column name: \"%s\"" % h)

    # These columns are added from the category rows.
    csv.fieldnames += ['Division', 'Sex']

    divstate = None
    sexstate = None

    for tr in trs[1:]:
        row = [x.text for x in tr.find_all('td')]

        # Rows of length >1 are actual results, of length 1 are categories.
        if len(row) == 1:
            # Extract sex information.
            text = row[0].lower()
            if 'damer' in text:
                sexstate = 'F'
                text = text.replace('damer', '').strip()
            elif 'herrer' in text:
                sexstate = 'M'
                text = text.replace('herrer', '').strip()

            # Extract division information.
            if 'ungdom' in text:
                divstate = text.replace('ungdom', 'Teen')
            elif 'junior' in text:
                divstate = text.replace('junior', 'Juniors')
            elif 'åpen' in text:
                divstate = 'Open'
            elif 'veteran' in text:
                divstate = text.replace('veteran', 'Masters')
            elif 'rekrutt' in text:
                divstate = text.capitalize()
            else:
                error("Unknown state: \"%s\"" % row[0])

        else:
            assert divstate
            assert sexstate

            # Accumulate the row, but we need to look at the class of each td
            # to figure out whether lifts were good or bad.
            row = []
            for td in tr.find_all('td'):
                text = td.text
                c = td.get('class')
                if c and 'underkjent' in c:  # Failed lift.
                    text = '-' + text
                if c and 'ikke_loftet' in c:  # Skipped lift.
                    text = ''
                row.append(text.strip().replace('  ', ' ').replace(',', ' '))

            row = row + [divstate, sexstate]
            csv.rows += [row]

    return csv


# The place is reported as "1.", just remove the period.
def fixplace(csv):
    placeidx = csv.index('Place')
    totalidx = csv.index('TotalKg')
    for row in csv.rows:
        row[placeidx] = row[placeidx].replace('.', '')
        if row[placeidx] == '-':
            row[placeidx] = 'DQ'
            row[totalidx] = ''  # Instead of a zero.
        elif row[placeidx].lower() == 'gj':  # Guest lifter, I think.
            row[placeidx] = 'G'


# The weight class is reported as "-72" or "84+", just remove the dash.
def fixclass(csv):
    clsidx = csv.index('WeightClassKg')
    for row in csv.rows:
        row[clsidx] = row[clsidx].replace('-', '')


# The equipment is marked as "*" for Raw, or nothing for Single-ply.
def fixequipment(csv):
    eqidx = csv.index('Equipment')
    for row in csv.rows:
        if '*' in row[eqidx]:
            row[eqidx] = 'Raw'
        elif row[eqidx] == '':
            row[eqidx] = 'Single-ply'
        else:
            error("Unknown equipment: \"%s\"" % row[eqidx])


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

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        csv.remove_column_by_index(idx)


def main(url):
    html = gethtml(url)

    # The HTML returned by the website is malformed, which breaks the parser.
    # Part of the results table looks like: <tr>Totalt</th>,
    #  which is treated as </table>.
    # Work around this by getting rid of all <th>.
    html = html.replace('<th>', '<td>')
    html = html.replace('</th>', '</td>')
    # An inline script is accidentally closed with too many quotation marks.
    html = html.replace(';"" ', ';" ')
    # The following text occurs for bench-only meets:
    #   <td align="center" colspan="3">Benkpress</strong>
    # Which again trips up the parser.
    html = html.replace('<strong>', '')
    html = html.replace('</strong>', '')

    soup = BeautifulSoup(html, 'html.parser')

    meetcsv = getmeetinfo(soup)
    dirname = getdirname(url)
    entriescsv = getresults(soup)
    if len(entriescsv.rows) == 0:
        error("No rows found!")

    fixplace(entriescsv)
    fixclass(entriescsv)
    fixequipment(entriescsv)

    # For old meets, the 1-2-3 results are blank, and only one of them is filled in.
    # If after removing empty columns, only one lift is left, then that's a
    # Best3 column.
    remove_empty_cols_ignore_fieldname(entriescsv)
    if 'Squat1Kg' in entriescsv.fieldnames:
        if ('Squat2Kg' not in entriescsv.fieldnames and
                'Squat3Kg' not in entriescsv.fieldnames):
            entriescsv.fieldnames[entriescsv.index('Squat1Kg')] = 'Best3SquatKg'
    if 'Bench1Kg' in entriescsv.fieldnames:
        if ('Bench2Kg' not in entriescsv.fieldnames and
                'Bench3Kg' not in entriescsv.fieldnames):
            entriescsv.fieldnames[entriescsv.index('Bench1Kg')] = 'Best3BenchKg'
    if 'Deadlift1Kg' in entriescsv.fieldnames:
        if ('Deadlift2Kg' not in entriescsv.fieldnames and
                'Deadlift3Kg' not in entriescsv.fieldnames):
            entriescsv.fieldnames[entriescsv.index('Deadlift1Kg')] = 'Best3DeadliftKg'

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

    if 'BirthDate' not in entriescsv.fieldnames:
        index = entriescsv.index('Name') + 1
        entriescsv.insert_column(index, 'BirthDate')

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
    if len(sys.argv) != 2:
        print("Usage: %s url" % sys.argv[0])
    main(sys.argv[1])
