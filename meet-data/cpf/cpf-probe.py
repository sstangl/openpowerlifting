#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# The list of meets is manually sourced in URLLIST.
# Given that list, determine which meets haven't been entered yet.


import sys
import os


FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist():
    listnames = ['URLLIST']
    meetlist = []
    for list in listnames:
        with open(FEDDIR + os.sep + list, 'r') as fd:
            for line in fd.readlines():
                meet = line.strip()
                if len(meet) == 0 or meet.startswith("#"):
                    continue
                meetlist.append(meet)
    return meetlist


def getenteredurls():
    urls = []
    for dirname, subdirs, files in os.walk(FEDDIR):
        if 'URL' in files:
            with open(dirname + os.sep + 'URL', 'r') as fd:
                for k in fd.readlines():
                    k = k.strip()
                    if k not in urls:
                        urls.append(k)
    return urls


def main():
    summarize = False
    if len(sys.argv) == 2 and sys.argv[1] == '--quick':
        summarize = True

    meetlist = getmeetlist()
    known = getenteredurls()

    count = 0
    for m in meetlist:
        if m not in known:
            count += 1
            if not summarize:
                try:
                    print(color('[CPF] ') + m)
                except BrokenPipeError:
                    return

    if count > 0:
        print(color('[CPF] ') + "%d meets remaining." % count)


if __name__ == '__main__':
    main()
