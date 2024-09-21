#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Adds a single meet from the FQD as if it were from the CPU database.

from bs4 import BeautifulSoup
import errno
from meethash import meethash
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


def parsehtml(html):
    # Decode network bytes as UTF-8.
    html = html.decode('utf-8', errors='replace')

    soup = BeautifulSoup(html, 'html.parser')
    content = soup.find('div', {'class': 'meet'})

    #######################################
    # Create the meet.csv.
    #######################################

    meetname = content.find('h1').text

    # Metadata are within a <strong> tag.
    # The first one that comes up is just the "Navigation".
    strong = content.find_all('strong')[1]

    # The metadata is formatted like "Drummondville, November 16, 2019".
    metadata = strong.text
    assert metadata.count(',') == 2

    # Pull apart the metadata string.
    (town, date_mid, year) = metadata.split(',')
    town = town.strip()
    date_mid = date_mid.strip()
    year = year.strip()
    (month_text, day) = date_mid.split()
    day = day.zfill(2)  # Left-pad with zeros.

    MONTHMAP = {
        'January': '01',
        'February': '02',
        'March': '03',
        'April': '04',
        'May': '05',
        'June': '06',
        'July': '07',
        'August': '08',
        'September': '09',
        'October': '10',
        'November': '11',
        'December': '12'
    }
    month = MONTHMAP[month_text]
    date = "%s-%s-%s" % (year, month, day)

    meetcsv = Csv()
    meetcsv.append_column('Federation')
    meetcsv.append_column('Date')
    meetcsv.append_column('MeetCountry')
    meetcsv.append_column('MeetState')
    meetcsv.append_column('MeetTown')
    meetcsv.append_column('MeetName')
    meetrow = ['CPU', date, 'Canada', 'QC', town, meetname]
    meetcsv.rows.append(meetrow)

    #######################################
    # Create the entries.csv.
    #######################################

    entriescsv = Csv()
    entriescsv.append_column('Sex')
    entriescsv.append_column('Name')
    entriescsv.append_column('BodyweightKg')
    entriescsv.append_column('Equipment')
    entriescsv.append_column('Division')
    entriescsv.append_column('WeightClassKg')
    entriescsv.append_column('Squat1Kg')
    entriescsv.append_column('Squat2Kg')
    entriescsv.append_column('Squat3Kg')
    entriescsv.append_column('Bench1Kg')
    entriescsv.append_column('Bench2Kg')
    entriescsv.append_column('Bench3Kg')
    entriescsv.append_column('Deadlift1Kg')
    entriescsv.append_column('Deadlift2Kg')
    entriescsv.append_column('Deadlift3Kg')
    entriescsv.append_column('TotalKg')
    entriescsv.append_column('Event')
    entriescsv.append_column('Wilks')  # To be deleted.

    # There can be multiple tables with results, and they can be in different formats.
    # Each table that contains results has a first column of 'Sex'.
    for table in content.find_all('table'):

        # Check that the first column is 'Sex' (and therefore a results table).
        if table.find('th').text != 'Sex':
            continue

        # The header contains the Event.
        # Hardcode what all the columns are based on the Event kind.
        header = table.find('caption').text
        if 'Powerlifting' in header:
            event = 'SBD'
            columns = ["Sex", "Name", "BodyweightKg", "Equipment", "Division",
                       "WeightClassKg", "Squat1Kg", "Squat2Kg", "Squat3Kg",
                       "Bench1Kg", "Bench2Kg", "Bench3Kg", "Deadlift1Kg",
                       "Deadlift2Kg", "Deadlift3Kg", "TotalKg", "Wilks"]
        elif 'Bench Press' in header:
            event = 'B'
            columns = ["Sex", "Name", "BodyweightKg", "Equipment", "Division",
                       "WeightClassKg", "Bench1Kg", "Bench2Kg", "Bench3Kg",
                       "TotalKg", "Wilks"]
        else:
            error("Unknown Event kind")

        # Skip the first two rows: they're the header.
        for tr in table.find_all('tr')[2:]:
            # Construct a new row.
            row = ['' for x in range(0, len(entriescsv.fieldnames))]
            row[entriescsv.index('Event')] = event

            for (i, td) in enumerate(tr.find_all('td')):
                colkind = columns[i]
                idx = entriescsv.index(colkind)

                text = td.text.strip()
                text = text.replace('\nNovice', '')
                if text == '-':
                    continue  # Leave missing fields blank (skipped lifts, etc).

                if 'Kg' in colkind:
                    row[idx] = canonicalize_number(text)
                else:
                    row[idx] = text

            entriescsv.rows.append(row)

    return (meetcsv, entriescsv)


def remove_empty_lift_columns(csv):
    def getemptycol(csv):
        for i in range(0, len(csv.fieldnames)):
            if 'Squat' not in csv.fieldnames[i] and 'Bench' not in csv.fieldnames[i] \
                    and 'Deadlift' not in csv.fieldnames[i]:
                continue

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
def make_best_columns(csv):

    def tonumber(s):
        if not s or s == "-":
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
                    row[idxbest] = canonicalize_number(str(best))
        elif numcols == 1:
            # If only one column is filled in, that's the best result column.
            idx1 = csv.index(lift + '1Kg')
            csv.fieldnames[idx1] = 'Best3' + lift + 'Kg'

    makebestcolumn(csv, 'Squat')
    makebestcolumn(csv, 'Bench')
    makebestcolumn(csv, 'Deadlift')


def canonicalize_sex(csv):
    idx = csv.index('Sex')
    SEXMAP = {
        'Women': 'F',
        'Men': 'M'
    }

    for row in csv.rows:
        row[idx] = SEXMAP[row[idx]]


def canonicalize_equipment(csv):
    idx = csv.index('Equipment')
    MAP = {
        'Classic': 'Raw',
        'Equipped': 'Single-ply'
    }

    for row in csv.rows:
        row[idx] = MAP[row[idx]]


def canonicalize_divisions(csv):
    idx = csv.index('Division')
    DIVMAP = {
        'Sub-Junior': 'Sub-Juniors',
        'Sub-junior': 'Sub-Juniors',
        'Junior': 'Juniors',
        'Master I': 'Masters 1',
        'Master II': 'Masters 2',
        'Master III': 'Masters 3',
        'Master IV': 'Masters 4'
    }

    for row in csv.rows:
        if row[idx] in DIVMAP:
            row[idx] = DIVMAP[row[idx]]


def canonicalize_weightclasskg(csv):
    idx = csv.index('WeightClassKg')

    for row in csv.rows:
        w = row[idx]
        w = w.replace('kg', '').replace('-', '')

        if w.startswith('+'):
            w = w[1:] + '+'

        row[idx] = w


def get_dirname(meetcsv):
    date = meetcsv.rows[0][meetcsv.index('Date')]
    assert len(date) == 10
    meetname = meetcsv.rows[0][meetcsv.index('MeetName')]
    return meethash(date, meetname)


def main(url):
    # Request the English site just to make the headings easier for me.
    html = gethtml(url + "?lang=en")
    (meetcsv, entriescsv) = parsehtml(html)

    remove_empty_lift_columns(entriescsv)
    make_best_columns(entriescsv)
    canonicalize_sex(entriescsv)
    canonicalize_divisions(entriescsv)
    canonicalize_equipment(entriescsv)
    canonicalize_weightclasskg(entriescsv)

    entriescsv.remove_column_by_name('Wilks')
    if 'BirthDate' not in entriescsv.fieldnames:
        idx = entriescsv.index('Name') + 1
        entriescsv.insert_column(idx, 'BirthDate')

    # Write out the files.
    dirname = get_dirname(meetcsv)
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
        fd.write(url + '\n')

    print("Imported into %s" % dirname)
    return 0


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s <URL>" % sys.argv[0])
        sys.exit(1)

    main(sys.argv[1])
