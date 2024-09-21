#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the World Power League.

from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["https://wpl.org.ua/category/%d0%bf%d1%80%d0%be%d1%82%d0%be%d0%ba%d0%be"
        "%d0%bb%d1%8b-%d1%81%d0%be%d1%80%d0%b5%d0%b2%d0%bd%d0%be%d0%b2%d0%b0%d0"
        "%bd%d0%b8%d0%b9/"]
BASEURL = "https://wpl.org.ua"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;36m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    content = soup.find_all("div", {"id": "content"})[0]

    urls = []
    for a in content.find_all('a'):
        url = a['href']

        if 'comment-area' in url:
            continue
        if '/author/' in url:
            continue
        if '/category/' in url:
            continue

        if 'http' not in url:
            url = BASEURL + url
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

    oplprobe.print_meets(color('[WPLeague]'), unentered)


if __name__ == '__main__':
    main()
