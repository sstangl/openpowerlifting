#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Import data from the Kraft.is website.
# Their data format is actually standardised!


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


def getdirname(url):
    return url.split('/meet/')[1]


def getmeetinfo(soup):
    csv = Csv()
    csv.fieldnames = ['Federation', 'Date', 'MeetCountry',
                      'MeetState', 'MeetTown', 'MeetName']

    info = soup.find('div', {'class': 'm-portlet portlet-btm-space'})

    name = info.find('h1').text
    name = name.replace("KRAFT", "").strip()

    # Get the date.
    dateplace = info.find_all('div')[-1].text

    split_text = [text.strip()
                  for text in dateplace.split('\r\n') if text.strip() != '']

    dashdate = split_text[0]
    if len(split_text) == 2:
        town = split_text[1].split(',')[0].strip()
    else:
        town = ''

    # Multiday meets list the full date seperately, which is nice of them
    if ' - ' in dashdate:
        dashdate = dashdate.split(' - ')[0]

    assert '-' in dashdate
    [day, month, year] = dashdate.split('-')
    date = '%s-%s-%s' % (year, month, day)

    fed = 'KRAFT'
    country = 'Iceland'
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
        return 'Open'
    elif 'subjunior' in div_text:
        return 'Sub-Juniors'
    elif 'junior' in div_text:
        return 'Juniors'
    elif 'masters 1' in div_text:
        return 'Masters 1'
    elif 'masters 2' in div_text:
        return 'Masters 2'
    elif 'masters 3' in div_text:
        return 'Masters 3'
    elif 'masters 4' in div_text:
        return 'Masters 4'
    elif 'other' in div_text:
        return 'Guest'
    return ''


def getresults(soup):
    csv = Csv()
    divstate = None
    sexstate = None
    wcstate = None

    # Get the results table.
    result_tables = soup.find_all('div',
                                  {'class': 'm-portlet__body portlet-body-padding'})
    if len(result_tables) < 2:
        error("Couldn't find the results table.")

    for table in result_tables[1:]:
        curr_csv = Csv()

        trs = table.find_all(['tr'])

        # Get the first division
        startdiv = table.find('div', {'class': 'm-section__sub'}).text.strip()
        if 'Women' in startdiv:
            sexstate = 'F'
        elif 'Men' in startdiv:
            sexstate = 'M'

        divstate = get_division(startdiv)
        # Get column information.
        expand_colspans(trs[0], soup)
        expand_colspans(trs[1], soup)
        header1 = trs[0].find_all('th')
        header2 = trs[1].find_all('th')
        event_text = [header1[ii].text.strip()
                      for ii in range(len(header1))]

        headers = [header2[ii].text.strip()
                   for ii in range(len(header2))]

        iterable = iter(range(len(headers)))

        curr_csv.fieldnames = []

        for ii in iterable:
            h = headers[ii]
            curr_event = event_text[ii]
            if h == '#':
                curr_csv.fieldnames += ['Place']
            elif h == 'Name':
                curr_csv.fieldnames += ['Name']
            elif h == 'Born':
                curr_csv.fieldnames += ['BirthYear']
            elif h == 'Team':
                curr_csv.fieldnames += ['Team']
            elif h == 'Weight':
                curr_csv.fieldnames += ['BodyweightKg']
            elif h == '1' and curr_event == 'Squat':
                curr_csv.fieldnames += ['Squat1Kg', 'Squat2Kg', 'Squat3Kg']
                [next(iterable) for x in range(2)]
            elif h == '1' and curr_event == 'Benchpress':
                curr_csv.fieldnames += ['Bench1Kg', 'Bench2Kg', 'Bench3Kg']
                [next(iterable) for x in range(2)]
            elif h == '1' and curr_event == 'Deadlift':
                curr_csv.fieldnames += ['Deadlift1Kg',
                                        'Deadlift2Kg', 'Deadlift3Kg']
                [next(iterable) for x in range(2)]
            elif h == 'Total':
                curr_csv.fieldnames += ['TotalKg']
            elif h == 'Wilks' or h == 'IPF Points' or h == 'IPF GL Points':
                curr_csv.fieldnames += ['Wilks']
            elif h == '':
                curr_csv.fieldnames += ['IGNORE']
            else:
                error("Unknown column name: \"%s\"" % h)

        # These columns are added from the category rows.
        curr_csv.fieldnames += ['Division', 'Sex', 'WeightClassKg']

        for tr in trs[1:]:
            row = [x.text for x in tr.find_all(["td", "th"])]

            # Rows of length >1 are actual results, of length 1 are categories.
            if len(row) == 1 and row[0].strip() != '':
                text = row[0].lower()
                if ' kg' in text:
                    wcstate = text.replace('kg', '').strip().replace(',', '.')
                    continue

                if 'women' in text:
                    sexstate = 'F'
                elif 'men' in text:
                    sexstate = 'M'

                div = get_division(text)

                # Extract division information.
                if div:
                    divstate = div
                else:
                    error("Unknown state: \"%s\"" % row[0])

            elif row != [] and row[0].strip() not in ['#', '']:
                assert divstate
                assert sexstate
                assert wcstate

                # Accumulate the row, but we need to look at the class of each td
                # to figure out whether lifts were good or bad.
                row = []
                for td in tr.find_all(['td', 'th']):
                    text = td.text.strip().replace('\r\n', '')
                    text = text.replace(',', '.')

                    if td.span:
                        c = td.span.get('style')
                        if c and 'color:red' in c:  # Failed lift.
                            text = '-' + text
                    elif text == '-':
                        text = ''

                    row.append(text.replace('  ', ' '))

                row = row + [divstate, sexstate, wcstate]
                curr_csv.rows += [row]
        csv.cat(curr_csv)

    return csv


# The place is reported as "1.", just remove the period.
def fixplace(csv):
    placeidx = csv.index('Place')
    totalidx = csv.index('TotalKg')
    dividx = csv.index('Division')
    for row in csv.rows:
        row[placeidx] = row[placeidx].replace('.', '')
        if row[placeidx] == '':
            row[placeidx] = 'DQ'
            row[totalidx] = ''  # Instead of a zero.
        if row[dividx] == 'Guest':
            row[placeidx] = 'G'
            row[dividx] = 'Open'


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


# Equipment is marked in the sheet name
def addequipment(csv, meetcsv):
    meet_name = meetcsv.rows[0][5]

    raw_meet = False
    if any(classic in meet_name.lower() for classic in ['klassískum', 'klassískri']):
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

    while 'IGNORE' in csv.fieldnames:
        csv.remove_column_by_name('IGNORE')

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        csv.remove_column_by_index(idx)


def combine_columns(csv, attemptlist):
    for k in attemptlist:
        assert k in csv.fieldnames

    one_lift = True
    for row in csv.rows:
        num_attempts = 0
        for k in attemptlist:
            if row[csv.index(k)] != '':
                num_attempts += 1
            if num_attempts == 2:
                one_lift = False
                break

    if one_lift:
        for row in csv.rows:
            for ii in range(1, len(attemptlist)):
                if row[csv.index(attemptlist[ii])] != '':
                    row[csv.index(attemptlist[0])
                        ] = row[csv.index(attemptlist[ii])]
                    row[csv.index(attemptlist[ii])] = ''
                    break


def main(url, input_dir):
    html = gethtml(url)

    soup = BeautifulSoup(html, 'html.parser')

    meetcsv = getmeetinfo(soup)
    dirname = input_dir     # getdirname(url)
    entriescsv = getresults(soup)
    if len(entriescsv.rows) == 0:
        error("No rows found!")

    fixplace(entriescsv)
    fixclass(entriescsv)
    fixbirthyear(entriescsv)
    addequipment(entriescsv, meetcsv)

    # Some old meet only have a single attempt column filled in, it is not consistent
    # which column this is so put it in the first attempt.
    if 'Squat1Kg' in entriescsv.fieldnames:
        combine_columns(entriescsv, ['Squat1Kg', 'Squat2Kg', 'Squat3Kg'])
    if 'Bench1Kg' in entriescsv.fieldnames:
        combine_columns(entriescsv, ['Bench1Kg', 'Bench2Kg', 'Bench3Kg'])
    if 'Deadlift1Kg' in entriescsv.fieldnames:
        combine_columns(
            entriescsv, ['Deadlift1Kg', 'Deadlift2Kg', 'Deadlift3Kg'])

    # For old meets, the 1-2-3 results are blank, and only one of them is filled in.
    # If after removing empty columns, only one lift is left, then that's a
    # Best3 column.
    remove_empty_cols_ignore_fieldname(entriescsv)
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
        print("Usage: %s url importdir" % sys.argv[0])
    main(sys.argv[1], sys.argv[2])
