#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Library for probes for new meets

import os
import shutil
import sys
import urllib.request

def die(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)

def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read()

def getenteredurls(feddir):
    urls = set()
    for dirname, subdirs, files in os.walk(feddir):
        if 'URL' in files:
            with open(dirname + os.sep + 'URL', 'r') as fd:
                for k in fd.readlines():
                    urls.add(k.strip())
    return urls

def getunknownmeets(meetlist, enteredmeets):
    unknown = []
    for m in meetlist:
        if not m in enteredmeets:
            unknown.append(m)
    return unknown
    