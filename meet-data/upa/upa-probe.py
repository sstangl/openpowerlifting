#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new event results from the UPA.
# Because of the content system they're using, only the last
# NUMPAGES event pages are checked (6 results per page), so if
# checking isn't done often enough, things might get lost.
# Uses a similar system as the SPF (WordPress, Tribe).


from bs4 import BeautifulSoup
import os
import sys
import urllib.request

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


# If NUMPAGES is very large, iteration stops at the first 404.
NUMPAGES = 4  # Number of pages to go through, 6 results per page.
FEDDIR = os.path.dirname(os.path.realpath(__file__))
FEDURL = "https://www.upapower.com/category/eventresults/"


def getpageurl(n):
    return FEDURL + "page/" + str(n)


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    events = soup.find_all("h2", {"class": "entry-title"})

    urls = []
    for e in events:
        for a in e.find_all('a'):
            url = a['href']
            if url not in urls:  # Sometimes the entries show up twice.
                urls.append(url)

    return urls


def main():
    meetlist = []

    # Iterate through pages until the site returns a 404.
    for i in range(1, NUMPAGES + 1):
        url = getpageurl(i)
        try:
            html = oplprobe.gethtml(url)
        except urllib.error.HTTPError:
            break
        meets = getmeetlist(html)
        meetlist = meetlist + meets

    with open(FEDDIR + '/todo/URLLIST') as fd:
        for line in fd:
            line = line.strip()
            if len(line) == 0 or line.startswith('#'):
                continue
            meetlist.append(line)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[UPA]'), unentered)


if __name__ == '__main__':
    main()
