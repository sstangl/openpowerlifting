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


URLS = ["http://www.wpc-wpo.ru/protocols.html"]
BASEURL = "http://www.wpc-wpo.ru"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    alist = soup.find_all("a")

    urls = []
    for a in alist:
        link = a['href']
        if 'files' not in link:
            continue
        # A bunch of links are just '/files/foo.xls'
        if link.startswith('/'):
            link = BASEURL + link
        urls.append(link)

    return urls


def main():

    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[WPC-RUS]'), unentered)


if __name__ == '__main__':
    main()
