#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Import data from the new CSST website

from bs4 import BeautifulSoup
import errno
import os
import sys
import re
import urllib.request

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read()


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def getmeetinfo(soup):
    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    name = soup.find('h2').text
    name = re.sub(r'\d{2}\. |\d{1}\. ', '', name)
    details = soup.findAll('div', {'class': 'col-xs-12'})

    dotdate = details[1].text.replace('Od-do:', '').replace(' ', '')
    town = details[2].text.replace('Místo konání: ', '')

    assert '.' in dotdate
    [day, month, year] = dotdate.split('.')
    day = day.strip()
    if len(day) == 1:
        day = '0'+day
    month = month.strip()
    if len(month) == 1:
        month = '0'+month
    year = year.strip()
    date = '%s-%s-%s' % (year, month, day)

    fed = 'CSST'
    country = 'Czechia'
    state = ''

    row = [fed, date, country, state, town, name]
    for i, r in enumerate(row):
        row[i] = r.replace(',', ' ').replace('  ', ' ').strip()
    csv.rows = [row]
    return csv


def getresults(soup, meetcsv):
    csv = Csv()

    cl_detail = soup.find('div', {'class': 'clanek-detail'})

    # Get the results table.
    results_table = cl_detail.find('div', {'id': 'results'})
    if results_table is None:
        error("Couldn't find the results table.")

    subtables = results_table.findAll('div', {'class', 'sutable'})

    hasAbsoluteRankings = False
    if results_table.find('h2'):
        hasAbsoluteRankings = True

    descriptors = results_table.find_all('div', {'class': 'table-title'})

    for ii in range(0, len(subtables)):
        if ii >= len(descriptors):
            break
        curr_table = subtables[ii]
        curr_descriptor = descriptors[ii]

        # Need to break when we hit the Absolute rankings section
        if hasAbsoluteRankings and curr_table.findNext('h2') is None:
            break

        data = curr_table.find('tbody')

        if csv.fieldnames == []:
            # Get column information.
            headers = [x.text.strip()
                       for x in curr_table.find('tr').find_all('th')]

            for ii in range(len(headers)):
                h = headers[ii]
                if h in ['#'] and ii == 0:
                    csv.fieldnames += ['Place']
                elif h in ['#']:
                    csv.fieldnames += ['IGNORE']
                elif h.lower() == 'jméno':
                    csv.fieldnames += ['Name']
                elif h in ['Nar/St.č/TH', 'Nar/TH']:
                    csv.fieldnames += ['BirthYear', 'Lot', 'BodyweightKg']
                elif h == 'Oddíl':
                    csv.fieldnames += ['Team']
                elif h == 'Univerzita':
                    csv.fieldnames += ["College/University"]
                elif h == 'DŘ1':
                    csv.fieldnames += ['Squat1Kg']
                elif h == 'DŘ2':
                    csv.fieldnames += ['Squat2Kg']
                elif h == 'DŘ3':
                    csv.fieldnames += ['Squat3Kg']
                elif h == 'DŘ':
                    csv.fieldnames += ['Best3SquatKg']
                elif h == 'BP1':
                    csv.fieldnames += ['Bench1Kg']
                elif h == 'BP2':
                    csv.fieldnames += ['Bench2Kg']
                elif h == 'BP3':
                    csv.fieldnames += ['Bench3Kg']
                elif h == 'BP':
                    csv.fieldnames += ['Best3BenchKg']
                elif h == 'MT1':
                    csv.fieldnames += ['Deadlift1Kg']
                elif h == 'MT2':
                    csv.fieldnames += ['Deadlift2Kg']
                elif h == 'MT3':
                    csv.fieldnames += ['Deadlift3Kg']
                elif h == 'DL1':
                    csv.fieldnames += ['Deadlift1Kg']
                elif h == 'DL2':
                    csv.fieldnames += ['Deadlift2Kg']
                elif h == 'DL3':
                    csv.fieldnames += ['Deadlift3Kg']
                elif h == 'MT':
                    csv.fieldnames += ['Best3DeadliftKg']
                elif h == 'Total':
                    csv.fieldnames += ['TotalKg']
                elif h == 'Body':
                    csv.fieldnames += ['Wilks']
                elif h == 'VT':  # I think this is a ranking?
                    csv.fieldnames += ['Ranking']
                else:
                    error("Unknown column name: \"%s\"" % h)

            # These columns are added from the category rows.
            csv.fieldnames += ['Division', 'Sex', 'WeightClassKg']

        wcstate = ''
        divstate = 'Open'
        sexstate = ''
        isTeamMeet = False
        team = ''

        # Extract sex information.
        div_text = curr_descriptor.text.lower()
        # Parse the division
        if 'ženy' in div_text:
            sexstate = 'F'
            div_text = div_text.replace('ženy', '').strip()
        elif 'muži' in div_text:
            sexstate = 'M'
            div_text = div_text.replace('muži', '').strip()

        # Extract division information.
        if 'dorostenky' in div_text:
            divstate = 'Sub-Juniors'
            sexstate = 'F'
        elif 'dorostenci' in div_text:
            divstate = 'Sub-Juniors'
            sexstate = 'M'
        elif 'juniorky' in div_text:
            divstate = 'Juniors'
            sexstate = 'F'
        elif 'junioři' in div_text:
            divstate = 'Juniors'
            sexstate = 'M'
        elif 'm1' in div_text:
            divstate = 'Masters 1'
        elif 'm2' in div_text:
            divstate = 'Masters 2'
        elif 'm3' in div_text:
            divstate = 'Masters 3'
        elif 'm4' in div_text:
            divstate = 'Masters 4'

        # Team meet
        elif re.search(r'\d{1}\. |\d{2}\. ', div_text):
            team = re.sub(r'\d{1}\. |\d{2}\. ', '', div_text)
            place = re.search(r'\d{1}\.|\d{2}\.', div_text).group(0)
            divstate = 'Open'
            sexstate = 'M'
            isTeamMeet = True
        else:
            divstate = 'Open'

        if isTeamMeet and 'Team' not in csv.fieldnames:
            csv.fieldnames += ['Team']
            csv.fieldnames += ['Place']

        for tr in data.findAll('tr'):
            row = [x.text for x in tr.find_all('td')]

            # Weightclass descriptor
            if tr.find('th'):
                wc_text = tr.text
                wcstate = re.search(
                    r'\d{3}|\d{2}|\+\d{3}|\+\d{2}', wc_text).group(0)
                if '+' in wcstate:
                    wcstate = wcstate.replace('+', '')+'+'
                if wc_text[0] == 'M':
                    sexstate = 'M'
                elif wc_text[0] == 'Ž':
                    sexstate = 'F'
                continue

            assert divstate
            assert sexstate is not None

            # Accumulate the row, but we need to look at the class of each td
            # to figure out whether lifts were good or bad.
            row = []
            for td in tr.find_all('td'):
                if len(row) in [csv.index('Name'), csv.index('BirthYear')]:
                    split_td = [x.strip() for x in td.text.split('/')]
                else:
                    split_td = [td.text.strip()]

                for text in split_td:
                    if text in ['-', '#', '0']:
                        text = ''

                    # They mark when a person has a degree...
                    text = text.replace(', Ing.', '')
                    text = text.replace(', Mgr.', '')
                    text = text.replace('\xa0', ' ')
                    text = text.replace('*', '')
                    text = text.replace('  ', ' ').replace(',', '.')

                    if (td.has_attr("class") and len(td['class']) > 1
                            and td['class'][1] == 'invalid'):
                        text = '-'+text
                    if td.find('span', {'class': 'record-attempt'}):
                        text = text.replace('R', '')

                    row.append(text.strip())

            row = row + [divstate, sexstate, wcstate]
            if isTeamMeet:
                row = row + [team, place]
            if len(row) > 5:
                csv.rows += [row]

    return csv


# The place is reported as "1.", just remove the period.
def fixplace(csv):
    placeidx = csv.index('Place')
    totalidx = csv.index('TotalKg')
    for row in csv.rows:
        row[placeidx] = row[placeidx].replace('.', '')
        if row[placeidx] in ['-', '', '0']:
            row[placeidx] = 'DQ'
            row[totalidx] = ''  # Instead of a zero.
        elif row[placeidx].lower() == 'gj':  # Guest lifter, I think.
            row[placeidx] = 'G'


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

# Equipment is marked in the meet name


def addequipment(csv, meetcsv):
    meet_name = meetcsv.rows[0][5]

    raw_meet = False
    classic_divs = ['klassisk', 'klbp', 'klsl', 'kl. sl', 'kl. bp', 'klasickém']
    if any(classic in meet_name.lower() for classic in classic_divs):
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


def addtotals(csv):

    if 'TotalKg' not in csv.fieldnames:
        csv.append_column('TotalKg')
        placeidx = csv.index('Place')
        totalidx = csv.index('TotalKg')
        for row in csv.rows:
            total = 0
            if row[placeidx] not in ['DQ', 'DD', 'DSQ']:
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
    entriescsv = getresults(soup, meetcsv)
    if len(entriescsv.rows) == 0:
        error("No rows found!")

    remove_empty_cols_ignore_fieldname(entriescsv)
    addtotals(entriescsv)

    if 'Place' in entriescsv.fieldnames:
        fixplace(entriescsv)
    addequipment(entriescsv, meetcsv)

    # Wilks will be automatically calculated later.
    # Feds get it wrong all the time.
    if 'Wilks' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('Wilks')

    if 'Ranking' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('Ranking')

    if 'Lot' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('Lot')

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
        sys.exit(1)
    main(sys.argv[1], sys.argv[2])
