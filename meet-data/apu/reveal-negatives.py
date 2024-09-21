#!/usr/bin/env python3
#
# When run through pdftotext, APU meet results seem to miss the negatives
# that indicate failed lifts, even though they are present visually in the PDF.
#
# It turns out that in the text file, the negatives are replaced by a special
# character that renders as space. This text replaces that 'MAGIC_SPACE' character
# with a negative sign, causing the negatives to once again manifest.
#

import sys

MAGIC_SPACE = '­'  # Not actually a normal space character.


def main(filename):
    with open(filename, 'r') as fd:
        text = fd.read()

    text = text.replace(MAGIC_SPACE, '-')

    with open(filename, 'w') as fd:
        fd.write(text)


if __name__ == '__main__':
    main(sys.argv[1])
