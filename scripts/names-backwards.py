#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Find names that have both "First Last" and "Last First" variants.
#

from usernames import SubstitutionMap, sub_from


def standardize_upper_ascii(s):
    s = s.lower()
    s = ''.join(map(lambda c: sub_from(SubstitutionMap, c), s))
    s = s.upper()

    # Uppercase-only changes.
    s = s.replace('İ', 'I')
    return s


# By default, try to establish names that collide, testing the algorithms.
if __name__ == '__main__':
    import os

    import oplcsv

    # Build an index of Names.
    names = set()
    already_printed = set()

    for dirname, subdirs, files in os.walk(os.getcwd()):
        if 'meet-data' in subdirs:
            subdirs[:] = ['meet-data']
        if 'entries.csv' in files:
            filepath = dirname + os.sep + 'entries.csv'
            csv = oplcsv.Csv(filepath)
            nameidx = csv.index('Name')
            for r in csv.rows:
                names.add(r[nameidx])

    for name in names:
        try:
            [first, last] = name.split()
        except ValueError:
            continue

        if name in already_printed:
            continue

        backwards = f'{last} {first}'
        if f'{last} {first}' in names:
            print(name)
            already_printed.add(name)
            already_printed.add(backwards)
