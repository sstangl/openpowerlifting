#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes the results page of XPS.


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
FEDURL = "http://hercpowerlifting.com/meet_info.html"
BASEURL = "http://hercpowerlifting.com"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    events = soup.find_all("div", {"id": "section"})

    urls = []
    for e in events:
        for a in e.find_all('a'):
            url = a['href']

            # Hercules Gym malformats their URLs all over the place.
            if not url.startswith('/') and 'herculesgym.net' not in url and \
                    'https://' not in url:
                url = BASEURL + '/' + url
            elif url.startswith('/'):
                url = BASEURL + url

            # Should do this for all the federations, honestly.
            url = url.replace(' ', '%20')

            if url not in urls:  # Sometimes the entries show up twice.
                urls.append(url)

    return urls


def main():
    meetlist = []

    html = oplprobe.gethtml(FEDURL)
    meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[XPS]'), unentered)


if __name__ == '__main__':
    main()
