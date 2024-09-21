#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the FEMEPO


from bs4 import BeautifulSoup
import sys
import os

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["http://www.powerlifting-femepo.com.mx/resultados.html"]
BASEURL = "http://www.powerlifting-femepo.com.mx/"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;33m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []

    div = soup.find("div", {"class": "full"})
    for a in div.find_all('a'):
        url = a['href']

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

    oplprobe.print_meets(color('[FEMEPO]'), unentered)


if __name__ == '__main__':
    main()
