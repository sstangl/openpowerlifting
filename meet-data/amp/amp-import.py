#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# USAPL has unfortunately stopped posting individual meet result spreadsheets,
# and now uploads everything to this usapl.liftingdatabase.com service.
# This script taken a liftingdatabase.com URL and converts the results to
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


def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read()


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


# Changes a number like '60.0' to '60'.
def canonicalize_number(s):
    if '.' in s:
        s = s.rstrip('0')
        s = s.rstrip('.')
    if s == '0':
        return ''
    return s


# Copied from uspa-fetch. If we need this often, maybe share it?
states = {
    'Alaska': 'AK',
    'Alabama': 'AL',
    'Arkansas': 'AR',
    'American Samoa': 'AS',
    'Arizona': 'AZ',
    'California': 'CA',
    'Colorado': 'CO',
    'Connecticut': 'CT',
    'District of Columbia': 'DC',
    'Delaware': 'DE',
    'Florida': 'FL',
    'Georgia': 'GA',
    'Guam': 'GU',
    'Hawaii': 'HI',
    'Iowa': 'IA',
    'Idaho': 'ID',
    'Illinois': 'IL',
    'Indiana': 'IN',
    'Kansas': 'KS',
    'Kentucky': 'KY',
    'Louisiana': 'LA',
    'Massachusetts': 'MA',
    'Massachussetts': 'MA',  # Work around spelling errors.
    'Maryland': 'MD',
    'Maine': 'ME',
    'Michigan': 'MI',
    'Minnesota': 'MN',
    'Missouri': 'MO',
    'Northern Mariana Islands': 'MP',
    'Mississippi': 'MS',
    'Montana': 'MT',
    'National': 'NA',
    'North Carolina': 'NC',
    'North Dakota': 'ND',
    'Nebraska': 'NE',
    'New Hampshire': 'NH',
    'New Jersey': 'NJ',
    'New Mexico': 'NM',
    'Nevada': 'NV',
    'New York': 'NY',
    'Ohio': 'OH',
    'Oklahoma': 'OK',
    'Oregon': 'OR',
    'Pennsylvania': 'PA',
    'Puerto Rico': 'PR',
    'Rhode Island': 'RI',
    'South Carolina': 'SC',
    'South Dakota': 'SD',
    'Tennessee': 'TN',
    'Texas': 'TX',
    'Utah': 'UT',
    'Virginia': 'VA',
    'Virgin Islands': 'VI',
    'Vermont': 'VT',
    'Washington': 'WA',
    'Washington DC': 'DC',
    'Wisconsin': 'WI',
    'West Virginia': 'WV',
    'Wyoming': 'WY',

    'Nationals': '',  # No information provided.
    'Regionals': '',  # No information provided.
    'NATIONAL OFFICE': '',
    'Australia': 'AUS',
}


# Returns a (division, equipment) tuple of strings.
def parsedivision(div):
    if 'Raw' in div or div.startswith('R-') or 'MR-' in div or 'FR-' in div:
        eq = 'Raw'
    else:
        eq = 'Single-ply'

    # If only USAPL would report Age.
    # Sometimes the divisions are just "M1", when they really mean
    # "both M1a and M1b", and it's impossible to distinguish the two.
    s = div
    s = s.replace('Raw ', 'R-')  # As in "Raw Master 1a"
    s = s.replace('Teen and Junior', 'TJ')
    s = s.replace('Master ', 'M')  # As in "Master 1a"
    s = s.replace('Master', 'M')  # As in "Master1a"
    s = s.replace('Teen', 'T')  # As in "Teen 1"
    s = s.replace('High School', 'HS')
    s = s.replace('Collegiate', 'C')
    s = s.replace('Special Olympian', 'SO')
    s = s.replace('Sub Junior', 'Sj')
    s = s.replace('Open', 'O')
    s = s.replace('Junior', 'Jr')
    s = s.replace('Youth', 'Y')
    s = s.replace('Police and Fire', 'PF')
    s = s.replace('Military Open', 'MO')
    s = s.replace('Military', 'ML')
    s = s.replace('Female-', 'F-')
    s = s.replace('Female - ', 'F-')
    s = s.replace('Male-', 'M-')
    s = s.replace('Male - ', 'M-')
    s = s.replace('GuestLifter', 'G')
    s = s.replace('Varsity', 'V')

    # Fix Some common mistakes that crop up.
    s = s.replace('SJr', 'Sj')
    s = s.replace('-R', 'R')
    s = s.replace('-W', 'W')
    s = s.replace(' ', '')

    return (s, eq)


def makeentriescsv(soup):
    # The page contains everything we'd need to know, except for Sex. Sigh.
    # That always has to be done manually, since men single-event can come
    # after women, although men usually are placed higher.
    csv = Csv()
    csv.append_column('Place')
    csv.append_column('Name')
    csv.append_column('Sex')
    csv.append_column('Event')
    csv.append_column('Division')
    csv.append_column('WeightClassKg')
    csv.append_column('Equipment')
    csv.append_column('BirthYear')
    csv.append_column('Team')
    csv.append_column('State')
    csv.append_column('BodyweightKg')
    csv.append_column('Squat1Kg')
    csv.append_column('Squat2Kg')
    csv.append_column('Squat3Kg')
    csv.append_column('Bench1Kg')
    csv.append_column('Bench2Kg')
    csv.append_column('Bench3Kg')
    csv.append_column('Deadlift1Kg')
    csv.append_column('Deadlift2Kg')
    csv.append_column('Deadlift3Kg')
    csv.append_column('TotalKg')

    table = soup.find("table", {"id": "competition_view_results"})
    table = table.find("tbody")

    state_sex = ''
    state_event = 'SBD'
    state_division = None
    state_equipment = None

    for row in table.find_all('tr'):
        k = len(row.find_all('td'))
        if k == 0:
            state_sex = ''
            # This is a control row, changing some state.
            s = row.find('th').text.strip()
            if s == 'Powerlifting':
                state_event = 'SBD'
            elif s == 'PL':
                state_event = 'SBD'
            elif s == 'Squat':
                state_event = 'S'
            elif s == 'Bench press':
                state_event = 'B'
            elif s == 'Deadlift':
                state_event = 'D'
            elif s == 'DL':
                state_event = 'D'
            elif s == 'Push Pull':
                state_event = 'BD'
            elif s == 'BP':
                state_event = 'B'
            elif s == 'DD':
                state_event = 'DRUGTEST'
            else:
                (state_division, state_equipment) = parsedivision(s)
                if 'Female' in s:
                    state_sex = 'F'
                elif 'Male' in s:
                    state_sex = 'M'

        elif k == 20:
            # This is a results row.
            assert state_event is not None
            assert state_division is not None
            assert state_equipment is not None

            cells = row.find_all('td')
            weightclasskg = cells[0].text.replace('-', '').strip()
            place = cells[1].text.replace('.', '').strip()
            name = cells[2].text.replace('Jr.', 'Jr').replace(
                'Sr.', 'Sr').replace('  ', ' ').strip()
            birthyear = cells[3].text.strip()
            team = cells[4].text.strip()
            state = cells[5].text.strip()
            # lot = cells[6].text.strip() # Not used.
            bodyweightkg = canonicalize_number(cells[6].text.strip())

            squat1kg = canonicalize_number(cells[7].text.strip())
            squat2kg = canonicalize_number(cells[8].text.strip())
            squat3kg = canonicalize_number(cells[9].text.strip())
            bench1kg = canonicalize_number(cells[10].text.strip())
            bench2kg = canonicalize_number(cells[11].text.strip())
            bench3kg = canonicalize_number(cells[12].text.strip())
            deadlift1kg = canonicalize_number(cells[13].text.strip())
            deadlift2kg = canonicalize_number(cells[14].text.strip())
            deadlift3kg = canonicalize_number(cells[15].text.strip())
            totalkg = canonicalize_number(cells[16].text.strip())
            # wilks = cells[18].text.strip() # Not used. Always recalculated.
            # drugtested = cells[19].text.strip() # Not used.
            # unknown = cells[20].text.strip() # Not used.

            row = ['' for x in csv.fieldnames]
            row[csv.index('WeightClassKg')] = weightclasskg
            row[csv.index('Place')] = place
            row[csv.index('Name')] = name
            row[csv.index('Sex')] = state_sex
            row[csv.index('BirthYear')] = birthyear
            row[csv.index('Team')] = team
            row[csv.index('State')] = state
            row[csv.index('BodyweightKg')] = bodyweightkg
            row[csv.index('Squat1Kg')] = squat1kg
            row[csv.index('Squat2Kg')] = squat2kg
            row[csv.index('Squat3Kg')] = squat3kg
            row[csv.index('Bench1Kg')] = bench1kg
            row[csv.index('Bench2Kg')] = bench2kg
            row[csv.index('Bench3Kg')] = bench3kg
            row[csv.index('Deadlift1Kg')] = deadlift1kg
            row[csv.index('Deadlift2Kg')] = deadlift2kg
            row[csv.index('Deadlift3Kg')] = deadlift3kg
            row[csv.index('TotalKg')] = totalkg
            row[csv.index('Division')] = state_division
            row[csv.index('Equipment')] = state_equipment
            row[csv.index('Event')] = state_event

            for i, c in enumerate(row):
                row[i] = c.replace(',', ' ')
            csv.rows.append(row)

        else:
            cells = row.find_all('td')
            for i, cell in enumerate(cells):
                print("%d: %s" % (i, cell))
            error("Unexpected row length: %s, debug information above" % str(k))

    return csv


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


# Assumes that empty columns were removed.
def makebestcolumns(csv):

    def tonumber(s):
        if not s:
            return 0
        return float(s)

    def makebestcolumn(csv, lift):
        numcols = len(list(filter(lambda x: lift in x, csv.fieldnames)))
        if numcols == 3:
            csv.insert_column(csv.index(lift + '3Kg') +
                              1, 'Best3' + lift + 'Kg')

            idxbest = csv.index('Best3' + lift + 'Kg')
            idx1 = csv.index(lift + '1Kg')
            idx2 = csv.index(lift + '2Kg')
            idx3 = csv.index(lift + '3Kg')

            for row in csv.rows:
                best = max(tonumber(row[idx1]), tonumber(
                    row[idx2]), tonumber(row[idx3]))
                if best > 0:
                    row[idxbest] = str(best)
        elif numcols == 1:
            # If only one column is filled in, that's the best result column.
            idx1 = csv.index(lift + '1Kg')
            csv.fieldnames[idx1] = 'Best3' + lift + 'Kg'

    makebestcolumn(csv, 'Squat')
    makebestcolumn(csv, 'Bench')
    makebestcolumn(csv, 'Deadlift')


# The website incorrectly marks disqualified lifters as placing.
# Normally it just doesn't include disqualified lifters... but sometimes
# it does.
def markdqs(csv):
    totalidx = csv.index('TotalKg')
    placeidx = csv.index('Place')
    for row in csv.rows:
        if row[totalidx] == '0':
            row[totalidx] = ''
            row[placeidx] = 'DQ'

        # It also sometimes uses dashes to mark DQ.
        if row[placeidx] == '-':
            # Don't change the total: sometimes this is an error.
            row[placeidx] = 'DQ'


# Some sex information can be inferred from weightclass information.
def infersex(csv):
    # Some meets unfortunately don't use weight classes,
    # and instead arrange lifters by sex manually.
    if 'WeightClassKg' not in csv.fieldnames:
        return

    sexidx = csv.index('Sex')
    wcidx = csv.index('WeightClassKg')

    # Note that ipf_men doesn't intersect ipf_women! Perfect matching!
    ipf_men = ['53', '59', '66', '74', '83', '93', '105', '120', '120+']
    ipf_women = ['43', '47', '52', '57', '63', '72', '84', '84+']

    # These unfortunately do intersect.
    old_men = ['56', '60', '67.5', '75', '82.5',
               '90', '100', '110', '125', '125+']
    old_women = ['48', '52', '56', '60', '67.5', '75', '82.5', '90', '90+']

    for row in csv.rows:
        if row[sexidx]:
            continue

        wc = row[wcidx]
        if not wc:
            continue

        if wc in ipf_men:
            row[sexidx] = 'M'
        elif wc in ipf_women:
            row[sexidx] = 'F'
        elif wc in old_men and wc not in old_women:
            row[sexidx] = 'M'
        elif wc in old_women and wc not in old_men:
            row[sexidx] = 'F'


def fixupdivisions(csv):
    dividx = csv.index("Division")
    sexidx = csv.index("Sex")

    # Add sex information to the division.
    for row in csv.rows:
        div = row[dividx]
        if not div.startswith('M') and not div.startswith('F'):
            div = row[sexidx] + "-" + div
            div = div.replace('-R', 'R')
            row[dividx] = div


def inferequipment(csv):
    dividx = csv.index("Division")
    eqpidx = csv.index("Equipment")

    for row in csv.rows:
        if 'R-' in row[dividx]:
            row[eqpidx] = 'Raw'


# If a lifter has an event of "DRUGTEST", we need to clean up their row.
# The event type must be inferred from the lifts they received
# which have all been marked as negative), and their Place must be marked "DD".
def markdrugtest(csv):
    placeidx = csv.index('Place')
    eventidx = csv.index('Event')

    for row in csv.rows:
        if row[eventidx] == 'DRUGTEST':
            row[placeidx] = 'DD'
            row[eventidx] = 'SBD'  # Good enough, who cares?


def makemeetcsv(soup):
    content = soup.find('div', {'id': 'content'})

    meetname = content.find('h3').text

    # Remove federation information from the meet name.
    meetname = meetname.replace('USAPL', '')
    meetname = meetname.replace('Powerlifting America', '')
    meetname = meetname.replace('2022', '')
    meetname = meetname.strip()

    table = content.find('table')

    # Date is given as MM/DD/YYYY
    origdate = table.find_all('tr')[0].find('td').text.strip()
    if ' - ' in origdate:
        origdate = origdate.split(' - ')[0]
    k = origdate.split('/')
    date = k[2] + '-' + k[0] + '-' + k[1]

    # State is given in long form.
    state = ''
    trs = table.find_all('tr')
    if len(trs) >= 3:
        state = table.find_all('tr')[2].find('td').text.strip()
        if state not in states:
            error("State not in the abbreviation lookup table: %s" % state)
        state = states[state]

    csv = Csv()
    csv.append_column('Federation')
    csv.append_column('Date')
    csv.append_column('MeetCountry')
    csv.append_column('MeetState')
    csv.append_column('MeetTown')
    csv.append_column('MeetName')

    row = ['AMP', date, 'USA', state, '', meetname]
    csv.rows.append(row)
    return csv


def getdirname(soup):
    content = soup.find('div', {'id': 'content'})
    table = content.find('table')
    sanction = table.find_all('tr')[1].find('td').text.strip()

    if sanction and sanction.count('-') == 2:
        return sanction

    # If there's no sanction number, just use the date.
    origdate = table.find_all('tr')[0].find('td').text.strip()
    if ' - ' in origdate:
        origdate = origdate.split(' - ')[0]
    k = origdate.split('/')
    date = k[2] + '-' + k[0] + '-' + k[1]
    return date


def main(url, importdir):
    html = gethtml(url)

    soup = BeautifulSoup(html, 'lxml')

    entriescsv = makeentriescsv(soup)
    removeemptycolumns(entriescsv)
    makebestcolumns(entriescsv)
    markdqs(entriescsv)
    infersex(entriescsv)
    fixupdivisions(entriescsv)
    inferequipment(entriescsv)
    markdrugtest(entriescsv)

    # Since USAPL only provides BirthYear info, and we keep having to add
    # Age and BirthDate columns by lifter request after importation,
    # just add blank column pre-emptively to avoid large file rewrites.
    if 'Age' not in entriescsv.fieldnames:
        entriescsv.append_column('Age')
    if 'BirthDate' not in entriescsv.fieldnames:
        entriescsv.append_column('BirthDate')

    meetcsv = makemeetcsv(soup)
    dirname = getdirname(soup)
    if importdir is not None:
        dirname = importdir

    try:
        os.makedirs(dirname)
    except OSError as exception:
        if exception.errno != errno.EEXIST:
            raise
        else:
            error("Directory '%s' already exists." % dirname)

    with open(dirname + os.sep + 'entries.csv', 'w', newline='\n') as fd:
        entriescsv.write(fd)
    with open(dirname + os.sep + 'meet.csv', 'w', newline='\n') as fd:
        meetcsv.write(fd)
    with open(dirname + os.sep + 'URL', 'w', newline='\n') as fd:
        fd.write(url + "\n")

    print("Imported into %s." % dirname)


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: %s url <importdir>" % sys.argv[0])

    url = sys.argv[1]
    importdir = sys.argv[2] if len(sys.argv) > 2 else None
    main(url, importdir)
