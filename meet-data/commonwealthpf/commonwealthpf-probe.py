#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from the Commonwealth Powerlifting Federation.
# All the results are on a single page, very nice! Meets are very rare.


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

FEDDIR = os.path.dirname(os.path.realpath(__file__))
URLS = ["http://commonwealthpowerlifting.com/results.html"]
BASEURL = "http://commonwealthpowerlifting.com/"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    div = soup.find("div", {"id": "content"})

    urls = []
    for a in div.find_all('a'):
        url = a['href']

        if 'mailto:' in url:
            continue

        if 'http://' not in url:
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

    oplprobe.print_meets(color('[CommonwealthPF]'), unentered)


if __name__ == '__main__':
    main()
