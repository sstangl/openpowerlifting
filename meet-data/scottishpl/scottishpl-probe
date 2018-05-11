#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from Scottish Powerlifting.
# They post meets to a page separated by year.

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
URL = "https://www.scottishpowerlifting.com/results/"
BASEURL = "https://www.scottishpowerlifting.com/"


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    articles = soup.find_all('article')
    assert len(articles) == 1

    urls = []
    for a in articles[0].find_all('a'):
        url = a['href']

        if url not in urls:
            urls.append(url)

    return urls


def main():
    html = oplprobe.gethtml(URL)
    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[ScottishPL]'), unentered)


if __name__ == '__main__':
    main()
