#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Uses existing age to data to estimate ages for all
# lifter meets. Only fills in data if it is consistent.

import datetime

AGE_IDX = 0
MINAGE_IDX = 1
MAXAGE_IDX = 2
DATE_IDX = 3
BY_IDX = 4
BD_IDX = 5

AGE_DIVISIONS = ['5-12', '13-15', '16-17', '18-19', '20-23', '24-34', '35-39',
                 '40-44', '45-49', '50-54', '55-59', '60-64', '65-69', '70-74',
                 '75-79', '80-999']


def get_year(date_string):
    return int(date_string[:4])


def get_monthday(date_string):
    return date_string[5:]


# Check that a lifter has a consistent birthyear
def is_by_consistent(lifter_data):
    global AGE_IDX
    global MINAGE_IDX
    global MAXAGE_IDX
    global DATE_IDX
    global BY_IDX

    if lifter_data[0][AGE_IDX] != '':
        # Want the lower age if age is derived from birthyear
        age = float(lifter_data[0][AGE_IDX]) - \
            (float(lifter_data[0][AGE_IDX]) % 1)
    else:
        age = -1

    # minage from division can be fractional (e.g 39.5, we want the lower age)
    minage = int(lifter_data[0][MINAGE_IDX])
    # maxage from division can be fractional (e.g 17.5, we want the upper age)
    maxage = round(lifter_data[0][MAXAGE_IDX])
    agedate = lifter_data[0][DATE_IDX]
    mindate = lifter_data[0][DATE_IDX]
    maxdate = lifter_data[0][DATE_IDX]

    if len(lifter_data) > 1:
        for age_data in lifter_data[1:]:
            newagedate = age_data[DATE_IDX]
            new_year = get_year(age_data[DATE_IDX])

            new_mindate = age_data[DATE_IDX]
            new_minage = age_data[MINAGE_IDX]
            new_maxdate = age_data[DATE_IDX]
            new_maxage = age_data[MAXAGE_IDX]

            if age_data[AGE_IDX] != '':
                # Want the lower age if age is derived from birthyear
                newage = float(age_data[AGE_IDX]) - \
                    (float(age_data[AGE_IDX]) % 1)
            else:
                newage = -1

            ageyeardiff = new_year - get_year(agedate)
            if get_monthday(newagedate) < get_monthday(agedate):
                ageyeardiff -= 1
            minyeardiff = new_year - get_year(mindate)
            if get_monthday(new_mindate) < get_monthday(mindate):
                minyeardiff -= 1
            maxyeardiff = new_year - get_year(maxdate)
            if get_monthday(new_maxdate) < get_monthday(maxdate):
                maxyeardiff -= 1

            # Check that the age is consistent
            if newage != -1:
                if age != -1 and newage < age + ageyeardiff:
                    return False
                elif newage < minage + minyeardiff:
                    return False
                elif newage > maxage + maxyeardiff + 1:
                    return False

            # Check that the minage is consistent
            if new_minage != 0 and new_minage > maxage + maxyeardiff+1:
                return False

            # Check that the maxage is consistent
            if new_maxage != 999 and new_maxage < minage + minyeardiff:
                return False

            if newage != -1:
                age = newage
                agedate = newagedate
            if new_minage != 0:
                minage = new_minage
                mindate = new_mindate
            if new_maxage != 999:
                maxage = new_maxage
                maxdate = new_maxdate

    return True

# Helper function for calculating lifter ages


def calc_age(birthday, date):
    birthyear = get_year(birthday)
    curr_year = get_year(date)

    # They haven't had their birthday
    if get_monthday(birthday) < get_monthday(date):
        return curr_year - birthyear + 1
    else:  # They've had their birthday
        return curr_year - birthyear


# Checks whether a lifter has a consistent birthday
def is_bd_consistent(lifter_data):
    global AGE_IDX
    global DATE_IDX
    global BD_IDX

    bdageidx = 0
    meetdateidx = 1

    birthday = ''
    for age_data in lifter_data:
        age = age_data[AGE_IDX]
        date = age_data[DATE_IDX]
        curr_birthday = age_data[BD_IDX]

        bd_data = []

        if curr_birthday != '':
            if birthday != '' and curr_birthday != birthday:
                return False
            else:
                birthday = curr_birthday
        # We don't want to check the consistency of age data with a birthday,
        # as they may have had their birthday over the course of the meet
        elif age != '':
            # this is an exact age
            if float(age) % 1 == 0:
                bd_data.append([int(age), date])

    if len(bd_data) != 0:
        # Sort the age data by day and month
        bd_data.sort(key=get_monthday)

        init_year = get_year(bd_data[0][meetdateidx])

        # If we have a BirthDay, check that the data is consistent.
        # If so, offset the age data so it is all from one year
        for age_date in bd_data:
            if (birthday != '' and
                    calc_age(birthday, age_date[meetdateidx]) != age_date[bdageidx]):
                return False

            curr_year = get_year(age_date[meetdateidx])
            age_date[bdageidx] += init_year - curr_year

        # Check that the age data is still sorted by age,
        # if not the birthdate isn't consistent
        for ii in range(1, len(bd_data)):
            if bd_data[ii-1][bdageidx] > bd_data[ii][bdageidx]:
                return False

    return True


# Gives the range that a lifters birthday lies in
def estimate_birthdate(lifter_data):
    global AGE_IDX
    global DATE_IDX
    global BY_IDX
    global BD_IDX
    bdageidx = 0
    meetdateidx = 1
    # If a lifter has their birthday over the length of the meet we don't know
    max_meetlength = 12

    min_date = ''
    max_date = ''
    bd_data = []
    for age_data in lifter_data:
        age = age_data[AGE_IDX]
        date = age_data[DATE_IDX]

        # If we have a birthday recorded just use that
        if age_data[BD_IDX] != '':
            return [age_data[BD_IDX], age_data[BD_IDX]]

        # this is an exact age
        if age != '' and float(age) % 1 == 0:
            bd_data.append([int(age), date])

    if len(bd_data) > 1:
        # Sort the age data by day and month
        bd_data.sort(key=get_monthday)

        init_year = get_year(bd_data[0][meetdateidx])

        # Offset the age data so it is all from one year
        for age_date in bd_data[1:]:
            curr_year = get_year(age_date[meetdateidx])
            age_date[0] += init_year - curr_year

        min_date = get_monthday(bd_data[0][meetdateidx])
        max_date = get_monthday(bd_data[0][meetdateidx])
        has_had_bd = False
        lower_age = bd_data[0][bdageidx]

        for age_date in bd_data[1:]:
            if age_date[bdageidx] == lower_age:
                min_date = get_monthday(age_date[meetdateidx])
            elif age_date[bdageidx] == lower_age + 1:
                max_date = get_monthday(age_date[meetdateidx])
                has_had_bd = True

                break

        # We can't estimate a birthdate
        if not has_had_bd:
            return []

        else:  # We've managed to bound the birthdate
            by = init_year - (lower_age + 1)
            min_date = str(by)+'-'+min_date
            max_datetime = datetime.date(
                by, int(max_date.split('-')[0]), int(max_date.split('-')[1]))

            fuzzed_end = max_datetime + datetime.timedelta(days=max_meetlength)

            return [min_date, fuzzed_end.strftime("%Y-%m-%d")]
    else:  # No recorded ages, can't estimate a birthdate
        return []


# Gets the range where we know that the lifter hasn't had a birthday
# this function assumes that there are no years where we see the lifter at different ages
def get_known_range(lifter_data):
    global AGE_IDX
    global DATE_IDX
    bdageidx = 0
    meetdateidx = 1

    min_date = ''
    max_date = ''
    bd_data = []
    for age_data in lifter_data:
        age = age_data[AGE_IDX]
        date = age_data[DATE_IDX]

        # this is an exact age
        if age != '' and float(age) % 1 == 0:
            bd_data.append([int(age), date])

    if len(bd_data) > 1:
        # Sort the age data by day and month
        bd_data.sort(key=get_monthday)

        init_year = get_year(bd_data[0][meetdateidx])

        # Offset the age data so it is all from one year
        for age_date in bd_data[1:]:
            curr_year = get_year(age_date[meetdateidx])
            age_date[bdageidx] += init_year - curr_year

        min_date = get_monthday(bd_data[0][meetdateidx])
        max_date = get_monthday(bd_data[0][meetdateidx])

        for age_date in bd_data[1:]:
            new_date = get_monthday(age_date[meetdateidx])
            if new_date < min_date:
                min_date = new_date
            elif new_date > max_date:
                max_date = new_date

        return [max_date, min_date]
    elif len(bd_data) == 1:
        return [get_monthday(bd_data[0][meetdateidx]),
                get_monthday(bd_data[0][meetdateidx])]
    else:
        return []


def add_birthyears(lifter_data):
    global AGE_IDX
    global MINAGE_IDX
    global MAXAGE_IDX
    global DATE_IDX
    global BY_IDX

    # First check if a birthyear is listed
    if any(age_data[BY_IDX] != '' for age_data in lifter_data):
        by = [age_data[BY_IDX]
              for age_data in lifter_data if age_data[BY_IDX] != ''][0]
        for age_data in lifter_data:
            age_data[BY_IDX] = by

    # If we don't have a birthyear, check whether we can see an age change over a year
    else:
        bd_range = estimate_birthdate(lifter_data)
        if bd_range != []:
            by = bd_range[0].split('-')[0]
            for age_data in lifter_data:
                age_data[BY_IDX] = by


def interpolate_lifter(lifter_data):
    global AGE_IDX
    global MINAGE_IDX
    global MAXAGE_IDX
    global DATE_IDX
    global BY_IDX

    if len(lifter_data) > 1:
        # This needs to be called first as we are
        # replacing some of the .5 ages with exact ages below
        add_birthyears(lifter_data)

        bd_range = estimate_birthdate(lifter_data)

        if bd_range != []:  # Then we have a birthday range and can be semi-accurate
            for age_data in lifter_data:
                if age_data[AGE_IDX] == '' or float(age_data[AGE_IDX]) % 1 == 0.5:
                    curr_year = get_year(age_data[DATE_IDX])
                    curr_monthday = get_monthday(age_data[DATE_IDX])

                    # Then we know their exact age at this time
                    if (calc_age(bd_range[0], age_data[DATE_IDX])
                            == calc_age(bd_range[1], age_data[DATE_IDX])):
                        age_data[AGE_IDX] = calc_age(
                            bd_range[0], age_data[DATE_IDX])
                        age_data[MINAGE_IDX] = age_data[AGE_IDX]
                        age_data[MAXAGE_IDX] = age_data[AGE_IDX]
                    else:  # We're not sure if they've had their birthday
                        age_data[AGE_IDX] = curr_year - \
                            get_year(bd_range[0])-0.5
                        age_data[MINAGE_IDX] = int(age_data[AGE_IDX]-0.5)
                        age_data[MAXAGE_IDX] = int(age_data[AGE_IDX]+0.5)
        else:  # We have only birthyears, a single age or only divisions

            by = 0
            approx_by = 0
            min_by = 0
            max_by = 9999
            known_range = []
            # Extract all the birthyear information possible

            for age_data in lifter_data:
                curr_year = get_year(age_data[DATE_IDX])
                # Then we have an age derived from birthyear
                if age_data[AGE_IDX] != '':
                    if float(age_data[AGE_IDX]) % 1 == 0.5:
                        by = curr_year - int((float(age_data[AGE_IDX])+0.5))
                    else:
                        approx_by = curr_year-int(age_data[AGE_IDX])

                # Find the tighest bounds given by divisions
                if max_by != 9999 and curr_year - age_data[MINAGE_IDX] < max_by:
                    max_by = curr_year - int(age_data[MINAGE_IDX])

                if min_by != 0 and curr_year - age_data[MAXAGE_IDX] > min_by:
                    min_by = curr_year - int(age_data[MAXAGE_IDX])

            # Then the division information let's us have an exact birthyear
            if min_by > approx_by:
                by = min_by
            elif max_by < approx_by:
                by = max_by

            # If we have at least one exact age,
            # then we have a range in which we know they don't have a birthday
            if approx_by != 0:
                known_range = get_known_range(lifter_data)

            # First deal with the case when we have a birthyear
            if by != 0:
                for age_data in lifter_data:
                    curr_year = get_year(age_data[DATE_IDX])
                    curr_monthday = get_monthday(age_data[DATE_IDX])
                    if age_data[AGE_IDX] == '' or float(age_data[AGE_IDX]) % 1 == 0.5:
                        if known_range == []:
                            age_data[AGE_IDX] = curr_year-by-0.5
                            age_data[MINAGE_IDX] = int(age_data[AGE_IDX]-0.5)
                            age_data[MAXAGE_IDX] = int(age_data[AGE_IDX]+0.5)
                        else:
                            # Check whether known_range is an upper
                            # or lower bound on the birthday
                            lower_bound = False
                            if approx_by < by:
                                lower_bound = True

                            # Then the lifter hasn't had their birthday yet
                            if lower_bound and curr_monthday <= known_range[1]:
                                age_data[AGE_IDX] = curr_year - by - 1
                                age_data[MINAGE_IDX] = age_data[AGE_IDX]
                                age_data[MAXAGE_IDX] = age_data[AGE_IDX]
                            # Then we're not sure if they've had their birthday
                            elif lower_bound and curr_monthday > known_range[1]:
                                age_data[AGE_IDX] = curr_year-by-0.5
                                age_data[MINAGE_IDX] = int(
                                    age_data[AGE_IDX]-0.5)
                                age_data[MAXAGE_IDX] = int(
                                    age_data[AGE_IDX]+0.5)
                            # Then the lifter has had their birthday
                            elif curr_monthday >= known_range[0]:
                                age_data[AGE_IDX] = curr_year-by
                                age_data[MINAGE_IDX] = age_data[AGE_IDX]
                                age_data[MAXAGE_IDX] = age_data[AGE_IDX]
                            # Then we're not sure if they've had their birthday
                            else:
                                age_data[AGE_IDX] = curr_year-by-0.5
                                age_data[MINAGE_IDX] = int(
                                    age_data[AGE_IDX]-0.5)
                                age_data[MAXAGE_IDX] = int(
                                    age_data[AGE_IDX]+0.5)

            # Then deal with the case where we have an age
            # and the division information doesn't give the birthyear
            elif approx_by != 0:

                # Assign upper and lower age bounds based on approximate birthyear
                for age_data in lifter_data:
                    curr_year = get_year(age_data[DATE_IDX])
                    curr_monthday = get_monthday(age_data[DATE_IDX])
                    if age_data[AGE_IDX] == '':
                        if curr_monthday < known_range[0]:
                            age_data[AGE_IDX] = curr_year - approx_by - 0.5
                            age_data[MINAGE_IDX] = curr_year - approx_by - 1
                            age_data[MAXAGE_IDX] = curr_year - approx_by
                        elif curr_monthday > known_range[1]:
                            age_data[AGE_IDX] = curr_year - approx_by + 0.5
                            age_data[MINAGE_IDX] = curr_year - approx_by
                            age_data[MAXAGE_IDX] = curr_year - approx_by + 1
                        # We know an exact age for this date
                        elif (curr_monthday >= known_range[0]
                              and curr_monthday <= known_range[1]):
                            age_data[AGE_IDX] = curr_year - approx_by
                            age_data[MINAGE_IDX] = curr_year - approx_by
                            age_data[MAXAGE_IDX] = curr_year - approx_by

            # Finally deal with the only division case
            else:
                # Set age bounds based on divisions
                for age_data in lifter_data:
                    curr_year = get_year(age_data[DATE_IDX])
                    if min_by != 0:
                        age_data[MINAGE_IDX] = curr_year - min_by - 1
                    if max_by != 9999:
                        age_data[MAXAGE_IDX] = curr_year - max_by

    return lifter_data


def interpolate_ages(LifterAgeHash, MeetDateHash):
    global AGE_IDX
    global MINAGE_IDX
    global MAXAGE_IDX
    global DATE_IDX
    global BY_IDX

    for lifter in LifterAgeHash:
        # Create an array of age data sorted by date
        lifter_data = []
        for age_data in LifterAgeHash[lifter]:
            # Replace the meet ID with the meet date and append the meet ID to the end
            lifter_data.append(age_data[:DATE_IDX]+[MeetDateHash[age_data[DATE_IDX]]]
                               + age_data[DATE_IDX+1:] + [age_data[DATE_IDX]])

        lifter_data.sort(key=lambda x: x[DATE_IDX])
        if is_by_consistent(lifter_data) and is_bd_consistent(lifter_data):
            lifter_data = interpolate_lifter(lifter_data)
        # Sort by meet ID
        lifter_data.sort(key=lambda x: x[-1])

        # Put this data back into the hashmap
        for ii in range(len(LifterAgeHash[lifter])):
            LifterAgeHash[lifter][ii][AGE_IDX] = lifter_data[ii][AGE_IDX]
            LifterAgeHash[lifter][ii][MINAGE_IDX] = lifter_data[ii][MINAGE_IDX]
            LifterAgeHash[lifter][ii][MAXAGE_IDX] = lifter_data[ii][MAXAGE_IDX]

            LifterAgeHash[lifter][ii][BY_IDX] = lifter_data[ii][BY_IDX]

    return LifterAgeHash


def generate_hashmap(entriescsv, meetcsv):
    # Hashtable for lifter age-data lookup
    # int -> [(str, int, int, int),....] LifterID -> Array of (Age,MinAge,MaxAge,MeetID).
    LifterAgeHash = {}
    # Hashtable for looking up meet-dates from IDs, int -> str
    MeetDateHash = {}

    lifterIDidx = entriescsv.index('LifterID')
    ageidx = entriescsv.index('Age')
    meetIDidx = entriescsv.index('MeetID')
    ageclassidx = entriescsv.index('AgeClass')
    byidx = entriescsv.index('BirthYear')
    bdidx = entriescsv.index('BirthDay')

    for row in entriescsv.rows:
        lifterID = int(row[lifterIDidx])
        age = row[ageidx]
        meetID = int(row[meetIDidx])

        birthyear = ''
        birthday = ''
        if row[byidx] != '':
            birthyear = int(row[byidx])
        if row[bdidx] != '':
            birthyear = int(row[bdidx].split('-')[0])
            birthday = row[bdidx]

        minage = 0
        maxage = 999

        if age == '' and row[ageclassidx] != '':
            [minage_str, maxage_str] = row[ageclassidx].split('-')
            minage = float(minage_str)
            maxage = float(maxage_str)
        elif age != '':
            if float(age) != float(age) % 1:
                minage = int(float(age))
                maxage = int(float(age)) + 1
            else:
                minage = int(age)
                maxage = int(age)

        if lifterID not in LifterAgeHash:
            LifterAgeHash[lifterID] = [
                [age, minage, maxage, meetID, birthyear, birthday]]
        else:
            LifterAgeHash[lifterID].append(
                [age, minage, maxage, meetID, birthyear, birthday])

    meetIDidx = meetcsv.index('MeetID')
    dateidx = meetcsv.index('Date')

    for row in meetcsv.rows:
        date = row[dateidx]
        meetID = int(row[meetIDidx])

        MeetDateHash[meetID] = date

    return [LifterAgeHash, MeetDateHash]


def get_ageclass(minage, maxage, yearage=None):
    global AGE_DIVISIONS

    for division in AGE_DIVISIONS:
        div_min = int(division.split('-')[0])
        div_max = int(division.split('-')[1])

        # Base division off the age they are turning that year, if we know it
        if yearage is not None:
            if yearage <= div_max and yearage >= div_min:
                return division
        elif maxage <= div_max and minage >= div_min:
            return division

    return ''


# Checks that the gaps between meets are reasonable, if not
# we don't write this data to the csv
def check_age_spacing(lifterdata):
    # If there is more than a 5 year gap between age data, it's probably two lifters
    max_gap = 5

    ages = [float(x[0]) for x in lifterdata if x[0] != '']
    if len(ages) > 1:
        ages.sort()
        diffs = [ages[ii + 1] - ages[ii] for ii in range(len(ages)-1)]
        if max(diffs) > max_gap:
            return False

    return True


# Adds the interpolated ages back to the csv file
def update_csv(entriescsv, MeetDateHash, LifterAgeHash):
    global AGE_IDX
    global MINAGE_IDX
    global MAXAGE_IDX
    global DATE_IDX
    global BY_IDX

    lifterIDidx = entriescsv.index('LifterID')
    ageidx = entriescsv.index('Age')
    meetIDidx = entriescsv.index('MeetID')

    if 'AgeClass' not in entriescsv.fieldnames:
        entriescsv.append_column('AgeClass')

    ageclassidx = entriescsv.index('AgeClass')

    for row in entriescsv.rows:
        lifterID = row[lifterIDidx]
        meetID = row[meetIDidx]

        if check_age_spacing(LifterAgeHash[int(lifterID)]):

            for age_data in LifterAgeHash[int(lifterID)]:
                if age_data[DATE_IDX] == int(meetID):
                    # The age that a lifter is turning that year
                    yearage = None
                    if age_data[BY_IDX] != '':
                        yearage = int(MeetDateHash[age_data[DATE_IDX]].split(
                            '-')[0]) - int(age_data[BY_IDX])

                    assert age_data[AGE_IDX] == '' or float(age_data[AGE_IDX]) > 3.5

                    row[ageidx] = str(age_data[AGE_IDX])
                    if row[ageclassidx] != '':
                        [oldmin, oldmax] = row[ageclassidx].split('-')

                        # Deal with the case where the divisions tell us the birthyear
                        if (float(oldmin) - age_data[MINAGE_IDX]) == 0.5:
                            row[ageclassidx] = str(get_ageclass(
                                float(oldmin), age_data[MAXAGE_IDX], yearage))
                        elif (float(oldmax) - age_data[MAXAGE_IDX]) == 0.5:
                            row[ageclassidx] = str(get_ageclass(
                                age_data[MINAGE_IDX], float(oldmax), yearage))
                        else:
                            row[ageclassidx] = str(get_ageclass(
                                age_data[MINAGE_IDX], age_data[MAXAGE_IDX], yearage))
                    else:
                        row[ageclassidx] = str(get_ageclass(
                            age_data[MINAGE_IDX], age_data[MAXAGE_IDX], yearage))
                    break
        elif row[ageclassidx] != '':  # Make sure all lifter data has a standard AgeClass
            [minage, maxage] = row[ageclassidx].split('-')
            row[ageclassidx] = str(get_ageclass(
                float(minage), float(maxage), None))

    return entriescsv


def interpolate(entriescsv, meetcsv):
    [LifterAgeHash, MeetDateHash] = generate_hashmap(entriescsv, meetcsv)
    LifterAgeHash = interpolate_ages(LifterAgeHash, MeetDateHash)

    return update_csv(entriescsv, MeetDateHash, LifterAgeHash)
