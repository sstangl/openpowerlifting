#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the NAPF, hosted by USAPL.
# All results from all years are on one page, but
# unfortunately they're all poorly-formatted PDFs.
#


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["http://www.usapowerlifting.com/IPF-NorthAmerica/results.shtml"]
BASEURL = "http://www.usapowerlifting.com/IPF-NorthAmerica/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    tables = soup.find_all("table", {"class": "maintable"})
    if len(tables) == 0:
        error("Page layout seems to have changed.")
    elif len(tables) > 1:
        error("Multiple result areas found.")

    urls = []
    for a in tables[0].find_all('a'):
        # Some URLs are bogus and just JS targets.
        try:
            url = a['href']
        except KeyError:
            continue

        # Simple false-positive detector.
        if '#' in url:
            continue
        if 'index.shtml' in url:
            continue
        if 'addthis.com' in url:
            continue
        if '/gallery/' in url:
            continue
        if '/newsletter/' in url:
            continue

        if 'http' not in url:
            url = BASEURL + url
            url = url.replace('.com//', '/')
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

    oplprobe.print_meets(color('[NAPF]'), unentered)


if __name__ == '__main__':
    main()
