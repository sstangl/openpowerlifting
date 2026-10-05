#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# RPS posts meets to a page separated by year.
# Each meet has a distinct URL, which is saved in the repo.
#


import datetime
import os
import sys
from os.path import dirname, join, realpath

from bs4 import BeautifulSoup

try:
    import oplprobe
except ImportError:
    sys.path.append(
        join(dirname(dirname(dirname(realpath(__file__)))), "scripts"))
    import oplprobe


# URL needs updating every year.
MEETSURLS = [
    "http://meets.revolutionpowerlifting.com/results/2025-meet-results/",
    "http://meets.revolutionpowerlifting.com/results/2026-meet-results/",
]
if datetime.datetime.now(datetime.timezone.utc).strftime("%Y") != "2026":
    print("Warning: RPS fetch URL needs updating for new year.", file=sys.stderr)
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    meetul = soup.find("ul", {"class": "display-pages-listing"})

    urls = []
    for a in meetul.find_all('a'):
        urls.append(a['href'])

    return urls


def main():
    entered = oplprobe.getenteredurls(FEDDIR)

    unentered = []
    for url in MEETSURLS:
        html = oplprobe.gethtml(url)
        meetlist = getmeetlist(html)
        unentered += oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[RPS]'), unentered)


if __name__ == '__main__':
    main()
