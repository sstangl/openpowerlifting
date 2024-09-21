#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:


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
FEDURL = "http://upc-germany.com/w0rdpress/?page_id=28"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    content = soup.find("div", {"class": "entry-content"})

    urls = []
    for a in content.find_all('a'):
        url = a['href']
        if url not in urls:  # Sometimes the entries show up twice.
            urls.append(url)

    return urls


def main():
    meetlist = []

    html = oplprobe.gethtml(FEDURL)
    meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[UPC-Germany]'), unentered)


if __name__ == '__main__':
    main()
