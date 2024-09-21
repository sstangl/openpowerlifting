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


URL = "http://australia.worldrawpowerlifting.com/competition-rules-results/results/"
BASEURL = "http://australia.worldrawpowerlifting.com/competition-rules-results/results/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;35m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    meet_list = soup.find("div", {"class": "entry-content"})

    urls = []
    for a in meet_list.find_all('a'):
        link = a['href']

        if link.startswith('/'):
            link = BASEURL + link
        urls.append(link)

    return urls


def main():

    meetlist = []
    html = oplprobe.gethtml(URL)
    meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[WRPF-AUS]'), unentered)


if __name__ == '__main__':
    main()
