#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Ensures that a Tested column exists, set to Yes for every entry.
#
# Overwrites the input file.
#

from oplcsv import Csv
import sys

if len(sys.argv) != 2:
    print(' Usage: %s entries.csv', file=sys.stderr)
    sys.exit(1)

csvname = sys.argv[1]

csv = Csv(csvname)
csv.ensure_column("Tested")

testedidx = csv.index("Tested")
for row in csv.rows:
    row[testedidx] = "Yes"

with open(csvname, 'w') as fd:
    csv.write(fd)
