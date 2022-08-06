#!/usr/bin/env python
# vim: set ts=8 sts=4 et sw=4 tw=99:
# Parses modern USPA results text files to CSV.
#
# USPA releases results in PDF format. Running those files through pdftotext
# generates a file that doesn't have well-separated columns. Running this
# script on that text file translates it to CSV format.

import sys


def list_to_csv(x):
    return ','.join(x)


# Given a line, return a list of (left, right) word ranges.
def find_word_ranges(line):
    ranges = []  # The accumulated list of ranges.
    inWord = False  # State tracker: whether currently constructing a range.
    leftIndex = 0  # if inWord, then the left index of the range.

    # For simplification below, guarantee that line ends with a space.
    # This guarantees that the append() call below will always finish a span.
    line = line + ' '

    for j in range(0, len(line)):
        if not line[j].isspace():
            if not inWord:
                leftIndex = j
                inWord = True
        else:
            if inWord:
                ranges.append((leftIndex, j - 1))
                inWord = False

    return ranges


# Given two (left, right) pair ranges, check whether they overlap.
def ranges_overlap(x, y):
    return x[1] > y[0] and x[0] < y[1]


# Sort the range list by left index.
def sort_range_list(r):
    return sorted(r, key=lambda x: x[0])


# Given two sets of range pairs, return a set with the ranges unioned.
def union_ranges(r1, r2):

    # For each element in r1, attempt to expand it via r2.
    def union_ranges_helper(r1, r2):
        union = []

        for p1 in r1:
            overlaps = False

            for p2 in r2:
                if ranges_overlap(p1, p2):
                    overlaps = True
                    union.append((min(p1[0], p2[0]), max(p1[1], p2[1])))
                    break

            if not overlaps:
                union.append(p1)

        return union

    # Expand each set by the other, to avoid missing columns.
    s1 = set(union_ranges_helper(r1, r2))
    s2 = set(union_ranges_helper(r2, r1))

    # Remove duplicate elements.
    return sort_range_list(list(s1.union(s2)))


# Given a range and a list of ranges, return the index of the range that
# most closely matches the range input.
def find_closest_range(r, ranges):
    bestMatch = 999999
    bestIndex = -1

    for i in range(0, len(ranges)):
        # The columns often don't line up well.
        midpoint1 = r[0] + (r[1] - r[0]) / 2
        midpoint2 = ranges[i][0] + (ranges[i][1] - ranges[i][0]) / 2
        distance = abs(midpoint1 - midpoint2)

        if distance < bestMatch:
            bestMatch = distance
            bestIndex = i

    assert bestIndex != -1
    return bestIndex


# Spreadsheet columns are represented as (left index, right index) tuples,
# with the indices inclusive. Builds that list for the lines of the given page.
def USPA_extract_sorted_page_column_ranges(lines):
    for i in range(0, len(lines)):
        line = lines[i]

        # The header at the top of the page is variable-length, but there is always
        # a column named NAME. Column names are distributed on that line and on
        # the line before that one. Look for "NAME".
        if 'NAME' not in line:
            continue

        # Guess roughly where the columns are by where the headers appear.
        lineHeaders = find_word_ranges(line)

        # Multi-word columns are spread over the previous line, too.
        # Union the word ranges of these two lines by peeking upward.
        prevHeaders = find_word_ranges(lines[0 if i == 0 else i - 1])

        # Union the two to get the full list of column boundaries.
        return union_ranges(lineHeaders, prevHeaders)

    # Some pages at the end have garbage data.
    return []


# Once again,
WroteColumnNames = False


def USPA_get_column_names_once(csv, lines, columnRanges):
    global WroteColumnNames

    if WroteColumnNames:
        return

    WroteColumnNames = True

    for i in range(0, len(lines)):
        if 'NAME' in lines[i]:
            break

    columns = []

    line1 = lines[i - 1]
    line2 = lines[i]

    for r in columnRanges:
        name = line1[r[0]: r[1] + 1].strip() + line2[r[0]: r[1] + 1].strip()
        lname = name.lower()

        # Make some common transformations.
        if lname == "name":
            name = "Name"
        elif lname == "bdywght":
            name = "BodyweightLBS"
        elif lname == "wtclass":
            name = "WeightClassLBS"
        elif lname == "wilksscore":
            name = "Wilks"
        elif lname == "Best3Bench":
            name = "Best3BenchLBS"
        elif lname == "Best3Squat":
            name = "Best3SquatLBS"
        elif lname == "Best3Deadlift":
            name = "Best3DeadliftLBS"

        columns.append(name)

    csv.append(columns)


# Add a USPA lifter line.
# This uses a literal parsing of the text file's column headers,
# so place will be, for now, recorded in the name field.
def USPA_add_lifter_row(csv, columnRanges, line):
    # Construct the row, using columnRanges for placement.
    row = [''] * len(columnRanges)

    # The hard part is picking out the name field.
    # People often have complicated names.
    # Subtract 3 for slack adjustment, since it doesn't usually
    # line up perfectly.
    name = line[0: columnRanges[1][0] - 3]

    # We're going to keep the name field around for determining
    # its column length. Sometimes the column has whitespace on the left,
    # so don't modify the string itself.
    row[0] = name.strip()

    # After the name, all the fields should be space-delimited.
    withoutName = (' ' * len(name)) + line[len(name):]

    # Get the boundaries of each word.
    # We'll use this to look up each word in the columnRanges table,
    # to get the index of the column, and then insert it into the row.
    ranges = find_word_ranges(withoutName)

    # For each word, look up what column range it mostly closely matches.
    for r in ranges:
        index = find_closest_range(r, columnRanges)

        # Emit debug output for failure case requiring manual intervention.
        if row[index]:
            print('Column matching error (manually align columns):')
            print('Matched "%s" to column already containing "%s"' %
                  (line[r[0]: r[1] + 1], row[index]))
            print(line)

        # If this assert fires, the input file must be manually fixed to
        # align columns reasonably. Debug output helpfully provided above.
        assert not row[index]
        row[index] = withoutName[r[0]: r[1] + 1]

    csv.append(row)


def USPA_add_4ths_row(csv, columnRanges, line):
    # Lines containing 4ths shouldn't have anything else on the line.
    words = line.split()
    for word in words:
        if '4th-' not in word and '4TH-' not in word:
            print("Error on line: %s" % line)
        assert '4th-' in word or '4TH-' in word

    # Construct the row, using columnRanges for placement.
    row = [''] * len(columnRanges)

    # Match ranges to a column.
    ranges = find_word_ranges(line.replace(
        '4th-', '    ').replace('4TH-', '    '))
    fullranges = find_word_ranges(line)
    assert len(ranges) == len(fullranges)

    # For each 4th attempt, look up what column it most closely matches.
    for i in range(0, len(ranges)):

        index = find_closest_range(ranges[i], columnRanges)

        assert not row[index]
        row[index] = line[fullranges[i][0]: fullranges[i][1] + 1]

    csv.append(row)


# Begin main script.
if len(sys.argv) != 2:
    print(" Usage: " + sys.argv[0] + " original.txt")
    sys.exit(1)

filename = sys.argv[1]
with open(filename) as fd:
    wholefile = fd.read()

# Get rid of all commas, since we're moving to CSV format.
wholefile = wholefile.replace(',', ' ')

# After pdftotext , pages are separated by formfeeds (0x0c).
# USPA's meet results always include column titles on each page,
# allowing each page to be treated as an independent unit.
# This is helpful because the columns tend to be in different places
# on different pages, as they change width based on each page's contents.
pages = wholefile.split("\f")

# We're going to be constructing the entire csv file in-memory.
# This is a list of string lists.
csv = []


# Parse and append the page's data to the csv array.
def parse_page(csv, page):
    lines = page.split("\n")

    columnRanges = USPA_extract_sorted_page_column_ranges(lines)
    if not columnRanges:
        return  # Not a page with data.

    USPA_get_column_names_once(csv, lines, columnRanges)

    # Iterate through the lifters.
    for i in range(0, len(lines)):
        line = lines[i]

        # Lifter rows begin with a low number indicating place, or 'DQ'.
        # Sometimes they begin with 'NS' for no good reason.
        # Sometimes lifters only count for records, which I'm using "NA" for.
        def is_lifter_row(line):
            words = line.split()
            if not words:
                return False
            return (words[0].isdigit() and int(words[0]) < 50) or \
                words[0] in ['DQ', 'NA', 'NS']

        if is_lifter_row(line):
            USPA_add_lifter_row(csv, columnRanges, line)

        # 4th attempts for a given lifter are recorded on the next line.
        elif '4th-' in line or '4TH-' in line:
            USPA_add_4ths_row(csv, columnRanges, line)

        # Lifters are organized by divisions.
        # Divisions are given their own rows.
        # They always precede a lifter row.
        elif i + 1 < len(lines) and is_lifter_row(lines[i + 1]) and 'NAME' not in line:
            # Just output the division into the csv array.
            # We will modify the array in post-processing.
            csv.append(["DIVISION:" + line.strip()])


# Get the page data into the csv array.
for page in pages:
    parse_page(csv, page)


# Remove all the 'DIVISION' line markers.
def csv_remove_divisions(csv):
    return [x for x in csv if 'DIVISION:' not in x[0]]


# First, get rid of all the lines starting with "DIVISION:",
# which we inserted for stateful division tracking.
def csv_integrate_divisions(csv):
    # The first line is for column headers and can be skipped.
    assert csv[0][0] == 'Name'

    # Arbitrary choice where we're inserting a column for the division.
    divcolumn = 1

    # Insert a division tracker in the column headers.
    csv[0].insert(divcolumn, "Division")

    division = ''
    for k in range(1, len(csv)):
        # If this row defines a stateful division, remember it.
        if 'DIVISION:' in csv[k][0]:
            division = csv[k][0][len('DIVISION:'):]

            # Edit: Don't format division here, because it messes up
            # the equipment extraction in the later function.
            # Instead, use scripts/fix-division.
            """
            # Try to remove some common formatting stuff from the division name.
            division = division.replace("(Wilks formula)", '')
            if "Open Men" in division:
                division = "Open Men"
            if "Open Women" in division:
                division = "Open Women"
            division = division.strip()
            """

        # Otherwise, this is either a row for results, or a 4ths row.
        # Rows for results have names, so check for a name.
        elif csv[k][0]:
            csv[k].insert(divcolumn, division)

        # Otherwise, this is a 4ths row, and we just need to pad it.
        else:
            csv[k].insert(divcolumn, '')

    return csv_remove_divisions(csv)


def csv_integrate_4ths(csv):
    # The first line is for column headers and can be skipped.
    assert csv[0][0] == 'Name'

    # First, we need to figure out what columns contain 4th attempts.
    # We need to know all of them at once, since we need to insert
    # extra columns for them in all rows.
    fourths = set()
    for row in csv:
        # Skip all rows that don't have fourth attempts.
        if row[0]:
            continue

        for colnum in range(0, len(row)):
            if '4th-' in row[colnum].lower():
                fourths.add(colnum)

    collist = list(fourths)
    collist = sorted(collist)

    # The fourths set now contains every column that needs a buddy
    # on the right. Iterate backwards these columns, and insert one
    # into each row.
    for x in range(len(collist) - 1, -1, -1):

        k = collist[x]
        for i in range(0, len(csv)):
            row = csv[i]

            # Insert a row to the right of the one containing the 4th attempt.
            row.insert(k + 1, '')

            # Move the 4th attempt to that column in the previous row.
            if '4th-' in row[k].lower():
                csv[i - 1][k +
                           1] = row[k].replace('4th-', '').replace('4TH-', '')
                row[k] = ''

        # Now figure out what the column header should be.
        # We can infer it from the previous column.
        if csv[0][k] == "Best3SquatLBS":
            csv[0][k + 1] = "Squat4LBS"
        elif csv[0][k] == "Best3BenchLBS":
            csv[0][k + 1] = "Bench4LBS"
        elif csv[0][k] == "Best3DeadliftLBS":
            csv[0][k + 1] = "Deadlift4LBS"
        else:
            csv[0][k + 1] = csv[0][k] + '4'

    # Get rid of all the blanked rows.
    return [x for x in csv if len(x[0]) != 0]


# The name field looks like "1 Maggie Haywood". Put that 1 into its own field.
def csv_extract_place_from_name(csv):
    # The first line is for column headers and can be skipped.
    assert csv[0][0] == 'Name'

    csv[0].insert(1, 'Place')

    for i in range(1, len(csv)):
        row = csv[i]
        namefield = row[0]

        # This is sometimes useful if other passes are skipped.
        if not namefield.strip():
            continue

        # Either a number or "DQ"
        placement = namefield.split()[0]
        name = namefield[namefield.find(' '):].strip()

        row[0] = name
        row.insert(1, placement)

    return csv


# The division field contains sex and equipment classification.
# Extract that into separate fields for easier parsing.
def csv_extract_sex_and_equipment(csv):
    # The first line is for column headers and can be skipped.
    assert csv[0][0] == 'Name'

    # After all the previous csv_ functions, the Division field moved.
    divcol = 2
    assert csv[0][divcol] == 'Division'

    csv[0].insert(divcol + 1, 'Sex')
    csv[0].insert(divcol + 2, 'Equipment')

    for i in range(1, len(csv)):
        row = csv[i]
        divfield = row[divcol].lower()

        sex = 'F' if 'women' in divfield else 'M'

        if 'raw' in divfield and 'classic' not in divfield:
            equipment = 'Raw'
        elif 'classic' in divfield:
            equipment = 'Wraps'
        elif 'multi' in divfield or 'equipped' in divfield:
            equipment = 'Multi-ply'
        elif 'single' in divfield:
            equipment = 'Single-ply'
        else:
            equipment = ''

        row.insert(divcol + 1, sex)
        row.insert(divcol + 2, equipment)

    return csv


csv = csv_integrate_divisions(csv)
csv = csv_integrate_4ths(csv)
csv = csv_extract_place_from_name(csv)
csv = csv_extract_sex_and_equipment(csv)

# Output the CSV file.
for row in csv:
    print(list_to_csv(row))
