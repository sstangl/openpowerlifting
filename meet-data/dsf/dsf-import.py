#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Import data from the DSF database


from bs4 import BeautifulSoup
import errno
import os
import sys
import urllib.request
import re
import urllib.parse

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read().decode('windows-1252')


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)

# Open the points page and get the bodyweights of each lifter


def add_bodyweights(csv, url):
    points_ext = '&v=p'
    points_url = url + points_ext

    points_soup = BeautifulSoup(gethtml(points_url), 'html.parser')
    table = points_soup.find('table')

    trs = table.find_all('tr')

    headers = [x.text for x in trs[0].find_all('td')]

    bwidx = -1
    for ii in range(0, len(headers)):
        if headers[ii] == 'Vægt':
            bwidx = ii
            break

    if bwidx == -1:
        return csv

    csv.append_column('BodyweightKg')

    lc_idx = csv.index('lifter_code')
    name_idx = csv.index('Name')
    for tr in trs[1:]:
        row = [x.text for x in tr.find_all('td')]

        if len(row) > 1:  # Skip non-lifter rows:

            # Get the code for this lifter
            lifter_code = tr.find_all('td')[csv.index('Name')].a

            if lifter_code:
                lifter_code = lifter_code['href']
                lifter_code = lifter_code.replace('?dbid=profil&licnum=', '')

            # Find the corresponding code in the data we already have
            # Need to loop through all the data to find lifters in multiple divisions
            for row2 in csv.rows:
                if lifter_code:
                    if row2[lc_idx] == lifter_code:
                        row2[-1] = row[bwidx]
                else:  # No lifter code, match by name
                    if row2[name_idx] == row[name_idx]:
                        row2[-1] = row[bwidx]

    return csv


# URL will be like "/?dbid=result&id=403".
def getdirname(url):
    if '&id=' in url:
        return url.split('&id=')[1]
    else:  # Older meets aren't numbered
        year = re.search(r'(?<=aar=)(.*?)(?=\&)', url).group(0)
        place = urllib.parse.unquote(
            re.search(r'(?<=sted=).*', url).group(0), encoding='windows-1252')
        name = urllib.parse.unquote(
            re.search(r'(?<=staevne=)(.*?)(?=\&)',
                      url).group(0), encoding='windows-1252')
        event = urllib.parse.unquote(
            re.search(r'(?<=deciplin=)(.*?)(?=\&)',
                      url).group(0), encoding='windows-1252')

        place = place.replace(' ', '')
        name = name.replace(' ', '').replace('-', '')
        event = event.replace(' ', '').replace('-', '').replace('.', '')

        if place == '':
            place = '_'

        return year + '-' + place + '-' + name + '-' + event


def getmeetinfo(soup):
    month_dict = {'Jan': '01', 'Feb': '02', 'Mar': '03', 'Apr': '04', 'Maj': '05',
                  'Jun': '06', 'Jul': '07', 'Aug': '08', 'Sep': '09', 'Okt': '10',
                  'Nov': '11', 'Dec': '12'}

    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    # Get the results table.
    dbcontent = soup.find_all('div', id='dbcontent')
    if len(dbcontent) != 1:
        error("Couldn't find the results table.")

    # Get the date.
    name = dbcontent[0].h2.contents[0]

    # Strip any years from the name
    name = re.sub(r'\d{4}', '', name)

    place_date = dbcontent[0].h3.contents[0].split(', d. ')
    town = place_date[0]
    town = town.split('/')[0]
    town = town.split('&')[0]
    town = re.sub(r'\(.*\)', '', town).strip()
    date_str = place_date[1].split(' ')

    # Need to deal with meets split over months
    day = date_str[0].split('-')[0]
    if len(day) == 1:
        day = '0'+day

    month = month_dict[date_str[1].split('-')[0]]
    year = date_str[2]

    date = year + '-' + month + '-' + day

    fed = 'DSF'
    country = 'Denmark'
    state = ''

    row = [fed, date, country, state, town, name]
    for i, r in enumerate(row):
        row[i] = r.replace(',', ' ').replace('  ', ' ').strip()

    csv.rows = [row]
    return csv


def getresults(soup):
    csv = Csv()

    # Get the results table.
    dbcontent = soup.find_all('div', id='dbcontent')
    if len(dbcontent) != 1:
        error("Couldn't find the results table.")

    table = dbcontent[0].find_next('table')

    trs = table.find_all('tr')

    # Get column information.
    headers = [x.text for x in trs[0].find_all('td')]

    csv.fieldnames = []
    for h in headers:
        if h == '#':
            csv.fieldnames += ['Place']
        elif h == 'Vægt':
            csv.fieldnames += ['BodyweightKg']
        elif h == 'Navn':
            csv.fieldnames += ['Name']
        elif h == 'Født' or h == 'Årgang':
            csv.fieldnames += ['BirthYear']
        elif h == 'Klub':
            csv.fieldnames += ['Team']
        elif h == 'SQ1':
            csv.fieldnames += ['Squat1Kg']
        elif h == 'SQ2':
            csv.fieldnames += ['Squat2Kg']
        elif h == 'SQ3':
            csv.fieldnames += ['Squat3Kg']
        elif h == 'SQ' or h == 'Squat':
            csv.fieldnames += ['Best3SquatKg']
        elif h == 'BP1':
            csv.fieldnames += ['Bench1Kg']
        elif h == 'BP2':
            csv.fieldnames += ['Bench2Kg']
        elif h == 'BP3':
            csv.fieldnames += ['Bench3Kg']
        elif h == 'BP' or h == 'Bænk':
            csv.fieldnames += ['Best3BenchKg']
        elif h == 'DL1':
            csv.fieldnames += ['Deadlift1Kg']
        elif h == 'DL2':
            csv.fieldnames += ['Deadlift2Kg']
        elif h == 'DL3':
            csv.fieldnames += ['Deadlift3Kg']
        elif h == 'DL' or h == 'Dødløft':
            csv.fieldnames += ['Best3DeadliftKg']
        elif h == 'Total':
            csv.fieldnames += ['TotalKg']
        elif h == 'Points':
            csv.fieldnames += ['Wilks']
        elif h == 'K' or h == 'RAW':
            csv.fieldnames += ['Equipment']
        elif h == 'Rekorder':
            csv.fieldnames += ['Records']
        else:
            error("Unknown column name: \"%s\"" % h)

    # These columns are added from the category rows.
    csv.fieldnames += ['Division', 'Sex', 'WeightClassKg', 'lifter_code']

    append_eqp = False
    # Classic meets and pre 2011 meets won't mark equipment, work it out contextually
    if 'Equipment' not in csv.fieldnames:
        csv.fieldnames += ['Equipment']
        append_eqp = True

    divstate = None
    sexstate = None
    wcstate = None

    for tr in trs[1:]:

        row = [x.text for x in tr.find_all('td')]
        if ''.join(row[0]) != '':  # Skip blank rows:

            # Rows of length >1 are actual results, of length 1 are categories.
            if len(row) == 1:

                # Extract sex information.
                text = row[0].lower().replace('\n', '')

                # Contextual rows are either weight classes or divisions
                if 'kg' in text:
                    wcstate = text.replace('kg', '').strip()
                else:
                    if 'damer' in text:
                        sexstate = 'F'
                        text = text.replace('damer', '').strip()
                    elif 'herrer' in text:
                        sexstate = 'M'
                        text = text.replace('herrer', '').strip()

                    # Extract division information.
                    if 'sub-junior' in text:
                        divstate = text.replace('sub-junior', 'Sub-Juniors')
                    elif 'junior' in text:
                        divstate = text.replace('junior', 'Juniors')
                    elif 'senior' in text:
                        divstate = 'Open'
                    elif 'master' in text:
                        divstate = text.replace('master', 'Master')
                    elif 'ungdom' in text:
                        divstate = text.replace('ungdom', 'Teen')
                    elif 'handicap' in text:
                        divstate = text.replace('handicap', 'Handicapped')
                    elif text == '':
                        divstate = 'Open'
                    else:
                        error("Unknown state: \"%s\"" % row[0])

            else:
                assert divstate
                assert sexstate
                assert wcstate

                lifter_code = tr.find_all('td')[csv.index('Name')].a
                if lifter_code:
                    lifter_code = tr.find_all(
                        'td')[csv.index('Name')].a['href']
                    lifter_code = lifter_code.replace(
                        '?dbid=profil&licnum=', '')
                else:  # Occasionally lifters don't have a code,if so identify by name
                    lifter_code = ''

                # Accumulate the row, but we need to look at the class of each td
                # to figure out whether lifts were good or bad.
                row = []
                for td in tr.find_all('td'):
                    text = td.text
                    c = td.get('class')
                    if c and 'tdliveneg' in c:  # Failed lift.
                        text = '-' + text

                    row.append(text.strip().replace('  ', ' ').replace(
                        '  ', ' ').replace(',', ' '))

                row = row + [divstate, sexstate, wcstate, lifter_code]
                if append_eqp:
                    row = row + ['']

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
        if '+' in row[clsidx]:
            row[clsidx] = row[clsidx].replace('+', '')+'+'


# The equipment is marked as "x" for Raw, or nothing for Single-ply.
def fixequipment(csv, meetcsv):
    meet_name = meetcsv.rows[0][5]

    raw_meet = False

    if 'klassisk' in meet_name.lower():
        raw_meet = True

    eqidx = csv.index('Equipment')
    for row in csv.rows:
        if raw_meet:
            row[eqidx] = 'Raw'
        else:
            if 'x' in row[eqidx]:
                row[eqidx] = 'Raw'
            elif row[eqidx] == '':
                row[eqidx] = 'Single-ply'
            else:
                error("Unknown equipment: \"%s\"" % row[eqidx])

# The birthyear column sometimes also contains age division, remove this.


def fixbirthyear(csv):
    if 'BirthYear' in csv.fieldnames:
        byidx = csv.index('BirthYear')
        for row in csv.rows:
            row[byidx] = re.sub(r'\(.*\)', '', row[byidx]).strip()

# Lifters without a recorded bodyweight have a dash, remove this.


def fixbodyweight(csv):
    if 'BodyweightKg' in csv.fieldnames:
        bwidx = csv.index('BodyweightKg')
        for row in csv.rows:
            if row[bwidx] == '-':
                row[bwidx] = ''

# The reported total doesn't take into account chips.


def fixtotals(csv):
    def weight(str):
        try:
            return float(str)
        except ValueError:
            return 0.0

    if 'TotalKg' in csv.fieldnames:
        totalidx = csv.index('TotalKg')

        sqidx = -1
        bpidx = -1
        dlidx = -1

        if 'Best3SquatKg' in csv.fieldnames:
            sqidx = csv.index('Best3SquatKg')

        if 'Best3BenchKg' in csv.fieldnames:
            bpidx = csv.index('Best3BenchKg')

        if 'Best3DeadliftKg' in csv.fieldnames:
            dlidx = csv.index('Best3DeadliftKg')

        for row in csv.rows:
            if row[totalidx] != '':
                sq = 0
                bp = 0
                dl = 0

                if sqidx != -1:
                    sq = weight(row[sqidx])

                if bpidx != -1:
                    bp = weight(row[bpidx])

                if dlidx != -1:
                    dl = weight(row[dlidx])

                # Do this instead of summing attempts for everyone
                # to pick up inconsistencies in reported total
                if sq % 2.5 != 0 or bp % 2.5 != 0 or dl % 2.5 != 0:
                    row[totalidx] = str(sq + bp + dl)

    else:  # Single-lift meets don't have totals
        sqidx = -1
        bpidx = -1
        dlidx = -1

        if 'Best3SquatKg' in csv.fieldnames:
            sqidx = csv.index('Best3SquatKg')

        if 'Best3BenchKg' in csv.fieldnames:
            bpidx = csv.index('Best3BenchKg')

        if 'Best3DeadliftKg' in csv.fieldnames:
            dlidx = csv.index('Best3DeadliftKg')

        if len([idx for idx in [sqidx, bpidx, dlidx] if idx != -1]) == 1:
            csv.fieldnames += ['TotalKg']

            for row in csv.rows:
                if sqidx != -1:
                    row.append(row[sqidx])
                elif bpidx != -1:
                    row.append(row[bpidx])
                elif dlidx != -1:
                    row.append(row[dlidx])


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

    soup = BeautifulSoup(html, 'html.parser')

    meetcsv = getmeetinfo(soup)
    dirname = getdirname(url)
    entriescsv = getresults(soup)

    if len(entriescsv.rows) == 0:
        error("No rows found!")

    # New meets have bodyweights on a different page
    if 'BodyweightKg' not in entriescsv.fieldnames:
        entriescsv = add_bodyweights(entriescsv, url)

    # Only need this column for processing
    entriescsv.remove_column_by_name('lifter_code')

    # Don't care about this column
    if 'Records' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('Records')

    # Wilks will be automatically calculated later.
    # Feds get it wrong all the time.
    if 'Wilks' in entriescsv.fieldnames:
        entriescsv.remove_column_by_name('Wilks')

    fixtotals(entriescsv)
    fixplace(entriescsv)
    fixclass(entriescsv)
    fixequipment(entriescsv, meetcsv)
    fixbirthyear(entriescsv)
    fixbodyweight(entriescsv)

    # Figure out event information.
    markevent(entriescsv)

    if 'BirthDate' not in entriescsv.fieldnames:
        entriescsv.insert_column(entriescsv.index('BirthYear'), 'BirthDate')

    try:
        os.makedirs(dirname)
    except OSError as exception:
        if exception.errno != errno.EEXIST:
            raise
        else:
            error("Directory '%s' already exists." % dirname)

    entriescsv.write_filename(dirname + os.sep + 'entries.csv')
    meetcsv.write_filename(dirname + os.sep + 'meet.csv')
    with open(dirname + os.sep + 'URL', 'w') as fd:
        fd.write(url + "\n")

    print("Imported into %s." % dirname)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: %s url" % sys.argv[0])
    main(sys.argv[1])
