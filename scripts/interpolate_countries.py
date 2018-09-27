#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Uses existing data to add country data to a lifter
# This is a super basic implementation, since we'll be writing the final version in Rust.

COUNTRY_IDX = 0
MEETID_IDX = 1

# Check that a lifter only has one country


def is_country_consistent(lifter_data):
    country = ''
    for entry in lifter_data:
        if lifter_data[COUNTRY_IDX] != '':
            if country != '' and lifter_data[COUNTRY_IDX] != country:
                return False

            country = lifter_data[COUNTRY_IDX]

    return True

# Get a lifters country if they have one


def get_country(lifter_data):
    global COUNTRY_IDX

    for entry in lifter_data:
        if entry[COUNTRY_IDX] != '':
            return entry[COUNTRY_IDX]

    return ''


def interpolate_countries(LifterCountryHash):
    global COUNTRY_IDX
    global MEETID_IDX
    newLifterHash = {}

    for lifter in LifterCountryHash:
        if is_country_consistent(LifterCountryHash[lifter]):
            country = get_country(LifterCountryHash[lifter])
            newLifterHash[lifter] = LifterCountryHash[lifter]

            for entry in newLifterHash[lifter]:
                entry[COUNTRY_IDX] = country

    return newLifterHash


def generate_hashmap(entriescsv):
    # Hashtable for lifter country data lookup
    # int -> [(str, int),....] LifterID -> Array of (Country,MeetID).
    LifterCountryHash = {}

    lifterIDidx = entriescsv.index('LifterID')
    meetIDidx = entriescsv.index('MeetID')
    countryidx = entriescsv.index('Country')

    for row in entriescsv.rows:
        lifterID = int(row[lifterIDidx])
        country = row[countryidx]
        meetID = int(row[meetIDidx])

        if lifterID not in LifterCountryHash:
            LifterCountryHash[lifterID] = [[country, meetID]]
        else:
            LifterCountryHash[lifterID].append([country, meetID])

    return LifterCountryHash


# Adds the interpolated countries back to the csv file
def update_csv(entriescsv, LifterCountryHash):
    global COUNTRY_IDX
    global MEETID_IDX

    lifterIDidx = entriescsv.index('LifterID')
    countryidx = entriescsv.index('Country')

    for row in entriescsv.rows:
        lifterID = int(row[lifterIDidx])
        if lifterID in LifterCountryHash:
            row[countryidx] = LifterCountryHash[int(lifterID)][0][COUNTRY_IDX]

    return entriescsv


def interpolate(entriescsv):
    LifterCountryHash = generate_hashmap(entriescsv)
    LifterCountryHash = interpolate_countries(LifterCountryHash)

    return update_csv(entriescsv, LifterCountryHash)
