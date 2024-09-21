#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:

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
URL = "https://canadianpowerliftingleague.ca/competitionresults"
BASEURL = "https://canadianpowerliftingleague.ca/"


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        # All the results have the text "Results" somewhere in the link.
        if 'results' not in a.text.lower():
            continue

        url = a['href']
        if url == '/competitionresults':
            continue

        # The CPL likes to link to OpenPowerlifting for official results.
        if 'openpowerlifting' in url.lower():
            continue

        if url not in urls:
            urls.append(url)

    return urls


def main():
    html = oplprobe.gethtml(URL)
    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[CPL]'), unentered)


if __name__ == '__main__':
    main()
