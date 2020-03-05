#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes the FQD (Quebec) provincial database for new meets.
# The FQD is part of the CPU, but maintains their own database and in practice
# fails to upload meets to the national CPU database.

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
FEDURL = "http://www.fqd-quebec.com/meets"
FEDPREFIX = "http://www.fqd-quebec.com"


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all("a"):
        url = a['href']
        if '/meet/' not in url:
            continue

        # URLs are always given relatively.
        if 'http' not in url:
            url = FEDPREFIX + url

        # URLs have a ";jsessionid=" prefix.
        url = url.split(';')[0]

        if url not in urls:
            urls.append(url)

    return urls


def main():
    html = oplprobe.gethtml(FEDURL)
    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[CPU]') + ' (FQD)', unentered)


if __name__ == '__main__':
    main()
