#!/usr/bin/env python3
#
# Given an entries.csv argument, marks all lifters within as tested
# by appending the letter "T" to their division.

import os
import sys

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv

for filename in sys.argv[1:]:
    csv = oplcsv.Csv(filename)
    dividx = csv.index("Division")
    changed = False

    for row in csv.rows:
        if row[dividx] and not row[dividx].endswith("T"):
            row[dividx] = row[dividx] + "T"
            changed = True

    if changed:
        csv.write_filename(filename)
