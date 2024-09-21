#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:


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

    assert 'Sex' not in csv.fieldnames
    assert 'WeightClassKg' not in csv.fieldnames

    csv.insert_column(csv.index('BodyweightKg'), 'Sex')
    csv.insert_column(csv.index('BodyweightKg'), 'WeightClassKg')

    dividx = csv.index('Division')
    nameidx = csv.index('Name')
    sexidx = csv.index('Sex')
    wtclsidx = csv.index('WeightClassKg')

    sex = None
    wtcls = None

    for row in csv.rows:
        if row[dividx] and not row[nameidx]:
            sex = row[dividx][0]
            wtcls = row[dividx][1:]
        else:
            assert sex
            assert wtcls
            row[sexidx] = sex
            row[wtclsidx] = wtcls

            # While here, also title-case the division.
            row[dividx] = row[dividx].title()

    csv.rows = [row for row in csv.rows if row[nameidx]]

    if 'Wilks' in csv.fieldnames:
        csv.remove_column_by_name('Wilks')

    with open(filename, 'w') as fd:
        csv.write(fd)


if __name__ == '__main__':
    main(sys.argv[1])
