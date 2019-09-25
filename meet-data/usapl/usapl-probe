#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# USAPL has unfortunately stopped posting individual meet result spreadsheets,
# and now uploads everything to this usapl.liftingdatabase.com service.
# This script reports whether any new sanction strings showed up.
# (IPF results and super old results are skipped, not having USAPL sanction numbers).
#


from bs4 import BeautifulSoup
import os
import sys
from os.path import join, realpath, dirname

try:
    import oplprobe
except ImportError:
    sys.path.append(
        join(dirname(dirname(dirname(realpath(__file__)))), "scripts"))
    import oplprobe


MEETSURL = "https://usapl.liftingdatabase.com/competitions"
BASEURL = "https://usapl.liftingdatabase.com/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    tables = soup.find_all("table", {"class": "tabledata"})
    if len(tables) > 1:
        error("Too many tables found.")
    elif len(tables) == 0:
        error("No table found.")

    urls = []
    t = tables[0].find('tbody')
    for row in t.find_all('tr'):
        cells = row.find_all('td')
        if len(cells) != 6:
            error("Table row doesn't contain expected number of cells.")

        url = BASEURL + cells[1].find('a')['href']
        if url not in urls:  # Sometimes the entries show up twice.
            urls.append(url)

    return urls


def main():
    html = oplprobe.gethtml(MEETSURL)
    meetlist = getmeetlist(html)
    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[USAPL]'), unentered)


if __name__ == '__main__':
    main()
