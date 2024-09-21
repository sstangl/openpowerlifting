#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# IPA posts meets to a different page each year.
#

from bs4 import BeautifulSoup
import datetime
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


# URL needs updating every year.
MEETSURL = "http://www.ipapower.com/?page_id=5243"
if datetime.datetime.now().strftime("%Y") != "2019":
    print("Warning: IPA fetch URL needs updating for new year.", file=sys.stderr)

FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    alist = soup.find_all("a")

    urls = []
    for a in alist:
        if 'results' in a.text.lower():
            urls.append(a['href'])

    return urls


def main():
    html = oplprobe.gethtml(MEETSURL)
    meetlist = getmeetlist(html)

    with open(FEDDIR + os.sep + 'URLLIST') as fd:
        for k in fd.readlines():
            meetlist.append(k.strip())

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[IPA]'), unentered)


if __name__ == '__main__':
    main()
