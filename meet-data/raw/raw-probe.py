#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from the 100%RAW Federation (RAW).
# RAW posts meets to a page separated by year.

from bs4 import BeautifulSoup
import datetime
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


# URL needs updating every year.
# Also be careful to check if there are any result links that don't contain
# the string "results", since they won't get picked up by the script below.
if datetime.datetime.now().strftime("%Y") != "2020":
    print("Warning: RAW fetch URL needs updating for new year.", file=sys.stderr)

FEDDIR = os.path.dirname(os.path.realpath(__file__))
URL = "https://rawpowerlifting.com/2020-results/"
BASEURL = "https://rawpowerlifting.com/2020-results/"


def color(s):
    return "\033[1;30m" + s + "\033[0;m"


def getmeetlist(html):

    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        # All the results have the text "Results" somewhere in the link.
        if 'results' not in a.text.lower():
            continue

        # Also all the results have "uploads" in the URL.
        if '/uploads/' not in a['href']:
            continue

        url = a['href']
        if 'http://' not in url and 'https://' not in url:
            url = BASEURL + url

        if url not in urls:
            urls.append(url)

    return urls


def main():
    html = oplprobe.gethtml(URL)
    meetlist = getmeetlist(html)

    with open(FEDDIR + '/todo/URLLIST') as fd:
        for url in fd:
            url = url.strip()
            if url not in meetlist:
                meetlist.append(url)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[RAW]'), unentered)


if __name__ == '__main__':
    main()
