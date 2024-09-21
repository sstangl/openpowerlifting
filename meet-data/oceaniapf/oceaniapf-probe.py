#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from OceaniaPF, all of which are on one page.

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


FEDDIR = os.path.dirname(os.path.realpath(__file__))
URL = "http://www.oceaniapowerlifting.com/results.htm"
BASEURL = "http://www.oceaniapowerlifting.com/"


def color(s):
    return "\033[1;30m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        # All the results are served from a Results/ subdirectory.
        if 'results' not in a['href'].lower():
            continue

        url = a['href']
        if 'http://' not in url:
            url = BASEURL + url

        if url not in urls:
            urls.append(url)

    return urls


def main():
    html = oplprobe.gethtml(URL)
    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[OceaniaPF]'), unentered)


if __name__ == '__main__':
    main()
