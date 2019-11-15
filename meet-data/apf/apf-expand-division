#!/usr/bin/env python3
#
# The APF encodes a lot of information in their division field.
# This script expands the Division column into Sex and Equipment columns.
#
# This works for APF meets that use classes that look like "M_OCR_AAPF".
#

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

    if 'Division' not in csv.fieldnames:
        return

    if 'Sex' not in csv.fieldnames:
        csv.insert_column(csv.index('Division'), 'Sex')

    if 'Equipment' not in csv.fieldnames:
        csv.insert_column(csv.index('Division'), 'Equipment')

    if 'Tested' not in csv.fieldnames:
        csv.append_column('Tested')

    dividx = csv.index('Division')
    sexidx = csv.index('Sex')
    eqpidx = csv.index('Equipment')
    testedidx = csv.index('Tested')

    for row in csv.rows:
        div = row[dividx]
        div = div.replace('-', '_')  # Sometimes dashes are used instead.

        if not div:
            continue
        if div.count('_') < 2:  # Also allow M_MR_4_APF.
            continue

        if div.startswith('M'):
            row[sexidx] = 'M'
        elif div.startswith('F'):
            row[sexidx] = 'F'
        else:
            raise Exception("unknown sex")

        if 'AAPF' in div or 'AWPC' in div:
            row[testedidx] = 'Yes'

        # The CPF uses "Raw Assisted" (RA) and "Assisted Raw" (AR)
        # as a category meaning "sleeves or wraps", so it counts as wraps.
        if 'CR_' in div or 'RA_' in div or 'AR_' in div:
            row[eqpidx] = 'Wraps'
        elif 'R_' in div:
            row[eqpidx] = 'Raw'
        elif 'EM_' in div:
            row[eqpidx] = 'Multi-ply'
        elif 'ES_' in div:
            row[eqpidx] = 'Single-ply'
        else:
            raise Exception("unknown equipment")

    with open(filename, 'w') as fd:
        csv.write(fd)


if __name__ == '__main__':
    main(sys.argv[1])
