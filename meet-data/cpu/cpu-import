#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Adds a single meet from the CPU in the new-style database-generated format.

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

    # The HTML is malformed. We do some spot corrections first.
    html = html.replace("<font size='2'>", "")
    html = html.replace("NAME</td>", "NAME")

    html = html.replace('">\n</div>', '"\\>\n</div>')

    html = html.replace('</table></font>', '</table>')

    soup = BeautifulSoup(html, 'html.parser')
    table = soup.find('table', {'id': 'customers'})

    # Build the meet.csv.
    date = table.find('font', {'size': '4'}).contents[0].string
    assert len(date) == 10

    # The header looks like "2019 Belle River Open, Belle River, ON".
    header = table.find('font', {'size': '5'}).contents[0].string
    header = header.strip().replace('  ', ' ')
    split_header = header.split(',')
    if len(split_header) == 2:
        by_spaces = split_header[-1].split()
        province = by_spaces[-1]
        town = ' '.join(by_spaces[:-1])
        split_header[1] = town
        split_header.append(province)
    assert len(split_header) >= 3

    province = split_header[-1].strip()
    town = split_header[-2].replace('  ', ' ').strip()
    meetname = ', '.join(split_header[0:-2]).replace('  ', ' ').strip()

    # Remove any year information from the start of the MeetName.
    if meetname.startswith('20'):
        meetname = meetname[meetname.index(' ') + 1:]

    meetcsv = Csv()
    meetcsv.append_column('Federation')
    meetcsv.append_column('Date')
    meetcsv.append_column('MeetCountry')
    meetcsv.append_column('MeetState')
    meetcsv.append_column('MeetTown')
    meetcsv.append_column('MeetName')
    meetrow = ['CPU', date, 'Canada', province, town, meetname]
    meetcsv.rows.append(meetrow)

    # Build the entries.csv.
    entriescsv = Csv()
    entriescsv.append_column('Place')
    entriescsv.append_column('Name')
    entriescsv.append_column('BirthDate')
    entriescsv.append_column('Sex')
    entriescsv.append_column('State')
    entriescsv.append_column('Division')
    entriescsv.append_column('Equipment')
    entriescsv.append_column('BodyweightKg')
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

    # The table is stateful and has control rows.
    current_equipment = None
    current_event = None
    current_place = 1

    for row in table.find_all('tr'):
        cells = row.find_all('td')
        num_cells = len(cells)

        if num_cells == 0 or num_cells == 2:
            # This happens with the meet information rows, which are malformed.
            continue

        elif num_cells in [16, 17]:
            # Skip the NAME row.
            if cells[0].text == "NAME":
                continue

            if num_cells == 17:
                name = cells[0].text.strip().replace('  ', ' ')
                state = cells[1].text.strip()
                sex = cells[2].text.strip()
                division = cells[3].text.strip()
                bodyweightkg = canonicalize_number(cells[4].text.strip())
                weightclasskg = cells[5].text.strip()
                squat1kg = canonicalize_number(cells[6].text.strip())
                squat2kg = canonicalize_number(cells[7].text.strip())
                squat3kg = canonicalize_number(cells[8].text.strip())
                bench1kg = canonicalize_number(cells[9].text.strip())
                bench2kg = canonicalize_number(cells[10].text.strip())
                bench3kg = canonicalize_number(cells[11].text.strip())
                deadlift1kg = canonicalize_number(cells[12].text.strip())
                deadlift2kg = canonicalize_number(cells[13].text.strip())
                deadlift3kg = canonicalize_number(cells[14].text.strip())
                totalkg = canonicalize_number(cells[15].text.strip())
                # Ignore column 16: IPF points.
            elif num_cells == 16:
                name = cells[0].text.strip().replace('  ', ' ')
                state = cells[1].text.strip()
                sex = ''
                division = cells[2].text.strip()
                bodyweightkg = canonicalize_number(cells[3].text.strip())
                weightclasskg = cells[4].text.strip()
                squat1kg = canonicalize_number(cells[5].text.strip())
                squat2kg = canonicalize_number(cells[6].text.strip())
                squat3kg = canonicalize_number(cells[7].text.strip())
                bench1kg = canonicalize_number(cells[8].text.strip())
                bench2kg = canonicalize_number(cells[9].text.strip())
                bench3kg = canonicalize_number(cells[10].text.strip())
                deadlift1kg = canonicalize_number(cells[11].text.strip())
                deadlift2kg = canonicalize_number(cells[12].text.strip())
                deadlift3kg = canonicalize_number(cells[13].text.strip())
                totalkg = canonicalize_number(cells[14].text.strip())
                # Ignore column 16: IPF points.

            # Handle control rows.
            if 'EQUIPPED' in cells[0].text:
                # Event.
                if 'POWERLIFTING' in cells[0].text:
                    current_event = 'SBD'
                elif 'BENCHPRESS' in cells[0].text:
                    current_event = 'B'
                else:
                    error("Unknown control row event: %s" % cells[0].text)

                # Equipment.
                if 'UNEQUIPPED' in cells[0].text:
                    current_equipment = 'Raw'
                else:
                    current_equipment = 'Single-ply'

                current_place = 1
                continue

            # Skip any rows with a blank first cell.
            # Blank rows are used to separate weight classes.
            # At least they /look/ blank: they are actually '.....' in white.
            if cells[0].text.strip().replace('.', '') == '':
                current_place = 1
                continue

            # Assert that we've seen a control row.
            assert current_equipment
            assert current_event

            entry = ['' for x in entriescsv.fieldnames]
            entry[entriescsv.index('Name')] = name
            entry[entriescsv.index('State')] = state
            entry[entriescsv.index('Sex')] = sex
            entry[entriescsv.index('Division')] = division
            entry[entriescsv.index('BodyweightKg')] = bodyweightkg
            entry[entriescsv.index('WeightClassKg')] = weightclasskg
            entry[entriescsv.index('Squat1Kg')] = squat1kg
            entry[entriescsv.index('Squat2Kg')] = squat2kg
            entry[entriescsv.index('Squat3Kg')] = squat3kg
            entry[entriescsv.index('Bench1Kg')] = bench1kg
            entry[entriescsv.index('Bench2Kg')] = bench2kg
            entry[entriescsv.index('Bench3Kg')] = bench3kg
            entry[entriescsv.index('Deadlift1Kg')] = deadlift1kg
            entry[entriescsv.index('Deadlift2Kg')] = deadlift2kg
            entry[entriescsv.index('Deadlift3Kg')] = deadlift3kg
            entry[entriescsv.index('TotalKg')] = totalkg
            entry[entriescsv.index('Equipment')] = current_equipment
            entry[entriescsv.index('Event')] = current_event

            # Entries are grouped by Place.
            # DQ entries have '0.0' for TotalKg (canonicalized to the empty string).
            if totalkg == '':
                entry[entriescsv.index('Place')] = 'DQ'
            else:
                entry[entriescsv.index('Place')] = str(current_place)
                current_place += 1

            # Ensure there are no stray commas.
            for i, c in enumerate(entry):
                entry[i] = c.replace(',', ' ')

            entriescsv.rows.append(entry)

        else:
            for i, cell in enumerate(cells):
                print("%d: %s" % (i, cell))
            error("Unexpected row length: %d, debug information above" % num_cells)

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


# Some sex information can be inferred from weightclass information.
def infer_sex_from_weightclass(csv):
    # Some meets unfortunately don't use weight classes,
    # and instead arrange lifters by sex manually.
    if 'WeightClassKg' not in csv.fieldnames:
        return

    sexidx = csv.index('Sex')
    wcidx = csv.index('WeightClassKg')

    # Note that ipf_men doesn't intersect ipf_women! Perfect matching!
    ipf_men = ['53', '59', '66', '74', '83', '93', '105', '120', '120+']
    ipf_women = ['43', '47', '52', '57', '63', '69', '72', '76', '84', '84+']

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


def canonicalize_divisions(csv):
    dividx = csv.index('Division')
    for row in csv.rows:
        if 'Master ' in row[dividx]:
            row[dividx] = row[dividx].replace('Master ', 'Masters ')
        elif row[dividx] == 'Junior':
            row[dividx] = 'Juniors'
        elif row[dividx] == 'Sub-Junior':
            row[dividx] = 'Sub-Juniors'


def canonicalize_state(csv):
    if 'State' not in csv.fieldnames:
        return

    stateidx = csv.index('State')
    for row in csv.rows:
        state = row[stateidx]
        if state == "QU":
            row[stateidx] = "QC"
        elif state == "NF":
            row[stateidx] = "NL"
        elif state == "PEI":
            row[stateidx] = "PE"


def get_dirname(meetcsv):
    date = meetcsv.rows[0][meetcsv.index('Date')]
    assert len(date) == 10
    meetname = meetcsv.rows[0][meetcsv.index('MeetName')]
    return meethash(date, meetname)


def main(url):
    html = gethtml(url)
    (meetcsv, entriescsv) = parsehtml(html)

    remove_empty_lift_columns(entriescsv)
    make_best_columns(entriescsv)
    infer_sex_from_weightclass(entriescsv)
    canonicalize_divisions(entriescsv)
    canonicalize_state(entriescsv)

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
