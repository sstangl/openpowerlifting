#!/usr/bin/env python3
#
# Creates a test case in `crates/disambig/tests/` for the given lifter's data.
#
# Usage: disambig-add-test.py "Lifter Name" (without disambiguation info)

import os
import pathlib
import re
import subprocess
import sys

from oplcsv import Csv
import usernames

REPOROOTDIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
MEETDATADIR = os.path.join(REPOROOTDIR, "meet-data")
TESTDIR = os.path.join(REPOROOTDIR, "crates", "disambig", "tests", "data")


def get_meetdirs_containing(name):
    '''Returns a list of meet directories where the entries.csv contains
       information for the given lifter. This function exists to prefer
       using `rg` where possible, since that is extremely fast.'''

    # ripgrep is extremely fast, so prefer using that if available.
    try:
        re_filter = f"^{name},|,{name},|,{name}$|^{name} #|,{name} #"
        res = subprocess.run(["rg", "--files-with-matches", re_filter, MEETDATADIR],
                             capture_output=True, text=True)
        return [os.path.dirname(os.path.realpath(x)) for x in res.stdout.splitlines()
                if x.endswith("entries.csv")]

    except FileNotFoundError:
        print("WARN: Binary 'rg' missing on system, and backup impl not yet written")
        return []


def extract_csv(meetdir, name):
    '''Given a meet directory, extracts necessary test information for the given lifter.
       Returns an oplcsv.Csv object.'''

    meetcsv = Csv(os.path.join(meetdir, "meet.csv"))
    entriescsv = Csv(os.path.join(meetdir, "entries.csv"))

    outcsv = Csv()

    outcsv.ensure_columns(meetcsv.fieldnames)  # Put the meet information first.

    # Integrate the entries.csv rows for this lifter. There may be multiple rows.
    nameidx = entriescsv.index("Name")
    entriescsv.rows = [x for x in entriescsv.rows if name in x[nameidx]]
    outcsv.cat(entriescsv)

    # Now for each row, fill in the meet information.
    for row in outcsv.rows:
        for header in meetcsv.fieldnames:
            row[outcsv.index(header)] = meetcsv.rows[0][meetcsv.index(header)]

    return outcsv


def build_testcsv(meetdirs, name):
    '''Given a list of meetdirs containing info for this lifter, creates and returns
       a testing file in a format usable by crates/disambig's regression test suite.'''

    # Integrate meets into one big CSV with joint meet and entries data.
    testcsv = Csv()
    for meetdir in meetdirs:
        testcsv.cat(extract_csv(meetdir, name))

    # Apply some standardization.
    dateidx = testcsv.index("Date")
    testcsv.rows.sort(key=lambda row: row[dateidx])
    testcsv.remove_empty_columns()
    testcsv.insert_column(0, "AssertGroup")  # Specifies expected test behavior.

    return testcsv


def get_next_filepath(name):
    '''Returns the filename to which test data should be written, avoiding collisions.
       Creates in the pattern: [johndoe.csv, johndoe1.csv, johndoe2.csv, etc.]'''
    username = usernames.get_username(name)

    # If possible, just use the username directly.
    basefile = os.path.join(TESTDIR, f"{username}.csv")
    if not os.path.exists(basefile):
        return basefile

    # Otherwise, find the greatest number so far:
    filepattern = re.compile(r"^" + username + r"(\d+)\.csv$")

    testdir = pathlib.Path(TESTDIR)
    n = max(
        (int(m.group(1)) for f in testdir.iterdir() if (m := filepattern.match(f.name))),
        default=0
    ) + 1
    return os.path.join(TESTDIR, f"{username}{n}.csv")


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: " + sys.argv[0] + " \"Lifter Name\"", file=sys.stderr)
        sys.exit(1)

    name = sys.argv[1]
    if '#' in name:
        print("error: specify the base name, not a disambiguation number")
        sys.exit(1)

    # Compile the test data.
    meetdirs = get_meetdirs_containing(name)
    if not meetdirs:
        print("error: no data found for '" + name + "'", file=sys.stderr)
        sys.exit(1)

    testcsv = build_testcsv(meetdirs, name)

    # Output the test data, making sure we don't overwrite anything.
    outpath = get_next_filepath(name)
    testcsv.write_filename(outpath)

    print("Added test " + outpath)
    print("Please edit the AssertGroup column to specify expected results")
    print("(Rows belonging to the same lifter should have the same AssertGroup)")
    print("(Values don't matter beyond equality: use A, B, C, etc)")
