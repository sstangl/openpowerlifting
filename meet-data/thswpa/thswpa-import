#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# THSWPA thankfully puts all their meets into a database, which is easy to scrape.
# This script taken a thswpa.com URL and converts the results to
# the OpenPowerlifting internal format. It also creates the directory.
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


BASEURL = "http://www.thswpa.com"


def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read().decode('utf-8')


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def strtokg(s):
    try:
        f = float(s)
        if f == 0.0:
            return ''
        r = "{:.2f}".format(f / 2.20462262)
        r = r.replace('.00', '')
        if '.' in r and r[-1] == '0':
            r = r[:-1]
        return r
    except ValueError:
        print("Field not a float: \"%s\"" % s, file=sys.stderr)
        sys.exit(1)


def weightclass2kg(s, sex):
    if s == '97':  # 97.5 by rulebook
        return '44.23'
    if s == '105':  # 105.5 by rulebook
        return '47.85'
    if s == '114':  # 114.5 by rulebook
        return '51.94'
    if s == '123':  # 123.5 by rulebook
        return '56.02'
    if s == '132':  # 132.5 by rulebook
        return '60.1'
    if s == '148':  # 148.5 by rulebook
        return '67.36'
    if s == '165':  # 165.5 by rulebook
        return '75.07'
    if s == '181':  # 181.5 by rulebook
        return '82.33'
    if s == '198':  # 198.5 by rulebook
        return '90.04'
    if s == '220':  # 220.5 by rulebook
        return '100.02'
    if s == '242':  # 242.5 by rulebook
        return '110'  # 109.996 rounded up
    if s == '259':  # 259.5 by rulebook
        return '117.71'

    if s == '220+':  # Assuming 220.5+
        return '100.02+'
    if s == '242+':  # 242.5+ by rulebook
        return '110+'
    if s == '259+':  # 259.5+ by rulebook
        return '117.71+'
    if s == 'SHW':
        return '110+'

    if s == 'UNC':  # Unconfirmed.
        return ''
    # Getting the impression these are free-form string fields...
    if s == 'N/A':
        return ''

    error("Unknown weight class: %s" % (s))


def makeentriescsv():
    csv = Csv()
    csv.append_column('Division')
    csv.append_column('WeightClassKg')
    csv.append_column('Place')
    csv.append_column('Name')
    csv.append_column('Team')
    csv.append_column('BodyweightKg')
    csv.append_column('Best3SquatKg')
    csv.append_column('Best3BenchKg')
    csv.append_column('Best3DeadliftKg')
    csv.append_column('TotalKg')
    csv.append_column('Equipment')
    csv.append_column('Event')
    csv.append_column('Sex')
    return csv


def getdivisionurls(soup):
    urls = []
    for a in soup.find_all("a"):
        try:
            url = a['href']
        except KeyError:
            continue

        if 'passedDivisionID=' not in url:
            continue

        if 'http' not in url:
            url = BASEURL + url.replace('./', '/')
        if url not in urls:
            urls.append(url)

    return urls


def enterdivision(csv, divurl):
    soup = BeautifulSoup(gethtml(divurl), 'html.parser')

    # Determine the division name from the lblHeading.
    heading = soup.find('span', {'id': 'lblHeading'}).text

    # The heading is formatted: "meetname - location - date - division".
    # Sometimes there's additional "Division 1" information after meetname.
    # But the division is always the last one.
    division = heading.split(' - ')[-1].replace('THSWPA', '').strip()
    division = division.replace('  ', ' ')

    # Division contains sex information.
    sex = 'F'

    # Get the table.
    table = soup.find('table', {'id': 'grdIndividualResults'})

    # Skip the first row, since that contains column information, which is
    # always the same.
    trlist = table.find_all('tr')[1:]

    for row in trlist:
        cells = row.find_all('td')

        # The optional 11th or 12th cell specifies whether it was a state qualifying
        # total. Very few meets actually have that.
        assert len(cells) >= 10 and len(cells) <= 12, \
               f"len(cells) assertion failed, len:{len(cells)}"

        # First column is WeightClassLBS, needs conversion to Kg.
        weightclass = weightclass2kg(cells[0].text.strip(), sex)

        # Second column is Place. BO == Bombed-Out, SC == Scratched, DQ == Disqualified.
        # No idea what NA means, but it shows up sometimes.
        place = cells[1].text.replace('BO', 'DQ').replace(
            'SC', 'DQ').replace('NA', 'DQ').strip()

        # Third column is "School Class", which is unused.

        # Fourth column is Name, given as "LASTNAME, FIRSTNAME".
        name = cells[3].text.strip()
        name = name.replace('.', '')

        if ',' in name:
            if name.count(',') == 1:
                [last, first] = name.split(',')
                jr = ''
            elif name.count(',') == 2:  # "Saldivar, Jr, Mario"
                [last, jr, first] = name.split(',')
                jr = ' ' + jr
            else:
                error("Don't know how to split name: %s" % name)

            # Sometimes only the first name is all-caps, for no good reason.
            if len(last) >= 3 and (last.isupper() or last.islower()):
                last = last.title()
            if len(first) >= 3 and (first.isupper() or first.islower()):
                first = first.title()

            name = first + ' ' + last + jr
            name = name.replace('Iii', 'III')
            name = name.replace('  ', ' ').strip()

        if name == name.upper():
            # Will get some things wrong, but not a lot of Scots in Texas.
            name = name.title()

        # Fifth column is Team.
        team = cells[4].text.strip()

        # Sixth column is BodyweightLBS, needs conversion to Kg.
        bw = strtokg(cells[5].text.replace('999', ''))

        # 7-10 columns are Squat,Bench,Deadlift,Total, need conversion to Kg.
        squat = strtokg(cells[6].text)
        bench = strtokg(cells[7].text)
        deadlift = strtokg(cells[8].text)
        total = strtokg(cells[9].text)

        # The federation does not specify canvas-only equipment.
        equipment = 'Unlimited'
        event = 'SBD'

        csv.rows.append([division, weightclass, place, name, team,
                         bw, squat, bench, deadlift, total,
                         equipment, event, sex])


# The CSV isn't quite in our format yet. Some post-processing is required.
def removeemptycolumns(csv):

    def getemptycol(csv):
        for i in range(0, len(csv.fieldnames)):
            hasdata = False
            for row in csv.rows:
                if row[i]:
                    hasdata = True
                    break
            if not hasdata:
                return i
        return -1

    # Remove all empty columns.
    while True:
        i = getemptycol(csv)
        if i == -1:
            break
        else:
            csv.remove_column_by_index(i)


# Some meets DQ people for bombing but still list a total, others allow lifters who bomb
# and event to place.
def fix_dqs(csv):
    totalidx = csv.index('TotalKg')
    placeidx = csv.index('Place')
    eventidx = csv.index('Event')
    bsqidx = csv.index('Best3SquatKg')
    bbpidx = csv.index('Best3BenchKg')
    bdlidx = csv.index('Best3DeadliftKg')

    for row in csv.rows:
        if row[placeidx] == 'DQ':
            row[totalidx] = ''
            event = 'SBD'
        else:
            event = ''
            if row[bsqidx] != '':
                event += 'S'
            if row[bbpidx] != '':
                event += 'B'
            if row[bdlidx] != '':
                event += 'D'
            row[eventidx] = event


# Lifters who have no bodyweight and no total should be marked "NS".
def fix_noshows(csv):
    bwidx = csv.index('BodyweightKg')
    totalidx = csv.index('TotalKg')
    placeidx = csv.index('Place')

    for row in csv.rows:
        if row[bwidx] == "" and row[totalidx] == "":
            row[placeidx] = "NS"


def makemeetcsv(soup):
    heading = soup.find('span', {'id': 'lblHeading'}).text
    heading = heading.replace(',', ' ')
    heading = heading.replace('  ', ' ')

    if heading.count(' - ') == 2:
        [meetname, location, origdate] = [x.strip()
                                          for x in heading.split(' - ')]
    elif heading.count(' - ') == 3:
        # Division is ignored.
        [meetname, division, location, origdate] = [x.strip()
                                                    for x in heading.split(' - ')]
    else:
        error('Unknown header format: "%s"' % heading)

    # Date is given as MM/DD/YYYY
    assert '/' in origdate
    k = origdate.split('/')
    if len(k[0]) == 1:
        k[0] = '0' + k[0]
    if len(k[1]) == 1:
        k[1] = '0' + k[1]
    date = k[2] + '-' + k[0] + '-' + k[1]

    location = location.replace(' TX', '')
    location = location.replace(' Tx', '')
    location = location.replace(' HS', '')
    location = location.replace('High School', '').strip()

    meetname = meetname.replace('#', '')
    meetname = meetname.replace('2019', '')
    meetname = meetname.replace('2018', '')
    meetname = meetname.replace('2017', '')
    meetname = meetname.replace('2016', '')
    meetname = meetname.replace('2015', '')
    meetname = meetname.replace('2014', '').strip()

    location = location.replace('  ', ' ')

    csv = Csv()
    csv.append_column('Federation')
    csv.append_column('Date')
    csv.append_column('MeetCountry')
    csv.append_column('MeetState')
    csv.append_column('MeetTown')
    csv.append_column('MeetName')

    row = ['THSWPA', date, 'USA', 'TX', location, meetname]
    csv.rows.append(row)
    return csv


def getdirname(url):
    return url.split('=')[1]


def main(url):
    # Since the dirname is derived from the URL, we can fail early.
    dirname = getdirname(url)
    if os.path.isdir(dirname):
        error("Directory '%s' already exists." % dirname)

    entriescsv = makeentriescsv()
    soup = BeautifulSoup(gethtml(url), 'html.parser')
    for divurl in getdivisionurls(soup):
        enterdivision(entriescsv, divurl)

    removeemptycolumns(entriescsv)
    entriescsv.append_column('BirthDate')

    fix_dqs(entriescsv)
    fix_noshows(entriescsv)

    meetcsv = makemeetcsv(soup)

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
