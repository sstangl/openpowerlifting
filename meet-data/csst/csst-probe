#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the CSST.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe

# Local & national results
URLS = ["https://www.powerlifting-csst.cz/cze/results"]
BASEURL = "http://www.powerlifting-csst.cz"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    cldetail = soup.find('div', {'class': 'clanek-detail'})

    urls = []
    for a in cldetail.find_all('a'):
        url = a['href']

        if 'http' not in url:
            url = BASEURL + url
            url = url.replace('../', '')
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

    oplprobe.print_meets(color('[CSST]'), unentered)


if __name__ == '__main__':
    main()
