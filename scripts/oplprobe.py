#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Library for common functionality for probe scripts.
#

import os
import shutil
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
    unentered = []
    for m in meetlist:
        if not m in enteredmeets:
            unentered.append(m)
    return unentered

