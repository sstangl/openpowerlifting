#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new FPO meets from the official calendar.

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URL = "http://fpopowerlifting.net/kilpailukalenteri/"
BASEURL = "http://fpopowerlifting.net"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;35m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    tables = soup.find_all("table", {"class": "tablepress"})
    assert tables

    urls = []

    for table in tables:
        for a in table.find_all('a'):
            if "tulokset" not in a.text.lower():  # ignore future and intl meets.
                continue
            url = a['href']
            if 'http' not in url:
                url = BASEURL + '/' + url
            if url not in urls:
                urls.append(url.strip())

    return urls


def main():
    html = oplprobe.gethtml(URL)
    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[FPO]'), unentered)


if __name__ == '__main__':
    main()
