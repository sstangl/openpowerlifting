#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new event results from FEPOA.

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
FEDURL = "http://www.fepoablog.com/resultados%20campeonatos%20Argentinos.html"
BASEURL = "http://www.fepoablog.com/"


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):

    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        if not a.has_attr('href'):
            continue
        url = a['href']

        if BASEURL not in url:
            url = BASEURL + url

        if any(x in url for x in ['mailto', 'index.html', 'indumentaria.html']):
            continue

        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []

    html = oplprobe.gethtml(FEDURL)

    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[FEPOA]'), unentered)


if __name__ == '__main__':
    main()
