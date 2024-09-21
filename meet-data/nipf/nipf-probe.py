#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the Northern Ireland Powerlifting Federation.
# What a nice federation! Results are already in tables!


from bs4 import BeautifulSoup
import sys
import os
from os.path import join, realpath, dirname

try:
    import oplprobe
except ImportError:
    sys.path.append(
        join(dirname(dirname(dirname(realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["http://www.nipfpowerlifting.co.uk/CompetitionList.php"]
BASEURL = "http://www.nipfpowerlifting.co.uk/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    tables = soup.find_all("table", {"class": "compTable"})

    if len(tables) == 0:
        error("Page layout seems to have changed.")
    elif len(tables) > 1:
        error("Multiple result areas found.")

    urls = []
    for a in tables[0].find_all('a'):
        url = a['href']
        if 'http' not in url:
            url = BASEURL + url
        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[NIPF]'), unentered)


if __name__ == '__main__':
    main()
