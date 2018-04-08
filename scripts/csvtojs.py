#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:

from oplcsv import Csv
import sys


def isnumber(s):
    try:
        float(s)
        return True
    except ValueError:
        return False

# Given a float in string form like "320.500", return "320.5", to save
# some space.


def trimzeros(s):
    if '.' not in s:
        return s

    s = s.rstrip('0')
    if s[-1] == '.':
        return s[:-1]
    return s


def arrayify(s):
    if len(s) == 0:
        return ''  # JS will treat as undefined.
    if isnumber(s):
        return trimzeros(s)
    return '"%s"' % s


def convert(csv, jsvarname, fd):
    fd.write("'use strict';")

    # Use a module-like pattern.
    fd.write("var %s = (function(){" % jsvarname)
    fd.write("var obj = {};")

    # Blat out constants into the global JS namespace for now.
    for i, field in enumerate(csv.fieldnames):
        fd.write("obj.%s = %s;" % (field.upper(), i))

    fd.write("obj.data = [")

    for row in csv.rows:
        fd.write('[' + ','.join([arrayify(x) for x in row]) + '],')

    fd.write("];")

    # End the module pattern.
    fd.write("return obj;")
    fd.write("}());")


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print(' Usage: %s csvfile jsvarname' % sys.argv[0], file=sys.stderr)
        sys.exit(1)

    (filename, jsvarname) = sys.argv[1:]
    csv = Csv(filename)

    convert(csv, jsvarname, sys.stdout)
