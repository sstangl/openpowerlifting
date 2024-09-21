#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for meet results from the CPA
# CPA posts all of their meets on a single HTML page. Thanks, CPA!


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


FEDDIR = os.path.dirname(os.path.realpath(__file__))
URLS = [
    "http://www.apa-wpa.com/cpa/index.php?page=resultat"
]
BASEURL = "http://www.apa-wpa.com/cpa/"


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def color(s):
    return "\033[1;31m" + s + "\033[0;m"


def getmeetlist(html):
    urls = []

    # Unfortunately we can't just use the entry_content because
    # BeautifulSoup believes that it doesn't cover all the year tables.
    # So this is the only way to not omit a bunch of results.
    soup = BeautifulSoup(html, 'html.parser')
    for a in soup.find_all('a'):

        # A bunch of the links are just "<a></a>", weirdly.
        try:
            url = a['href']
        except KeyError:
            continue

        if 'mailto:' in url:
            continue

        if 'http://' not in url:
            if url.startswith('/'):
                url = BASEURL + url[1:]
            else:
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

    oplprobe.print_meets(color('[CPA]'), unentered)


if __name__ == '__main__':
    main()
