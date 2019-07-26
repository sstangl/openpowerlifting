#!/usr/bin/env python3
#
# Calculates the unique folder ID for a given meet.

import zlib


# `date` is a ISO8601 datestring like "2019-01-16".
# `name` is the name of the competition.
def meethash(date, name):
    assert len(date) == 10
    namehash = format(zlib.crc32(bytes(name, 'utf-8')), '08x')
    assert len(namehash) == 8
    return "%s-%s" % (date, namehash)


if __name__ == '__main__':
    import sys

    if len(sys.argv) != 3:
        print(" Usage: %s <MeetDate> <MeetName>" % sys.argv[0])
        sys.exit(1)

    print(meethash(sys.argv[1], sys.argv[2]))
