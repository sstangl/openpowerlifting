#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# AAU posts all results to a single page.
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

MEETSURL = "http://aaustrengthsports.org/page.php?page_id=101061"
ROOTURL = "http://www.aaupowerlifting.org"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;35m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    meetul = soup.find("div", {"id": "main-content"})

    urls = []
    for a in meetul.find_all('a'):
        href = a['href']

        # AAU keeps generating invalid URLs, solved by this replace.
        href = href.replace('ing//results', 'ing/results')

        if href.startswith('/'):
            href = ROOTURL + href

        if href not in urls:
            urls.append(href)

    return urls


def main():
    html = oplprobe.gethtml(MEETSURL)
    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)
    oplprobe.print_meets(color('[AAU]'), unentered)


if __name__ == '__main__':
    main()
