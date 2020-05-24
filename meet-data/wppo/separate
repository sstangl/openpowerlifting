#!/usr/bin/env python3
#
# Given a humongous CSV containing a bunch of WPPO meets (in their export format),
# separates them out into separate data files in the CWD.

import os
import sys

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def main(filename):
    csv = Csv(filename)

    names = set()
    for row in csv.rows:
        names.add(row[csv.index("MeetName")])

    for (i, name) in enumerate(sorted(names)):
        # Helper for skipping ones already entered.
        if i <= -1:
            continue

        rows = list(filter(lambda r: r[csv.index("MeetName")] == name, csv.rows))

        filename = str(i) + "-data.csv"
        entries = Csv()
        entries.rows = rows
        entries.fieldnames = csv.fieldnames
        entries.write_filename(filename)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(" Usage: %s export.csv")
        sys.exit(1)
    main(sys.argv[1])
