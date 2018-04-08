#!/usr/bin/env python3
#
# Given an SPF file that's a free-from text dump,
# use heuristics to separate out lifter information
# from division/class/event information.
#

import sys


def main(filename):
    with open(filename) as fd:
        text = fd.read()

    # Quotes are generally placed in unhelpful positions unrelated to CSV
    # stuff.
    text = text.replace('"', '')

    # For our purposes, two spaces are enough.
    while '   ' in text:
        text = text.replace('   ', '  ')

    # Commas at the end of lines serve no purpose.
    while ',\n' in text:
        text = text.replace(',\n', '\n')

    # The text "Class " clearly designates new information.
    text = text.replace('Class ', 'Class\n')

    text = text.replace(' lb. Class', ' Class')

    # Look for some markers and dump them on their own lines.
    text = text.replace(' 114 Class', '\n114 Class')
    text = text.replace(' 123 Class', '\n123 Class')
    text = text.replace(' 148 Class', '\n148 Class')
    text = text.replace(' 165 Class', '\n165 Class')
    text = text.replace(' 181 Class', '\n181 Class')
    text = text.replace(' 198 Class', '\n198 Class')
    text = text.replace(' 220 Class', '\n220 Class')
    text = text.replace(' 242 Class', '\n242 Class')
    text = text.replace(' 259 Class', '\n259 Class')
    text = text.replace(' 275 Class', '\n275 Class')
    text = text.replace(' 308 Class', '\n308 Class')
    text = text.replace(' Super Heavy', '\nSuper Heavy')
    text = text.replace(' SHW', '\nSHW')

    text = text.replace(' Sub-Masters', '\nSubmasters')
    text = text.replace(' Masters', '\nMasters')
    text = text.replace(' Men', '\nMen')
    text = text.replace(' Women', '\nWomen')
    text = text.replace(' Juniors', '\nJuniors')
    text = text.replace(' Teen', '\nTeen')
    text = text.replace(' Pre-Teen', '\nPre-Teen')
    text = text.replace(' Police', '\nPolice')

    text = text.replace(' Raw', '\nRaw')
    text = text.replace(' Single', '\nSingle')
    text = text.replace(' Multi', '\nMulti')
    text = text.replace(' Equipped', '\nEquipped')

    # Get rid of trailing spaces.
    while ' \n' in text:
        text = text.replace(' \n', '\n')

    with open(filename, 'w') as fd:
        fd.write(text)


if __name__ == '__main__':
    main(sys.argv[1])
