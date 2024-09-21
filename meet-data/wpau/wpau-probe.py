#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the WPAU
#

from bs4 import BeautifulSoup
import sys
import os
import datetime
from os.path import join, realpath, dirname

try:
    import oplprobe
except ImportError:
    sys.path.append(
        join(dirname(dirname(dirname(realpath(__file__)))), "scripts"))
    import oplprobe

RESULTURL = "http://wpa-ukraine.com/rezultaty/"
BASEURL = "http://wpa-ukraine.com/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))

if datetime.datetime.now().strftime("%Y") != "2019":
    print("Warning: WPAU url needs updating for new year.", file=sys.stderr)

MINYEAR = 2011
MAXYEAR = 2021


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    div = soup.find("div", {"class": "art-PostContent"})
    urls = []
    for a in div.find_all('a'):
        url = a['href']

        if 'http' not in url:
            url = BASEURL + url
            url = url.replace('.com//', '/')
        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []

    for year in range(MINYEAR, MAXYEAR):
        html = oplprobe.gethtml(RESULTURL+str(year)+'-2/')
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[WPAU]'), unentered)


if __name__ == '__main__':
    main()
