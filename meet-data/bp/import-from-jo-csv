#!/usr/bin/env python3
#
# Converts from Jo's spreadsheet format to our internal format.
# Requires that columns in the source spreadsheet match our internal names.
#
# Also requires that the "Male" and "Female" sheets be merged into one CSV.
#


import os
import sys

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def process_row(csv, dirs, row):
    (meetcsv, entriescsv, url, subfed) = dirs[row[csv.index('Competition')]]

    if not url and row[csv.index('URL')]:
        url = row[csv.index('URL')]

    if not subfed and row[csv.index('Sub-Fed')]:
        subfed = row[csv.index('Sub-Fed')]

    if len(meetcsv.rows) == 0:
        (month, day, year) = row[csv.index('Date')].split('/')
        meetcsv.rows = [[
            "BP",
            "%s-%s-%s" % (year, month.zfill(2), day.zfill(2)),
            "Britain",
            "",
            "",
            row[csv.index('Competition')]
        ]]

    newrow = [
        row[csv.index('Name')],
        row[csv.index('Sex')],
        row[csv.index('Event')],
        row[csv.index('Equipment')],
        row[csv.index('Division')],
        row[csv.index('BodyweightKg')],
        row[csv.index('WeightClassKg')],
        row[csv.index('Squat1Kg')],
        row[csv.index('Squat2Kg')],
        row[csv.index('Squat3Kg')],
        row[csv.index('Best3SquatKg')],
        row[csv.index('Bench1Kg')],
        row[csv.index('Bench2Kg')],
        row[csv.index('Bench3Kg')],
        row[csv.index('Best3BenchKg')],
        row[csv.index('Deadlift1Kg')],
        row[csv.index('Deadlift2Kg')],
        row[csv.index('Deadlift3Kg')],
        row[csv.index('Best3DeadliftKg')],
        row[csv.index('TotalKg')]
    ]

    # Remove some common markings for "nothing in this column".
    for i in range(len(newrow)):
        if newrow[i] == 'x' or newrow[i] == 'X' or newrow[i] == '0':
            newrow[i] = ''

    entriescsv.rows.append(newrow)
    dirs[row[csv.index('Competition')]] = (meetcsv, entriescsv, url, subfed)


def main(filename):
    csv = Csv(filename)

    meetnames = set()
    for row in csv.rows:
        meetnames.add(row[csv.index('Competition')])

    dirs = dict()
    for meetname in meetnames:
        meetcsv = Csv()
        meetcsv.fieldnames = ["Federation", "Date",
                              "MeetCountry", "MeetState", "MeetTown", "MeetName"]

        entriescsv = Csv()
        entriescsv.fieldnames = ["Name", "Sex", "Event", "Equipment", "Division",
                                 "BodyweightKg", "WeightClassKg", "Squat1Kg",
                                 "Squat2Kg", "Squat3Kg", "Best3SquatKg", "Bench1Kg",
                                 "Bench2Kg", "Bench3Kg", "Best3BenchKg", "Deadlift1Kg",
                                 "Deadlift2Kg", "Deadlift3Kg", "Best3DeadliftKg",
                                 "TotalKg"]

        url = None
        subfed = None

        dirs[meetname] = (meetcsv, entriescsv, url, subfed)

    for row in csv.rows:
        process_row(csv, dirs, row)

    num = 0
    for meetname in sorted(list(meetnames)):
        num += 1
        dirname = '17' + str(num).zfill(2)

        (meetcsv, entriescsv, url, subfed) = dirs[meetname]

        # These aren't great names, but at least they'll be unique.
        if subfed:
            dirname = "%s-%s" % (subfed, dirname)

        os.mkdir(dirname)

        with open(dirname + '/meet.csv', 'w') as fd:
            meetcsv.write(fd)
        with open(dirname + '/entries.csv', 'w') as fd:
            entriescsv.write(fd)
        if url:
            with open(dirname + '/URL', 'w') as fd:
                fd.write(url + "\n")


if __name__ == "__main__":
    main(sys.argv[1])
