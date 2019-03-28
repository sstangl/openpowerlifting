#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new event results from the AAP.

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
FEDURL = "http://aap-powerlifting.com.ar/index.php/resultados-2"
BASEURL = "http://aap-powerlifting.com.ar"


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):

    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        if not a.has_attr('href'):
            continue
        url = a['href']

        if 'Docs' not in url:
            continue

        if BASEURL not in url:
            url = BASEURL + url

        if url not in urls:
            urls.append(url)

    return urls


def main():
    meetlist = []

    html = oplprobe.gethtml(FEDURL)

    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[AAP]'), unentered)


if __name__ == '__main__':
    main()
