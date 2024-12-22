#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Standardize the original.csv to the OpenPowerlifting
# internal format.
#

import os
import sys

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


# Some strange UTF-8 space rune that shows up occasionally.
weirdspace = '  '


def error(msg):
    print("Error: %s" % msg, file=sys.stderr)
    sys.exit(1)


def strip_whitespace(csv):
    for i, x in enumerate(csv.fieldnames):
        csv.fieldnames[i] = x.strip().replace('  ', ' ')

    for row in csv.rows:
        for i, x in enumerate(row):
            row[i] = x.strip().replace('  ', ' ')


def remove_empty_rows(csv):
    def getemptyidx(csv):
        for i, row in enumerate(csv.rows):
            if ''.join(row) == '':
                return i
        return -1

    while True:
        idx = getemptyidx(csv)
        if idx == -1:
            return
        del csv.rows[idx]


def remove_rows_by_empty_column(csv, colname):
    def getemptyidx(csv, colidx):
        for i, row in enumerate(csv.rows):
            if row[colidx] == '':
                return i
        return -1

    colidx = csv.index(colname)
    if colidx < 0:
        error("Column %s not found in remove_rows_by_empty_column()." % colname)

    while True:
        idx = getemptyidx(csv, colidx)
        if idx == -1:
            return
        del csv.rows[idx]


# Helper for detecting events, used by thread_event_state() and
# remove_event_rows().
def str_to_event(s):
    s = s.lower()
    if s == 'full power' or s == 'powerlifting':
        return 'SBD'
    if s == 'squat' or s == 'squat only':
        return 'S'
    if s == 'bench' or s == 'bench only':
        return 'B'
    if s == 'deadlift' or s == 'dl' or s == 'deadlift only' or s == 'dl only':
        return 'D'
    if s == 'ironman' or ('push' in s and 'pull' in s):
        return 'BD'
    return None


# Mark events by keeping track of Event state.
def thread_event_state(csv):
    assert 'Event' not in csv.fieldnames
    csv.append_column('Event')
    evtidx = csv.index('Event')

    state = 'SBD'

    for row in csv.rows:
        evt = str_to_event(row[0])
        if evt:
            state = evt
        else:
            row[evtidx] = state


# Removes rows that just contain things like "Bench" and "Ironman".
def remove_event_rows(csv):
    def geteventidx(csv):
        for i, row in enumerate(csv.rows):
            for k in row:
                if str_to_event(k):
                    return i
        return -1

    while True:
        idx = geteventidx(csv)
        if idx == -1:
            return
        del csv.rows[idx]


def remove_stars(csv):
    for row in csv.rows:
        for i, x in enumerate(row):
            row[i] = x.replace('*', '').replace("†", "")


def remove_zeros(csv):
    for row in csv.rows:
        for i, x in enumerate(row):
            # The CSV conversion via LibreOffice already standardized
            # all decimal forms of 0.00 and such to just '0'.
            if x == '0':
                row[i] = ''


def standardize_fieldnames(csv):

    # If the first field is "Full Power", we can guess at some
    # of the other fieldnames, which are usually left blank.
    s = csv.fieldnames[0].lower()
    if 'full power' in s or 'saturday' in s or 'ironman' in s or 'push' in s:
        # The next 6 fieldnames must be empty.
        if ''.join(csv.fieldnames[1:6]) != '':
            error("Field names expected to be blank, but aren't!")

        csv.fieldnames[0] = 'Sex'
        csv.fieldnames[1] = 'Class'
        csv.fieldnames[2] = 'Equipment'
        csv.fieldnames[3] = 'Division'
        csv.fieldnames[4] = 'WeightClassLBS'
        csv.fieldnames[5] = 'Name'

    for i, x in enumerate(csv.fieldnames):
        s = x.lower()
        if s == 'gender':
            x = 'Sex'
        elif s == 'class':
            x = 'Class'  # TODO: What to do with this
        elif s == 'eq division':
            x = 'Equipment'
        elif s == 'age division':
            x = 'Division'
        elif s == 'wt' or s == 'wt class':
            x = 'WeightClassLBS'
        elif s == 'name' or s == 'powerlifter' or s == 'lifter name' or s == 'lifter':
            x = 'Name'
        elif s == 'actual bwt.' or s == 'acutal b/w' or s == 'bwt' or s == 'actual bwt':
            x = 'BodyweightLBS'
        elif s == 'act. bwt' or s == 'act bwt' or s == 'btw' or s == 'act wt':
            x = 'BodyweightLBS'
        elif s == 'squat' or s == 'bestsquatlbs' or s == 'squart' or s == 'sq':
            x = 'Best3SquatLBS'
        elif s == 'squat lbs' or s == 'squatlbs':
            x = 'Best3SquatLBS'
        elif s == 'bench' or s == 'bestbenchlbs' or s == 'bp':
            x = 'Best3BenchLBS'
        elif s == 'bench lbs' or s == 'benchlbs':
            x = 'Best3BenchLBS'
        elif s == 'deadlift' or s == 'bestdeadliftlbs' or s == 'dl':
            x = 'Best3DeadliftLBS'
        elif s == 'deadlift lbs' or s == 'deadliftlbs':
            x = 'Best3DeadliftLBS'
        elif s == 'total' or s == 'totallbs':
            x = 'TotalLBS'
        elif s == 'place':
            x = 'Place'

        elif s == 'squat4lbs':
            x = 'Squat4LBS'
        elif s == 'bench4lbs':
            x = 'Bench4LBS'
        elif s == 'deadlift4lbs':
            x = 'Deadlift4LBS'

        elif s == 'event':
            x = 'Event'
        elif s == 'sex':
            x = 'Sex'
        elif s == 'class':
            x = 'Class'
        elif s == 'equipment' or s == 'equip':
            x = 'Equipment'
        elif s == 'division':
            x = 'Division'
        elif s == 'weightclasslbs' or s == 'wclass':
            x = 'WeightClassLBS'
        elif s == 'weightclasskg':
            x = 'WeightClassKg'
        elif s == 'bodyweightlbs':
            x = 'BodyweightLBS'
        elif s == 'bodyweightkg':
            x = 'BodyweightKg'
        elif s == 'bestsquatkg':
            x = 'Best3SquatKg'
        elif s == 'bestbenchkg' or s == 'best bench':
            x = 'Best3BenchKg'
        elif s == 'bestdeadliftkg':
            x = 'Best3DeadliftKg'
        elif s == 'totalkg':
            x = 'TotalKg'
        elif s == 'squat4kg':
            x = 'Squat4Kg'
        elif s == 'bench4kg':
            x = 'Bench4Kg'
        elif s == 'deadlift4kg':
            x = 'Deadlift4Kg'
        elif s == 'state' or s == 'st':
            x = 'State'
        elif s == 'country':
            x = 'Country'
        elif s == 'age':
            x = 'Age'
        elif s == 'coef.' or s == 'total coef.':
            x = 'DELETEME'
        else:
            error("Teach me what to do with column '%s'" % x)

        csv.fieldnames[i] = x


def fixequipment(csv):
    idx = csv.index('Equipment')
    if idx < 0:
        error("No Equipment column in fixequipment().")

    for i, row in enumerate(csv.rows):
        s = row[idx].lower().replace(weirdspace, ' ')
        # Override -- sometimes RPS specifies sleeves for our benefit.
        if 'sleeve' in s:
            x = 'Raw'

        # "Classic" means belt and wrist wraps only to the RPS.
        elif (s == 'raw classic' or s == 'raw clas' or s == 'raw class' or
                s == 'raw c' or s == 'rc'):
            x = 'Bare'
        # This one below looks the same, but actually has some unicode BS going
        # on.
        elif s == 'raw clas' or s == 'raw classic' or s == 'raw' or s == 'raw -u':
            x = 'Bare'
        elif s == 'raw – u' or 'classic' in s or s == 'raw calssic':
            x = 'Bare'

        # "Modern" allows wraps and sleeves for the squat and maybe the deadlift.
        # They track rankings for sleeves independently, but don't mark that
        # anywhere.
        elif s == 'raw modern' or s == 'raw mod' or s == 'raw m' or s == 'rm':
            x = 'Wraps'
        elif 'modern' in s:
            x = 'Wraps'

        elif s == 'single-ply' or s == 'single ply' or s == 'sp' or s == 'singleply':
            x = 'Single-ply'
        elif 'single' in s or s == 'sin ply' or s == 'singly-ply':
            x = 'Single-ply'

        elif s == 'multi-ply' or s == 'mulit-ply' or s == 'multi ply' or s == 'eq':
            x = 'Multi-ply'
        elif s == 'multiply' or s == 'mp' or s == 'multy ply' or s == 'gear':
            x = 'Multi-ply'
        elif 'multi' in s or 'multy' in s:
            x = 'Multi-ply'

        elif s == 'adaptive athlete':
            x = 'Raw'  # I guess

        # This is used for benchers who have to choose between Raw or Wraps.
        # Seriously, though, what the hell?
        elif s == 'no choice made':
            x = 'Wraps'

        elif s == 'unlimited':
            x = 'Unlimited'

        elif s == '' and i > 0:
            x = csv.rows[i - 1][idx]

        else:
            print("Full row: %s" % ','.join(row), file=sys.stderr)
            error("Teach fixequipment() what to do with '%s'" % s)

        row[idx] = x


def fixsex(csv):
    idx = csv.index('Sex')
    if idx < 0:
        error("No Sex column in fixsex().")

    for i, row in enumerate(csv.rows):
        s = row[idx].lower()
        if s == 'male' or s == 'm' or 'men' in s:
            x = 'M'
        elif s == 'female' or s == 'f' or 'women' in s or s == 'femaie':
            x = 'F'
        elif s == '' and i > 0:
            x = csv.rows[i - 1][idx]
        else:
            error("Teach fixsex() what to do with '%s'" % s)

        row[idx] = x


def fixname(csv):
    idx = csv.index('Name')
    if idx < 0:
        error("No Name column in fixname().")

    for row in csv.rows:
        s = row[idx]
        s = s.replace('Jr.', 'Jr')
        s = s.replace('Sr.', 'Sr')

        # RPS uses this to look fancy in names like O'Brien.
        s = s.replace("’", "'")

        # This is used to denote someone who entered amateur but totaled pro.
        s = s.replace("†", "")

        # The first argument is some weird unicode space that keeps
        # creeping into the name field.
        s = s.replace("  ", " ")
        s = s.replace("  ", " ")  # A second kind of weird space.
        s = s.replace(" ", " ")  # Yet more more space.

        # I'm not sure what this means, but it shows up in names.
        s = s.replace("NSB", "")

        row[idx] = s.strip()


def fixclass(csv):
    idx = csv.index('Class')
    if idx < 0:
        error("No Class column in fixclass().")

    for i, row in enumerate(csv.rows):
        s = row[idx].lower()
        if s == 'pro':
            x = 'Pro'
        elif s == 'am' or s == 'amateur' or s == 'ameteur' or s == 'am af':
            x = 'Amateur'
        elif s == 'elite' or s == 'elite am' or s == 'el am':
            x = 'Elite'
        elif s == 'military am' or s == 'military' or s == 'am military':
            x = 'Military Amateur'
        elif s == 'mil am' or s == 'military amateur':
            x = 'Military Amateur'
        elif s == 'police am' or s == 'pol am' or s == 'police':
            x = 'Police Amateur'
        elif s == 'police pro' or s == 'polce pro' or s == 'pol pro':
            x = 'Police Pro'
        elif (s == 'military pro' or s == 'mil pro' or s == 'pro mil' or
                s == 'militarypro' or s == 'mil p' or s == 'pro military'):
            x = 'Military Pro'
        elif s == 'crossfit' or s == 'cross fit' or s == 'crossift':
            x = 'Crossfit'
        elif s == 'police/fire am' or s == 'am police / fire' or s == 'police / fire am':
            x = 'Police-Fire Amateur'
        elif s == 'police /fire am':
            x = 'Police-Fire Amateur'
        elif s == 'pro police/fire' or s == 'police /fire pro':
            x = 'Police-Fire Pro'
        elif s == 'police/ fire pro' or s == 'police / fire pro':
            x = 'Police-Fire Pro'
        elif s == 'fire':
            x = 'Fire Amateur'
        elif s == 'am police/fire':
            x = 'Police-Fire Amateur'
        elif s == 'police fire am':
            x = 'Police-Fire Amateur'
        elif s == 'armed forc pro':
            x = 'Armed Forces Pro'
        elif s == 'armed forc am' or s == "armed forces am":
            x = 'Armed Forces Amateur'
        elif s == 'junior pro':
            x = 'Junior Pro'
        elif s == 'pro/ police fire' or s == 'police/fire pro':
            x = 'Police-Fire Pro'
        elif s == 'pol/fire':
            x = 'Police-Fire'
        elif s == 'crossfit am':
            x = 'Crossfit Amateur'
        elif s == 'adaptive athlete' or s == 'aai':
            x = "Adaptive"

        elif s == '' and i > 0:
            x = csv.rows[i - 1][idx]

        else:
            error("Teach fixclass() what to do with '%s'" % s)

        row[idx] = x


def fixweightclass(csv):
    idx = csv.index('WeightClassLBS')
    if idx < 0:
        error("No WeightClassLBS column in fixweightclass().")
    sexidx = csv.index('Sex')
    if sexidx < 0:
        error("No Sex column in fixweightclass().")

    for i, row in enumerate(csv.rows):
        if row[idx].lower() == 'shw' or row[idx].lower() == 'swh':
            if row[sexidx] == 'M':
                row[idx] = '308+'
            elif row[sexidx] == 'F':
                row[idx] = '242+'
            else:
                error("Unknown sex in fixweightclass(): '%s'" % row[sexidx])
        elif row[idx] == '' and i > 0:
            row[idx] = csv.rows[i - 1][idx]


def fixdivision(csv):
    idx = csv.index('Division')
    nameidx = csv.index('Name')
    for i, row in enumerate(csv.rows):
        if i > 0 and row[nameidx] and not row[idx]:
            row[idx] = csv.rows[i - 1][idx]
        # Use plural Masters/Submasters
        row[idx] = row[idx].replace('aster ', 'asters ')
        row[idx] = row[idx].replace(
            'Junior ', 'Juniors ')  # Use plural Juniors


def isnumber(s):
    try:
        float(s)
        return True
    except ValueError:
        return False


def fixplace(csv):
    idx = csv.index('Place')
    if idx < 0:
        error("No Place column in fixplace().")

    for row in csv.rows:
        x = row[idx]
        x = x.replace('pl', '')
        x = x.replace('st', '')
        x = x.replace('nd', '')
        x = x.replace('rd', '')
        x = x.replace('th', '')
        x = x.strip()

        if not isnumber(x) and x != 'DQ':
            error("Unknown place in fixplace(): '%s'" % x)

        row[idx] = x


def fixlift(csv, liftfield):
    idx = csv.index(liftfield)
    if idx < 0:
        error("No %s column in fixlift()." % liftfield)

    # Get rid of things like 'bomb' and 'no lift'.
    for row in csv.rows:
        if not isnumber(row[idx]):
            row[idx] = ''


# Fixes things like bench-only lifters where TotalLBS != Best3BenchLBS.
def fixtotals(csv):
    if 'Event' not in csv.fieldnames:
        return
    if 'TotalLBS' not in csv.fieldnames:
        return

    for row in csv.rows:
        # Only fill in blank cells.
        if row[csv.index('TotalLBS')]:
            continue

        total = 0.0
        event = row[csv.index('Event')]
        if 'S' in event and row[csv.index('Best3SquatLBS')]:
            total += float(row[csv.index('Best3SquatLBS')])
        if 'B' in event and row[csv.index('Best3BenchLBS')]:
            total += float(row[csv.index('Best3BenchLBS')])
        if 'D' in event and row[csv.index('Best3DeadliftLBS')]:
            total += float(row[csv.index('Best3DeadliftLBS')])

        if total != 0.0:
            row[csv.index('TotalLBS')] = "{:.2f}".format(total)


def mergeclassintodivision(csv):
    classidx = csv.index('Class')
    if classidx < 0:
        error("No Class column in mergeclassintodivision().")
    dividx = csv.index('Division')
    if dividx < 0:
        error("No Division column in mergeclassintodivision().")

    for row in csv.rows:
        x = "%s %s" % (row[classidx], row[dividx])
        row[dividx] = x.strip()

    csv.remove_column_by_index(classidx)


def main(filename):
    csv = Csv(filename)

    remove_stars(csv)
    strip_whitespace(csv)
    remove_empty_rows(csv)
    thread_event_state(csv)
    remove_event_rows(csv)
    remove_zeros(csv)
    standardize_fieldnames(csv)

    # Enable as temporary fix for broken files.
    # remove_rows_by_empty_column(csv, "Sex")

    if 'Name' not in csv.fieldnames:
        error("Couldn't find a name column.")
    if 'Equipment' not in csv.fieldnames:
        error("Couldn't find an equipment column.")
    if 'Sex' not in csv.fieldnames:
        error("Couldn't find a sex column.")

    fixdivision(csv)
    fixequipment(csv)
    fixsex(csv)
    fixname(csv)
    fixclass(csv)

    # Depends on fixsex(), since it needs sex information for SHWs.
    if 'WeightClassLBS' in csv.fieldnames:
        fixweightclass(csv)

    # Depends on fixclass(), since it merges class into division.
    if 'Class' in csv.fieldnames and 'Division' in csv.fieldnames:
        mergeclassintodivision(csv)

    for f in csv.fieldnames:
        if 'Squat' in f or 'Bench' in f or 'Deadlift' in f or 'Total' in f:
            fixlift(csv, f)

    fixtotals(csv)

    if 'Place' in csv.fieldnames:
        fixplace(csv)

    csv.append_column('BirthDate')

    csv.write(sys.stdout)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s original.csv" % sys.argv[0], file=sys.stderr)
        sys.exit(1)
    main(sys.argv[1])
