#!/usr/bin/env python3
#
# Given a raw CSV dump of the CPU database, standardize the table
# to a form more similar to the OpenPowerlifting format.
#

from oplcsv import Csv
import sys


MonthLookup = {
    'Jan': '01',
    'Feb': '02',
    'Mar': '03',
    'Apr': '04',
    'May': '05',
    'Jun': '06',
    'Jul': '07',
    'Aug': '08',
    'Sep': '09',
    'Oct': '10',
    'Nov': '11',
    'Dec': '12'
}


def standardize_date(csv):
    assert 'Year' in csv.fieldnames
    assert 'Date' in csv.fieldnames
    yearidx = csv.index('Year')
    dateidx = csv.index('Date')

    for row in csv.rows:
        # In a format like "30-Mar-07".
        olddate = row[dateidx]

        # Fix errors like "2--Jun-12".
        olddate = olddate.replace('--', '-')

        # Round unknown dates like "1986 ?" to "1986-01-01".
        if ' ?' in olddate:
            year = row[yearidx]
            assert len(year) == 4
            row[dateidx] = year + '-01-01'
            continue

        # There is one meet entered as "July 25/1981".
        if olddate == 'July 25/1981':
            row[dateidx] = '1981-07-25'
            continue

        d, m, y = olddate.split('-')
        if len(d) == 1:
            d = '0' + d
        if len(m) == 1:
            m = '0' + m

        # Fix "00" entered as day and month.
        if d == '00':
            d = '01'
        if m == '00':
            m = '01'

        # Sometimes dates are entered as '06-19-2016', so swap month and day.
        if not m[0].isalpha():
            d, m = m, d
            newdate = "%s-%s-%s" % (row[yearidx], m, d)
            assert len(newdate) == 10
            row[dateidx] = newdate
            continue

        newdate = '%s-%s-%s' % (row[yearidx], MonthLookup[m], str(d))
        row[dateidx] = newdate

    csv.remove_column_by_index(yearidx)


def standardize_sex(csv):
    assert 'Sex' in csv.fieldnames
    sexidx = csv.index('Sex')

    for row in csv.rows:
        sex = row[sexidx]

        # Some sex information is entered as 'm'.
        if sex != 'M' and sex != 'F':
            assert sex == 'm' or sex == 'f'
            row[sexidx] = sex.upper()


def standardize_equipment(csv):
    assert 'Unequipped' in csv.fieldnames
    eqpidx = csv.index('Unequipped')
    csv.fieldnames[eqpidx] = 'Equipment'

    for row in csv.rows:
        unequipped = row[eqpidx]

        # Exciting error correction.
        if unequipped == "yea":
            unequipped = "yes"
        elif unequipped == "YES":
            unequipped = "yes"

        assert unequipped == '' or unequipped == 'yes'
        if unequipped == 'yes':
            row[eqpidx] = 'Raw'
        else:
            row[eqpidx] = 'Single-ply'


def standardize_event(csv):
    assert '3Lift' in csv.fieldnames
    evtidx = csv.index('3Lift')
    csv.fieldnames[evtidx] = 'Event'

    squatidx = csv.index('Best3SquatKg')
    benchidx = csv.index('Best3BenchKg')
    deadliftidx = csv.index('Best3DeadliftKg')

    for row in csv.rows:
        event = row[evtidx]
        if event == 'All' or event == '3-lift':
            row[evtidx] = 'SBD'
            continue

        if event == 'Two':
            row[evtidx] = 'BD'
            continue

        # Otherwise, we don't really know. Make a best guess.
        assert event == 'Single' or event == ''
        s = ''
        if row[squatidx]:
            s += 'S'
        if row[benchidx]:
            s += 'B'
        if row[deadliftidx]:
            s += 'D'
        row[evtidx] = s


def standardize_weightclass(csv):
    assert 'WeightClassOldKg' in csv.fieldnames
    assert 'WeightClassNewKg' in csv.fieldnames
    oldidx = csv.index('WeightClassOldKg')
    newidx = csv.index('WeightClassNewKg')
    sexidx = csv.index('Sex')
    bodyweightidx = csv.index('BodyweightKg')

    csv.fieldnames[newidx] = 'WeightClassKg'

    for row in csv.rows:
        # Move the old column into the new column.
        if row[oldidx]:
            assert not row[newidx]

            # Sometimes those columns are "SHW".
            if row[oldidx].upper() == 'SHW':
                if row[sexidx] == 'F':
                    row[oldidx] = '90+'
                else:
                    row[oldidx] = '125+'

            row[newidx] = row[oldidx]

        # Make sure that the new column is an actual number.
        if row[newidx]:
            assert float(row[newidx].replace('+', ''))

        # Make sure that the bodyweight is sane-ish.
        if row[bodyweightidx]:
            # Special case, one-time thing.
            if row[bodyweightidx] == 'not avail.':
                row[bodyweightidx] = ''
                continue

            # Some meets have accidentally swapped weight class and bodyweight,
            # but we can leave those alone for the time being.
            # Those can be corrected manually, since they'll trip the error
            # checks.
            assert float(row[bodyweightidx].replace('+', ''))

    csv.remove_column_by_index(oldidx)


def standardize_meetname(csv):
    assert 'MeetName' in csv.fieldnames
    meetnameidx = csv.index('MeetName')

    for row in csv.rows:
        m = row[meetnameidx]
        # Sometimes meet categories are entered as:
        #   Foo Bar - Bench Only
        # Which should not count as a separate meet.
        if ' - ' in m:
            m = m.split(' - ')[0]
        m = m.replace('&', ' and ')
        m = m.replace(';', ' ')
        m = m.replace('  ', ' ').strip()

        row[meetnameidx] = m


def standardize_name(csv):
    assert 'Name' in csv.fieldnames
    nameidx = csv.index('Name')

    for row in csv.rows:
        name = row[nameidx]
        if '(' in name:
            assert name.count('(') == 1
            assert name.count(')') == 1
            name = name[0:name.index('(')] + name[name.index(')') + 1:-1]
            name = name.replace('  ', ' ')

        name = name.replace('Jr.', 'Jr')
        name = name.replace('Sr.', 'Sr')
        row[nameidx] = name.strip()


def standardize_location(csv):
    assert 'Location' in csv.fieldnames
    locationidx = csv.index('Location')
    csv.insert_column(locationidx + 1, 'MeetTown')
    csv.insert_column(locationidx + 1, 'MeetState')
    locationidx = csv.index('Location')
    townidx = csv.index('MeetTown')
    stateidx = csv.index('MeetState')

    for row in csv.rows:
        loc = row[locationidx]
        if not loc:
            continue

        # Handle exceptions first, then main case at end.
        if loc == 'Ontario':
            row[stateidx] == 'ON'
        elif loc == 'Nova Scotia':
            row[stateidx] = 'NS'
        elif loc == 'Richmond BC':
            row[stateidx] = 'BC'
            row[townidx] = 'Richmond'
        elif loc == 'Montreal. QU':
            row[stateidx] = 'QC'
            row[townidx] = 'Montreal'
        elif loc == 'N. Battleford SK':
            row[stateidx] = 'SK'
            row[townidx] = 'N. Battleford'
        elif loc == 'Yarmouth NS':
            row[stateidx] = 'NS'
            row[townidx] = 'Yarmouth'
        elif loc == 'Newfoundland':
            row[stateidx] = 'NL'
        elif loc == 'Not Available':
            pass
        elif loc == 'Quebec':
            row[stateidx] = 'QC'
        elif loc == 'Alberta':
            row[stateidx] = 'AB'
        else:
            # Fix a particular erroneous row.
            if row[csv.index('MeetName')] == 'Mike Laroche Memorial Open':
                row[csv.index('MeetName')] = 'Mike LaRoche Memorial Open'
                loc = 'Halifax; NS'

            # Work around typos.
            if loc == "Toronto. ON":
                loc = "Toronto; ON"
            if loc == "Summerside PE":
                loc = "Summerside; PE"

            assert ';' in loc

            # If there are more than two ';', it looks like:
            # "Killeen; Texas; USA". So just grab the first 2.
            town, state = loc.split(';')[0:2]

            # Sometimes 'QU' is harde-coded.
            if state.strip() == 'QU':
                state = 'QC'

            row[stateidx] = state.strip()
            row[townidx] = town.strip()

    csv.remove_column_by_index(locationidx)


def remove_international_meets(csv):
    assert 'MeetName' in csv.fieldnames
    assert 'State' in csv.fieldnames  # Lifter state. "CAN" if foreign.
    meetnameidx = csv.index('MeetName')
    stateidx = csv.index('State')

    def helper(row):
        m = row[meetnameidx]
        # Catch obvious international meets.
        if 'IPF' in m or 'NAPF' in m or 'The Arnold' in m or 'Commonwealth' in m:
            return False
        if 'World Games' in m:
            return False

        # Catch misspellings.
        if "!PF" in m:
            return False

        # Signals a foreign competition.
        if row[stateidx] == 'CAN':
            return False
        return True

    csv.rows = list(filter(helper, csv.rows))


def standardize(csv):
    remove_international_meets(csv)
    csv.remove_column_by_name('Wilks')
    standardize_date(csv)
    standardize_sex(csv)
    standardize_equipment(csv)
    standardize_event(csv)
    standardize_weightclass(csv)
    standardize_name(csv)
    standardize_meetname(csv)
    standardize_location(csv)

    # Sort the database stably.
    # Order isn't too important, as long as it doesn't jump around a lot.
    dateidx = csv.index('Date')
    meetnameidx = csv.index('MeetName')
    eventidx = csv.index('Event')
    equipmentidx = csv.index('Equipment')
    divisionidx = csv.index('Division')
    weightclassidx = csv.index('WeightClassKg')
    nameidx = csv.index('Name')

    def sorter(r):
        a = r[dateidx]
        b = r[meetnameidx]
        c = r[eventidx]
        d = r[equipmentidx]
        e = r[divisionidx]
        f = r[weightclassidx]
        g = r[nameidx]
        return (a, b, c, d, e, f, g)

    csv.rows = sorted(csv.rows, key=lambda r: sorter(r), reverse=True)


def main(filename):
    csv = Csv(filename)
    standardize(csv)

    csv.write(sys.stdout)


if __name__ == '__main__':
    main(sys.argv[1])
