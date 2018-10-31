#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Given bounding box information of words from the PDF,
# knowing that the underlying structure is more-or-less columnar,
# output a CSV file matching the file structure in a parseable way.
#

import sys
import xml.etree.ElementTree as ET


# Attempt to merge the line above the "Name" line into the header line?
# This works on about half of the target PDFs. Thus the manual toggle.
GlobalTryMergeHeader = False


# Separator for the CSV file.
SEP = ';'

# Pages tend to have the same column spans, but we auto-detect
# on each page just to be sure. Sometimes that's impossible because
# pages don't contain lifter information, but we don't want to miss
# non-lifter information like 4th attempts. So remember the last spans.
PrevColSpans = []


# For pages that don't contain any placement information.
class NoLifterInformationException(Exception):
    pass

# For pages that don't contain any header information.


class NoHeaderInformationException(Exception):
    pass


def error(text):
    print('Error: %s' % text, file=sys.stderr)
    sys.exit(1)


class Word:

    def __init__(self, word):
        self.xmin = float(word.get('xMin'))
        self.xmax = float(word.get('xMax'))
        self.ymin = float(word.get('yMin'))
        self.ymax = float(word.get('yMax'))
        self.text = word.text

    def samerowas(self, other):
        if self.ymin <= other.ymin:
            return self.ymax >= other.ymin
        return other.ymax >= self.ymin

    def aboveorbelow(self, other):
        if self.xmin <= other.xmin:
            return self.xmax >= other.xmin
        return other.xmax >= self.xmin

    # How much of this Word is covered by the span, in units?
    def spancoverage(self, span):
        return min(self.xmax, span[1]) - max(self.xmin, span[0])

    def height(self):
        return self.ymax - self.ymin

    def xmid(self):
        return (self.xmin + self.xmax) / 2

    def ymid(self):
        return (self.ymin + self.ymax) / 2

    def __str__(self):
        return "%s (%s, %s) -> (%s, %s)" % (self.text, str(self.xmin), str(self.ymin),
                                            str(self.xmax), str(self.ymax))

    def __repr__(self):
        return self.text


# Given a sorted-by-xmin array of Words and a list of (min,max) column spans,
# generate a string with column separators placed appropriately.
def rowtocsv(row, colspans):
    columns = [''] * len(colspans)

    # For each word, assign it to a span.
    # Iterating over words prevents repeating words that ignore the column
    # system.
    for word in row:
        bestcoverage = -999
        bestidx = 0
        for i, span in enumerate(colspans):
            coverage = word.spancoverage(span)
            if coverage > bestcoverage:
                bestcoverage = coverage
                bestidx = i
        columns[bestidx] += (word.text + ' ')

    columns = [x.strip() for x in columns]
    return SEP.join(columns)


# This is how ElementTree has chosen to name the tags. Exciting!
def tag(str):
    return '{http://www.w3.org/1999/xhtml}%s' % str


def getdocroot(root):
    return root.find(tag('body')).find(tag('doc'))


def parseword(word):
    return Word(word)


# Given a set of words on a single page, sort into rows by y-value,
# with each row sorted by x-value.
def sortintorows(words):
    # The input data looks mostly-sorted, but unfortunately isn't actually.
    words = sorted(words, key=lambda x: x.ymin)
    rows = []

    row = [words[0]]
    for word in words:
        if row[0].samerowas(word):
            row.append(word)
        else:
            row = sorted(row, key=lambda x: x.xmin)
            rows.append(row)
            row = [word]

    row = sorted(row, key=lambda x: x.xmin)
    rows.append(row)
    return rows


# Find the row with 'NAME' in it, for grounding on column information.
# Some PDFs have header information on every page, some don't.
def getheaderidx(rows):
    for i, row in enumerate(rows):
        # Sometimes the PDF has a 'Place' column header.
        if (row[0].text.lower() == 'name' or
                (len(row) > 1 and row[1].text.lower() == 'name')):
            return i
    raise NoHeaderInformationException


# Find a row with a lifter that has a place.
def getlifteridx(rows):
    for i, row in enumerate(rows):
        if row[0].text == '1' or row[0].text.lower() == 'dq':
            return i
    raise NoLifterInformationException


def spanintersects(a, b):
    if a[0] <= b[0]:
        return a[1] >= b[0]
    return b[1] >= a[0]


def spanunion(a, b):
    return (min(a[0], b[0]), max(a[1], b[1]))


# Now that we know the rows, we can hardcode some information about how
# the USPA spreadsheets are formatted to roughly get column limits.
def getcolumnspans(rows):

    # First, get the row that has 'NAME' in it.
    headeridx = getheaderidx(rows)
    spans = [(x.xmin, x.xmax) for x in rows[headeridx]]

    # The row above the 'NAME' row will also contain column information.
    # Union the spans we have so far with any text in the pseudo-row above them.
    # This row contains stuff like "WT", "Squat", "Total", etc.
    # Only do this if the column above is pretty close.
    if rows[headeridx - 1][0].ymin <= \
            rows[headeridx][0].ymax + rows[headeridx][0].height():
        for x in rows[headeridx - 1]:
            above = (x.xmin, x.xmax)
            for i, span in enumerate(spans):
                if spanintersects(above, span):
                    spans[i] = spanunion(above, span)
                    break

    # There are then two special columns:
    # 1) The column for placement, which occurs before the name and isn't marked;
    # 2) The column for the name, which is actually very wide and needs to be
    # expanded.
    lifteridx = getlifteridx(rows)

    # Insert the Place column if it's not already there.
    if rows[headeridx][0].text.lower() == 'name':
        placeword = rows[lifteridx][0]
        placespan = (placeword.xmin, placeword.xmax)

        # The place comes before NAME, so insert at the beginning of the span
        # list.
        spans.insert(0, placespan)

    # Then the NAME span (spans[1]) is the start of the name
    # until roughly halfway before the beginning of the next column (spans[2]).
    # This provides a buffer for if the next column is very long.
    midpoint = spans[1][1] + ((spans[2][0] - spans[1][1]) / 2)
    namespan = (rows[lifteridx][1].xmin, midpoint)
    spans[1] = namespan

    return spans


def parsepage(page):
    words = [parseword(x) for x in page]
    rows = sortintorows(words)

    # Don't give up if column spans can't be auto-detected for this page.
    # Usually this means the page contains no lifting information,
    # but it might still contain 4th attempts!
    global PrevColSpans
    try:
        colspans = getcolumnspans(rows)
        if len(PrevColSpans) > 0:
            assert len(colspans) == len(PrevColSpans)
        PrevColSpans = colspans
    except NoLifterInformationException:
        assert PrevColSpans
        colspans = PrevColSpans
    except NoHeaderInformationException:
        assert PrevColSpans
        colspans = PrevColSpans

    # Output header information, if any.
    try:
        # The column names for headers are spread over two rows.
        # Make it easier by joining them up.
        # This assumes that there is at most one word per row!
        headeridx = getheaderidx(rows)

        if GlobalTryMergeHeader:
            for word in rows[headeridx]:
                for above in rows[headeridx - 1]:
                    if (above.ymin < word.ymax + word.height() and
                            word.aboveorbelow(above)):
                        word.text = '%s %s' % (above.text, word.text)
                        break

        # Print the header information first.
        csv = ''
        if rows[headeridx][0].text.lower() != 'place':
            csv = 'Place;'
        csv += SEP.join([word.text for word in rows[headeridx]])
        csv += '\n'

    except NoHeaderInformationException:
        csv = ''
        headeridx = -1

    for row in rows[headeridx + 1:]:
        newrow = rowtocsv(row, colspans)
        csv += newrow + '\n'

    return csv


def main():
    filename = sys.argv[1]
    with open(filename) as fd:
        html = fd.read()
    root = ET.fromstring(html)

    doc = getdocroot(root)
    csv = ''
    for page in doc:
        csv += parsepage(page)

    print(csv)


if __name__ == '__main__':
    main()
