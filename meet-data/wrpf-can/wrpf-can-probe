#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from WRPF-CAN, all of which are on one page
# as HTML tables.

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
URL = "https://wrpfcanada.com/resultats/"
BASEURL = "https://wrpfcanada.com/resultats/"


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        if 'href=' not in str(a):
            continue

        if '#' not in a['href']:
            continue

        url = a['href']
        if 'https://' not in url:
            url = BASEURL + url

        if url == "https://wrpfcanada.com/resultats/#":
            continue

        if url not in urls:
            urls.append(url)

    return urls


def main():
    html = oplprobe.gethtml(URL)
    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[WPRF-CAN]'), unentered)


if __name__ == '__main__':
    main()
