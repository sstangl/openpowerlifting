#!/usr/bin/env python3
#
# Processes a database dump from 100% RAW (as supplied by Ed Kutin), and automatically
# files meets, with many errors and false positives.
#
# The major errors here are that the following information is missing:
#  1. Whether a lift was a best-of-3 or best-of-4.
#  2. Event.
#
# For a sample database dump file, use the attachment on GitLab issue #9172.
# But use LibreOffice to process it into a CSV file, not XLS.

import sys
import os

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv

FEDDIR = os.path.dirname(os.path.realpath(__file__))
RAWUKRDIR = os.path.join(os.path.dirname(FEDDIR), "raw-ukr")


def make_iso8601_dates(csv):
    """Converts dates from mm/dd/yyyy to yyyy-mm-dd"""
    idx = csv.index('Date')
    for row in csv.rows:
        date = row[idx]
        (m, d, yyyy) = date.split('/')
        mm = m.zfill(2)
        dd = d.zfill(2)
        date = "%s-%s-%s" % (yyyy, mm, dd)
        row[idx] = date


def fold_entries(csv):
    """The export format reports one lift per line, with the total on another line.
       Entries for the same lifter are next to each other. So this pass runs over each
       entry, folding it into the previous entry if it's for the same lifter."""

    def signature(row):
        """Hashing helper. Entries with the same signature get folded."""
        lastname = row[csv.index('LastName')]
        firstname = row[csv.index('FirstName')]
        division = row[csv.index('Division')]
        date = row[csv.index('Date')]
        contest = row[csv.index('Contest')]
        return (lastname, firstname, division, date, contest)

    csv.ensure_columns(["Best3SquatKg", "Best3BenchKg", "Best3DeadliftKg", "TotalKg"])
    liftmap = {
        "Squat": "Best3SquatKg",
        "Bench": "Best3BenchKg",
        "Deadlift": "Best3DeadliftKg",
        "Total": "TotalKg"
    }

    new_rows = []  # Gather new rows to swap them in at the end.
    current = None  # The current row undergoing stateful mutation.
    current_sig = None  # Signature of the current row.

    for row in csv.rows:
        # Check the signature. If new, finalize the current entry and promote a new one.
        sig = signature(row)
        if sig != current_sig:
            if current is not None:
                new_rows.append(current)
            current = row
            current_sig = sig

        # Get the target column for this lift type.
        lift = row[csv.index('Lift')]
        column = liftmap[lift]

        # If the target column already has data, take the maximum.
        # This seems to happen for deadlifts, where the ".5" in "272.5" gets truncated.
        # We see near-duplicate entries: one with "272", one with "272.5".
        if current[csv.index(column)] == "":
            current[csv.index(column)] = row[csv.index('Amount')]
        else:
            curr_amount = float(current[csv.index(column)])
            this_amount = float(row[csv.index('Amount')])
            if this_amount > curr_amount:
                current[csv.index(column)] = row[csv.index('Amount')]

    # Finalize and swap-in the new rows.
    if current is not None:
        new_rows.append(current)
    csv.rows = new_rows

    # The "Lift" and "Amount" columns may now be removed.
    csv.remove_column_by_name('Lift')
    csv.remove_column_by_name('Amount')


def combine_names(csv):
    """Combines the LastName and FirstName columns into our Name column"""
    lastidx = csv.index('LastName')
    firstidx = csv.index('FirstName')

    for row in csv.rows:
        last = row[lastidx]
        first = row[firstidx]

        # Remove points, which are used sometimes to represent absence.
        if last == ".":
            last = ""
        if first == ".":
            first = ""

        # Store it in the 'LastName' column for now.
        name = "%s %s" % (first, last)
        name = name.strip()
        row[lastidx] = name

    # Full names are now in the LastName column.
    csv.remove_column_by_index(firstidx)
    csv.fieldnames[csv.index('LastName')] = 'Name'


def rewrite_sex(csv):
    """Translates the Gender column into our Sex column"""
    sexmap = {
        "Male": "M",
        "Female": "F"
    }
    genderidx = csv.index('Gender')

    for row in csv.rows:
        row[genderidx] = sexmap[row[genderidx]]
    csv.fieldnames[genderidx] = 'Sex'


def fill_missing_totals(csv):
    """Assumes that every lifter in the DB was non-DQ, and fills in TotalKg."""
    def as_float(s):
        if s == "":
            return 0.0
        return float(s)

    def to_at_most_one_decimal_place(f64):
        s = "{:.1f}".format(f64)
        s = s.replace('.0', '')
        return s

    sqidx = csv.index('Best3SquatKg')  # Note: all these are best-of-4.
    bpidx = csv.index('Best3BenchKg')
    dlidx = csv.index('Best3DeadliftKg')
    totalidx = csv.index('TotalKg')

    for row in csv.rows:
        if row[totalidx] != "":
            continue

        sq = as_float(row[sqidx])
        bp = as_float(row[bpidx])
        dl = as_float(row[dlidx])
        assert sq >= 0.0
        assert bp >= 0.0
        assert dl >= 0.0

        total = to_at_most_one_decimal_place(sq + bp + dl)
        row[totalidx] = total


def add_mandatory_columns(csv):
    """Adds BirthDate, Equipment, and Event columns."""

    # Insert the BirthDate column after the Name column.
    csv.insert_column(csv.index('Name') + 1, 'BirthDate')

    # Put the Event and Equipment columns at the end.
    csv.append_columns(["Event", "Equipment"])

    evidx = csv.index('Event')
    eqidx = csv.index('Equipment')
    sqidx = csv.index('Best3SquatKg')
    bpidx = csv.index('Best3BenchKg')
    dlidx = csv.index('Best3DeadliftKg')

    for row in csv.rows:
        row[eqidx] = "Raw"

        event = ''
        if row[sqidx] != "":
            event += "S"
        if row[bpidx] != "":
            event += "B"
        if row[dlidx] != "":
            event += "D"
        row[evidx] = event


def standardize_divisions(csv):
    """Ensures that the Division column will pass the checker."""
    divmap = {
        "11 and under": "Youth 11 & Under",
        "12-13": "Teen 12-13",
        "14-15": "Teen 14-15",
        "16-17": "Teen 16-17",
        "18-19": "Teen 18-19",
        "20-24": "Juniors 20-24",
        "Open": "Open",
        "35-39": "Submasters 35-39",
        "40-44": "Masters 40-44",
        "45-49": "Masters 45-49",
        "50-54": "Masters 50-54",
        "55-59": "Masters 55-59",
        "60-64": "Masters 60-64",
        "65-69": "Masters 65-69",
        "70-74": "Masters 70-74",
        "75-79": "Masters 75-79",
        "POST90": "Masters 90+",
        "PFM": "Law/Fire/Military",
        "Special": "Special Olympics",
        "HND": "Handicapped",
    }

    dividx = csv.index('Division')
    for row in csv.rows:
        row[dividx] = divmap[row[dividx]]


def build_lifterdateset():
    """Looks through the 100% RAW meet folder, building a (lifter_name, date) set.
       This set is then used to filter out meets that have already been entered.
       We don't use the MeetName, because it's unreliable."""
    lifterdateset = set()

    # Also process RAW-UKR data, since the DB includes foreign results too.
    for toplevel in [FEDDIR, RAWUKRDIR]:
        for dirname, subdirs, files in os.walk(toplevel):
            if 'entries.csv' in files and 'meet.csv' in files:
                meetcsv = oplcsv.Csv(dirname + os.sep + 'meet.csv')
                entriescsv = oplcsv.Csv(dirname + os.sep + 'entries.csv')

                date = meetcsv.rows[0][meetcsv.index('Date')]
                for row in entriescsv.rows:
                    name = row[entriescsv.index('Name')]
                    lifterdateset.add((name, date))

    return lifterdateset


def split_by_meet(csv):
    """Splits the export CSV into per-meet CSVs."""
    contestidx = csv.index('MeetName')
    dateidx = csv.index('Date')

    csv_acc = []  # List of CSV files after splitting.

    # Signature information for matching contests.
    # Using Year instead of Date, since it seems to be EntryDate.
    current_contest = csv.rows[0][contestidx]
    current_year = csv.rows[0][dateidx].split('-')[0]

    # State tracking.
    current = oplcsv.Csv()
    current.fieldnames = csv.fieldnames[:]

    for row in csv.rows:
        contest = row[contestidx]
        year = row[dateidx].split('-')[0]

        # If this is a new meet, finalize the old one.
        if contest != current_contest or year != current_year:
            csv_acc.append(current)
            current_contest = contest
            current_year = year
            current = oplcsv.Csv()
            current.fieldnames = csv.fieldnames[:]

        # Clone the row into the new tracker.
        current.rows.append(row[:])

    csv_acc.append(current)
    return csv_acc


def retain_unentered(csvs, lifterdateset):
    """Returns a list of Csvs that do not intersect the lifterdateset."""
    acc = []

    for csv in csvs:
        nameidx = csv.index('Name')
        dateidx = csv.index('Date')
        intersects = False

        for row in csv.rows:
            entry = (row[nameidx], row[dateidx])
            if entry in lifterdateset:
                intersects = True
                break

        if not intersects:
            acc.append(csv)

    return acc


def write_entries_meet_csvs(csv, unique_id, outdir):
    """Splits a CSV containing a single meet's data into {entries,meet}.csv."""
    meetdir = outdir + os.sep + str(unique_id)
    os.mkdir(meetdir)  # Error if the directory already exists.

    entriespath = meetdir + os.sep + "entries.csv"
    meetpath = meetdir + os.sep + "meet.csv"

    # First, write the meet.csv.
    with open(meetpath, "w") as fd:
        row = csv.rows[0]
        date = row[csv.index('Date')].strip()
        town = row[csv.index('MeetTown')].strip()
        country = row[csv.index('MeetCountry')].strip()
        state = row[csv.index('MeetState')].strip()
        name = row[csv.index('MeetName')].strip()

        name = name.replace("2010", "").strip()
        name = name.replace("2011", "").strip()
        name = name.replace("2012", "").strip()
        name = name.replace("2013", "").strip()
        name = name.replace("2014", "").strip()
        name = name.replace("2015", "").strip()
        name = name.replace("2016", "").strip()
        name = name.replace("2017", "").strip()
        name = name.replace("2018", "").strip()
        name = name.replace("2019", "").strip()
        name = name.replace("2020", "").strip()
        name = name.replace("2021", "").strip()
        name = name.replace("2022", "").strip()
        name = name.replace("2023", "").strip()
        name = name.replace("2024", "").strip()
        name = name.replace("  ", " ")

        print("Federation,Date,MeetCountry,MeetState,MeetTown,MeetName", file=fd)
        print(f"RAW,{date},{country},{state},{town},{name}", file=fd)

    # Then, write the entries.csv.
    # We do this by chopping out the meet.csv columns and saving the rest.
    csv.remove_column_by_name("Date")
    csv.remove_column_by_name("MeetCountry")
    csv.remove_column_by_name("MeetState")
    csv.remove_column_by_name("MeetTown")
    csv.remove_column_by_name("MeetName")
    csv.write_filename(entriespath)


def main(csv_filename, outdir):
    print("loading CSV", end="...")
    csv = oplcsv.Csv(csv_filename)
    print("%u rows" % len(csv.rows))

    print("converting dates", end="...")
    make_iso8601_dates(csv)
    print("ok")

    print("folding entries", end="...")
    fold_entries(csv)
    print("%u rows" % len(csv.rows))

    print("combining name columns", end="...")
    combine_names(csv)
    print("ok")

    print("rewriting the sex column", end="...")
    rewrite_sex(csv)
    print("ok")

    print("filling missing totals", end="...")
    fill_missing_totals(csv)
    print("ok")

    print("adding mandatory columns", end="...")
    add_mandatory_columns(csv)
    print("ok")

    print("renaming columns", end="...")
    csv.fieldnames[csv.index('Contest')] = 'MeetName'
    csv.fieldnames[csv.index('City')] = 'MeetTown'
    csv.fieldnames[csv.index('State')] = 'MeetState'
    csv.fieldnames[csv.index('Country')] = 'MeetCountry'
    csv.fieldnames[csv.index('Bodyweight')] = 'BodyweightLBS'
    csv.fieldnames[csv.index('Weight Class')] = 'WeightClassLBS'
    csv.fieldnames[csv.index('HomeState')] = 'State'
    print("ok")

    print("standardizing divisions", end="...")
    standardize_divisions(csv)
    print("ok")

    print("filtering into per-meet CSV files", end="...")
    csvs = split_by_meet(csv)
    print("%u meets" % len(csvs))

    print("building set of already-entered lifters/dates", end="...")
    lifterdateset = build_lifterdateset()
    print("ok")

    print("filtering out already-entered meets", end="...")
    csvs = retain_unentered(csvs, lifterdateset)
    print("%u meets" % len(csvs))

    print("formatting into entries/meet csvs", end="...")
    os.makedirs(outdir, exist_ok=True)  # Guarantee the outdir exists.
    for (i, csv) in enumerate(csvs):
        write_entries_meet_csvs(csv, i, outdir)
        if i == 9:
            break  # Just do the first one for now.
    print("ok")


if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: %s export.csv outdir" % sys.argv[0])
        print(" This script will create many folders in the outdir.")
        sys.exit(1)
    main(csv_filename=sys.argv[1], outdir=sys.argv[2])
