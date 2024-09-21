#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meets from FECAPOLIF.
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


MEETSURL = "http://www.fecapolif.cm/documentatiion.html"
FEDDIR = os.path.dirname(os.path.realpath(__file__))
BASEURL = "http://www.fecapolif.cm/"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;34m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for a in soup.find_all('a'):
        try:
            href = a["href"]
            # All results are under wa_files/.
            if "wa_files" in href:
                if href.startswith('wa_files'):
                    href = BASEURL + href
                if href not in urls:
                    urls.append(href)
        except KeyError:
            pass

    return urls


def main():
    meetlist = []

    html = oplprobe.gethtml(MEETSURL)
    meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[FECAPOLIF]'), unentered)


if __name__ == '__main__':
    main()
