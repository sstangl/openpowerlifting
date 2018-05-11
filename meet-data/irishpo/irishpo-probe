#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Probes for new meets from the Irish Powerlifting Organization.


from bs4 import BeautifulSoup
import os
import sys

try:
    import oplprobe
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    import oplprobe


URLS = ["http://ipopowerlifting.ie/website/site/index.php/results/"]
BASEURL = "http://ipopowerlifting.ie"
FEDDIR = os.path.dirname(os.path.realpath(__file__))


def color(s):
    return "\033[1;32m" + s + "\033[0;m"


def getmeetlist(html):
    soup = BeautifulSoup(html, 'html.parser')

    urls = []

    # Just look at all links from the entire page.
    for a in soup.find_all('a'):
        # Apparently some <a> tags don't have an href.
        try:
            url = a['href']
        except KeyError:
            continue

        if 'results' not in url:
            continue

        if 'http' not in url:
            url = BASEURL + url
        if url not in urls:
            urls.append(url)

    # Show most recent first.
    urls.reverse()
    return urls


def main():
    meetlist = []
    for url in URLS:
        html = oplprobe.gethtml(url)
        meetlist = meetlist + getmeetlist(html)

    entered = oplprobe.getenteredurls(FEDDIR)
    unentered = oplprobe.getunenteredurls(meetlist, entered)

    oplprobe.print_meets(color('[IrishPO]'), unentered)


if __name__ == '__main__':
    main()
