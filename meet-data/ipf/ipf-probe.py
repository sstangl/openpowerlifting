#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the IPF.
# One main page branches to a page per-year, all of which must be checked.
#

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


URLS = ["https://www.powerlifting.sport/championships/results"]
BASEURL = "https://www.powerlifting.sport/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;34m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    sections = soup.find_all("section", {"id": "main_content"})
    if len(sections) == 0:
        error("Page layout seems to have changed.")
    elif len(sections) > 1:
        error("Multiple result areas found.")

    # The main page gives a list of sub-pages, each of which contain
    # results for a year or for a group of years.
    searchurls = []
    for a in sections[0].find_all('a'):
        url = a['href']
        if 'http' not in url:
            url = BASEURL + url
            url = url.replace('.com//', '/')
        if url not in searchurls:
            # Ignore some URLs that are clearly bogus.
            if 'worldpowerlifting.info' in url:
                continue
            searchurls.append(url)

    urls = []

    # Do a lookup for each page.
    for searchurl in searchurls:
        html = oplprobe.gethtml(searchurl)
        soup = BeautifulSoup(html, 'html.parser')

        sections = soup.find_all("section", {"id": "main_content"})
        if len(sections) == 0:
            error("Child page layout seems to have changed.")
        elif len(sections) > 1:
            error("Multiple result areas found.")

        for a in sections[0].find_all('a'):
            url = a['href']

            if 'http' not in url:
                url = BASEURL + url
                url = url.replace('.com//', '/')
            if url not in urls:
                if 'wilks' in url:
                    continue
                elif 'team_point' in url:
                    continue
                elif 'statistics' in url:
                    continue
                elif 'teampoints' in url:
                    continue
                urls.append(url)

    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    entered.update(
        [v.replace("powerlifting-ipf.com", "powerlifting.sport") for v in entered])
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[IPF]'), unentered)


if __name__ == '__main__':
    main()
