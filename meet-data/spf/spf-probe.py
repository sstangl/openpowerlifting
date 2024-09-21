#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes the new-style (WordPress) SPF pages for any new meets.
# Because of the calendering system they're using, only the last
# NUMPAGES event pages are checked (10 results per page), so if
# checking isn't done often enough, things might get lost.


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


NUMPAGES = 3  # Number of pages to go back in results, 10 results per page.
FEDDIR = os.path.dirname(os.path.realpath(__file__))
FEDURL = "http://www.southernpowerlifting.com/events/list/?tribe_event_display=past"


def getpageurl(n):
    return FEDURL + "&tribe_paged=" + str(n)


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    events = soup.find_all("div", {"class": "tribe-events-content"})

    urls = []
    for e in events:
        for a in e.find_all('a'):
            # SPF seems to always post result links in text like
            # "View all results from North Alabama Classic".
            # Gating on this cuts down false-positives.
            if 'result' not in a.text.lower():
                continue

            url = a['href']
            if url not in urls:  # Sometimes the entries show up twice.
                urls.append(url)

    return urls


def main():
    meetlist = []

    for i in range(1, NUMPAGES + 1):
        url = getpageurl(i)
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[SPF]'), unentered)


if __name__ == '__main__':
    main()
