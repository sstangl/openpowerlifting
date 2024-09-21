#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# The list of meets is manually sourced in URLLIST.
# Given that list, determine which meets haven't been entered yet.


import sys
import os
from os.path import join, realpath, dirname

try:
    import oplprobe
except ImportError:
    sys.path.append(
        join(dirname(dirname(dirname(realpath(__file__)))), "scripts"))
    import oplprobe


FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist():
    listnames = ['URLLIST', 'URL-CA', 'URL-MA', 'URL-MI', 'URL-MS', 'URL-TX', 'URL-WA']
    meetlist = []
    for list in listnames:
        with open(FEDDIR + os.sep + list, 'r') as fd:
            meetlist = meetlist + [x.strip() for x in fd.readlines()]
    return meetlist


def main():
    meetlist = getmeetlist()
    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)
    oplprobe.print_meets(color('[USAPL Archive]'), unentered)


if __name__ == '__main__':
    main()
