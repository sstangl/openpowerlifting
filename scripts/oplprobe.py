#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Library for common functionality for probe scripts.
#

import os
import sys
import urllib.request


def die(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def gethtml(url):
    request = urllib.request.Request(url)
    request.add_header('User-Agent', 'Mozilla/5.0 Gecko/20100101 Firefox/52.0')

    with urllib.request.urlopen(request) as r:
        return r.read()


def getenteredurls(feddir):
    urls = set()
    for dirname, subdirs, files in os.walk(feddir):
        if 'URL' in files:
            with open(dirname + os.sep + 'URL', 'r') as fd:
                for k in fd.readlines():
                    urls.add(k.strip())
    return urls


def getunenteredurls(meetlist, enteredmeets):
    # Calculate some variants of the entered meets.
    variants = set()
    for k in enteredmeets:
        if 'https://' in k:
            variants.add(k.replace("https://", "http://"))
        if 'http://' in k:
            variants.add(k.replace("http://", "https://"))

        if "%20" in k:
            variants.add(k.replace("%20", " "))
        if " " in k:
            variants.add(k.replace(" ", "%20"))

    enteredmeets = enteredmeets.union(variants)

    unentered = []
    for m in meetlist:
        if m not in enteredmeets:
            unentered.append(m)
    return unentered


# Given a federation string and a list of unentered meets, print to stdout.
def print_meets(fedstr, meetlist):
    count = len(meetlist)

    # If a quick mode was requested, just show the newest meets.
    if '--quick' in sys.argv:
        meetlist = meetlist[0:5]

    try:
        for url in meetlist:
            print("%s %s" % (fedstr, url.replace(' ', '%20')))

        if count > 3:
            print("%s %d meets remaining." % (fedstr, count))
    except BrokenPipeError:
        pass
