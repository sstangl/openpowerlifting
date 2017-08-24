#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Library for probes for new meets

from bs4 import BeautifulSoup
import datetime
import os
import shutil
import sys
import urllib.request

FEDDIR = os.path.dirname(os.path.realpath(__file__))

def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)

def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read()

def getenteredurls():
    urls = []
    for dirname, subdirs, files in os.walk(FEDDIR):
        if 'URL' in files:
            with open(dirname + os.sep + 'URL', 'r') as fd:
                for k in fd.readlines():
                    urls.append(k.strip())
    return urls

def main(federation, color, getmeetlist, URL):
    html = gethtml(URL)
    meetlist = getmeetlist(html)

    known = getenteredurls()

    for m in meetlist:
        if not m in known:
            print(color(federation+' ') + m)
    print(color(federation+' ') + "Continue working through archive.")