#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the NSF.


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


URLS = ["https://styrkeloft.no/stevner/?page=protokoller"]
BASEURL = "https://styrkeloft.no/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        url = a['href']

        # Result visualization.
        if 'page=protokoll_vis' not in url:
            continue

        # Easy way to visually see EPF and IPF meets.
        klubb = a.next.next.next.next.text
        url = url + ' (%s)' % klubb

        if 'http' not in url:
            url = BASEURL + url
            url = url.replace('../', '')
        if url not in urls:
            urls.append(url.split()[0])

    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[NSF]'), unentered)


if __name__ == '__main__':
    main()
