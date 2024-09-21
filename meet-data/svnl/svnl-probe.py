#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the SVNL.


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
URLS = ["https://www.suomenvoimanostoliitto.fi/kilpailut/tulosarkisto/"]
BASEURL = "https://suomenvoimanostoliitto.fi"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []
    for div in soup.find_all('div', {'class': 'w-grid-item-h'}):
        url = div.a['href']

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

    oplprobe.print_meets(color('[SVNL]'), unentered)


if __name__ == '__main__':
    main()
