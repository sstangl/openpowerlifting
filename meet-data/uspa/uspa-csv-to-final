#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Given a CSV in PDF format, parse it to extract information
# in the OpenPowerlifting internal format.
#

import sys
import os

try:
    import oplcsv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplcsv


def error(text):
    print("Error: %s" % text, file=sys.stderr)
    sys.exit(1)


def isint(str):
    try:
        int(str)
        return True
    except ValueError:
        return False


# Many CSVs are detected in the same useless way, with columns like:
#  Place;NAME;State;Class;Wt(KG);Age;Kg;Bench,;Kg,;Kg;Kg;Score;LBS;LBS;LBS;LBS
# This function tries to auto-correct those lines.
def replace_common_fieldnames(csv):
    s = ';'.join(csv[0])

    if s == 'Place;NAME;State;Class;Wt(KG);Age;Kg;Bench,;Kg;Kg;Kg;Score;LBS;LBS;LBS;LBS':
        csv[0] = 'Place;Name;State;WeightClassKg;BodyweightKg;Age;' \
                 'sq;bp;bp;dl;total;Wilks;LBS;LBS;LBS;LBS'.split(';')
    if s == 'Place;Name;State;Wt;Weight;Age;SQ;Kg;BP;Kg;DL;Kg;Total;Kg;Wilks;McC;Total':
        csv[0] = 'Place;Name;State;WeightClassKg;BodyweightKg;Age;' \
                 'sq;sq;bp;bp;dl;dl;total;total;wilks;wilks;lbs'.split(';')
    if s == 'Place;NAME;State;Class;Wt(KG);Age;Kg;Kg;Kg;Kg;Score;LBS;LBS;LBS;LBS':
        csv[0] = 'Place;Name;State;WeightClassKg;BodyweightKg;Age;' \
                 'sq;bp;dl;total;wilks;lbs;lbs;lbs;lbs'.split(';')
    if s == 'Place;Name;State;Wt;Weight;Age;SQ;Kg;BP;Kg;DL;Kg;Total;' \
            'Wilks;McC;SQ;BP;DL;Total':
        csv[0] = 'Place;Name;State;WeightClassKg;BodyweightKg;Age;' \
                 'sq;sq;bp;bp;dl;dl;total;wilks;wilks;lbs;lbs;lbs;lbs'.split(';')
    if s == 'Place;Name;State;Wt;class;Weight;Age;SQ;Kg;BP' \
            ';Kg;DL;Kg;Total;Wilks;McC;SQ;BP;DL;Total':
        csv[0] = 'Place;Name;State;WeightClassKg;WeightClassKg;BodyweightKg;' \
                 'Age;sq;sq;bp;bp;dl;dl;total;wilks;wilks;lbs;lbs;lbs;lbs'.split(';')
    if s == 'Place;Name;State;Wt;Weight;Age;SQ;Kg;BP;Kg;DL;Kg;Total;Wilks;McC;Total':
        csv[0] = 'Place;Name;State;Wt;Weight;Age;SQ;sq;BP;' \
                 'bp;DL;dl;Total;Wilks;McC;lbs'.split(';')
    if s == 'Place;Name;State;Wt;Weight;Age;SQ;Kg;BP;Kg;DL;Kg;Total;Kg;Wilks;McC':
        csv[0] = 'Place;Name;State;Wt;Weight;Age;SQ;SQ;BP;BP;' \
                 'DL;DL;Total;Total;Wilks;McC'.split(';')
    if s == 'Place;Name;State;Wt;Weight;Age;SQ;Kg;BP;Kg;DL;Kg;Total;Wilks;McC':
        csv[0] = 'Place;Name;State;Wt;Weight;Age;SQ;SQ;BP;BP;' \
                 'DL;DL;Total;Wilks;McC'.split(';')


def fixcolnames(columns):
    for i, name in enumerate(columns):
        low = name.lower()

        if low == 'name':
            columns[i] = 'Name'
        elif low == 'place':
            columns[i] = 'Place'
        elif low == 'division':
            columns[i] = 'Division'
        elif low == 'state' or low == 'st':
            columns[i] = 'State'
        elif low == 'country, state':
            columns[i] = 'Country'
        elif low == 'college- university':
            columns[i] = 'College/University'
        elif low == 'wt class':  # Sometimes lbs, sometimes kg, sigh.
            columns[i] = 'WeightClassKg'
        elif low == 'clas s':
            columns[i] = 'WeightClassKg'
        elif low == 'weightclasskg':
            columns[i] = 'WeightClassKg'
        elif low == 'bdy wt(kg)' or low == 'wght (kg)':
            columns[i] = 'BodyweightKg'
        elif low == 'wght(k g)':
            columns[i] = 'BodyweightKg'
        elif low == 'bodyweightkg':
            columns[i] = 'BodyweightKg'
        elif low == 'bdywt (kg)':
            columns[i] = 'BodyweightKg'
        elif low == 'wght( kg)':
            columns[i] = 'BodyweightKg'
        elif low == 'bdy (kg)':
            columns[i] = 'BodyweightKg'
        elif low == 'bdy wght':
            columns[i] = 'BodyweightLBS'
        elif low == 'wght( lb)':
            columns[i] = 'BodyweightLBS'
        elif low == 'wght lbs':
            columns[i] = 'BodyweightLBS'
        elif low == 'age':
            columns[i] = 'Age'
        elif low == 'squat':
            columns[i] = 'Best3SquatKg'
        elif low == 'squat, kg' or low == 'sq kg':
            columns[i] = 'Best3SquatKg'
        elif low == 'squat kg':
            columns[i] = 'Best3SquatKg'
        elif low == 'squat lbs' or low == 'sq lbs':
            columns[i] = 'Best3SquatLBS'
        elif low == 'bench':
            columns[i] = 'Best3BenchKg'
        elif low == 'bench, kg' or low == 'bp kg':
            columns[i] = 'Best3BenchKg'
        elif low == 'bench kg':
            columns[i] = 'Best3BenchKg'
        elif low == 'bench lbs' or low == 'bp lbs':
            columns[i] = 'Best3BenchLBS'
        elif low == 'deadlift':
            columns[i] = 'Best3DeadliftKg'
        elif low == 'deadlift, kg' or low == 'dl kg':
            columns[i] = 'Best3DeadliftKg'
        elif low == 'deadlift kg':
            columns[i] = 'Best3DeadliftKg'
        elif low == 'deadlift lbs' or low == 'dl lbs':
            columns[i] = 'Best3DeadliftLBS'
        elif low == 'total':
            columns[i] = 'TotalKg'
        elif low == 'total, kg':
            columns[i] = 'TotalKg'
        elif low == 'total kg':
            columns[i] = 'TotalKg'
        elif low == 'total lbs':
            columns[i] = 'TotalLBS'
        elif low == 'wilks':
            columns[i] = 'Wilks'
        elif low == 'score':
            columns[i] = 'Wilks'
        elif low == 'wilks score':
            columns[i] = 'Wilks'
        elif low == 'wilks total':
            columns[i] = 'Wilks'
        elif low == 'school':
            columns[i] = 'School'
        elif low == 'lbs':
            columns[i] = 'DELETEME'
        elif low == 'weightclasskg':
            columns[i] = 'WeightClassKg'
        elif low == 'wt':
            columns[i] = 'WeightClassKg'
        elif low == 'weight':
            columns[i] = 'BodyweightKg'
        elif low == 'sq':
            columns[i] = 'Best3SquatKg'
        elif low == 'bp':
            columns[i] = 'Best3BenchKg'
        elif low == 'dl':
            columns[i] = 'Best3DeadliftKg'
        elif low == 'mcc' or low == 'mcc total':
            columns[i] = 'McCulloch'
        elif low == 'lbs':
            columns[i] = 'DELETEME'
        elif low == 'college':
            columns[i] = 'College'
        elif low == 'country':
            columns[i] = 'Country'
        else:
            error("Unknown column %s" % name)


def colnameto4thname(colname):
    if colname == 'Best3SquatKg':
        return 'Squat4Kg'
    elif colname == 'Best3SquatLBS':
        return 'Squat4LBS'
    elif colname == 'Best3BenchKg':
        return 'Bench4Kg'
    elif colname == 'Best3BenchLBS':
        return 'Bench4LBS'
    elif colname == 'Best3DeadliftKg':
        return 'Deadlift4Kg'
    elif colname == 'Best3DeadliftLBS':
        return 'Deadlift4LBS'
    elif colname == 'DELETEME':
        return 'DELETEME'
    else:
        error("Don't know how to make 4th column out of %s" % colname)


def add4thcolumns(columns, csv):
    for row in csv:
        for col, text in enumerate(row):
            if '4th' in text.lower():
                colname = columns[col]
                newcol = colnameto4thname(colname)

                if newcol not in columns:
                    columns.append(newcol)


def parse(csv):
    columns = csv[0]
    assert columns[0].lower() == 'place'
    assert columns[1].lower() == 'name'

    fixcolnames(columns)

    if 'Sex' not in columns:
        columns.append('Sex')
    if 'Division' not in columns:
        columns.append('Division')
    if 'Equipment' not in columns:
        columns.append('Equipment')

    # Are there any 4th attempts? Then we'll need even more columns.
    add4thcolumns(columns, csv)

    if 'Event' not in columns:
        columns.append('Event')

    # Expand every row of the csv to have the right number of columns.
    for row in csv:
        while len(row) < len(columns):
            row.append('')

    # This is the final CSV-like array-of-arrays we're building up below.
    rval = [columns]

    divisionstate = None
    sexstate = None
    equipmentstate = None
    lifterstate = None
    eventstate = None

    # Alright, we have all the columns we'll need set up!
    # Time to start running the state machine!
    for row in csv:
        # Column information: we already got that.
        if row[0] == 'Place':
            pass

        # If the first position is filled in, it has place information.
        elif row[0] != '':
            # Maybe USPA stopped using NA and NS? That would be nice.
            # G == Guest Lifter (no placement)
            if row[0].lower() == 'gl' or row[0].lower() == 'guest':
                row[0] = 'G'
            assert isint(row[0]) or row[0].lower(
            ) == 'dq' or row[0].lower() == 'g'

            # Insert the stateful information into the row.
            divisioncol = columns.index('Division')
            sexcol = columns.index('Sex')
            equipmentcol = columns.index('Equipment')
            eventcol = columns.index('Event')

            if not row[divisioncol]:
                assert divisionstate
                row[divisioncol] = divisionstate
            if not row[sexcol]:
                assert sexstate
                row[sexcol] = sexstate
            if not row[equipmentcol]:
                assert equipmentstate
                row[equipmentcol] = equipmentstate
            if not row[eventcol]:
                assert eventstate
                row[eventcol] = eventstate

            rval.append(row)

            # Remember the last lifter processed in case we have to add 4th
            # attempts.
            lifterstate = row

        # If the first position is blank, this is state information, 4th
        # attempts, or garbage.
        else:
            searcher = ';'.join(row).lower()
            # Sometimes the rows just switch the sex state.
            if ';women' in searcher:
                sexstate = 'F'
            elif ';men' in searcher:
                sexstate = 'M'

            # Some PDFs put this information with the sexstate.
            if ' classic raw' in searcher or ' cl raw' in searcher:
                equipmentstate = 'Wraps'
            elif ' raw' in searcher:
                equipmentstate = 'Raw'
            elif ' single' in searcher:
                equipmentstate = 'Single-ply'
            elif ' multi' in searcher:
                equipmentstate = 'Multi-ply'

            # Event information is included... sometimes.
            if 'powerl' in searcher:  # The full word is easy to misspell.
                eventstate = 'SBD'
            elif 'bench' in searcher:
                eventstate = 'B'
            elif 'deadlift' in searcher:
                eventstate = 'D'
            elif ('push' in searcher and 'pull' in searcher) or 'ironman' in searcher:
                eventstate = 'BD'

            # This is a row that specifies 4th attempts for the last lifter.
            if '4th' in searcher:
                assert lifterstate

                for i, text in enumerate(row):
                    text = text.lower()
                    if '4th' in text:
                        amount = text.replace('4th', '').strip()
                        amount = amount.replace('-', '').strip()
                        amount = amount.replace(':', '').strip()
                        if amount == "":
                            error("4th amount split over rows: \"%s\"" % text)
                        newcol = colnameto4thname(columns[i])
                        assert newcol in columns

                        newcolidx = columns.index(newcol)
                        lifterstate[newcolidx] = amount

            # This is a row that specifies division and other information.
            elif ('open' in searcher or 'junior' in searcher or
                    'jr' in searcher or 'master' in searcher):
                # If this is division information, it disregards columns.
                info = ' '.join(row).strip().lower()

                # Divisions are {Junior, Open, Submaster, Master} +
                #  {Women, Men} + (Age Range if non-Open)
                if ('junior' not in info and 'jr' not in info and
                        'open' not in info and 'master' not in info):
                    continue

                # Get the sex state.
                if 'women' in info:
                    sexstate = 'F'
                elif 'men' in info:
                    sexstate = 'M'

                # Get the division state.
                # Normally looks like: "52kg/114 Junior Men 13-15 Classic Raw"
                # and we want from that: "Junior 13-15"
                if '-' in info:
                    # Extract 20-23, but don't extract "sub-master".
                    k = [x for x in info.split() if '-' in x and x[0].isdigit()]
                    # Don't assert on len(k): the freeform text at the bottom
                    # confuses it.
                    if len(k) == 1:
                        agerange = ' ' + k[0].replace(',', '')
                    else:
                        agerange = ''
                elif '80+' in info:
                    agerange = '80+'
                else:
                    agerange = ''

                if 'open' in info:
                    divisionstate = 'Open' + agerange
                elif 'submaster' in info:
                    if agerange:
                        divisionstate = 'Submaster' + agerange
                    else:
                        divisionstate = 'Submaster 35-39'
                elif 'junior' in info:
                    divisionstate = 'Junior' + agerange
                elif 'jr' in info:
                    divisionstate = 'Junior' + agerange
                elif 'master' in info:
                    divisionstate = 'Master' + agerange
                else:
                    error('Unknown division: %s' % ' '.join(row).strip())

                # Get the equipment state.
                if 'classic' in info or 'cl raw' in info:
                    equipmentstate = 'Wraps'
                elif 'raw' in info:
                    equipmentstate = 'Raw'
                elif 'single' in info:
                    equipmentstate = 'Single-ply'
                elif 'multi' in info:
                    equipmentstate = 'Multi-ply'
                else:
                    # This happens when the USPA forgets how to spell things.
                    # For example, "Mulitply". Comment out on a per-import
                    # basis.
                    assert equipmentstate
                    # error('Unknown equipment: %s' % ' '.join(row).strip())

    return rval


# USPA SHW weightclasses are 140+ for men, 90+ for women.
def fixshw(rows):
    weightclassidx = rows[0].index('WeightClassKg')
    assert weightclassidx >= 0

    sexidx = rows[0].index('Sex')
    assert sexidx >= 0

    for row in rows[1:]:
        if row[weightclassidx] == 'SHW':
            if row[sexidx] == 'M':
                row[weightclassidx] = '140+'
            else:
                row[weightclassidx] = '90+'


# Remove a LBS column if there is a matching Kg column.
# Needs to be very careful, since it's deleting data!
def removeredundantlbs(rows):
    cols = list([col for col in rows[0] if 'LBS' in col])
    for col in cols:
        if col.replace('LBS', 'Kg') in rows[0]:
            i = rows[0].index(col)
            for row in rows:
                del row[i]


def remove_kg_from_weightclass(rows):
    weightclassidx = rows[0].index('WeightClassKg')

    for row in rows[1:]:
        row[weightclassidx] = row[weightclassidx].replace('kg', '')


def main():
    with open(sys.argv[1]) as fd:
        csv = [[y.strip() for y in x.split(';')] for x in fd.readlines()]

    replace_common_fieldnames(csv)
    rval = parse(csv)

    # Fixups.
    fixshw(rval)
    removeredundantlbs(rval)
    remove_kg_from_weightclass(rval)

    # Sometimes commas get snuck into numbers, like "1,036.16". Sigh.
    # Also get rid of quotation marks, since the JS doesn't like that.
    for row in rval:
        for i, word in enumerate(row):
            row[i] = row[i].replace('"', '\'')
            row[i] = row[i].replace('(', '\'')
            row[i] = row[i].replace(')', '\'')
            row[i] = row[i].replace(',', '')

            # Fix some common errors.
            if row[i] in ['O', '0', '#VALUE!', '#REF!']:
                row[i] = ''

            # Place is the first column.
            if i > 0 and row[i] == 'DQ':
                row[i] = ''

    # Load into a CSV file.
    csv = oplcsv.Csv()
    csv.fieldnames = rval[0]
    csv.rows = rval[1:]

    # Remove some unnecessary columns.
    csv.remove_empty_columns()
    while 'DELETEME' in csv.fieldnames:
        csv.remove_column_by_name('DELETEME')
    while 'Wilks' in csv.fieldnames:
        csv.remove_column_by_name('Wilks')
    while 'McCulloch' in csv.fieldnames:
        csv.remove_column_by_name('McCulloch')

    csv.write(sys.stdout)


if __name__ == '__main__':
    main()
