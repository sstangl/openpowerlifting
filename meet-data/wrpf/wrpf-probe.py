#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# WRPF posts results onto a single page for now,
# but they don't post every meet (still missing Boss of Bosses 3).
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


MEETSURL = "http://wrpf.pro/protokol/"
DOMAIN = "http://wrpf.pro"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    content = soup.find("main", {"class": "content"})
    alist = content.find_all("a")

    urls = []
    for a in alist:
        link = a['href']
        if link.startswith('#'):
            continue

        # A bunch of links are just '/files/foo.xls'
        if link.startswith('/'):
            link = DOMAIN + link

        if link not in urls:
            urls.append(link)

    return urls


def main():
    html = oplprobe.gethtml(MEETSURL)
    meetlist = getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[WRPF]'), unentered)


if __name__ == '__main__':
    main()
