#!/usr/bin/env python3
#
# Rewrites entries.csv so that it only contains rows with Sex data.

import sys
import os

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv


csv = oplcsv.Csv(sys.argv[1])
sexidx = csv.index('Sex')

rows2 = []
for row in csv.rows:
    if row[sexidx]:
        rows2.append(row)

csv.rows = rows2
with open(sys.argv[1], 'w') as fd:
    csv.write(fd)
