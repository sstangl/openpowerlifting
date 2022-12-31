#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new WRPF-Spain meets.

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URL = "http://wrpfspain.es/resultados/"

FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    div = soup.find("div", {"class": "entry-content"})

    urls = []
    pars = div.find_all("p")

    for par in pars:

        link = par.find('a')

        if link is None:
            continue

        url = link['href'].strip()

        if url not in urls:
            urls.append(url)

    return urls


def main():
    html = oplprobe.gethtml(URL)
    meet_list = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meet_list, entered)

    oplprobe.print_meets(color('[WRPF-Spain]'), unentered)


if __name__ == '__main__':
    main()
